use crate::tree::{Arena, NodeId};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Ancestors<'a, T> {
    tree: &'a Arena<T>,
    node: Option<NodeId>,
}

impl<'a, T> Iterator for Ancestors<'a, T> {
    type Item = (Option<&'a str>, NodeId);

    fn next(&mut self) -> Option<(Option<&'a str>, NodeId)> {
        let node = self.node.take()?;
        self.node = node.parent(self.tree);
        Some((node.name(self.tree), node))
    }
}

impl<'a, T> Ancestors<'a, T> {
    pub(crate) fn new(tree: &'a Arena<T>, current: NodeId) -> Self {
        Self {
            tree: tree,
            node: Some(current),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TraverseType {
    BreadthFirst,
    DepthFirst,
}

#[derive(Clone)]
pub struct Traverse<'a, T> {
    search_type: TraverseType,
    tree: &'a Arena<T>,
    frontier: VecDeque<(Option<&'a str>, NodeId)>,
}

impl<'a, T> Iterator for Traverse<'a, T> {
    type Item = (Option<&'a str>, NodeId);

    fn next(&mut self) -> Option<(Option<&'a str>, NodeId)> {
        let (name, node) = match self.search_type {
            TraverseType::BreadthFirst => self.frontier.pop_front()?,
            TraverseType::DepthFirst => self.frontier.pop_back()?,
        };
        for (k, v) in node.iter(self.tree) {
            self.frontier.push_back((Some(k.as_str()), *v));
        }
        Some((name, node))
    }
}

impl<'a, T> Traverse<'a, T> {
    pub(crate) fn new(tree: &'a Arena<T>, current: NodeId, search_type: TraverseType) -> Self {
        Traverse {
            search_type: search_type,
            tree: tree,
            frontier: VecDeque::from([(current.name(tree), current)]),
        }
    }

    pub fn search_type(&self) -> TraverseType {
        self.search_type
    }
}
