use crate::{node::NodeData, Node, NodeId};
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Eq, Clone, Debug)]
/// Arena to hold the tree data
pub struct Arena<T> {
    nodes: Vec<Node<T>>,
    first_free_node: Option<NodeId>,
    last_free_node: Option<NodeId>,
}

impl<T> Index<NodeId> for Arena<T> {
    type Output = Node<T>;

    fn index(&self, node: NodeId) -> &Node<T> {
        &self.nodes[node.index()]
    }
}

impl<T> IndexMut<NodeId> for Arena<T> {
    fn index_mut(&mut self, node: NodeId) -> &mut Node<T> {
        &mut self.nodes[node.index()]
    }
}

impl<T> Arena<T> {
    /// Creates an empty Arena
    pub fn new() -> Self {
        Arena {
            nodes: Vec::new(),
            first_free_node: None,
            last_free_node: None,
        }
    }

    /// Creates an empty Arena with an initial capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Arena {
            nodes: Vec::with_capacity(capacity),
            first_free_node: None,
            last_free_node: None,
        }
    }

    /// Returns the current capacity
    pub fn capacity(&self) -> usize {
        self.nodes.capacity()
    }

    /// Returns the size of the arena
    pub fn count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns whether the arena is empty or not
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    pub(crate) fn push_free_node(&mut self, next_free: NodeId) {
        if let Some(last) = self.last_free_node {
            self[last].data = NodeData::NextFree(Some(next_free));
        } else {
            self.first_free_node = Some(next_free);
            self.last_free_node = Some(next_free);
        }
    }

    pub(crate) fn pop_free_node(&mut self) -> Option<NodeId> {
        let next_free = self.first_free_node;
        if let Some(node) = next_free {
            self.first_free_node = match self[node].data {
                NodeData::NextFree(next) => next,
                _ => None,
            };
            if self.first_free_node.is_none() {
                self.last_free_node = None;
            }
        }
        next_free
    }

    /// Creates a new node in the arena and returns the ID
    pub fn new_node(&mut self, name: String, data: T) -> NodeId {
        match self.pop_free_node() {
            Some(node) => {
                self[node].name = Some(name);
                self[node].data = NodeData::Data(data);
                node
            }
            None => {
                let index = self.nodes.len();
                self.nodes.push(Node::new(name, data));
                NodeId::from_usize(index)
            }
        }
    }

    pub(crate) fn free_node(&mut self, id: NodeId) {
        let removal: Vec<NodeId> = id.descendents(self).map(|(_, n)| n).collect();
        for node in removal {
            self[node].free();
            self.push_free_node(node);
        }
    }

    /// Returns a reference to the node
    pub fn get(&self, id: NodeId) -> Option<&Node<T>> {
        self.nodes.get(id.index())
    }

    /// Returns a mutable reference to the node
    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node<T>> {
        self.nodes.get_mut(id.index())
    }

    /// Returns the ID of the node
    pub fn get_node_id(&self, node: &Node<T>) -> Option<NodeId> {
        let p = node as *const Node<T> as usize;
        let index = self
            .nodes
            .iter()
            .position(|n| n as *const Node<T> as usize == p)?;
        Some(NodeId::from_usize(index))
    }

    /// Clears the arena
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.first_free_node = None;
        self.last_free_node = None;
    }
}
