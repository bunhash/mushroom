//! Content Metadata

use crate::{
    decode, encode,
    types::{WzInt, WzOffset, WzString},
    Decode, Encode, WzReader, WzWriter,
};
use crypto::{Decryptor, Encryptor};
use std::io::{Read, Seek, Write};

/// Content Types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentRef {
    Package(Metadata),
    Image(Metadata),
}

impl ContentRef {
    pub fn name(&self) -> &str {
        match &self {
            ContentRef::Package(ref data) => data.name(),
            ContentRef::Image(ref data) => data.name(),
        }
    }

    pub fn size(&self) -> WzInt {
        match &self {
            ContentRef::Package(ref data) => data.size(),
            ContentRef::Image(ref data) => data.size(),
        }
    }

    pub fn checksum(&self) -> WzInt {
        match &self {
            ContentRef::Package(ref data) => data.checksum(),
            ContentRef::Image(ref data) => data.checksum(),
        }
    }

    pub fn offset(&self) -> WzOffset {
        match &self {
            ContentRef::Package(ref data) => data.offset(),
            ContentRef::Image(ref data) => data.offset(),
        }
    }
}

impl Decode for ContentRef {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let tag = reader.read_byte()?;
        let (tag, name, size, checksum, offset) = match tag {
            2 => {
                // A tag of 2 indicates a reference elsewhere. This is probably used as a form of
                // compression so duplicate strings are only written once to the WZ archive. This
                // just de-references the strings. Writing duplicate strings to in a WZ pacakge
                // should not be invalid when parsing. Maybe in the future, I can implement this
                // form of compression but I'd rather not waste the time now.

                // Read the offset of where the name and "real" tag are located
                let off = i32::decode(reader)?;

                // Read the rest while the content is still in the buffer
                let size = WzInt::decode(reader)?;
                let checksum = WzInt::decode(reader)?;
                let offset = WzOffset::decode(reader)?;
                let (tag, name) = Metadata::dereference_name(off, reader)?;

                (tag, name, size, checksum, offset)
            }
            3 | 4 => (
                tag,
                WzString::decode(reader)?,
                WzInt::decode(reader)?,
                WzInt::decode(reader)?,
                WzOffset::decode(reader)?,
            ),
            t => return Err(decode::Error::InvalidContentType(t)),
        };
        match tag {
            3 => Ok(ContentRef::Package(Metadata {
                name,
                size,
                checksum,
                offset,
            })),
            4 => Ok(ContentRef::Image(Metadata {
                name,
                size,
                checksum,
                offset,
            })),
            t => Err(decode::Error::InvalidContentType(t)),
        }
    }
}

impl Encode for ContentRef {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        match &self {
            ContentRef::Package(ref data) => {
                3u8.encode(writer)?;
                data.encode(writer)
            }
            ContentRef::Image(ref data) => {
                4u8.encode(writer)?;
                data.encode(writer)
            }
        }
    }
}

/// Content metadata
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    /// Name of the content
    pub(crate) name: WzString,

    /// Size of the content
    pub(crate) size: WzInt,

    /// Checksum of the content. Sum of all the bytes
    pub(crate) checksum: WzInt,

    /// Position of the content within the WZ archive
    pub(crate) offset: WzOffset,
}

impl Metadata {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn size(&self) -> WzInt {
        self.size
    }

    pub fn checksum(&self) -> WzInt {
        self.checksum
    }

    pub fn offset(&self) -> WzOffset {
        self.offset
    }

    fn dereference_name<R, D>(
        offset: i32,
        reader: &mut WzReader<R, D>,
    ) -> Result<(u8, WzString), decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        if offset.is_negative() {
            // sanity check
            return Err(decode::Error::InvalidOffset(offset));
        }

        // Get current position
        let pos = reader.position()?;

        // Move to the referenced content--this empties the buffer
        reader.seek_from_start(offset as u32)?;

        // Read the "real" tag and name
        let tag = reader.read_byte()?;
        let name = WzString::decode(reader)?;

        // Seek back
        reader.seek(pos)?;

        // Sanity check to ensure the tag is valid
        match tag {
            3 | 4 => Ok((tag, name)),
            t => Err(decode::Error::InvalidContentType(t)),
        }
    }
}

impl Encode for Metadata {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        self.name.encode(writer)?;
        self.size.encode(writer)?;
        self.checksum.encode(writer)?;
        self.offset.encode(writer)
    }
}
