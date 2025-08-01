//! WZ Archive Tree

use crate::{
    archive::{Content, ContentType, Error, Header, Offset, Package},
    decode::{Decode, Reader},
};
use crypto::Decryptor;
use ego_tree::{NodeId, NodeMut, NodeRef, Tree};
use std::{
    collections::VecDeque,
    fmt,
    io::{Read, Seek},
};

/// This a mapping of the archived Images. Its structure is actually a tree but `ContentMap` made
/// the most sense to me.
#[derive(Debug)]
pub struct ContentMap {
    tree: Tree<Content>,
}

impl fmt::Display for ContentMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.tree)
    }
}

impl ContentMap {
    /// Parses a WZ Archive and maps all the content
    pub fn parse<R, D>(header: &Header, reader: &mut Reader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        reader.rewind()?;
        let root = Content {
            content_type: ContentType::Package("".into()),
            size: header.size.into(),
            checksum: 0.into(),
            offset: Offset::from(reader.position()?),
        };
        let mut tree = Tree::new(root);
        let mut queue = VecDeque::from([tree.root().id()]);
        while let Some(id) = queue.pop_front() {
            let mut node = tree.get_mut(id).expect("panic! node should exist");
            let offset = node.value().offset;
            reader.seek(*offset)?;
            let package = Package::decode(reader)?;
            for c in package.contents.into_iter() {
                match &c.content_type {
                    ContentType::Package(_) => {
                        let child = node.append(c);
                        queue.push_back(child.id());
                    }
                    ContentType::Image(_) => {
                        node.append(c);
                    }
                }
            }
        }
        Ok(ContentMap { tree })
    }

    /// Iterator over the archive content
    pub fn iter(&self) -> ContentPathIter<'_> {
        ContentPathIter::new(&self.tree)
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
        let mut queue = VecDeque::from([(node.id(), tree.root().id())]);
        while let Some((src_id, dst_id)) = queue.pop_front() {
            let src_node = self.tree.get(src_id).expect("panic! node should exist");
            let mut dst_node = tree.get_mut(dst_id).expect("panic! node should exist");
            for src_child in src_node.children() {
                let dst_child = dst_node.append(src_child.value());
                queue.push_back((src_child.id(), dst_child.id()));
            }
        }
        Some(tree)
    }
}

/// Iterator over the content. Returns a tuple: (package path, content)
pub struct ContentPathIter<'a> {
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
