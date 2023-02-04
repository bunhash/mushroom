use crate::tree::{Node, NodeId};
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Eq, Clone, Debug)]
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

    /// Creates a new node in the arena and returns the ID
    pub fn new_node(&mut self, data: T) -> NodeId {
        let index = self.nodes.len();
        self.nodes.push(Node::new(data));
        NodeId::from_usize(index)
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
        let index = self.nodes.iter().position(|n| n as *const Node<T> as usize == p)?;
        Some(NodeId::from_usize(index))
    }

    /// Clears the arena
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.first_free_node = None;
        self.last_free_node = None;
    }
}

impl<T> Arena<T> {
    pub(crate) fn free_node(&mut self, id: NodeId) {}
}
