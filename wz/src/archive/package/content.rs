//! WZ Archive Package Content types

use crate::{
    archive::{Error, Offset},
    decode::{Decode, Reader},
    Int32,
};
use crypto::Decryptor;
use std::{
    fmt,
    io::{Read, Seek},
};

/// Content
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct Content {
    /// Type of content
    pub content_type: ContentType,

    /// Size of content
    pub size: Int32,

    /// Content checksum
    pub checksum: Int32,

    /// Offset of content in the archive
    pub offset: Offset,
}

impl Content {
    /// Returns the name of the content
    pub fn name(&self) -> &str {
        match &self.content_type {
            ContentType::Package(name) => name.as_str(),
            ContentType::Image(name) => name.as_str(),
        }
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self.content_type {
            ContentType::Package(name) => write!(
                f,
                "Package({}, size={}, checksum={}, offset={})",
                name, self.size, self.checksum, self.offset
            ),
            ContentType::Image(name) => write!(
                f,
                "Image({}, size={}, checksum={}, offset={})",
                name, self.size, self.checksum, self.offset
            ),
        }
    }
}

impl Decode for Content {
    type Error = Error;

    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Self::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let content_type = ContentType::decode(reader)?;
        let size = Int32::decode(reader)?;
        if size.is_negative() {
            return Err(Error::Size(*size));
        }
        let checksum = Int32::decode(reader)?;
        let offset = Offset::decode(reader)?;
        Ok(Content {
            content_type,
            size,
            checksum,
            offset,
        })
    }
}

/// Content Types
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum ContentType {
    /// Package tag=3
    Package(String),

    /// Image tag=4
    Image(String),
}

impl Decode for ContentType {
    type Error = Error;

    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Self::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        Ok(match reader.read_byte()? {
            2 => {
                // A tag of 2 indicates a reference elsewhere. This is probably used as a form of
                // compression so duplicate strings are only written once to the WZ archive. This
                // just de-references the strings. Writing duplicate strings to in a WZ pacakge
                // should not be invalid when parsing. Maybe in the future, I can implement this
                // form of compression but I'd rather not waste the time now.

                // Read the offset of where the name and "real" tag are located
                let offset = i32::decode(reader)?;
                if offset.is_negative() {
                    // sanity check
                    return Err(Error::Offset(offset));
                }
                reader.decode_at::<ContentType>(offset as u32)?
            }
            3 => ContentType::Package(String::decode(reader)?),
            4 => ContentType::Image(String::decode(reader)?),
            t => Err(Error::Tag(t))?,
        })
    }
}
