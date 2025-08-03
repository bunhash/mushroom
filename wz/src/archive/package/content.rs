//! WZ Archive Package Content types

use crate::{
    Int32,
    archive::{Error, Offset},
    decode::{Decode, Decoder},
    encode::{Encode, Encoder, SizeHint},
};
use std::fmt;

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
        self.content_type.name()
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

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        let content_type = ContentType::decode(decoder)?;
        let size = Int32::decode(decoder)?;
        if size.is_negative() {
            return Err(Error::Size(*size));
        }
        let checksum = Int32::decode(decoder)?;
        let offset = Offset::decode(decoder)?;
        Ok(Content {
            content_type,
            size,
            checksum,
            offset,
        })
    }
}

impl Encode for Content {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        self.content_type.encode(encoder)?;
        self.size.encode(encoder)?;
        self.checksum.encode(encoder)?;
        self.offset.encode(encoder)?;
        Ok(())
    }
}

impl SizeHint for Content {
    fn size_hint(&self) -> u64 {
        self.content_type.size_hint()
            + self.size.size_hint()
            + self.checksum.size_hint()
            + self.offset.size_hint()
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

impl ContentType {
    /// Returns the name of the content
    pub fn name(&self) -> &str {
        match self {
            ContentType::Package(name) => name.as_str(),
            ContentType::Image(name) => name.as_str(),
        }
    }
}

impl Decode for ContentType {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        Ok(match u8::decode(decoder)? {
            2 => {
                // A tag of 2 indicates a reference elsewhere. This is probably used as a form of
                // compression so duplicate strings are only written once to the WZ archive. This
                // just de-references the strings. Writing duplicate strings to in a WZ pacakge
                // should not be invalid when parsing. Maybe in the future, I can implement this
                // form of compression but I'd rather not waste the time now.

                // Read the offset of where the name and "real" tag are located
                let offset = i32::decode(decoder)?;
                if offset.is_negative() {
                    // sanity check
                    return Err(Error::Offset(offset));
                }
                decoder.decode_at::<ContentType>(offset as u32)?
            }
            3 => ContentType::Package(String::decode(decoder)?),
            4 => ContentType::Image(String::decode(decoder)?),
            t => Err(Error::Tag(t))?,
        })
    }
}

impl Encode for ContentType {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        match self {
            Self::Package(name) => {
                3u8.encode(encoder)?;
                name.encode(encoder)?;
            }
            Self::Image(name) => {
                4u8.encode(encoder)?;
                name.encode(encoder)?;
            }
        }
        Ok(())
    }
}

impl SizeHint for ContentType {
    fn size_hint(&self) -> u64 {
        1 + self.name().size_hint()
    }
}
