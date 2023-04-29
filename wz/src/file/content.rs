//! WZ File Content

use crate::{
    decode::Error,
    error::Result,
    map::{Metadata, SizeHint},
    types::{WzInt, WzOffset, WzString},
    Decode, Encode, WzReader, WzWriter,
};
use crypto::{Decryptor, Encryptor};
use std::io::{Read, Seek, Write};

/// Info describing the known [`Content`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Params {
    /// Size of the [`Content`]
    pub(crate) size: WzInt,

    /// Checksum of the [`Content`]
    ///
    /// [`Content::Package`] is always 0. [`Content::Image`] is the sum of every byte.
    pub(crate) checksum: WzInt,

    /// Position of the [`Content`] data in the WZ file
    pub(crate) offset: WzOffset,
}

impl Params {
    pub fn size(&self) -> WzInt {
        self.size
    }

    pub fn checksum(&self) -> WzInt {
        self.checksum
    }

    pub fn offset(&self) -> WzOffset {
        self.offset
    }
}

impl Decode for Params {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        Ok(Params {
            size: WzInt::decode(reader)?,
            checksum: WzInt::decode(reader)?,
            offset: WzOffset::decode(reader)?,
        })
    }
}

impl Encode for Params {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<()>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        self.size.encode(writer)?;
        self.checksum.encode(writer)?;
        self.offset.encode(writer)
    }
}

impl SizeHint for Params {
    fn size_hint(&self) -> WzInt {
        self.size.size_hint() + self.checksum.size_hint() + self.offset.size_hint()
    }
}

/// `Content` found in WZ files
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Content {
    /// Package contains `name`, [`Params`] and `num_contents`.
    ///
    /// `num_contents` is maintained for re-serialization
    Package(WzString, Params, WzInt),

    /// Image contains `name` and [`Params`]--treated as a binary blob
    Image(WzString, Params),
}

impl Decode for Content {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let tag = reader.read_byte()?;
        let (tag, name, params) = match tag {
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
                    return Err(Error::InvalidOffset(off).into());
                }
                // Read the parameters while the content is still in the buffer
                let params = Params::decode(reader)?;

                // Store the current location
                let pos = reader.position()?;

                // Move to the referenced content--this empties the buffer
                reader.seek_from_start(off as u32)?;

                // Read the "real" tag and name
                let tag = reader.read_byte()?;
                let name = WzString::decode(reader)?;

                // Seek back
                reader.seek(pos)?;

                (tag, name, params)
            }
            3 => (tag, WzString::decode(reader)?, Params::decode(reader)?),
            4 => (tag, WzString::decode(reader)?, Params::decode(reader)?),
            t => return Err(Error::InvalidContentType(t).into()),
        };

        // Make Metadata
        match tag {
            3 => Ok(Content::Package(name, params, WzInt::from(0))),
            4 => Ok(Content::Image(name, params)),
            t => Err(Error::InvalidContentType(t).into()),
        }
    }
}

impl Encode for Content {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<()>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        let (name, tag, params) = match self {
            Content::Package(name, params, _) => (name, 3u8, params),
            Content::Image(name, params) => (name, 4u8, params),
        };
        name.encode(writer)?;
        tag.encode(writer)?;
        params.encode(writer)
    }
}

impl Metadata for Content {
    fn update(&mut self, name: &WzString, children_sizes: &[WzInt]) {
        let size = WzInt::from(
            children_sizes
                .iter()
                .map(|size| i32::from(*size))
                .sum::<i32>(),
        );
        match self {
            Content::Package(ref mut current_name, ref mut params, ref mut num_content) => {
                *current_name = name.clone();
                params.size = size;
                *num_content = WzInt::from(children_sizes.len() as u32);
            }
            Content::Image(ref mut name, _) => *name = name.clone(),
        }
    }
}

impl SizeHint for Content {
    fn size_hint(&self) -> WzInt {
        match self {
            Content::Package(name, params, num_content) => {
                1 + name.size_hint() + params.size_hint() + num_content.size_hint() + params.size
            }
            Content::Image(name, params) => 1 + name.size_hint() + params.size_hint() + params.size,
        }
    }
}
