//! WZ Archive Builder

use crate::{
    encode::SizeHint,
    error::{Error, Result, WzError},
    file::{
        package::{ContentRef, Metadata},
        Header,
    },
    map::{CursorMut, Map},
    types::{WzInt, WzOffset, WzString},
    writer::DummyEncryptor,
    Encode, WzWriter,
};
use crypto::{checksum, Encryptor};
use std::{
    convert::AsRef,
    ffi::OsStr,
    fs::File,
    io::{self, BufWriter, ErrorKind, Seek, Write},
    num::Wrapping,
    path::Path,
};

/// Trait for representing Images
pub trait ImageRef {
    /// Returns the size of the serialized Image
    fn size(&self) -> Result<WzInt>;

    /// Returns the checksum of the serialized Image
    fn checksum(&self) -> Result<WzInt>;

    /// Writes the serialized Image
    fn write<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<()>
    where
        W: Write + Seek,
        E: Encryptor;
}

#[derive(Debug)]
pub enum Node<I>
where
    I: ImageRef,
{
    Package {
        size: WzInt,
        checksum: WzInt,
        offset: WzOffset,
    },
    Image {
        image: I,
        offset: WzOffset,
    },
}

#[derive(Debug)]
pub struct Builder<I>
where
    I: ImageRef,
{
    map: Map<Node<I>>,
}

impl<I> Builder<I>
where
    I: ImageRef,
{
    pub fn new(name: &str) -> Self {
        Self {
            map: Map::new(
                WzString::from(name),
                Node::Package {
                    size: WzInt::from(0),
                    checksum: WzInt::from(0),
                    offset: WzOffset::from(0),
                },
            ),
        }
    }

    pub fn map(&self) -> &Map<Node<I>> {
        &self.map
    }

    pub fn add_package<S>(&mut self, path: &S) -> Result<()>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        self.make_package_path(path)?;
        Ok(())
    }

    pub fn add_image<S>(&mut self, path: &S, image: I) -> Result<()>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        let path = Path::new(path);
        let parent = match path.parent() {
            Some(p) => p,
            None => return Err(WzError::InvalidPathName.into()),
        };
        let name = match path.file_name() {
            Some(name) => match name.to_str() {
                Some(name) => name,
                None => return Err(WzError::InvalidPathName.into()),
            },
            None => return Err(WzError::InvalidPathName.into()),
        };
        let mut cursor = self.make_package_path(parent)?;
        cursor.create(
            WzString::from(name),
            Node::Image {
                image,
                offset: WzOffset::from(0),
            },
        )?;
        Ok(())
    }

    // *** PRIVATES *** //

    fn make_package_path<S>(&mut self, path: &S) -> Result<CursorMut<Node<I>>>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        let mut cursor = self.map.cursor_mut();
        let path = match Path::new(path).strip_prefix(cursor.name()) {
            Ok(p) => p,
            Err(_) => return Err(WzError::MultipleRoots.into()),
        };
        for part in path.iter() {
            let name = match part.to_str() {
                Some(n) => n,
                None => return Err(WzError::InvalidPathName.into()),
            };
            if !cursor.has_child(name) {
                cursor.create(
                    WzString::from(name),
                    Node::Package {
                        size: WzInt::from(0),
                        checksum: WzInt::from(0),
                        offset: WzOffset::from(0),
                    },
                )?;
            }
            cursor.move_to(name)?;
        }
        Ok(cursor)
    }

    pub fn calculate_metadata(
        &mut self,
        absolute_position: i32,
        version_checksum: u32,
    ) -> Result<()> {
        let mut cursor = self.map.cursor_mut();
        recursive_calculate_size_and_checksum(absolute_position, version_checksum, &mut cursor)?;
        Ok(())
    }
}

fn encode_obj<T>(absolute_position: i32, version_checksum: u32, obj: &T) -> Result<Vec<u8>>
where
    T: Encode,
{
    let mut dummy_writer = WzWriter::new(
        absolute_position,
        version_checksum,
        io::Cursor::new(Vec::new()),
        DummyEncryptor,
    );
    obj.encode(&mut dummy_writer)?;
    Ok(dummy_writer.into_inner().into_inner())
}

fn recursive_calculate_size_and_checksum<I>(
    absolute_position: i32,
    version_checksum: u32,
    cursor: &mut CursorMut<Node<I>>,
) -> Result<(WzInt, WzInt)>
where
    I: ImageRef,
{
    // Calculate the sibling offset and return the number of children
    let mut num_children = match cursor.get() {
        Node::Package { .. } => cursor.children().count(),
        Node::Image { ref image, .. } => 0,
    };

    let num_content = encode_obj(
        absolute_position,
        version_checksum,
        &WzInt::from(num_children as i32),
    )?;

    // Set the size to 0--num_content is part of the package "size"
    let mut calc_size = Wrapping(0);

    // Set checksum to 0--not sure if the checksum includes num_content. But since size does not, I
    // felt it was safe to assume checksum doesn't either.
    let mut calc_checksum = Wrapping(0);

    if num_children > 0 {
        // Calculate children checksums first
        cursor.first_child()?;
        loop {
            // Calculate the checksum of the child and get its encoded size
            let (child_size, child_checksum) =
                recursive_calculate_size_and_checksum(absolute_position, version_checksum, cursor)?;
            calc_size = calc_size + Wrapping(*child_size);
            calc_checksum = calc_checksum + Wrapping(*child_checksum);
            num_children = num_children - 1;
            if num_children <= 0 {
                break;
            }
            cursor.next_sibling()?;
        }
        cursor.parent()?;
    }

    match cursor.get_mut() {
        // Set the size and checksum of the package
        Node::Package {
            ref mut size,
            ref mut checksum,
            ..
        } => {
            *size = WzInt::from(calc_size.0);
            *checksum = WzInt::from(calc_checksum.0);
        }
        // Skip for images
        Node::Image { .. } => {}
    };

    // Encode the content metadata
    let name = WzString::from(cursor.name());
    let content_ref = match cursor.get_mut() {
        Node::Package {
            size,
            checksum,
            offset,
        } => ContentRef::Package(Metadata::new(name, *size, *checksum, *offset)),
        Node::Image { image, offset } => ContentRef::Image(Metadata::new(
            name,
            image.size()?,
            image.checksum()?,
            *offset,
        )),
    };
    let content_data = encode_obj(absolute_position, version_checksum, &content_ref)?;

    // Include content metadata here
    let (calc_size, calc_checksum) = match cursor.get() {
        Node::Package { .. } => (
            calc_size + Wrapping(num_content.len() as i32) + Wrapping(content_ref.size_hint()),
            calc_checksum
                + num_content
                    .iter()
                    .map(|b| Wrapping(*b as i32))
                    .sum::<Wrapping<i32>>()
                + content_data
                    .iter()
                    .map(|b| Wrapping(*b as i32))
                    .sum::<Wrapping<i32>>(),
        ),
        Node::Image { image, .. } => (
            Wrapping(*image.size()?) + Wrapping(content_ref.size_hint()),
            Wrapping(*image.checksum()?)
                + content_data
                    .iter()
                    .map(|b| Wrapping(*b as i32))
                    .sum::<Wrapping<i32>>(),
        ),
    };
    Ok((WzInt::from(calc_size.0), WzInt::from(calc_checksum.0)))
}
