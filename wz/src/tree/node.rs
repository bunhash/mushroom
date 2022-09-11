use std::{
    collections::{hash_set::Iter, HashSet},
    ops::{Deref, DerefMut},
};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::tree) struct Node<T> {
    pub(in crate::tree) object: T,
    pub(in crate::tree) parent: Option<usize>,
    pub(in crate::tree) children: HashSet<usize>,
}

impl<T> Node<T> {
    pub(in crate::tree) fn new(obj: T) -> Self {
        Node {
            object: obj,
            parent: None,
            children: HashSet::new(),
        }
    }
}

pub struct NodeRef<'a, T> {
    pub(in crate::tree) index: usize,
    pub(in crate::tree) node: &'a Node<T>,
}

impl<'a, T> NodeRef<'a, T> {
    pub fn children(&self) -> Iter<'_, T> {
        self.node.children.iter()
    }
}

impl<'a, T> Deref for NodeRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.node.object
    }
}

impl<'a, T> Iterator for NodeRef<'a, T> {}

pub struct NodeRefMut<'a, T> {
    pub(in crate::tree) index: usize,
    pub(in crate::tree) node: &'a mut Node<T>,
}

impl<'a, T> Deref for NodeRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.node.object
    }
}

impl<'a, T> DerefMut for NodeRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node.object
    }
}
