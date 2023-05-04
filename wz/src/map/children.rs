//! Children iterator

use crate::map::MapNode;
use indextree::{self, Arena, NodeId};

/// Iterator over the child names
pub struct ChildNames<'a, T> {
    arena: &'a Arena<MapNode<T>>,
    children: indextree::Children<'a, MapNode<T>>,
}

impl<'a, T> ChildNames<'a, T> {
    pub(crate) fn new(current: NodeId, arena: &'a Arena<MapNode<T>>) -> Self {
        Self {
            arena,
            children: current.children(arena),
        }
    }
}

impl<'a, T> Iterator for ChildNames<'a, T> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let id = self.children.next()?;
        Some(
            self.arena
                .get(id)
                .expect("node should exist")
                .get()
                .name
                .as_ref(),
        )
    }
}

/// Iterator over the child data
pub struct Children<'a, T> {
    arena: &'a Arena<MapNode<T>>,
    children: indextree::Children<'a, MapNode<T>>,
}

impl<'a, T> Children<'a, T> {
    pub(crate) fn new(current: NodeId, arena: &'a Arena<MapNode<T>>) -> Self {
        Self {
            arena,
            children: current.children(arena),
        }
    }
}

impl<'a, T> Iterator for Children<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let id = self.children.next()?;
        Some(&self.arena.get(id).expect("node should exist").get().data)
    }
}
