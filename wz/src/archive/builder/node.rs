///! Node of the builder
use crate::{
    archive::{builder::Image, Content, ContentType, Error, Offset},
    encode::{self, SizeHint},
    Int32,
};
use ego_tree::NodeMut;

pub(crate) static MAX_SIZE: u64 = i32::MAX as u64;

pub(crate) enum Node<I>
where
    I: Image,
{
    Package(Content),
    Image { offset: Offset, inner: I },
}

impl<I> Node<I>
where
    I: Image,
{
    pub fn name(&self) -> &str {
        match self {
            Node::Package(content) => content.name(),
            Node::Image { inner, .. } => inner.name(),
        }
    }

    pub fn size(&self) -> u64 {
        match self {
            Node::Package(content) => *content.size as u64,
            Node::Image { inner, .. } => inner.size(),
        }
    }

    pub fn checksum(&self) -> i32 {
        match self {
            Node::Package(content) => *content.checksum,
            Node::Image { inner, .. } => inner.checksum(),
        }
    }

    pub fn set_size(&mut self, size: u64) {
        match self {
            Node::Package(content) => content.size = Int32::from(size),
            Node::Image { .. } => unreachable!(),
        }
    }

    pub fn set_offset(&mut self, off: Offset) {
        match self {
            Node::Package(content) => content.offset = off,
            &mut Node::Image { ref mut offset, .. } => *offset = off,
        }
    }
}

impl<I> SizeHint for Node<I>
where
    I: Image,
{
    fn size_hint(&self) -> u64 {
        match self {
            Node::Package(content) => content.size_hint(),
            Node::Image { offset, inner } => {
                1 + inner.name().size_hint()
                    + Int32::from(inner.size()).size_hint()
                    + Int32::from(inner.checksum()).size_hint()
                    + offset.size_hint()
            }
        }
    }
}

/// Mutable reference to a package node
pub struct Package<'a, I>
where
    I: Image,
{
    pub(crate) inner: NodeMut<'a, Node<I>>,
}

impl<'a, I> Package<'a, I>
where
    I: Image,
{
    /// Appends a package to the node
    pub fn add_package(&mut self, name: &str) -> Package<'_, I> {
        Package {
            inner: self.inner.append(Node::Package(Content {
                content_type: ContentType::Package(name.into()),
                size: 0.into(),
                checksum: 0.into(),
                offset: Offset::from(0),
            })),
        }
    }

    /// Appends an image to the node
    pub fn add_image(&mut self, image: I) -> Result<(), Error> {
        if image.size() > MAX_SIZE {
            return Err(encode::Error::TooLarge(image.size()))?;
        }

        self.inner.append(Node::Image {
            offset: Offset::from(0),
            inner: image,
        });

        Ok(())
    }
}
