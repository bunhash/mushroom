//! Map Cursor
//!
//! Used to navigate the map. This is to abstract the internals so no undefined behavior can occur.

use crate::{
    map::{Error, Metadata, SizeHint},
    types::{WzInt, WzString},
};
use indextree::{Arena, NodeId};
use std::collections::VecDeque;

/// A cursor with mutable access to the contents of the [`Map`](crate::map::Map)
pub struct CursorMut<'a, T>
where
    T: Metadata + SizeHint,
{
    pub(crate) position: NodeId,
    arena: &'a mut Arena<T>,
    clipboard: Option<NodeId>,
}

impl<'a, T> CursorMut<'a, T>
where
    T: Metadata + SizeHint,
{
    pub(crate) fn new(position: NodeId, arena: &'a mut Arena<T>) -> Self {
        Self {
            position,
            arena,
            clipboard: None,
        }
    }

    /// Returns the path of the current position as a vector of names starting with the root
    pub fn pwd(&'a self) -> Vec<&'a str> {
        let mut path = VecDeque::new();
        for id in self.position.ancestors(&self.arena) {
            path.push_front(
                self.arena
                    .get(id)
                    .expect("pwd() node should exist")
                    .get()
                    .name(),
            );
        }
        path.into()
    }

    /// Returns a vector containing the names of the current position's children
    pub fn list(&'a self) -> Vec<&'a str> {
        self.position
            .children(self.arena)
            .map(|id| {
                self.arena
                    .get(id)
                    .expect("list() node should exist")
                    .get()
                    .name()
            })
            .collect::<Vec<&'a str>>()
    }

    /// Returns true if the child exists. This is slightly more efficient than doing
    /// list().contains().
    pub fn has_child(&self, name: &str) -> bool {
        self.get_id(name).is_ok()
    }

    /// Returns the name of the current position
    pub fn name(&'a self) -> &'a str {
        self.arena
            .get(self.position)
            .expect("get() node should exist")
            .get()
            .name()
    }

    /// Returns the data at the current position
    pub fn get(&self) -> &T {
        self.arena
            .get(self.position)
            .expect("get() node should exist")
            .get()
    }

    /// Moves the cursor to the child with the given name. Errors when the child does not exist.
    pub fn move_to(&mut self, name: &str) -> Result<&mut Self, Error> {
        let id = self.get_id(name)?;
        self.position = id;
        Ok(self)
    }

    /// Moves the cursor to the parent. Errors when already at the root node.
    pub fn parent(&mut self) -> Result<&mut Self, Error> {
        match self
            .arena
            .get(self.position)
            .expect("current position should exist")
            .parent()
        {
            Some(id) => {
                self.position = id;
                Ok(self)
            }
            None => Err(Error::AlreadyRoot),
        }
    }

    // *** Mutable Functions *** //

    /// Renames the node at the current position
    pub fn rename(&mut self, name: WzString) {
        self.arena
            .get_mut(self.position)
            .expect("current position should exist")
            .get_mut()
            .rename(name);
        self.send_update();
    }

    /// Modifies the data at the current position
    pub fn modify(&mut self, closure: impl Fn(&mut T) -> ()) {
        let data = self
            .arena
            .get_mut(self.position)
            .expect("current position should exist")
            .get_mut();
        closure(data);
        self.send_update();
    }

    /// Creates a new child at the current position. Errors when a child with the provided name
    /// already exists.
    pub fn create(&mut self, data: T) -> Result<&mut Self, Error> {
        if self.has_child(data.name()) {
            Err(Error::DuplicateError(data.name().to_string()))
        } else {
            let node = self.arena.new_node(data);
            self.position.append(node, &mut self.arena);
            self.send_update();
            Ok(self)
        }
    }

    /// Detaches the child with the given name at the current position. This function adds that
    /// child to a clipboard. If the clipboard already contains a node previously cut, that node
    /// will be purged from the map. Errors when the child does not exist. If an error occurs, the
    /// clipboard will remain unchanged.
    pub fn cut(&mut self, name: &str) -> Result<&mut Self, Error> {
        let id = self.get_id(name)?;
        id.detach(&mut self.arena);
        if let Some(to_delete) = self.clipboard {
            to_delete.remove_subtree(&mut self.arena);
        }
        self.clipboard = Some(id);
        self.send_update();
        Ok(self)
    }

    /// Pastes the contents of the clipboard at the current position. Errors when the clipboard is
    /// empty.
    pub fn paste(&mut self) -> Result<&mut Self, Error> {
        match self.clipboard {
            Some(id) => {
                self.position.append(id, &mut self.arena);
                self.clipboard = None;
                self.send_update();
                Ok(self)
            }
            None => Err(Error::ClipboardEmpty),
        }
    }

    /// Deletes the child with the given name and all of its contents. Errors when the child with
    /// the provided name does not exist.
    pub fn delete(&mut self, name: &str) -> Result<&mut Self, Error> {
        let id = self.get_id(name)?;
        id.remove_subtree(&mut self.arena);
        self.send_update();
        Ok(self)
    }

    /// Sends an update to the current node and all its ancestors. This calls Metadata::update()
    /// internally so each node can update their metadata. This function should only be called if
    /// the internal data was modified outside of [`CursorMut`] by the
    /// [`get_mut`](CursorMut::get_mut) function.
    pub fn send_update(&mut self) {
        let to_update = self
            .position
            .ancestors(&self.arena)
            .collect::<Vec<NodeId>>();
        for id in to_update {
            self.update(id);
        }
    }

    // *** PRIVATES *** //

    fn update(&mut self, position: NodeId) {
        let children_sizes = position
            .children(&self.arena)
            .map(|id| {
                self.arena
                    .get(id)
                    .expect("child should exist")
                    .get()
                    .size_hint()
            })
            .collect::<Vec<WzInt>>();
        let current = &mut self
            .arena
            .get_mut(position)
            .expect("current node should exist")
            .get_mut();
        current.update(&children_sizes);
    }

    fn get_id(&self, name: &str) -> Result<NodeId, Error> {
        match self
            .position
            .children(self.arena)
            .filter(|id| {
                self.arena
                    .get(*id)
                    .expect("get_id() node should exist")
                    .get()
                    .name()
                    == name
            })
            .next()
        {
            Some(id) => Ok(id),
            None => Err(Error::NotFound(String::from(name))),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        map::{Map, Metadata, SizeHint},
        types::{WzInt, WzString},
    };

    #[derive(Debug)]
    struct SimpleNode(WzString, i32);

    impl Metadata for SimpleNode {
        fn name(&self) -> &str {
            self.0.as_ref()
        }

        fn rename(&mut self, name: WzString) {
            self.0 = name;
        }

        fn update(&mut self, _: &[WzInt]) {}
    }

    impl SizeHint for SimpleNode {
        fn size_hint(&self) -> WzInt {
            self.0.size_hint() + self.1.size_hint()
        }
    }

    #[test]
    fn add_nodes() {
        let mut map = Map::new(SimpleNode(WzString::from("n1"), 100));
        let mut cursor = map.cursor_mut();
        cursor
            .create(SimpleNode(WzString::from("n1_1"), 150))
            .expect("error creating n1_1")
            .create(SimpleNode(WzString::from("n1_2"), 3500))
            .expect("error creating n1_2");
        assert!(
            cursor
                .create(SimpleNode(WzString::from("n1_2"), 0))
                .is_err(),
            "should have errored here"
        );
        assert_eq!(&cursor.list(), &["n1_1", "n1_2"]);
    }

    #[test]
    fn remove_node() {
        let mut map = Map::new(SimpleNode(WzString::from("n1"), 100));
        let mut cursor = map.cursor_mut();
        cursor
            .create(SimpleNode(WzString::from("n1_1"), 150))
            .expect("error creating n1_1")
            .create(SimpleNode(WzString::from("n1_2"), 3500))
            .expect("error creating n1_2")
            .delete("n1_1")
            .expect("should have deleted n1_1");
        assert!(cursor.delete("n1_1").is_err(), "should have errored");
        assert_eq!(&cursor.list(), &["n1_2"]);
    }

    #[test]
    fn remove_subtree() {
        let mut map = Map::new(SimpleNode(WzString::from("n1"), 100));
        let mut cursor = map.cursor_mut();
        cursor
            .create(SimpleNode(WzString::from("n1_1"), 150))
            .expect("error creating n1_1")
            .create(SimpleNode(WzString::from("n1_2"), 3500))
            .expect("error creating n1_2")
            .move_to("n1_1")
            .expect("error moving into n1_1")
            .create(SimpleNode(WzString::from("n1_2_1"), 50))
            .expect("error creating n1_2_1")
            .parent()
            .expect("error moving back to n1")
            .delete("n1_1")
            .expect("should have deleted n1_1");
        assert_eq!(&cursor.list(), &["n1_2"]);
    }

    #[test]
    fn move_node() {
        let mut map = Map::new(SimpleNode(WzString::from("n1"), 100));
        let mut cursor = map.cursor_mut();
        cursor
            .create(SimpleNode(WzString::from("n1_1"), 150))
            .expect("error creating n1_1")
            .create(SimpleNode(WzString::from("n1_2"), 3500))
            .expect("error creating n1_2")
            .move_to("n1_1")
            .expect("error moving into n1_1")
            .create(SimpleNode(WzString::from("n1_2_1"), 50))
            .expect("error creating n1_2_1")
            .parent()
            .expect("error moving back to n1")
            .cut("n1_1")
            .expect("should have cut n1_1")
            .move_to("n1_2")
            .expect("error moving to n1_2")
            .paste()
            .expect("should paste n1_1");
        assert_eq!(&cursor.list(), &["n1_1"]);
    }

    #[test]
    fn pwd() {
        let mut map = Map::new(SimpleNode(WzString::from("n1"), 100));
        let mut cursor = map.cursor_mut();
        cursor
            .create(SimpleNode(WzString::from("n1_1"), 150))
            .expect("error creating n1_1")
            .move_to("n1_1")
            .expect("error moving into n1_1")
            .create(SimpleNode(WzString::from("n1_1_1"), 155))
            .expect("error creating n1_1_1")
            .create(SimpleNode(WzString::from("n1_1_2"), 175))
            .expect("error creating n1_1_1")
            .move_to("n1_1_1")
            .expect("error moving into n1_1_1")
            .create(SimpleNode(WzString::from("n1_1_1_1"), 255))
            .expect("error creating n1_1_1_1")
            .move_to("n1_1_1_1")
            .expect("error moving into n1_1_1_1");
        assert_eq!(&cursor.pwd(), &["n1", "n1_1", "n1_1_1", "n1_1_1_1"]);
    }
}
