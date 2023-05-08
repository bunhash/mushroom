//! WZ Archive Builder

use crate::{
    error::{Result, WzError},
    file::{
        package::{ContentRef, Metadata},
        Header,
    },
    io::{encode::SizeHint, DummyEncryptor, Encode, WzWriter},
    map::{Cursor, CursorMut, Map},
    types::{WzInt, WzOffset, WzString},
};
use crypto::{checksum, Encryptor};
use std::{
    convert::AsRef,
    ffi::OsStr,
    fs::File,
    io::{self, Seek, Write},
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

/// Map node representing the contents of the WZ archive
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

/// WZ archive builder. Structure for building a WZ archive from a file system directory. The
/// archive's root directory should only contain other directories and images. It will treat all
/// non-directory files as images.
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
    /// Creates a new builder with a root directory named `name`. This name is unimportant and
    /// won't be written to the WZ archive. However, [`Map`] requires a name and it helps organize
    /// the data.
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

    /// Returns a reference to the inner map
    pub fn map(&self) -> &Map<Node<I>> {
        &self.map
    }

    /// Adds a package to the builder. A package is essentially a directory but WZ calls it a
    /// package. When it and its contents are serialized, it is treated as a binary blob.
    ///
    /// `path` should be a [`OsStr`]. Example: "root/subdir_a/subdir_b" where `root` is the name given to the
    /// builder at creation. The entire chain of packages will be created if they don't already
    /// exist.
    ///
    /// Errors when `path` does not start with the root package name.
    pub fn add_package<S>(&mut self, path: &S) -> Result<()>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        self.make_package_path(path)?;
        Ok(())
    }

    /// Adds an image to the builder. An image is treated as a binary blob.
    ///
    /// `path` should be a [`OsStr`]. Example: "root/subdir_a/image_a.img" where `root` is the name
    /// given to the builder at creation. The entire chain of packages preceeding the image will be
    /// created if they don't already exist.
    ///
    /// Errors when `path` does not start with the root package name or when a package or image
    /// already exists at the specified `path`.
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

    /// Generates the WZ archive and writes it to disk.
    ///
    /// The version must match the [`Header`] and should match the added imges. If the image versions do
    /// not match the version provided here, decoding offsets contained in the images may not align
    /// properly.
    ///
    /// Errors when the provided version does not match the header's version hash. Or if any IO
    /// error occurs.
    pub fn save<E>(
        &mut self,
        version: u16,
        mut header: Header,
        mut file: File,
        encryptor: E,
    ) -> Result<()>
    where
        E: Encryptor,
    {
        let absolute_position = header.absolute_position;
        let (version_hash, version_checksum) = checksum(&version.to_string());
        if version_hash != header.version_hash {
            return Err(WzError::InvalidChecksum.into());
        }
        self.calculate_metadata(absolute_position, version_checksum)?;

        // Modify the header sizes
        let cursor = self.map.cursor();
        let root_num_content = WzInt::from(cursor.children().count() as i32);
        header.size = match cursor.get() {
            Node::Package { ref size, .. } => {
                **size as u64 + root_num_content.size_hint() as u64 + 2
            }
            _ => panic!("should never get here"),
        };

        let mut writer = WzWriter::new(absolute_position, version_checksum, &mut file, encryptor);
        header.encode(&mut writer)?;
        recursive_save(&mut self.map.cursor(), &mut writer)
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

    fn calculate_metadata(&mut self, absolute_position: i32, version_checksum: u32) -> Result<()> {
        recursive_calculate_size_and_checksum(
            absolute_position,
            version_checksum,
            &mut self.map.cursor_mut(),
        )?;
        recursive_calculate_offset(
            WzOffset::from(absolute_position as u32 + 2),
            &mut self.map.cursor_mut(),
        )?;
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

/// Calculates the size and checksum of everything recursively
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
        Node::Image { .. } => 0,
    };

    let num_content = encode_obj(
        absolute_position,
        version_checksum,
        &WzInt::from(num_children as i32),
    )?;

    // Set the size to 0--num_content is part of the package "size". It should panic if it
    // overflows.
    let mut calc_size = 0;

    // Set checksum to 0--not sure if the checksum includes num_content. But since size does not, I
    // felt it was safe to assume checksum doesn't either. Doesn't matter if it overflows.
    let mut calc_checksum = Wrapping(0);

    if num_children > 0 {
        // Calculate children checksums first
        cursor.first_child()?;
        loop {
            // Calculate the checksum of the child and get its encoded size
            let (child_size, child_checksum) =
                recursive_calculate_size_and_checksum(absolute_position, version_checksum, cursor)?;
            calc_size = calc_size + *child_size;
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
            *size = WzInt::from(calc_size);
            *checksum = WzInt::from(calc_checksum.0);
        }
        // Skip for images
        Node::Image { .. } => {}
    };

    // Encode the content metadata
    let name = WzString::from(cursor.name());
    let content_ref = match cursor.get() {
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
            calc_size + num_content.len() as i32 + content_ref.size_hint() as i32,
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
            *image.size()? + content_ref.size_hint() as i32,
            Wrapping(*image.checksum()?)
                + content_data
                    .iter()
                    .map(|b| Wrapping(*b as i32))
                    .sum::<Wrapping<i32>>(),
        ),
    };
    Ok((WzInt::from(calc_size), WzInt::from(calc_checksum.0)))
}

/// Calculates the offsets. If overflow occurs here, it should absolutely panic.
fn recursive_calculate_offset<I>(
    current_offset: WzOffset,
    cursor: &mut CursorMut<Node<I>>,
) -> Result<WzOffset>
where
    I: ImageRef,
{
    // Apply the current offset
    match cursor.get_mut() {
        Node::Package { ref mut offset, .. } => *offset = current_offset,
        Node::Image { ref mut offset, .. } => *offset = current_offset,
    }

    // Calculate the sibling offset and return the number of children
    let next_offset = match cursor.get() {
        Node::Package { size, .. } => *current_offset + u32::from(*size),
        // If it is an image, return the next offset and stop here. Image's have no children.
        Node::Image { ref image, .. } => return Ok(current_offset + *image.size()? as u32),
    };

    // Get num content dn update next_offset
    let num_content = cursor.children().count() as i32;
    let header_size = WzInt::from(num_content).size_hint() as i32;
    let next_offset = WzOffset::from(next_offset + header_size as u32);

    if num_content > 0 {
        // Total the metadata size to get the position of the first child
        let mut metadata_size = header_size;
        let mut count = num_content;
        cursor.first_child()?;
        loop {
            let name = WzString::from(cursor.name());
            let content_ref = match cursor.get() {
                Node::Package {
                    ref size,
                    ref checksum,
                    ref offset,
                } => ContentRef::Package(Metadata::new(name, *size, *checksum, *offset)),
                Node::Image {
                    ref image,
                    ref offset,
                } => ContentRef::Image(Metadata::new(
                    name,
                    image.size()?,
                    image.checksum()?,
                    *offset,
                )),
            };
            metadata_size = metadata_size + content_ref.size_hint() as i32;
            count = count - 1;
            if count <= 0 {
                break;
            }
            cursor.next_sibling()?;
        }
        cursor.parent()?;

        // Modify children. The order is always the order of insertion.
        let mut child_offset = WzOffset::from(current_offset + metadata_size);
        let mut count = num_content;
        cursor.first_child()?;
        loop {
            child_offset = recursive_calculate_offset(child_offset, cursor)?;
            count = count - 1;
            if count <= 0 {
                break;
            }
            cursor.next_sibling()?;
        }
        cursor.parent()?;
    }

    Ok(next_offset)
}

/// Saves the WZ archive recursively
fn recursive_save<I, W, E>(cursor: &mut Cursor<Node<I>>, writer: &mut WzWriter<W, E>) -> Result<()>
where
    I: ImageRef,
    W: Write + Seek,
    E: Encryptor,
{
    let num_content = match cursor.get() {
        // Get number of children
        Node::Package { .. } => cursor.children().count() as i32,
        // Write the image and return
        Node::Image { ref image, .. } => return image.write(writer),
    };

    // Encode the length
    WzInt::from(num_content).encode(writer)?;
    if num_content > 0 {
        // Encode the package metadata
        let mut count = num_content;
        cursor.first_child()?;
        loop {
            let name = WzString::from(cursor.name());
            let content_ref = match cursor.get() {
                Node::Package {
                    ref size,
                    ref checksum,
                    ref offset,
                } => ContentRef::Package(Metadata::new(name, *size, *checksum, *offset)),
                Node::Image {
                    ref image,
                    ref offset,
                } => ContentRef::Image(Metadata::new(
                    name,
                    image.size()?,
                    image.checksum()?,
                    *offset,
                )),
            };
            content_ref.encode(writer)?;
            count = count - 1;
            if count <= 0 {
                break;
            }
            cursor.next_sibling()?;
        }
        cursor.parent()?;

        // Encode the children
        let mut count = num_content;
        cursor.first_child()?;
        loop {
            recursive_save(cursor, writer)?;
            count = count - 1;
            if count <= 0 {
                break;
            }
            cursor.next_sibling()?;
        }
        cursor.parent()?;
    }

    Ok(())
}
