use crate::{Arena, NodeId};
use std::collections::VecDeque;

#[derive(Clone)]
/// Iterator over the direct ancestors
pub struct Ancestors<'a, T> {
    arena: &'a Arena<T>,
    node: Option<NodeId>,
}

impl<'a, T> Iterator for Ancestors<'a, T> {
    type Item = (Option<&'a str>, NodeId);

    fn next(&mut self) -> Option<(Option<&'a str>, NodeId)> {
        let node = self.node.take()?;
        self.node = node.parent(self.arena);
        Some((node.name(self.arena), node))
    }
}

impl<'a, T> Ancestors<'a, T> {
    pub(crate) fn new(arena: &'a Arena<T>, current: NodeId) -> Self {
        Self {
            arena: arena,
            node: Some(current),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Type of downward traversal
pub enum TraverseType {
    BreadthFirst,
    DepthFirst,
}

#[derive(Clone)]
/// Iterator over the descendants
pub struct Traverse<'a, T> {
    search_type: TraverseType,
    arena: &'a Arena<T>,
    frontier: VecDeque<(Option<&'a str>, NodeId)>,
}

impl<'a, T> Iterator for Traverse<'a, T> {
    type Item = (Option<&'a str>, NodeId);

    fn next(&mut self) -> Option<(Option<&'a str>, NodeId)> {
        let (name, node) = match self.search_type {
            TraverseType::BreadthFirst => self.frontier.pop_front()?,
            TraverseType::DepthFirst => self.frontier.pop_back()?,
        };
        for (k, v) in node.iter(self.arena) {
            self.frontier.push_back((Some(k.as_str()), *v));
        }
        Some((name, node))
    }
}

impl<'a, T> Traverse<'a, T> {
    pub(crate) fn new(arena: &'a Arena<T>, current: NodeId, search_type: TraverseType) -> Self {
        Traverse {
            search_type: search_type,
            arena: arena,
            frontier: VecDeque::from([(current.name(arena), current)]),
        }
    }

    pub fn search_type(&self) -> TraverseType {
        self.search_type
    }
}
