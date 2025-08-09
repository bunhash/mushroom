//! WZ Archive Package Content types

use crate::{
    archive::{Error, Offset},
    decode::{self, Decode, Decoder},
    encode::{Encode, Encoder, SizeHint},
    Int32,
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
            ContentType::Referenced(offset) => write!(
                f,
                "Ref({:08x}, size={}, checksum={}, offset={})",
                offset, self.size, self.checksum, self.offset
            ),
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
            return Err(decode::Error::size("Content size is negative"))?;
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
    /// Referenced tag=2
    Referenced(u32),

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
            _ => "",
        }
    }
}

impl Decode for ContentType {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        Ok(match u8::decode(decoder)? {
            2 => ContentType::Referenced(u32::decode(decoder)?),
            3 => ContentType::Package(String::decode(decoder)?),
            4 => ContentType::Image(String::decode(decoder)?),
            t => Err(Error::tag(&format!("invalid ContentType tag: {}", t)))?,
        })
    }
}

impl Encode for ContentType {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        match self {
            Self::Referenced(offset) => {
                2u8.encode(encoder)?;
                offset.encode(encoder)?;
            }
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
        match self {
            Self::Referenced(offset) => 1 + offset.size_hint(),
            Self::Package(name) => 1 + name.size_hint(),
            Self::Image(name) => 1 + name.size_hint(),
        }
    }
}
