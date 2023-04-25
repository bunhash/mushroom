//! WZ Package Metadata
//!
//! Metadata such as type and size hints for encoding/decoding contents

use crate::{
    error::{PackageError, Result},
    types::{WzInt, WzOffset, WzString},
    Decode, Encode, Reader, Writer,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Metadata {
    /// I don't know what these bytes are
    Unknown([u8; 10]),

    /// Reference to another location in the file
    Reference(i32, Params),

    /// Package or directory
    Package(WzString, Params),

    /// Image or binary blob
    Image(WzString, Params),
}

impl Decode for Metadata {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        match reader.read_byte()? {
            1 => {
                let mut bytes = [0u8; 10];
                reader.read_exact(&mut bytes)?;
                panic!("found unknown bytes: {:?}", bytes);
                //Ok(Self::Unknown(bytes))
            }
            2 => Ok(Self::Reference(
                i32::decode(reader)?,
                Params::decode(reader)?,
            )),
            3 => Ok(Self::Package(
                WzString::decode(reader)?,
                Params::decode(reader)?,
            )),
            4 => Ok(Self::Image(
                WzString::decode(reader)?,
                Params::decode(reader)?,
            )),
            t => Err(PackageError::InvalidContentType(t).into()),
        }
    }
}

impl Encode for Metadata {
    #[inline]
    fn encode_size(&self) -> u64 {
        match self {
            Metadata::Unknown(bytes) => 1 + bytes.len() as u64,
            Metadata::Reference(offset, info) => 1 + offset.encode_size() + info.encode_size(),
            Metadata::Package(name, info) => 1 + name.encode_size() + info.encode_size(),
            Metadata::Image(name, info) => 1 + name.encode_size() + info.encode_size(),
        }
    }

    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        match self {
            Metadata::Unknown(bytes) => {
                writer.write_byte(1)?;
                writer.write_all(bytes)
            }
            Metadata::Reference(offset, info) => {
                writer.write_byte(2)?;
                offset.encode(writer)?;
                info.encode(writer)
            }
            Metadata::Package(name, info) => {
                writer.write_byte(3)?;
                name.encode(writer)?;
                info.encode(writer)
            }
            Metadata::Image(name, info) => {
                writer.write_byte(4)?;
                name.encode(writer)?;
                info.encode(writer)
            }
        }
    }
}

/// Info describing the known `Content`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Params {
    /// Size of the `Content`
    pub size: WzInt,

    /// Checksum of the `Content` -- basically ignored
    pub checksum: WzInt,

    /// Position of the `Content` in the WZ file
    pub offset: WzOffset,
}

impl Decode for Params {
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

impl Encode for Params {
    #[inline]
    fn encode_size(&self) -> u64 {
        self.size.encode_size() + self.checksum.encode_size() + self.offset.encode_size()
    }

    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
        Self: Sized,
    {
        self.size.encode(writer)?;
        self.checksum.encode(writer)?;
        self.offset.encode(writer)
    }
}
