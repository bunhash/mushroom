//! Package Map
//!
//! Maps the package contents in an arena-style tree structure

use crate::{
    error::{PackageError, Result},
    package::Content,
    types::WzString,
};
use indextree::{Arena, Node, NodeId};

#[derive(Debug)]
pub struct Map {
    arena: Arena<Content>,
    root: NodeId,
}

impl Map {
    /// Returns the child's NodeId given the parent and name
    fn get_child_id_by_name(&self, id: NodeId, name: &str) -> Option<NodeId> {
        for child in id.children(&self.arena) {
            let child_name = match self.arena.get(child)?.get() {
                Content::Directory(n) => n,
                Content::Image(n, _, _) => n,
            };
            if child_name == name {
                return Some(child);
            }
        }
        None
    }

    /// Returns the child given the parent ID and name
    fn get_child_by_name(&self, id: NodeId, name: &str) -> Option<&Content> {
        Some(self.arena.get(self.get_child_id_by_name(id, name)?)?.get())
    }

    /// Recursively goes down the path
    fn get_id_recursive(&self, uri: &[&str], current: NodeId) -> Option<NodeId> {
        if uri.is_empty() {
            Some(current)
        } else {
            self.get_id_recursive(&uri[1..], self.get_child_id_by_name(current, uri[0])?)
        }
    }

    /// Gets the NodeId given the URI
    fn get_id(&self, uri: &str) -> Option<NodeId> {
        let uri = uri.split("/").collect::<Vec<&str>>();
        if uri[0] == "" {
            self.get_id_recursive(&uri[1..], self.root)
        } else {
            None
        }
    }

    /// Gets the [`Content`] given the URI
    pub fn get(&self, uri: &str) -> Option<&Content> {
        Some(self.arena.get(self.get_id(uri)?)?.get())
    }

    /// Creates a directory at the node
    fn create_directory_at(&mut self, id: NodeId, name: &str) -> Result<()> {
        if name.contains("/") {
            Err(PackageError::DirectoryName(String::from(name)).into())
        } else {
            match self.get_child_by_name(id, name) {
                Some(child) => {
                    if child.is_image() {
                        return Err(PackageError::CreateError.into());
                    } else {
                        return Ok(());
                    }
                }
                None => {}
            }
            let node = self
                .arena
                .new_node(Content::Directory(WzString::from(name)));
            id.append(node, &mut self.arena);
            Ok(())
        }
    }

    /*
    /// Add a directory
    pub fn add_directory(&self, uri: &str) -> Result<()> {
        let uri = uri.split("/").collect::<Vec<&str>>();
        if uri[0] == "" {
        } else {
        }
    }
    */
}
