//! Map Cursor
//!
//! Used to navigate the map. This is to abstract the internals so no undefined behavior can occur.

use crate::{
    error::MapError,
    map::{ChildNames, Children, Cursor, MapNode},
};
use indextree::{Arena, DebugPrettyPrint, NodeId};
use std::{collections::VecDeque, fmt::Debug};

/// A cursor with mutable access to the contents of the [`Map`](crate::map::Map)
#[derive(Debug)]
pub struct CursorMut<'a, T> {
    pub(crate) position: NodeId,
    arena: &'a mut Arena<MapNode<T>>,
    clipboard: Option<NodeId>,
}

impl<'a, T> CursorMut<'a, T> {
    pub(crate) fn new(position: NodeId, arena: &'a mut Arena<MapNode<T>>) -> Self {
        Self {
            position,
            arena,
            clipboard: None,
        }
    }

    /// Returns the path of the current position as a vector of names starting with the root
    pub fn pwd(&'a self) -> String {
        let mut path = VecDeque::new();
        for id in self.position.ancestors(self.arena) {
            path.push_front(
                self.arena
                    .get(id)
                    .expect("pwd() node should exist")
                    .get()
                    .name
                    .as_str(),
            );
        }
        path.make_contiguous().join("/")
    }

    /// Returns a vector containing the names of the current position's children
    pub fn list(&'a self) -> ChildNames<'a, T> {
        ChildNames::new(self.position, self.arena)
    }

    /// Returns a vector containing the children of the current position
    pub fn children(&'a self) -> Children<'a, T> {
        Children::new(self.position, self.arena)
    }

    /// Returns true if the child exists.
    pub fn has_child(&self, name: &str) -> bool {
        self.get_id(self.position, name).is_ok()
    }

    /// Returns the name of the current position
    pub fn name(&'a self) -> &'a str {
        self.arena
            .get(self.position)
            .expect("get() node should exist")
            .get()
            .name
            .as_str()
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

    /// Moves the cursor to the child with the given name. Errors when the child does not exist.
    pub fn move_to(&mut self, name: &str) -> Result<&mut Self, MapError> {
        self.position = self.get_id(self.position, name)?;
        Ok(self)
    }

    /// Moves the cursor to the first child.
    pub fn first_child(&mut self) -> Result<&mut Self, MapError> {
        let id = self
            .arena
            .get(self.position)
            .expect("current node should exist")
            .first_child()
            .ok_or(MapError::NoChildren)?;
        self.position = id;
        Ok(self)
    }

    /// Moves the cursor to the last child.
    pub fn last_child(&mut self) -> Result<&mut Self, MapError> {
        let id = self
            .arena
            .get(self.position)
            .expect("current node should exist")
            .last_child()
            .ok_or(MapError::NoChildren)?;
        self.position = id;
        Ok(self)
    }

    /// Moves the cursor to the previous sibling node
    pub fn previous_sibling(&mut self) -> Result<&mut Self, MapError> {
        let id = self
            .arena
            .get(self.position)
            .expect("current node should exist")
            .previous_sibling()
            .ok_or(MapError::NoChildren)?;
        self.position = id;
        Ok(self)
    }

    /// Moves the cursor to the next sibling node
    pub fn next_sibling(&mut self) -> Result<&mut Self, MapError> {
        let id = self
            .arena
            .get(self.position)
            .expect("current node should exist")
            .next_sibling()
            .ok_or(MapError::NoChildren)?;
        self.position = id;
        Ok(self)
    }

    /// Moves the cursor to the parent. Errors when already at the root node.
    pub fn parent(&mut self) -> Result<&mut Self, MapError> {
        let id = self
            .arena
            .get(self.position)
            .expect("current node should exist")
            .parent()
            .ok_or(MapError::NoParent)?;
        self.position = id;
        Ok(self)
    }

    /// Walks the map depth-first
    pub fn walk<E>(&self, mut closure: impl FnMut(Cursor<T>) -> Result<(), E>) -> Result<(), E>
    where
        E: Debug,
    {
        for id in self.position.descendants(self.arena) {
            closure(Cursor::new(id, self.arena))?;
        }
        Ok(())
    }

    /// Creates a printable string of the tree structure. To be used in `{:?}` formatting.
    pub fn debug_pretty_print(&'a self) -> DebugPrettyPrint<'a, MapNode<T>> {
        self.position.debug_pretty_print(self.arena)
    }

    // *** Mutable Functions *** //

    /// Renames the node at the current position. Errors when a child with the new name already
    /// exists.
    pub fn rename(&mut self, name: String) -> Result<&mut Self, MapError> {
        if self.has_child(name.as_str()) {
            Err(MapError::Duplicate(name))
        } else {
            self.arena
                .get_mut(self.position)
                .expect("current position should exist")
                .get_mut()
                .name = name;
            Ok(self)
        }
    }

    /// Returns the mutable data at the current position
    pub fn get_mut(&mut self) -> &mut T {
        &mut self
            .arena
            .get_mut(self.position)
            .expect("get() node should exist")
            .get_mut()
            .data
    }

    /// Creates a new child at the current position. Errors when a child with the provided name
    /// already exists.
    pub fn create(&mut self, name: String, data: T) -> Result<&mut Self, MapError> {
        if self.has_child(name.as_str()) {
            Err(MapError::Duplicate(name))
        } else {
            let node = self.arena.new_node(MapNode::new(name, data));
            self.position.append(node, self.arena);
            Ok(self)
        }
    }

    /// Detaches the child with the given name at the current position. This function adds that
    /// child to a clipboard. If the clipboard already contains a node previously cut, that node
    /// will be purged from the map. Errors when the child does not exist. If an error occurs, the
    /// clipboard will remain unchanged.
    pub fn cut(&mut self, name: &str) -> Result<&mut Self, MapError> {
        let id = self.get_id(self.position, name)?;
        id.detach(self.arena);
        if let Some(to_delete) = self.clipboard {
            to_delete.remove_subtree(self.arena);
        }
        self.clipboard = Some(id);
        Ok(self)
    }

    /// Pastes the contents of the clipboard at the current position. Errors when the clipboard is
    /// empty or another node with the same name exists.
    pub fn paste(&mut self) -> Result<&mut Self, MapError> {
        let id = self.clipboard.ok_or(MapError::ClipboardEmpty)?;
        let name = self
            .arena
            .get(id)
            .expect("id should exist")
            .get()
            .name
            .as_str();
        if self.get_id(self.position, name).is_ok() {
            return Err(MapError::Duplicate(name.to_string()));
        }
        self.position.append(id, self.arena);
        self.clipboard = None;
        Ok(self)
    }

    /// Deletes the child with the given name and all of its contents. Errors when the child with
    /// the provided name does not exist.
    pub fn delete(&mut self, name: &str) -> Result<&mut Self, MapError> {
        let id = self.get_id(self.position, name)?;
        id.remove_subtree(self.arena);
        Ok(self)
    }

    // *** PRIVATES *** //

    fn get_id(&self, position: NodeId, name: &str) -> Result<NodeId, MapError> {
        position
            .children(self.arena)
            .find(|id| {
                self.arena
                    .get(*id)
                    .expect("child position should exist")
                    .get()
                    .name
                    .as_str()
                    == name
            })
            .ok_or_else(|| MapError::NotFound(String::from(name)))
    }
}

#[cfg(test)]
mod tests {

    use crate::{error::MapError, map::Map};

    #[test]
    fn add_nodes() {
        let mut map = Map::new(String::from("n1"), 100);
        let mut cursor = map.cursor_mut();
        cursor
            .create(String::from("n1_1"), 150)
            .expect("error creating n1_1")
            .create(String::from("n1_2"), 3500)
            .expect("error creating n1_2");
        match cursor.create(String::from("n1_2"), 0) {
            Err(MapError::Duplicate(_)) => {}
            _ => panic!("should have failed with MapError::Duplicate"),
        }
        assert_eq!(&cursor.list().collect::<Vec<&str>>(), &["n1_1", "n1_2"]);
    }

    #[test]
    fn remove_node() {
        let mut map = Map::new(String::from("n1"), 100);
        let mut cursor = map.cursor_mut();
        cursor
            .create(String::from("n1_1"), 150)
            .expect("error creating n1_1")
            .create(String::from("n1_2"), 3500)
            .expect("error creating n1_2")
            .delete("n1_1")
            .expect("should have deleted n1_1");
        match cursor.delete("n1_1") {
            Err(MapError::NotFound(_)) => {}
            r => panic!("expected MapError::NotFound, found {:?}", r),
        }
        assert_eq!(&cursor.list().collect::<Vec<&str>>(), &["n1_2"]);
    }

    #[test]
    fn remove_subtree() {
        let mut map = Map::new(String::from("n1"), 100);
        let mut cursor = map.cursor_mut();
        cursor
            .create(String::from("n1_1"), 150)
            .expect("error creating n1_1")
            .create(String::from("n1_2"), 3500)
            .expect("error creating n1_2")
            .move_to("n1_1")
            .expect("error moving into n1_1")
            .create(String::from("n1_2_1"), 50)
            .expect("error creating n1_2_1")
            .parent()
            .expect("error moving back to n1")
            .delete("n1_1")
            .expect("should have deleted n1_1");
        assert_eq!(&cursor.list().collect::<Vec<&str>>(), &["n1_2"]);
    }

    #[test]
    fn move_node() {
        let mut map = Map::new(String::from("n1"), 100);
        let mut cursor = map.cursor_mut();
        cursor
            .create(String::from("n1_1"), 150)
            .expect("error creating n1_1")
            .create(String::from("n1_2"), 3500)
            .expect("error creating n1_2")
            .move_to("n1_1")
            .expect("error moving into n1_1")
            .create(String::from("n1_2_1"), 50)
            .expect("error creating n1_2_1")
            .parent()
            .expect("error moving back to n1")
            .cut("n1_1")
            .expect("should have cut n1_1")
            .move_to("n1_2")
            .expect("error moving to n1_2")
            .paste()
            .expect("should paste n1_1");
        assert_eq!(&cursor.list().collect::<Vec<&str>>(), &["n1_1"]);
        match cursor.paste() {
            Err(MapError::ClipboardEmpty) => {}
            r => panic!("expected MapError::ClipboardEmpty, found {:?}", r),
        }
        cursor
            .parent()
            .expect("error moving to n1")
            .create(String::from("n1_1"), 0)
            .expect("error recreating n1_1")
            .cut("n1_1")
            .expect("error cutting new n1_1")
            .move_to("n1_2")
            .expect("error moving to n1_2");
        match cursor.paste() {
            Err(MapError::Duplicate(_)) => {}
            r => panic!("expected MapError::Duplicate, found {:?}", r),
        }
    }

    #[test]
    fn pwd() {
        let mut map = Map::new(String::from("n1"), 100);
        let mut cursor = map.cursor_mut();
        cursor
            .create(String::from("n1_1"), 150)
            .expect("error creating n1_1")
            .move_to("n1_1")
            .expect("error moving into n1_1")
            .create(String::from("n1_1_1"), 155)
            .expect("error creating n1_1_1")
            .create(String::from("n1_1_2"), 175)
            .expect("error creating n1_1_1")
            .move_to("n1_1_1")
            .expect("error moving into n1_1_1")
            .create(String::from("n1_1_1_1"), 255)
            .expect("error creating n1_1_1_1")
            .move_to("n1_1_1_1")
            .expect("error moving into n1_1_1_1");
        assert_eq!(&cursor.pwd(), "n1/n1_1/n1_1_1/n1_1_1_1");
    }

    #[test]
    fn modify_node() {
        let mut map = Map::new(String::from("n1"), 100);
        let mut cursor = map.cursor_mut();
        cursor
            .create(String::from("n1_1"), 150)
            .expect("error creating n1_1")
            .move_to("n1_1")
            .expect("error moving into n1_1")
            .create(String::from("n1_1_1"), 155)
            .expect("error creating n1_1_1")
            .create(String::from("n1_1_2"), 175)
            .expect("error creating n1_1_1")
            .move_to("n1_1_1")
            .expect("error moving into n1_1_1")
            .create(String::from("n1_1_1_1"), 255)
            .expect("error creating n1_1_1_1")
            .move_to("n1_1_1_1")
            .expect("error moving into n1_1_1_1");
        let mut cursor = map.cursor_mut();
        cursor.move_to("n1_1").expect("n1_1 should exist");
        assert_eq!(*cursor.get(), 150);
        *cursor.get_mut() = 100;
        assert_eq!(*cursor.get(), 100);
    }
}
