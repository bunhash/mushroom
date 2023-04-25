//! WZ Content Enumeration

use crate::{
    error::{Result, WzError},
    types::{WzInt, WzOffset, WzString},
    Decode, Encode, Reader, Writer,
};
use std::io::SeekFrom;

pub enum ContentType {
    Unknown = 1,
    Referencce = 2,
    Package = 3,
    Image = 4,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentReference {
    /// Tye type of content referenced
    pub content_type: ContentType,

    /// Size of the `Content`
    pub size: WzInt,

    /// Checksum of the `Content` -- basically ignored
    pub checksum: WzInt,

    /// Position of the `Content` in the WZ file
    pub offset: WzOffset,
}

/// Possible `Content` types found in a `Package`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Content {
    /// I don't know what this is...
    Unknown([u8; 10]),

    /// Reference object
    Reference(Reference, ContentInfo),

    /// Package
    Package(WzString, ContentInfo),

    /// Image
    Image(WzString, ContentInfo),
}

impl Decode for Content {
    fn decode<R>(reader: &mut R) -> Result<Self>
        where
            R: Reader,
        {
            match reader.read_byte()? {
                1 => {
                    let mut data = [0u8; 10];
                    reader.read_exact(&mut data)?;
                    Ok(Content::Unknown(data))
                }
                2 => {
                    let offset = i32::decode(reader)?;
                    if offset.is_negative() {
                        return Err(WzError::InvalidPackage.into());
                    }
                    let info = ContentInfo::decode(reader)?;
                    let pos = reader.position()?;
                    reader.seek_from_start(offset as u64)?;
                    let content = match reader.read_byte()? {
                        3 => Content::Package(WzString::decode(reader)?, info),
                        4 => Content::Image(WzString::decode(reader)?, info),
                        _ => return Err(WzError::InvalidPackage.into()),
                    };
                    reader.seek(SeekFrom::Start(pos))?;
                    Ok(content)
                }
                3 => {
                    let name = WzString::decode(reader)?;
                    let info = ContentInfo::decode(reader)?;
                    Ok(Content::Package(name, info))
                }
                4 => {
                    let name = WzString::decode(reader)?;
                    let info = ContentInfo::decode(reader)?;
                    Ok(Content::Image(name, info))
                }
                _ => Err(WzError::InvalidContentType.into()),
            }
        }
}

impl Encode for Content {
    /// Get the size
    fn encode_size(&self) -> u64 {
        match self {
            Content::Unknown(bytes) => 1 + bytes.len() as u64,
            Content::Package(name, info) => 1 + name.encode_size() + info.encode_size(),
            Content::Image(name, info) => 1 + name.encode_size() + info.encode_size(),
        }
    }

    /// Encodes objects
    fn encode<W>(&self, _writer: &mut W) -> Result<()>
        where
            W: Writer,
            Self: Sized,
            {
                unimplemented!()
            }
}

/// Info describing the known `Content`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentInfo {
    /// Size of the `Content`
    pub size: WzInt,

    /// Checksum of the `Content` -- basically ignored
    pub checksum: WzInt,

    /// Position of the `Content` in the WZ file
    pub offset: WzOffset,
}

impl Decode for ContentInfo {
    fn decode<R>(reader: &mut R) -> Result<Self>
        where
            R: Reader,
        {
            let size = WzInt::decode(reader)?;
            let checksum = WzInt::decode(reader)?;
            let offset = WzOffset::decode(reader)?;
            Ok(Self {
                size,
                checksum,
                offset,
            })
        }
}

impl Encode for ContentInfo {
    #[inline]
    fn encode_size(&self) -> u64 {
        self.size.encode_size() + self.checksum.encode_size() + self.offset.encode_size()
    }

    fn encode<W>(&self, _writer: &mut W) -> Result<()>
        where
            W: Writer,
            Self: Sized,
            {
                unimplemented!()
            }
}
