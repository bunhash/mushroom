//! Map Cursor
//!
//! Used to navigate the map. This is to abstract the internals so no undefined behavior can occur.

use crate::{
    error::MapError,
    map::{ChildNames, Children, MapNode},
};
use indextree::{Arena, DebugPrettyPrint, NodeId};
use std::{collections::VecDeque, fmt::Debug};

/// A cursor with read-only access to the contents of the [`Map`](crate::map::Map)
#[derive(Debug)]
pub struct Cursor<'a, T> {
    pub(crate) position: NodeId,
    arena: &'a Arena<MapNode<T>>,
}

impl<'a, T> Cursor<'a, T> {
    pub(crate) fn new(position: NodeId, arena: &'a Arena<MapNode<T>>) -> Self {
        Self { position, arena }
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

    /// Returns true if the child exists. This is slightly more efficient than doing
    /// list().contains().
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
        match self
            .arena
            .get(self.position)
            .expect("current node should exist")
            .first_child()
        {
            Some(id) => {
                self.position = id;
                Ok(self)
            }
            None => Err(MapError::NoChildren),
        }
    }

    /// Moves the cursor to the last child.
    pub fn last_child(&mut self) -> Result<&mut Self, MapError> {
        match self
            .arena
            .get(self.position)
            .expect("current node should exist")
            .last_child()
        {
            Some(id) => {
                self.position = id;
                Ok(self)
            }
            None => Err(MapError::NoChildren),
        }
    }

    /// Moves the cursor to the previous sibling node
    pub fn previous_sibling(&mut self) -> Result<&mut Self, MapError> {
        match self
            .arena
            .get(self.position)
            .expect("current node should exist")
            .previous_sibling()
        {
            Some(id) => {
                self.position = id;
                Ok(self)
            }
            None => Err(MapError::NoSibling),
        }
    }

    /// Moves the cursor to the next sibling node
    pub fn next_sibling(&mut self) -> Result<&mut Self, MapError> {
        match self
            .arena
            .get(self.position)
            .expect("current node should exist")
            .next_sibling()
        {
            Some(id) => {
                self.position = id;
                Ok(self)
            }
            None => Err(MapError::NoSibling),
        }
    }

    /// Moves the cursor to the parent. Errors when already at the root node.
    pub fn parent(&mut self) -> Result<&mut Self, MapError> {
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
            None => Err(MapError::NoParent),
        }
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

    // *** PRIVATES *** //

    fn get_id(&self, position: NodeId, name: &str) -> Result<NodeId, MapError> {
        match position.children(self.arena).find(|id| {
            self.arena
                .get(*id)
                .expect("child position should exist")
                .get()
                .name
                .as_str()
                == name
        }) {
            Some(id) => Ok(id),
            None => Err(MapError::NotFound(String::from(name))),
        }
    }
}
