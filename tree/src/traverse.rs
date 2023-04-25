use crate::{Arena, NodeId};
use std::collections::VecDeque;

#[derive(Clone)]
/// Iterator over the direct ancestors
pub struct Ancestors<'a, T> {
    arena: &'a Arena<T>,
    node: Option<NodeId>,
}

impl<'a, T> Iterator for Ancestors<'a, T> {
    type Item = (&'a str, NodeId);

    fn next(&mut self) -> Option<(&'a str, NodeId)> {
        let node = self.node.take()?;
        self.node = node.parent(self.arena);
        match node.name(self.arena) {
            Some(name) => Some((name, node)),
            None => None,
        }
    }
}

impl<'a, T> Ancestors<'a, T> {
    pub fn new(arena: &'a Arena<T>, current: NodeId) -> Self {
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
    frontier: VecDeque<(&'a str, NodeId)>,
}

impl<'a, T> Iterator for Traverse<'a, T> {
    type Item = (&'a str, NodeId);

    fn next(&mut self) -> Option<(&'a str, NodeId)> {
        let (name, node) = match self.search_type {
            TraverseType::BreadthFirst => self.frontier.pop_front()?,
            TraverseType::DepthFirst => self.frontier.pop_back()?,
        };
        for (k, v) in node.iter(self.arena) {
            self.frontier.push_back((k.as_str(), *v));
        }
        Some((name, node))
    }
}

impl<'a, T> Traverse<'a, T> {
    pub fn new(arena: &'a Arena<T>, current: NodeId, search_type: TraverseType) -> Self {
        Self {
            search_type: search_type,
            arena: arena,
            frontier: match current.name(arena) {
                Some(name) => VecDeque::from([(name, current)]),
                None => VecDeque::new(),
            },
        }
    }

    pub fn search_type(&self) -> TraverseType {
        self.search_type
    }
}

#[derive(Clone)]
/// Iterator over the descendants
pub struct TraverseUri<'a, T> {
    search_type: TraverseType,
    arena: &'a Arena<T>,
    frontier: VecDeque<(String, &'a str, NodeId)>,
}

impl<'a, T> Iterator for TraverseUri<'a, T> {
    type Item = (String, NodeId);

    fn next(&mut self) -> Option<(String, NodeId)> {
        let (path, name, node) = match self.search_type {
            TraverseType::BreadthFirst => self.frontier.pop_front()?,
            TraverseType::DepthFirst => self.frontier.pop_back()?,
        };
        let uri = format!("{}/{}", path, name);
        for (k, v) in node.iter(self.arena) {
            self.frontier.push_back((uri.clone(), k.as_str(), *v));
        }
        Some((uri, node))
    }
}

impl<'a, T> TraverseUri<'a, T> {
    pub fn new(arena: &'a Arena<T>, uri: &str, current: NodeId, search_type: TraverseType) -> Self {
        let mut frontier = VecDeque::new();
        for (k, v) in current.iter(arena) {
            frontier.push_back((String::from(uri), k.as_str(), *v));
        }
        Self {
            search_type: search_type,
            arena: arena,
            frontier,
        }
    }

    pub fn search_type(&self) -> TraverseType {
        self.search_type
    }
}
