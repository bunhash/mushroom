use crate::tree::{NodeError, NodeId, NodeResult};
use std::collections::{
    hash_map::{Iter, Keys},
    HashMap,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NodeData<T> {
    /// Stores the data
    Data(T),
    /// Stores the next free node
    NextFree(Option<NodeId>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A node within the `Arena`
pub struct Node<T> {
    pub(crate) parent: Option<NodeId>,
    pub(crate) children: HashMap<String, NodeId>,
    pub(crate) data: NodeData<T>,
}

impl<T> Node<T> {
    pub(crate) fn new(data: T) -> Self {
        Node {
            parent: None,
            children: HashMap::new(),
            data: NodeData::Data(data),
        }
    }

    pub(crate) fn free(&mut self, next_free: Option<NodeId>) {
        self.parent = None;
        self.children.clear();
        self.data = NodeData::NextFree(next_free);
    }

    /// Returns a reference to the node data
    pub fn get(&self) -> NodeResult<&T> {
        match self.data {
            NodeData::Data(ref data) => Ok(data),
            _ => Err(NodeError::Removed),
        }
    }

    /// Returns a mutable reference to the node data
    pub fn get_mut(&mut self) -> NodeResult<&mut T> {
        match self.data {
            NodeData::Data(ref mut data) => Ok(data),
            _ => Err(NodeError::Removed),
        }
    }

    /// Returns the node ID of the named child
    pub fn get_child(&self, name: &str) -> Option<NodeId> {
        match self.children.get(name) {
            Some(c) => Some(*c),
            None => None,
        }
    }

    /// Returns the node ID of the parent
    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }

    /// Returns true if the named child exists
    pub fn has_child(&self, name: &str) -> bool {
        self.children.contains_key(name)
    }

    /// Returns an iterator over the children keys
    pub fn children(&self) -> Keys<'_, String, NodeId> {
        self.children.keys()
    }

    /// Returns an iterator over the children keys
    pub fn iter(&self) -> Iter<'_, String, NodeId> {
        self.children.iter()
    }
}
