//! WZ Image Structures

use crate::decode::{Decode, Decoder};
use ego_tree::{NodeId, NodeMut, NodeRef, Tree};
use std::fmt;

mod content;
mod error;
mod object;
mod package;
mod reader;
mod string;
mod value;

pub use content::Content;
pub use error::Error;
pub use object::{Object, ObjectTag, UolObjectTag};
pub use package::Package;
pub use reader::Reader;
pub use string::UolString;
pub use value::Value;

/// This a mapping of the WZ image contents
#[derive(Debug)]
pub struct Image {
    tree: Tree<Content>,
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.tree)
    }
}

impl Image {
    /// Parses a WZ Image and maps all the content
    pub(crate) fn parse<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        decoder.seek(0)?;
        let mut tree = Tree::new(Content {
            name: UolString::placed(""),
            value: Value::Object(Object::decode(decoder)?),
        });
        Self::parse_recur(tree.root_mut(), decoder)?;
        Ok(Self { tree })
    }

    fn parse_recur<'a, D: Decoder>(
        mut node: NodeMut<'a, Content>,
        decoder: &mut D,
    ) -> Result<(), Error> {
    }

    /// Iterator over the archive content
    pub fn iter(&self) -> ContentPathIter<'_> {
        ContentPathIter::new(&self.tree)
    }

    /// Perform a mutable function on all the archive content
    pub fn for_each<F>(&mut self, func: F)
    where
        F: Fn(&str, &mut Content) -> (),
    {
        func("", self.tree.root_mut().value());
        let node = self.tree.root();
        let path = String::from("");
        let ids = node.children().map(|c| c.id()).collect::<Vec<_>>();
        for id in ids {
            self.for_each_recur(path.as_str(), id, &func);
        }
    }

    fn for_each_recur<F>(&mut self, path: &str, id: NodeId, func: &F)
    where
        F: Fn(&str, &mut Content) -> (),
    {
        func(
            path,
            self.tree
                .get_mut(id)
                .expect("panic! node should exist")
                .value(),
        );
        let node = self.tree.get(id).expect("panic! node should exist");
        let path = format!("{}/{}", path, node.value().name());
        let ids = node.children().map(|c| c.id()).collect::<Vec<_>>();
        for id in ids {
            self.for_each_recur(path.as_str(), id, func);
        }
    }

    /// Returns a node reference at the specified path
    pub fn get_node(&self, path: &str) -> Option<NodeRef<'_, Content>> {
        let mut parts = path.rsplit("/").collect::<Vec<_>>();
        let root = parts.last()?;
        if root.is_empty() {
            let _ = parts.pop();
        }
        let mut content = Some(self.tree.root());
        while let Some(name) = parts.pop() {
            let search = content.take();
            match search {
                Some(s) => {
                    for child in s.children() {
                        if name == child.value().name() {
                            content = Some(child);
                            break;
                        }
                    }
                }
                None => break,
            }
        }
        content
    }

    /// Returns a mutable node reference at the specified path
    pub fn get_mut_node(&mut self, path: &str) -> Option<NodeMut<'_, Content>> {
        self.tree.get_mut(self.get_node(path)?.id())
    }

    /// Returns a reference to the content at the specified path
    pub fn get(&self, path: &str) -> Option<&Content> {
        Some(self.get_node(path)?.value())
    }

    /// Builds a new subtree at the path
    pub fn get_subtree(&self, path: &str) -> Option<Tree<&Content>> {
        let node = self.get_node(path)?;
        let mut tree = Tree::new(node.value());
        Self::build_subtree_recur(tree.root_mut(), node);
        Some(tree)
    }

    fn build_subtree_recur<'a, 'b>(mut dst: NodeMut<'a, &'b Content>, src: NodeRef<'b, Content>) {
        for child in src.children() {
            Self::build_subtree_recur(dst.append(child.value()), child);
        }
    }
}

/// Iterator over the content. Returns a tuple: (package path, content)
pub struct ObjectPathIter<'a> {
    node: Option<NodeRef<'a, Content>>,
    path: Vec<String>,
}

impl<'a> ContentPathIter<'a> {
    fn new(tree: &'a Tree<Content>) -> Self {
        Self {
            node: Some(tree.root()),
            path: Vec::new(),
        }
    }
}

impl<'a> Iterator for ContentPathIter<'a> {
    type Item = (String, &'a Content);

    fn next(&mut self) -> Option<Self::Item> {
        let path = self.path.join("/");
        let node = self.node?;
        if let Some(child) = node.first_child() {
            self.path.push(node.value().name().to_string());
            self.node = Some(child);
        } else if let Some(sibling) = node.next_sibling() {
            self.node = Some(sibling);
        } else {
            let _ = self.path.pop();
            self.node = match node.parent() {
                Some(parent) => parent.next_sibling(),
                None => None,
            };
        }
        Some((path, node.value()))
    }
}
