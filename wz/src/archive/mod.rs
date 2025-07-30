//! WZ Archive Structures

use crate::{
    decode::{Decode, Reader},
    Int32,
};
use crypto::{checksum, Decryptor, DummyKeyStream};
use ego_tree::{NodeMut, NodeRef, Tree};
use std::{collections::VecDeque, fs::File, io::BufReader, path::Path};

mod error;
mod header;
mod offset;
mod package;

pub use error::Error;
pub use header::Header;
pub use offset::Offset;
pub use package::{Content, ContentType, Package};

/// WZ Archive structure
#[derive(Debug)]
pub struct Archive {
    /// WZ Archive header
    pub header: Header,

    /// Tree
    pub tree: Tree<Content>,
}

impl Archive {
    /// Opens a WZ Archive
    pub fn parse<P, D>(path: P, decryptor: D) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        D: Decryptor,
    {
        let mut read = BufReader::new(File::open(path)?);
        let header = Header::from_read(&mut read)?;
        let mut reader = bruteforce_version(&header, read, decryptor)?;
        let tree = Self::build("".into(), header.size.into(), &mut reader)?;
        Ok(Archive { header, tree })
    }

    /// Opens a WZ Archive as a specific version
    pub fn parse_as_version<P, D>(path: P, version: u16, decryptor: D) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        D: Decryptor,
    {
        let mut read = BufReader::new(File::open(path)?);
        let header = Header::from_read(&mut read)?;
        let (version_hash, version_checksum) = checksum(&version.to_string());
        if header.version_hash != version_hash {
            return Err(Error::Version);
        }
        let mut reader = Reader::new(header.content_start, version_checksum, read, decryptor);
        let tree = Self::build("".into(), header.size.into(), &mut reader)?;
        Ok(Archive { header, tree })
    }

    /// Opens an unencrypted WZ Archive
    pub fn parse_unencrypted<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        Archive::parse(path, DummyKeyStream)
    }

    /// Opens an unencrypted WZ Archive
    pub fn parse_unencrypted_as_version<P>(path: P, version: u16) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        Archive::parse_as_version(path, version, DummyKeyStream)
    }

    /// Returns a mutable node reference at the specified path
    pub fn get_node(&self, path: &str) -> Option<NodeRef<'_, Content>> {
        let mut parts = path.rsplit("/").collect::<Vec<_>>();
        let root = parts.last()?;
        if root.is_empty() {
            let _ = parts.pop();
        }
        let mut content = self.tree.root();
        loop {
            match parts.pop() {
                Some(name) => {
                    let mut child = content.first_child();
                    loop {
                        match child {
                            Some(c) => {
                                if name == c.value().name() {
                                    content = c;
                                    break;
                                }
                                child = c.next_sibling();
                            }
                            None => return None,
                        }
                    }
                }
                None => break,
            }
        }
        Some(content)
    }

    /// Returns a mutable node reference at the specified path
    pub fn get_mut_node(&mut self, path: &str) -> Option<NodeMut<'_, Content>> {
        self.tree.get_mut(self.get_node(path)?.id())
    }

    /// Clones a subtree to a new `Tree`. This is a bad ego-tree workaround.
    pub fn clone_subtree(&self, path: &str) -> Option<Tree<Content>> {
        let node = self.get_node(path)?;
        let mut tree = Tree::new(node.value().clone());
        let mut queue = VecDeque::from([(node.id(), tree.root().id())]);
        loop {
            match queue.pop_front() {
                Some((src_id, dst_id)) => {
                    let src_node = self.tree.get(src_id).expect("panic! node should exist");
                    let mut dst_node = tree.get_mut(dst_id).expect("panic! node should exist");
                    for src_child in src_node.children() {
                        let dst_child = dst_node.append(src_child.value().clone());
                        queue.push_back((src_child.id(), dst_child.id()));
                    }
                }
                None => break,
            }
        }
        Some(tree)
    }

    /// Returns a reference to the content at the specified path
    pub fn get(&self, path: &str) -> Option<&Content> {
        Some(self.get_node(path)?.value())
    }

    fn build<D>(
        name: String,
        size: Int32,
        reader: &mut Reader<BufReader<File>, D>,
    ) -> Result<Tree<Content>, Error>
    where
        D: Decryptor,
    {
        reader.rewind()?;
        let root = Content {
            content_type: ContentType::Package(name),
            size,
            checksum: 0.into(),
            offset: Offset::from(reader.position()?),
        };
        let mut tree = Tree::new(root);
        let mut queue = VecDeque::from([tree.root().id()]);
        loop {
            match queue.pop_front() {
                Some(id) => {
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
                None => break,
            }
        }
        Ok(tree)
    }
}

fn bruteforce_version<D>(
    header: &Header,
    buf: BufReader<File>,
    decryptor: D,
) -> Result<Reader<BufReader<File>, D>, Error>
where
    D: Decryptor,
{
    let lower_bound = Offset::from(header.content_start);
    let upper_bound = Offset::from(header.content_start + header.size as u32);
    let mut reader = Reader::new(header.content_start, 0u32, buf, decryptor);
    for version_checksum in Header::possible_versions(header.version_hash) {
        reader.version_checksum = version_checksum;
        reader.rewind()?;

        // Decodes the top-level directory contents. If all contents lie within the lower and
        // upper bounds, we can assume the version checksum is good.
        let package = Package::decode(&mut reader)?;
        let filtered_len = package
            .contents
            .iter()
            .map(|content| content.offset)
            .filter(|off| *off >= lower_bound && *off < upper_bound)
            .count();
        if package.contents.len() == filtered_len {
            return Ok(reader);
        }
    }
    Err(Error::Bruteforce)
}
