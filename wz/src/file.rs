//! WZ File

use crate::{
    error::{Result, WzError},
    map::{CursorMut, Map},
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

pub use content::{Content, Params};
pub use metadata::Metadata;

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

    /// Gets the filename of the provided path
    pub fn file_name(&self) -> Result<&str> {
        match self.path.file_name() {
            Some(name) => match name.to_str() {
                Some(name) => Ok(name),
                None => return Err(ErrorKind::NotFound.into()),
            },
            None => return Err(ErrorKind::NotFound.into()),
        }
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

    pub fn map_at<D>(&self, name: &str, offset: WzOffset, decryptor: D) -> Result<Map<Content>>
    where
        D: Decryptor,
    {
        let mut reader = self.to_reader(decryptor)?;
        reader.seek(offset)?;
        let mut map = Map::new(
            WzString::from(name),
            Content::Package(
                WzString::from(name),
                Params {
                    size: WzInt::from(0),
                    checksum: WzInt::from(0),
                    offset,
                },
                WzInt::from(0),
            ),
        );
        let mut cursor = map.cursor_mut();
        for content in WzFile::decode_package_contents(&mut reader)? {
            WzFile::map_content_to(content, &mut cursor, &mut reader)?;
        }
        Ok(map)
    }

    pub fn map_unencrypted_at(&self, name: &str, offset: WzOffset) -> Result<Map<Content>> {
        self.map_at(name, offset, DummyDecryptor)
    }

    pub fn map_encrypted_at(
        &self,
        name: &str,
        offset: WzOffset,
        keystream: KeyStream,
    ) -> Result<Map<Content>> {
        self.map_at(name, offset, keystream)
    }

    pub fn map<D>(&self, decryptor: D) -> Result<Map<Content>>
    where
        D: Decryptor,
    {
        let offset = WzOffset::from(self.metadata.absolute_position + 2);
        self.map_at(self.file_name()?, offset, decryptor)
    }

    pub fn map_unencrypted(&self) -> Result<Map<Content>> {
        self.map(DummyDecryptor)
    }

    pub fn map_encrypted(&self, keystream: KeyStream) -> Result<Map<Content>> {
        self.map(keystream)
    }

    // *** PRIVATES *** //

    fn decode_package_contents<R, D>(reader: &mut WzReader<R, D>) -> Result<Vec<Content>>
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
            contents.push(Content::decode(reader)?);
        }
        Ok(contents)
    }

    fn map_content_to<'a, R, D>(
        content: Content,
        cursor: &mut CursorMut<'a, Content>,
        reader: &mut WzReader<R, D>,
    ) -> Result<()>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let (name, offset) = match &content {
            Content::Package(name, params, _) => (name.clone(), Some(params.offset)),
            Content::Image(name, _) => (name.clone(), None),
        };
        cursor.create(name.clone(), content)?;
        match offset {
            Some(offset) => {
                reader.seek(offset)?;
                cursor.move_to(name.as_ref())?;
                for content in WzFile::decode_package_contents(reader)? {
                    WzFile::map_content_to(content, cursor, reader)?;
                }
                cursor.parent()?;
                Ok(())
            }
            None => Ok(()),
        }
    }
}
