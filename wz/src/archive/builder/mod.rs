//! WZ Archive Builder

use crate::{
    archive::{Archive, Content, ContentType, Error, Header, Offset, Writer},
    encode::{self, Encode, Encoder, SizeHint},
    Int32,
};
use crypto::Encryptor;
use ego_tree::{NodeMut, NodeRef, Tree};
use std::path::Path;

mod image;
mod node;

use node::{Node, MAX_SIZE};

pub use image::Image;
pub use node::Package;

/// Archive builder
pub struct Builder<I>
where
    I: Image,
{
    tree: Tree<Node<I>>,
}

impl<I> Builder<I>
where
    I: Image,
{
    /// Creates an archive builder
    pub fn new() -> Self {
        Self {
            tree: Tree::new(Node::Package {
                name: String::new(),
                size: 0,
                offset: Offset::from(0),
                length: 0,
            }),
        }
    }

    /// Returns the root node
    pub fn root(&mut self) -> Package<'_, I> {
        Package {
            inner: self.tree.root_mut(),
        }
    }

    /// Consumes itself, calculates the offsets, and builds the `Archive`
    pub fn build(mut self, version: u16) -> Result<Archive, Error> {
        let mut header = Header::new(version);
        self.finalize(&mut header)?;
        Ok(Archive {
            header,
            tree: self.tree.map(|node| match node {
                Node::Package {
                    name, size, offset, ..
                } => Content {
                    content_type: ContentType::Package(name),
                    size: size.into(),
                    checksum: 0.into(),
                    offset,
                },
                Node::Image { inner, offset } => Content {
                    content_type: ContentType::Image(inner.name().to_string()),
                    size: inner.size().into(),
                    checksum: inner.checksum().into(),
                    offset,
                },
            }),
        })
    }

    /// Consumes itself, calculates the offsets, and writes the WZ archive
    pub fn write<E>(mut self, writer: &mut Writer<E>) -> Result<(), Error>
    where
        E: Encryptor,
    {
        self.finalize(&mut writer.header)?;
        writer.write_header()?;
        write_node(writer, self.tree.root())?;
        Ok(())
    }

    fn finalize(&mut self, header: &mut Header) -> Result<(), Error> {
        // Set the offset to after the version hash
        let offset = Offset::from(header.content_start + 2);
        self.tree.root_mut().value().set_offset(offset);

        // The root size includes the version hash and package length for whatever reason.
        let size = self.tree.root().value().content_size() + 2;
        self.tree.root_mut().value().set_size(size);

        // Caclulate the offsets
        calculate_offsets(self.tree.root_mut(), *offset)?;

        // Set the size in the header
        header.size = self.tree.root().value().size();
        Ok(())
    }
}

fn write_node<'a, E, I>(writer: &mut Writer<E>, node: NodeRef<'a, Node<I>>) -> Result<(), Error>
where
    E: Encryptor,
    I: Image,
{
    // Sanity check
    let position = writer.position()?;
    if position != *node.value().offset() {
        return Err(Error::other(&format!(
            "writer position does not match offset ({} at {})",
            node.value().name(),
            node.value().offset()
        )));
    }

    // Encode node
    match node.value() {
        // Encode package
        Node::Package { length, .. } => {
            Int32::from(*length).encode(writer)?;
            for child in node.children() {
                match child.value() {
                    Node::Package {
                        name, size, offset, ..
                    } => Content {
                        content_type: ContentType::Package(name.to_string()),
                        size: Int32::from(*size),
                        checksum: 0.into(),
                        offset: *offset,
                    }
                    .encode(writer)?,
                    Node::Image { inner, offset } => Content {
                        content_type: ContentType::Image(inner.name().to_string()),
                        size: inner.size().into(),
                        checksum: inner.checksum().into(),
                        offset: *offset,
                    }
                    .encode(writer)?,
                }
            }
        }

        // Encode image
        Node::Image { inner, .. } => writer.write_image(inner)?,
    }

    // Encode children
    for child in node.children() {
        write_node(writer, child)?;
    }

    Ok(())
}

fn calculate_offsets<'a, I>(
    mut node: NodeMut<'a, Node<I>>,
    start: u32,
) -> Result<NodeMut<'a, Node<I>>, Error>
where
    I: Image,
{
    if node.has_children() {
        // Calculate metadata size first
        let mut meta_size = Int32::from(node.value().len()).size_hint();
        let mut node = node.into_first_child();
        loop {
            match node {
                Ok(mut child) => {
                    meta_size = meta_size + child.value().meta_size();
                    node = child.into_next_sibling();
                }
                Err(child) => {
                    node = child.into_parent();
                    break;
                }
            }
        }

        // Write offsets
        let mut offset = start + meta_size as u32;
        let mut node = match node {
            Ok(n) => n.into_first_child(),
            _ => panic!("panic! node should exist"),
        };
        loop {
            match node {
                Ok(mut child) => {
                    child.value().set_offset(Offset::from(offset));
                    let mut child = calculate_offsets(child, offset)?;
                    offset = offset + child.value().content_size() as u32;
                    node = child.into_next_sibling();
                }
                Err(child) => {
                    node = child.into_parent();
                    break;
                }
            }
        }

        Ok(match node {
            Ok(n) => n,
            _ => panic!("panic! node should exist"),
        })
    } else {
        Ok(node)
    }
}
