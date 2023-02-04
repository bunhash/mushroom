use crate::tree::{node::NodeData, Ancestors, Arena, NodeError, NodeResult, Traverse, TraverseType};
use std::collections::hash_map::{Iter, Keys};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId(usize);

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeId({})", self.index())
    }
}

impl NodeId {
    pub(crate) fn from_usize(index: usize) -> Self {
        NodeId(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }

    /// Returns the name of the node
    pub fn name<'a, T>(self, arena: &'a Arena<T>) -> Option<&'a str> {
        let parent = arena.get(self)?.parent()?;
        for (k, v) in parent.iter(arena) {
            if *v == self {
                return Some(k.as_str());
            }
        }
        return None;
    }

    /// Returns whether the node was removed
    pub fn is_removed<T>(self, arena: &Arena<T>) -> bool {
        match arena[self].data {
            NodeData::Data(_) => false,
            _ => true,
        }
    }

    /// Inserts the node as a child
    pub fn insert<T>(self, name: String, id: NodeId, arena: &mut Arena<T>) -> NodeResult<()> {
        if self.is_removed(arena) || id.is_removed(arena) {
            return Err(NodeError::Removed);
        }
        arena[id].parent = Some(self);
        arena[self].children.insert(name, id);
        Ok(())
    }

    /// Returns the node ID of the named child
    pub fn get_child<T>(self, name: &str, arena: &Arena<T>) -> Option<NodeId> {
        let child = arena.get(self)?.children.get(name)?;
        Some(*child)
    }

    /// Returns the node ID of the parent
    pub fn parent<T>(self, arena: &Arena<T>) -> Option<NodeId> {
        arena.get(self)?.parent
    }

    /// Returns true if the named child exists
    pub fn has_child<T>(self, name: &str, arena: &Arena<T>) -> bool {
        arena[self].children.contains_key(name)
    }

    /// Returns an iterator over the children keys
    pub fn children<'a, T>(self, arena: &'a Arena<T>) -> Keys<'a, String, NodeId> {
        arena[self].children()
    }

    /// Returns an iterator over the children keys
    pub fn iter<'a, T>(self, arena: &'a Arena<T>) -> Iter<'a, String, NodeId> {
        arena[self].iter()
    }

    /// Iterator that traverses the ancestors
    pub fn ancestors<'a, T>(self, arena: &'a Arena<T>) -> Ancestors<'a, T> {
        Ancestors::new(arena, self)
    }

    /// Iterator that traverses the ancestors
    pub fn descendents<'a, T>(self, arena: &'a Arena<T>) -> Traverse<'a, T> {
        Traverse::new(arena, self, TraverseType::DepthFirst)
    }

    /// Iterator that traverses the (sub)arena breadth-first
    pub fn breadth_first<'a, T>(self, arena: &'a Arena<T>) -> Traverse<'a, T> {
        Traverse::new(arena, self, TraverseType::BreadthFirst)
    }

    /// Iterator that traverses the (sub)arena depth-first
    pub fn depth_first<'a, T>(self, arena: &'a Arena<T>) -> Traverse<'a, T> {
        Traverse::new(arena, self, TraverseType::DepthFirst)
    }

    /// Detaches the (sub)tree
    pub fn detach<T>(self, arena: &mut Arena<T>) {
        if let Some(parent) = self.parent(arena) {
            let name = match self.name(arena) {
                Some(n) => String::from(n),
                None => return,
            };
            if !parent.has_child(&name, arena) {
                return;
            }
            arena[parent].children.remove(&name);
        }
    }

    /// Removes the entire (sub)tree
    pub fn remove<T>(self, arena: &mut Arena<T>) {
        self.detach(arena);
        let descendents: Vec<NodeId> = self.descendents(arena).map(|(_, v)| v).collect();
    }
}
