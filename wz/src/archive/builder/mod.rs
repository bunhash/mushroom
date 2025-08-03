//! WZ Archive Builder

use crate::{
    archive::{Archive, Content, ContentType, Error, Header, Offset},
    encode::{self, SizeHint},
    Int32,
};
use ego_tree::{NodeMut, Tree};

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
    version: u16,
    tree: Tree<Node<I>>,
}

impl<I> Builder<I>
where
    I: Image,
{
    /// Creates an archive builder
    pub fn new(version: u16) -> Self {
        Self {
            version,
            tree: Tree::new(Node::Package(Content {
                content_type: ContentType::Package("".into()),
                size: 0.into(),
                checksum: 0.into(),
                offset: Offset::from(0),
            })),
        }
    }

    /// Returns the root node
    pub fn root(&mut self) -> Package<'_, I> {
        Package {
            inner: self.tree.root_mut(),
        }
    }

    /// Calculates the sizes and builds the archive structure
    pub fn build(mut self) -> Result<Archive, Error> {
        Ok(Archive {
            header: self.finalize()?,
            tree: self.tree.map(|node| match node {
                Node::Package(content) => content,
                Node::Image { offset, inner } => Content {
                    content_type: ContentType::Image(inner.name().to_string()),
                    size: inner.size().into(),
                    checksum: inner.checksum().into(),
                    offset,
                },
            }),
        })
    }

    fn finalize(&mut self) -> Result<Header, Error> {
        // Create empty header
        let mut header = Header::new(self.version);

        // Set the offset to after the version hash
        let offset = Offset::from(header.content_start + 2);
        self.tree.root_mut().value().set_offset(offset);

        // Calculate the size of everything (and add back version hash size)
        calculate_size(self.tree.root_mut())?;
        let size = self.tree.root().value().size() + 2;
        self.tree.root_mut().value().set_size(size);

        // Caclulate the offsets
        calculate_offsets(self.tree.root_mut(), *offset)?;

        // Set the size in the header
        header.size = self.tree.root().value().size();
        Ok(header)
    }
}

fn calculate_size<'a, I>(mut node: NodeMut<'a, Node<I>>) -> Result<NodeMut<'a, Node<I>>, Error>
where
    I: Image,
{
    let is_package = match node.value() {
        Node::Package(_) => true,
        _ => false,
    };
    match is_package {
        true => {
            if node.has_children() {
                // Calculate children first
                let mut node = node.into_first_child();
                loop {
                    match node {
                        Ok(child) => {
                            let child = calculate_size(child)?;
                            node = child.into_next_sibling();
                        }
                        Err(child) => {
                            node = child.into_parent();
                            break;
                        }
                    }
                }

                // Count and sum up the children
                let mut count = 0i32;
                let mut size = 0u64;
                let mut node = match node {
                    Ok(n) => n.into_first_child(),
                    _ => panic!("panic! node should exist"),
                };
                loop {
                    match node {
                        Ok(mut child) => {
                            count = count + 1;
                            size = size + child.value().size() + child.value().size_hint();
                            node = child.into_next_sibling();
                        }
                        Err(child) => {
                            node = child.into_parent();
                            break;
                        }
                    }
                }

                // Write size and return node
                if size > MAX_SIZE {
                    return Err(encode::Error::TooLarge(size))?;
                }
                let mut node = match node {
                    Ok(n) => n,
                    _ => panic!("panic! node should exist"),
                };
                let size = Int32::from(count).size_hint() + size;
                node.value().set_size(size);
                Ok(node)
            } else {
                let size = Int32::from(0).size_hint();
                node.value().set_size(size);
                Ok(node)
            }
        }
        false => Ok(node),
    }
}

fn calculate_offsets<'a, I>(
    node: NodeMut<'a, Node<I>>,
    start: u32,
) -> Result<NodeMut<'a, Node<I>>, Error>
where
    I: Image,
{
    if node.has_children() {
        // Calculate metadata size first
        let mut meta_size = 0u32;
        let mut node = node.into_first_child();
        loop {
            match node {
                Ok(mut child) => {
                    meta_size = meta_size + child.value().size_hint() as u32;
                    node = child.into_next_sibling();
                }
                Err(child) => {
                    node = child.into_parent();
                    break;
                }
            }
        }

        // Write offsets
        let mut offset = start + meta_size;
        let mut node = match node {
            Ok(n) => n.into_first_child(),
            _ => panic!("panic! node should exist"),
        };
        loop {
            match node {
                Ok(mut child) => {
                    child.value().set_offset(Offset::from(offset));
                    let mut child = calculate_offsets(child, offset)?;
                    offset = offset + child.value().size() as u32;
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
