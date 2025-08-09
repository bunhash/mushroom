//! Node of the builder
use crate::{
    archive::{builder::Image, Error, Offset},
    encode::{self, SizeHint},
    Int32,
};
use ego_tree::NodeMut;

pub(crate) static MAX_SIZE: u64 = i32::MAX as u64;
pub(crate) static MAX_LEN: usize = i32::MAX as usize;

pub(crate) enum Node<I>
where
    I: Image,
{
    Package {
        name: String,
        size: u64,
        offset: Offset,
        length: usize,
    },
    Image {
        inner: I,
        offset: Offset,
    },
}

impl<I> Node<I>
where
    I: Image,
{
    pub fn name(&self) -> &str {
        match self {
            Node::Package { name, .. } => name.as_str(),
            Node::Image { inner, .. } => inner.name(),
        }
    }

    pub fn size(&self) -> u64 {
        match self {
            Node::Package { size, .. } => *size,
            Node::Image { inner, .. } => inner.size(),
        }
    }

    pub fn offset(&self) -> Offset {
        match self {
            Node::Package { offset, .. } => *offset,
            Node::Image { offset, .. } => *offset,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Node::Package { length, .. } => *length,
            Node::Image { .. } => 0,
        }
    }

    pub fn set_size(&mut self, new_size: u64) {
        match self {
            &mut Node::Package { ref mut size, .. } => *size = new_size,
            Node::Image { .. } => unreachable!(),
        }
    }

    pub fn set_offset(&mut self, new_offset: Offset) {
        match self {
            &mut Node::Package { ref mut offset, .. } => *offset = new_offset,
            &mut Node::Image { ref mut offset, .. } => *offset = new_offset,
        }
    }

    pub fn set_len(&mut self, new_length: usize) {
        match self {
            &mut Node::Package { ref mut length, .. } => *length = new_length,
            Node::Image { .. } => {}
        }
    }

    pub fn meta_size(&self) -> u64 {
        match self {
            Node::Package {
                name, size, offset, ..
            } => {
                // 1 for ContentType tag + 1 for checksum (checksum = 0)
                2 + name.size_hint() + Int32::from(*size).size_hint() + offset.size_hint()
            }
            Node::Image { inner, offset } => {
                // 1 for ContentType tag
                1 + inner.name().size_hint()
                    + Int32::from(inner.size()).size_hint()
                    + Int32::from(inner.checksum()).size_hint()
                    + offset.size_hint()
            }
        }
    }

    pub fn content_size(&self) -> u64 {
        match self {
            Node::Package { size, length, .. } => Int32::from(*length).size_hint() + size,
            Node::Image { inner, .. } => inner.size(),
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
    pub fn add_package(&mut self, name: &str) -> Result<Package<'_, I>, Error> {
        Ok(Package {
            inner: self.add_node(Node::Package {
                name: name.to_string(),
                size: 0,
                offset: Offset::from(0),
                length: 0,
            })?,
        })
    }

    /// Appends an image to the node
    pub fn add_image(&mut self, image: I) -> Result<(), Error> {
        self.add_node(Node::Image {
            offset: Offset::from(0),
            inner: image,
        })?;
        Ok(())
    }

    fn add_node<'b>(&'b mut self, node: Node<I>) -> Result<NodeMut<'b, Node<I>>, Error> {
        self.update_size(node.meta_size() + node.content_size(), true)?;
        Ok(self.inner.append(node))
    }

    fn real_size(&mut self) -> u64 {
        let size = self.inner.value().meta_size();
        size + self.inner.value().content_size()
    }

    fn update_size(&mut self, delta_size: u64, append_child: bool) -> Result<(), Error> {
        let old_real_size = self.real_size();

        // Update length
        if append_child {
            let old_length = self.inner.value().len();
            let new_length = old_length + 1;
            if new_length > MAX_LEN {
                return Err(encode::Error::size(&format!(
                    "{} is too large",
                    self.inner.value().name()
                )))?;
            }
            self.inner.value().set_len(new_length);
        }

        // Update size
        let old_size = self.inner.value().size();
        let new_size = old_size + delta_size;
        if new_size > MAX_SIZE {
            return Err(encode::Error::size(&format!(
                "{} is too large",
                self.inner.value().name()
            )))?;
        }
        self.inner.value().set_size(new_size);

        // Get new real size
        let new_real_size = self.real_size();

        // Update parent
        if let Some(parent) = self.inner.parent() {
            Package { inner: parent }.update_size(new_real_size - old_real_size, false)?;
        }
        Ok(())
    }
}
