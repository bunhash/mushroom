use crate::tree::{node::Node, NodeRef, NodeRefMut};

/// Simple Tree datastructure implementation
pub struct Tree<T> {
    pub(in crate::tree) data: Vec<T>,
    pub(in crate::tree) root: Option<usize>,
    pub(in crate::tree) size: usize,
}

impl<T> Tree<T> {
    /// Creates an empty `Tree`
    pub fn new() -> Self {
        Tree {
            data: Vec::new(),
            root: None,
            size: 0,
        }
    }

    /// Creates a `Tree` with a single node
    pub fn with_root(obj: T) -> Self {
        Tree {
            data: vec![Node::new(obj)],
            root: Some(0),
            size: 1,
        }
    }

    fn make_ref<'a>(&'a self, index: usize) -> NodeRef<'a, T> {
        NodeRef {
            index: index,
            node: &self.data[index],
        }
    }

    fn make_ref_mut<'a>(&'a mut self, index: usize) -> NodeRefMut<'a, T> {
        NodeRefMut {
            index: index,
            node: &mut self.data[index],
        }
    }

    /// Returns root node
    pub fn root(&self) -> Option<NodeRef<T>> {
        match self.root {
            Some(i) => Some(self.make_ref(i)),
            None => None,
        }
    }

    /* remove(node: NodeRef)
     * add_child(parent: NodeRef, obj: T)
     * move_node(new_parent: NodeRef, child: NodeRef)
     * split(node: NodeRef)
     */
}
