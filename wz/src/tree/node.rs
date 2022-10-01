pub struct Node<T> {
    pub(crate) parent: Option<NodeRef>,
    pub(crate) children: Vec<NodeRef>,
    pub(crate) data: T,
}

pub struct NodeRef {
    pub(crate) index: usize,
}
