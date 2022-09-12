use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct NodeRef {
    pub(in crate::tree) index: usize,
}

impl NodeRef {
    pub(in crate::tree) fn new(index: usize) -> Self {
        NodeRef { index }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::tree) struct Node {
    pub(in crate::tree) index: usize,
    pub(in crate::tree) parent: Option<NodeRef>,
    pub(in crate::tree) children: HashSet<NodeRef>,
}

impl Node {
    pub(in crate::tree) fn new(index: usize) -> Self {
        Node {
            index: index,
            parent: None,
            children: HashSet::new(),
        }
    }

    pub(in crate::tree) fn with_parent(index: usize, parent: NodeRef) -> Self {
        Node {
            index: index,
            parent: Some(parent),
            children: HashSet::new(),
        }
    }
}
