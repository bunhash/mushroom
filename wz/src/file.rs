//! WZ File
//!
//! ```no_run
//! use wz::{error::Result, WzFile};
//!
//! let file = WzFile::open("Base.wz").expect("error reading Base.wz");
//! let map = file.map_unencrypted().expect("error mapping Base.wz");
//! println!("{:?}", map.debug_pretty_print());
//! ```
//!
//! ```no_run
//! use crypto::{Decryptor, KeyStream, GMS_IV, TRIMMED_KEY};
//! use wz::{error::Result, WzFile};
//!
//! let file = WzFile::open("Base.wz").expect("error reading Base.wz");
//! let map = file.map_encrypted(KeyStream::new(&TRIMMED_KEY, &GMS_IV))
//!     .expect("error mapping Base.wz");
//! println!("{:?}", map.debug_pretty_print());
//! ```

use crate::{
    error::{Result, WzError},
    map::{CursorMut, Map, SizeHint},
    reader::DummyDecryptor,
    types::{WzInt, WzOffset, WzString},
    Decode, WzReader,
};
use crypto::{checksum, Decryptor, KeyStream};
use std::{
    fs::File,
    io::{BufReader, ErrorKind, Read, Seek, SeekFrom, Take},
    path::PathBuf,
};

mod content;
mod metadata;
mod raw;

pub use content::{ContentRef, ImageRef, PackageRef};
pub use metadata::Metadata;

use raw::RawContentRef;

/// How the file was opened
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum OpenOption {
    ReadUnknown,
    ReadKnown(u16, u32),
    Write(u16, u32),
}

/// Represents a WZ file. This object should point to a location on disk.
#[derive(Debug, Clone)]
pub struct WzFile {
    path: PathBuf,
    open_option: OpenOption,
    metadata: Metadata,
}

impl WzFile {
    /// Creates a new `WzFile` as write-only. Errors when the file already exists.
    pub fn create(path: &str, version: u16) -> Result<Self> {
        let path = PathBuf::from(path);
        let metadata = Metadata::new(version);
        let (_, version_checksum) = checksum(&version.to_string());
        Ok(Self {
            path,
            open_option: OpenOption::Write(version, version_checksum),
            metadata,
        })
    }

    /// Creates a `WzFile` as read-only. Errors when it cannot read the file.
    pub fn open(path: &str) -> Result<Self> {
        let path = PathBuf::from(path);
        let metadata = Metadata::from_reader(File::open(&path)?)?;
        Ok(Self {
            path,
            open_option: OpenOption::ReadUnknown,
            metadata,
        })
    }

    /// Creates a `WzFile` as read-only. Errors when it cannot read the file.
    pub fn open_as_version(path: &str, version: u16) -> Result<Self> {
        let path = PathBuf::from(path);
        let metadata = Metadata::from_reader(File::open(&path)?)?;
        let (calc_version, version_checksum) = checksum(&version.to_string());
        if calc_version != metadata.encrypted_version {
            Err(WzError::InvalidChecksum.into())
        } else {
            Ok(Self {
                path,
                open_option: OpenOption::ReadKnown(version, version_checksum),
                metadata,
            })
        }
    }

    /// Returns a list of possible versions
    pub fn possible_versions(&self) -> Vec<(u16, u32)> {
        Metadata::possible_versions(self.metadata.encrypted_version)
    }

    /// Gets the filename of the WZ file
    pub fn file_name(&self) -> Result<&str> {
        match self.path.file_name() {
            Some(name) => match name.to_str() {
                Some(name) => Ok(name),
                None => return Err(ErrorKind::NotFound.into()),
            },
            None => return Err(ErrorKind::NotFound.into()),
        }
    }

    /// Returns an immutable reference to the metadata
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Guess the WZ version
    pub fn guess_version<D>(&mut self, decryptor: D) -> Result<()>
    where
        D: Decryptor,
    {
        if self.open_option == OpenOption::ReadUnknown {
            let (version, version_checksum) = self.bruteforce_version(decryptor)?;
            self.open_option = OpenOption::ReadKnown(version, version_checksum);
        }
        Ok(())
    }

    /// Creates a new [`WzReader`]. This creates a new file descriptor as well. This method can be
    /// called multiple times. However, it is important to ensure the file is read-only while these
    /// reader(s) exist.
    pub fn to_reader<D>(&self, decryptor: D) -> Result<WzReader<BufReader<File>, D>>
    where
        D: Decryptor,
    {
        match self.open_option {
            OpenOption::ReadUnknown => Err(WzError::InvalidChecksum.into()),
            OpenOption::ReadKnown(_, version_checksum) => Ok(WzReader::new(
                self.metadata.absolute_position,
                version_checksum,
                BufReader::new(File::open(&self.path)?),
                decryptor,
            )),
            OpenOption::Write(_, _) => Err(WzError::WriteOnly.into()),
        }
    }

    /// Creates a new unencrypted [`WzReader`]. See [`WzFile::to_reader`].
    pub fn to_unencrypted_reader(&self) -> Result<WzReader<BufReader<File>, DummyDecryptor>> {
        self.to_reader(DummyDecryptor)
    }

    /// Creates a new encrypted [`WzReader`]. See [`WzFile::to_reader`].
    pub fn to_encrypted_reader(
        &self,
        keystream: KeyStream,
    ) -> Result<WzReader<BufReader<File>, KeyStream>> {
        self.to_reader(keystream)
    }

    /// Maps the WZ file at a specific offset. This offset is expected to be a Package content
    /// type. The decryptor is used to decrypt strings. A [`DummyDecryptor`] is provided for WZ
    /// files that do not use string encryption.
    pub fn map_at<D>(&self, name: &str, offset: WzOffset, decryptor: D) -> Result<Map<ContentRef>>
    where
        D: Decryptor,
    {
        let mut reader = self.to_reader(decryptor)?;
        reader.seek(offset)?;
        let name = WzString::from(name);
        let content = ContentRef::Package(PackageRef {
            name_size: name.size_hint(),
            size: WzInt::from(0),
            checksum: WzInt::from(0),
            offset,
            num_content: WzInt::from(0),
        });
        let mut map = Map::new(name, content);
        let mut cursor = map.cursor_mut();
        for raw_content in WzFile::decode_package_contents(&mut reader)? {
            WzFile::map_content_to(&raw_content, &mut cursor, &mut reader)?;
        }
        Ok(map)
    }

    /// Maps the WZ file at a specific offset without string encryption. See
    /// [`map_at`](WzFile::map_at) for more information.
    pub fn map_unencrypted_at(&self, name: &str, offset: WzOffset) -> Result<Map<ContentRef>> {
        self.map_at(name, offset, DummyDecryptor)
    }

    /// Maps the WZ file at a specific offset while decrypting string. See
    /// [`map_at`](WzFile::map_at) for more information.
    pub fn map_encrypted_at(
        &self,
        name: &str,
        offset: WzOffset,
        keystream: KeyStream,
    ) -> Result<Map<ContentRef>> {
        self.map_at(name, offset, keystream)
    }

    /// Maps the entire WZ file starting from the absolute position. The decryptor is used to
    /// decrypt strings. A [`DummyDecryptor`] is provided for WZ files that do not use string
    /// encryption.
    pub fn map<D>(&self, decryptor: D) -> Result<Map<ContentRef>>
    where
        D: Decryptor,
    {
        let offset = WzOffset::from(self.metadata.absolute_position + 2);
        self.map_at(self.file_name()?, offset, decryptor)
    }

    /// Maps the entire WZ file without decrypting any strings. If the WZ file is encrypted, it is
    /// very likely UTF-8/UTF-16 errors will occur trying to decode the strings from bytes.
    pub fn map_unencrypted(&self) -> Result<Map<ContentRef>> {
        self.map(DummyDecryptor)
    }

    /// Maps the entire WZ file while decrypting the strings. If the WZ file is not encrypted, it
    /// is very likely UTF-8/UTF-16 errors will occur trying to decode the strings from bytes.
    pub fn map_encrypted(&self, keystream: KeyStream) -> Result<Map<ContentRef>> {
        self.map(keystream)
    }

    /// Creates a reader for [`ImageRef`] objects.
    pub fn image_reader(&self, image_ref: &ImageRef) -> Result<Take<BufReader<File>>> {
        match self.open_option {
            OpenOption::ReadUnknown => Err(WzError::InvalidChecksum.into()),
            OpenOption::ReadKnown(_, _) => {
                let mut reader = BufReader::new(File::open(&self.path)?);
                reader.seek(SeekFrom::Start(*image_ref.offset as u64))?;
                Ok(reader.take(*image_ref.size as u64))
            }
            OpenOption::Write(_, _) => Err(WzError::WriteOnly.into()),
        }
    }

    /// Calculates the checksums and offsets
    pub fn calculate_checksum_and_offsets(&self, map: &mut Map<ContentRef>) -> Result<()> {
        let mut cursor = map.cursor_mut();
        WzFile::recursive_calculate_checksum(&mut cursor)?;
        WzFile::recursive_calculate_offset(
            WzOffset::from(self.metadata.absolute_position as u32 + 2),
            &mut cursor,
        )?;
        Ok(())
    }

    // *** PRIVATES *** //

    fn bruteforce_version<D>(&self, decryptor: D) -> Result<(u16, u32)>
    where
        D: Decryptor,
    {
        let lower_bound = WzOffset::from(self.metadata.absolute_position as u32);
        let upper_bound =
            WzOffset::from(self.metadata.absolute_position as u32 + self.metadata.size as u32);
        let mut reader = WzReader::new(
            self.metadata.absolute_position,
            0u32,
            BufReader::new(File::open(&self.path)?),
            decryptor,
        );
        for (version, version_checksum) in self.possible_versions() {
            reader.set_version_checksum(version_checksum);
            reader.seek(WzOffset::from(self.metadata.absolute_position + 2))?;

            // Decodes the top-level directory contents. If all contents lie within the lower and
            // upper bounds, we can assume the version checksum is good.
            let contents = WzFile::decode_package_contents(&mut reader)?;
            let filtered = contents
                .iter()
                .map(|raw_content| raw_content.offset)
                .filter(|off| *off >= lower_bound && *off < upper_bound)
                .collect::<Vec<WzOffset>>();
            if contents.len() == filtered.len() {
                return Ok((version, version_checksum));
            }
        }
        Err(WzError::BruteForceChecksum.into())
    }

    fn decode_package_contents<R, D>(reader: &mut WzReader<R, D>) -> Result<Vec<RawContentRef>>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let num_contents = WzInt::decode(reader)?;
        if num_contents.is_negative() {
            return Err(WzError::InvalidPackage.into());
        }
        let num_contents = *num_contents as usize;
        let mut contents = Vec::with_capacity(num_contents);
        for _ in 0..num_contents {
            let content = RawContentRef::decode(reader)?;
            contents.push(content);
        }
        Ok(contents)
    }

    fn map_content_to<'a, R, D>(
        raw_content: &RawContentRef,
        cursor: &mut CursorMut<'a, ContentRef>,
        reader: &mut WzReader<R, D>,
    ) -> Result<()>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        cursor.create(raw_content.name.clone(), raw_content.try_into()?)?;
        if raw_content.tag == 3 {
            reader.seek(raw_content.offset)?;
            cursor.move_to(raw_content.name.as_ref())?;
            for raw_content in WzFile::decode_package_contents(reader)? {
                WzFile::map_content_to(&raw_content, cursor, reader)?;
            }
            cursor.parent()?;
        }
        Ok(())
    }

    fn metadata_size(content_ref: &ContentRef) -> u32 {
        match content_ref {
            ContentRef::Package(ref package) => {
                1 + *package.name_size as u32
                    + *package.size.size_hint() as u32
                    + *package.checksum.size_hint() as u32
                    + *package.offset.size_hint() as u32
            }
            ContentRef::Image(ref image) => {
                1 + *image.name_size as u32
                    + *image.size.size_hint() as u32
                    + *image.checksum.size_hint() as u32
                    + *image.offset.size_hint() as u32
            }
        }
    }

    fn recursive_calculate_checksum(cursor: &mut CursorMut<ContentRef>) -> Result<()> {
        // Calculate the sibling offset and return the number of children
        let mut num_content = match cursor.get() {
            ContentRef::Package(ref package) => package.num_content,
            // If it is an image, return the next checksum and stop here. Image's have no children.
            ContentRef::Image(_) => return Ok(()),
        };

        let mut checksum = 0i32;
        if num_content > 0 {
            // Calculate children checksums first
            cursor.first_child()?;
            loop {
                WzFile::recursive_calculate_checksum(cursor)?;
                num_content = num_content - 1;
                if num_content <= 0 {
                    break;
                }
                cursor.next_sibling()?;
            }
            cursor.parent()?;

            // Sum children checksums (overflow may occur here...)
            for child in cursor.children() {
                checksum = checksum.wrapping_add(*child.checksum());
            }
        }

        // Set the checksum
        cursor.modify::<WzError>(|content| match content {
            ContentRef::Package(ref mut package) => Ok(package.checksum = WzInt::from(checksum)),
            ContentRef::Image(_) => panic!("should not get here..."),
        })?;

        Ok(())
    }

    fn recursive_calculate_offset(
        current_offset: WzOffset,
        cursor: &mut CursorMut<ContentRef>,
    ) -> Result<WzOffset> {
        // Apply the current offset
        cursor.modify::<WzError>(|content| match content {
            ContentRef::Package(ref mut package) => Ok(package.offset = current_offset),
            ContentRef::Image(ref mut image) => Ok(image.offset = current_offset),
        })?;

        // Calculate the sibling offset and return the number of children
        let (next_offset, mut num_content) = match cursor.get() {
            ContentRef::Package(ref package) => (
                current_offset + *(package.num_content.size_hint() + package.size) as u32,
                package.num_content,
            ),
            // If it is an image, return the next offset and stop here. Image's have no children.
            ContentRef::Image(ref image) => return Ok(current_offset + *image.size as u32),
        };

        if num_content > 0 {
            // Calculate the start of the children (after all the metadata)
            let metadata_size = WzOffset::from(
                *num_content.size_hint() as u32
                    + cursor
                        .children()
                        .map(|child| WzFile::metadata_size(child))
                        .sum::<u32>(),
            );

            // Modify children. The order is always the order of insertion.
            let mut child_offset = current_offset + metadata_size;
            cursor.first_child()?;
            loop {
                child_offset = WzFile::recursive_calculate_offset(child_offset, cursor)?;
                num_content = num_content - 1;
                if num_content <= 0 {
                    break;
                }
                cursor.next_sibling()?;
            }
            cursor.parent()?;
        }

        Ok(next_offset)
    }
}
