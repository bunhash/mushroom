use crate::tree::node::{Node, NodeRef};
use std::collections::HashSet;

/// Simple Tree datastructure implementation
pub struct Tree<T> {
    pub(in crate::tree) arena: Vec<T>,
    pub(in crate::tree) tree: Vec<Node>,
    pub(in crate::tree) root: Option<NodeRef>,
}

impl<T> Tree<T> {
    /// Creates an empty `Tree`
    pub fn new() -> Self {
        Tree {
            arena: Vec::new(),
            tree: Vec::new(),
            root: None,
        }
    }

    /// Creates a `Tree` with a single node
    pub fn with_root(obj: T) -> Self {
        Tree {
            arena: vec![obj],
            tree: vec![Node::new(0)],
            root: Some(NodeRef::new(0)),
        }
    }

    pub fn traverse<F>(&self, f: &mut F)
    where
        F: FnMut(&T),
    {
        if let Some(root) = self.root {
            let mut frontier = vec![root];
            let mut explored = HashSet::new();

            while !frontier.is_empty() {
                if let Some(item) = frontier.pop() {
                    if !explored.contains(&item) {
                        f(self.get_ref(item));
                        explored.insert(item);
                        self.tree[item.index]
                            .children
                            .iter()
                            .for_each(|c| frontier.push(*c));
                    }
                }
            }
        }
    }

    fn get_ref<'a>(&'a self, node_ref: NodeRef) -> &'a T {
        let ind = self.tree[node_ref.index].index;
        &self.arena[ind]
    }

    fn make_ref(&self, node_ref: NodeRef) -> NodeRef {
        node_ref
    }

    /// Returns root node
    pub fn root(&self) -> Option<NodeRef> {
        match self.root {
            Some(i) => Some(self.make_ref(i)),
            None => None,
        }
    }

    /// Adds a child
    pub fn add_child(&mut self, parent: NodeRef, obj: T) {
        let arena_ind = self.arena.len();
        let node_ind = self.tree.len();

        let child = Node::with_parent(arena_ind, parent);
        self.arena.push(obj);

        self.tree
            .get_mut(parent.index)
            .unwrap()
            .children
            .insert(NodeRef::new(node_ind));
        self.tree.push(child);
    }

    /* remove(node: NodeRef)
     * add_child(parent: NodeRef, obj: T)
     * move_node(new_parent: NodeRef, child: NodeRef)
     * split(node: NodeRef)
     */
}
