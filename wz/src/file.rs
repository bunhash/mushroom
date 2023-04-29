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
use crypto::{Decryptor, KeyStream};
use std::{
    fs::File,
    io::{BufReader, ErrorKind, Read, Seek},
    path::PathBuf,
};

mod content;
mod metadata;
mod raw;

pub use content::{ContentRef, ImageRef, PackageRef};
pub use metadata::Metadata;

use raw::RawContentRef;

/// Represents a WZ file. This object should point to a location on disk.
pub struct WzFile {
    path: PathBuf,
    metadata: Metadata,
}

impl WzFile {
    /// Creates a `WzFile` object by reading the file at `path`
    pub fn open(path: &str) -> Result<Self> {
        let path = PathBuf::from(path);
        let metadata = Metadata::from_reader(File::open(&path)?)?;
        Ok(Self { path, metadata })
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

    /// Creates a new [`WzReader`]. This creates a new file descriptor as well. This method can be
    /// called multiple times. However, it is important to ensure the file is read-only while these
    /// reader(s) exist.
    pub fn to_reader<D>(&self, decryptor: D) -> Result<WzReader<BufReader<File>, D>>
    where
        D: Decryptor,
    {
        Ok(WzReader::new(
            &self.metadata,
            BufReader::new(File::open(&self.path)?),
            decryptor,
        ))
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

    // *** PRIVATES *** //

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
            contents.push(RawContentRef::decode(reader)?);
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
}
