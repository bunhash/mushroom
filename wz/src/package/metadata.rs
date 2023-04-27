//! WZ Package Metadata
//!
//! Metadata such as type and size hints for encoding/decoding contents

use crate::{
    error::Result,
    map::{self, Cursor, SizeHint},
    package::{Content, Error},
    types::{WzInt, WzOffset, WzString},
    Decode, Encode, Reader, Writer,
};

/// Info describing the known `Content`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Params {
    /// Name of the content
    pub(crate) name: WzString,

    /// Size of the `Content`
    pub(crate) size: WzInt,

    /// Checksum of the `Content` -- basically ignored
    pub(crate) checksum: WzInt,

    /// Position of the `Content` in the WZ file
    pub(crate) offset: WzOffset,
}

impl SizeHint for Params {
    fn size_hint(&self, _: usize) -> WzInt {
        self.name.size_hint()
            + self.size.size_hint()
            + self.checksum.size_hint()
            + self.offset.size_hint()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Metadata {
    /// Package or directory
    Package(Params),

    /// Image or binary blob
    Image(Params),
}

impl Decode for Metadata {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let tag = reader.read_byte()?;
        let (tag, name) = match tag {
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

                (tag, name)
            }
            3 => (tag, WzString::decode(reader)?),
            4 => (tag, WzString::decode(reader)?),
            t => Err(Error::InvalidContentType(t).into()),
        };

        let params = Params {
            name,
            size: WzInt::decode(reader)?,
            checksum: WzInt::decode(reader)?,
            offset: WzOffset::decode(reader)?,
        };

        // Make Metadata
        match tag {
            3 => Ok(Metadata::Package(params)),
            4 => Ok(Metadata::Image(params)),
            t => Err(Error::InvalidContentType(t).into()),
        }
    }
}

impl Encode for Metadata {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        let params = match self {
            Metadata::Package(params) => {
                writer.write_byte(3)?;
                params
            }
            Metadata::Image(params) => {
                writer.write_byte(4)?;
                params
            }
        };
        params.name.encode(writer)?;
        params.size.encode(writer)?;
        params.checksum.encode(writer)?;
        params.offset.encode(writer)
    }
}

impl SizeHint for Metadata {
    fn size_hint(&self, _: usize) -> WzInt {
        match self {
            Metadata::Package(params) => 1 + params.size_hint(0),
            Metadata::Image(params) => 1 + params.size_hint(0),
        }
    }
}

impl map::Metadata<Content> for Metadata {
    fn new(name: &str, content: &Content, children: &[&Content]) -> Self {
        let size = match content {
            Content::Package(_) => WzInt::from(0),
            Content::Image(_, size) => *size,
        };
        Self {
            name: WzString::from(name),
            size,
            checksum: WzInt::from(0),
            offset: WzInt::from(0),
        }
    }

    fn name<'a>(&'a self) -> &'a str {
        self.name.as_ref()
    }

    fn size(&self) -> WzInt {
        self.size
    }

    fn rename(&mut self, name: &str) {
        self.name = WzString::from(name);
    }

    fn update_size(&mut self, child_sizes: &[WzInt]) {
        let new_size = match self {
            Metadata::Package(params) => {
                WzInt::from(child_size.len()).size_hint(0) + child_sizes.iter().sum::<i32>()
            }
            Metadata::Image(params) => params.size,
        };
    }
}
