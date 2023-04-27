//! Map Cursor
//!
//! Used to navigate the map. This is to abstract the internals so no undefined behavior can occur.

use crate::{
    map::{Error, MapNode, Metadata, SizeHint},
    types::WzInt,
};
use indextree::{Arena, NodeId};
use std::collections::VecDeque;

/// A cursor with read-only access to the contents of the [`Map`](crate::map::Map)
pub struct Cursor<'a, M, T>
where
    M: Metadata<T>,
    T: SizeHint,
{
    pub(crate) position: NodeId,
    arena: &'a Arena<MapNode<M, T>>,
}

impl<'a, M, T> Cursor<'a, M, T>
where
    M: Metadata<T>,
    T: SizeHint,
{
    pub(crate) fn new(position: NodeId, arena: &'a Arena<MapNode<M, T>>) -> Self {
        Self { position, arena }
    }

    /// Returns the path of the current position as a vector of names starting with the root
    pub fn pwd(&'a self) -> Vec<&'a str> {
        let mut path = VecDeque::new();
        for id in self.position.ancestors(self.arena) {
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