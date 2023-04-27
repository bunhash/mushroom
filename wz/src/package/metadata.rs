//! WZ Package Metadata
//!
//! Metadata such as type and size hints for encoding/decoding contents

use crate::{
    error::Result,
    map::SizeHint,
    package::Error,
    types::{WzInt, WzOffset, WzString},
    Decode, Encode, Reader, Writer,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Metadata {
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
            2 => {
                // A tag of 2 indicates a reference elsewhere. This is probably used as a form of
                // compression so duplicate strings are only written once to the WZ file. This just
                // de-references the strings. Writing duplicate strings to in a WZ pacakge should
                // not be invalid when parsing. Maybe in the future, I can implement this form of
                // compression but I'd rather not waste the time now.

                // Read the offset of where the name and "real" tag are located
                let off = i32::decode(reader)?;
                if off.is_negative() {
                    // sanity check
                    return Err(Error::InvalidPackage.into());
                }
                // Read the parameters while the content is still in the buffer
                let params = Params::decode(reader)?;

                // Store the current location
                let pos = reader.position()?;

                // Move to the referenced content--this empties the buffer
                reader.seek_from_start(off as u32)?;

                // Read the name and "real" tag
                let tag = reader.read_byte()?;
                let name = WzString::decode(reader)?;

                // Seek back
                reader.seek(pos)?;

                // Return content
                match tag {
                    3 => Ok(Self::Package(name, params)),
                    4 => Ok(Self::Image(name, params)),
                    _ => Err(Error::InvalidPackage.into()),
                }
            }
            3 => Ok(Self::Package(
                WzString::decode(reader)?,
                Params::decode(reader)?,
            )),
            4 => Ok(Self::Image(
                WzString::decode(reader)?,
                Params::decode(reader)?,
            )),
            t => Err(Error::InvalidContentType(t).into()),
        }
    }
}

impl Encode for Metadata {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        match self {
            Metadata::Package(name, params) => {
                writer.write_byte(3)?;
                name.encode(writer)?;
                params.encode(writer)
            }
            Metadata::Image(name, params) => {
                writer.write_byte(4)?;
                name.encode(writer)?;
                params.encode(writer)
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

impl SizeHint for Params {
    fn data_size(&self) -> WzInt {
        self.size.data_size() + self.checksum.data_size() + self.offset.data_size()
    }
}
