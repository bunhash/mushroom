//! Map Cursor
//!
//! Used to navigate the map. This is to abstract the internals so no undefined behavior can occur.

use crate::{
    map::{Cursor, Error, MapNode, Metadata, SizeHint},
    types::{WzInt, WzString},
};
use indextree::{Arena, NodeId};
use std::collections::VecDeque;

/// A cursor with mutable access to the contents of the [`Map`](crate::map::Map)
pub struct CursorMut<'a, M, T>
where
    M: Metadata<T>,
    T: SizeHint,
{
    pub(crate) position: NodeId,
    arena: &'a mut Arena<MapNode<T>>,
    clipboard: Option<NodeId>,
}

impl<'a, M, T> CursorMut<'a, M, T>
where
    M: Metadata<T>,
    T: SizeHint,
{
    pub(crate) fn new(position: NodeId, arena: &'a mut Arena<MapNode<M, T>>) -> Self {
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
                    .name
                    .as_ref(),
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
                    .name
                    .as_ref()
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
        &self
            .arena
            .get(self.position)
            .expect("get() node should exist")
            .get()
            .name
            .as_ref()
    }

    /// Renames the node at the current position
    pub fn rename(&mut self, name: WzString) {
        self.arena
            .get_mut(self.position)
            .expect("get() node should exist")
            .get_mut()
            .name = name;
    }

    /// Returns the data at the current position
    pub fn get(&self) -> &T {
        &self
            .arena
            .get(self.position)
            .expect("get() node should exist")
            .get()
            .data
    }

    /// Returns the mutable data at the current position
    pub fn get_mut(&mut self) -> &mut T {
        &mut self
            .arena
            .get_mut(self.position)
            .expect("get_mut() node should exist")
            .get_mut()
            .data
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

    /// Creates a new child at the current position. Errors when a child with the provided name
    /// already exists.
    pub fn create(&mut self, name: WzString, data: T) -> Result<&mut Self, Error> {
        if self.list().contains(&name.as_ref()) {
            Err(Error::DuplicateError(name.to_string()))
        } else {
            let node = self.arena.new_node(MapNode::new(name, data));
            self.position.append(node, &mut self.arena);
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
        Ok(self)
    }

    /// Pastes the contents of the clipboard at the current position. Errors when the clipboard is
    /// empty.
    pub fn paste(&mut self) -> Result<&mut Self, Error> {
        match self.clipboard {
            Some(id) => {
                self.position.append(id, &mut self.arena);
                self.clipboard = None;
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
        Ok(self)
    }

    // *** PRIVATES *** //

    fn get_id(&self, name: &str) -> Result<NodeId, Error> {
        match self
            .position
            .children(self.arena)
            .filter(|id| {
                self.arena
                    .get(*id)
                    .expect("get_id() node should exist")
                    .get()
                    .name
                    .as_ref()
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

    use crate::{map::Map, types::WzString};

    #[test]
    fn add_nodes() {
        let mut map = Map::new(WzString::from("n1"), 100);
        let mut cursor = map.cursor_mut();
        cursor
            .create(WzString::from("n1_1"), 150)
            .expect("error creating n1_1")
            .create(WzString::from("n1_2"), 3500)
            .expect("error creating n1_2");
        assert!(
            cursor.create(WzString::from("n1_2"), 0).is_err(),
            "should have errored here"
        );
        assert_eq!(&cursor.list(), &["n1_1", "n1_2"]);
    }

    #[test]
    fn remove_node() {
        let mut map = Map::new(WzString::from("n1"), 100);
        let mut cursor = map.cursor_mut();
        cursor
            .create(WzString::from("n1_1"), 150)
            .expect("error creating n1_1")
            .create(WzString::from("n1_2"), 3500)
            .expect("error creating n1_2")
            .delete("n1_1")
            .expect("should have deleted n1_1");
        assert!(cursor.delete("n1_1").is_err(), "should have errored");
        assert_eq!(&cursor.list(), &["n1_2"]);
    }

    #[test]
    fn remove_subtree() {
        let mut map = Map::new(WzString::from("n1"), 100);
        let mut cursor = map.cursor_mut();
        cursor
            .create(WzString::from("n1_1"), 150)
            .expect("error creating n1_1")
            .create(WzString::from("n1_2"), 3500)
            .expect("error creating n1_2")
            .move_to("n1_1")
            .expect("error moving into n1_1")
            .create(WzString::from("n1_2_1"), 50)
            .expect("error creating n1_2_1")
            .parent()
            .expect("error moving back to n1")
            .delete("n1_1")
            .expect("should have deleted n1_1");
        assert_eq!(&cursor.list(), &["n1_2"]);
    }

    #[test]
    fn move_node() {
        let mut map = Map::new(WzString::from("n1"), 100);
        let mut cursor = map.cursor_mut();
        cursor
            .create(WzString::from("n1_1"), 150)
            .expect("error creating n1_1")
            .create(WzString::from("n1_2"), 3500)
            .expect("error creating n1_2")
            .move_to("n1_1")
            .expect("error moving into n1_1")
            .create(WzString::from("n1_2_1"), 50)
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
        let mut map = Map::new(WzString::from("n1"), 100);
        let mut cursor = map.cursor_mut();
        cursor
            .create(WzString::from("n1_1"), 150)
            .expect("error creating n1_1")
            .move_to("n1_1")
            .expect("error moving into n1_1")
            .create(WzString::from("n1_1_1"), 155)
            .expect("error creating n1_1_1")
            .create(WzString::from("n1_1_2"), 175)
            .expect("error creating n1_1_1")
            .move_to("n1_1_1")
            .expect("error moving into n1_1_1")
            .create(WzString::from("n1_1_1_1"), 255)
            .expect("error creating n1_1_1_1")
            .move_to("n1_1_1_1")
            .expect("error moving into n1_1_1_1");
        assert_eq!(&cursor.pwd(), &["n1", "n1_1", "n1_1_1", "n1_1_1_1"]);
    }
}
