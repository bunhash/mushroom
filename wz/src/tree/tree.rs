use crate::tree::{Node, NodeRef};

pub struct Tree<T> {
    arena: Vec<Node<T>>,
    root: Option<NodeRef>,
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree {
            arena: Vec::new(),
            root: None,
        }
    }

    pub fn with_root(obj: T) -> Tree<T> {
        Tree {
            arena: vec![Node {
                parent: None,
                children: Vec::new(),
                data: obj,
            }],
            root: Some(NodeRef { index: 0 }),
        }
    }

    pub fn new_node(&mut self, obj: T) -> NodeRef {
        let index = self.arena.len();
        self.arena.push(Node {
            parent: None,
            children: Vec::new(),
            data: obj,
        });
        NodeRef { index }
    }

    pub fn set_root(&mut self, nref: NodeRef) {

    }
}
