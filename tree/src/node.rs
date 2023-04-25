use crate::{
    error::{Error, Result},
    NodeId,
};
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
    pub(crate) name: Option<String>,
    pub(crate) parent: Option<NodeId>,
    pub(crate) children: HashMap<String, NodeId>,
    pub(crate) data: NodeData<T>,
}

impl<T> Node<T> {
    pub(crate) fn new(name: String, data: T) -> Self {
        Node {
            name: Some(name),
            parent: None,
            children: HashMap::new(),
            data: NodeData::Data(data),
        }
    }

    pub(crate) fn free(&mut self) {
        self.name = None;
        self.parent = None;
        self.children.clear();
        self.data = NodeData::NextFree(None);
    }

    /// Returns the name of the node
    pub fn name(&self) -> Option<&str> {
        Some(self.name.as_ref()?.as_str())
    }

    /// Returns a reference to the node data
    pub fn get(&self) -> Result<&T> {
        match self.data {
            NodeData::Data(ref data) => Ok(data),
            _ => Err(Error::Removed),
        }
    }

    /// Returns a mutable reference to the node data
    pub fn get_mut(&mut self) -> Result<&mut T> {
        match self.data {
            NodeData::Data(ref mut data) => Ok(data),
            _ => Err(Error::Removed),
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

    /// Returns an iterator over the children
    pub fn iter(&self) -> Iter<'_, String, NodeId> {
        self.children.iter()
    }
}
