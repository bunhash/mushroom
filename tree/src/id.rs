use crate::{node::NodeData, Ancestors, Arena, NodeError, NodeResult, Traverse, TraverseType};
use std::collections::hash_map::{Iter, Keys};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// ID used to represent the node in the Arena
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
    pub fn name<T>(self, arena: &Arena<T>) -> Option<&str> {
        arena[self].name()
    }

    /// Returns whether the node was removed
    pub fn is_removed<T>(self, arena: &Arena<T>) -> bool {
        match arena[self].data {
            NodeData::Data(_) => false,
            _ => true,
        }
    }

    /// Inserts the node as a child
    pub fn insert<T>(self, id: NodeId, arena: &mut Arena<T>) -> NodeResult<()> {
        // TODO: Add more insert constraints
        if self.is_removed(arena) || id.is_removed(arena) {
            return Err(NodeError::Removed);
        }
        let name = match id.name(arena) {
            Some(n) => String::from(n),
            None => return Err(NodeError::NotNamed),
        };
        if let Some(old_child) = self.get_child(&name, arena) {
            old_child.remove(arena);
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
            arena[self].parent = None;
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
        arena.free_node(self);
    }
}
