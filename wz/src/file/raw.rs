//! Raw Content Metadata

use crate::{
    decode::Error,
    error::Result,
    types::{WzInt, WzOffset, WzString},
    Decode, Encode, WzReader, WzWriter,
};
use crypto::{Decryptor, Encryptor};
use std::io::{Read, Seek, Write};

/// Raw content metadata. Used as an intermediary for encode/decode
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RawContentRef {
    /// Tag indicating the content type
    pub(crate) tag: u8,

    /// Name of the content
    pub(crate) name: WzString,

    /// Size of the content
    pub(crate) size: WzInt,

    /// Checksum of the content
    ///
    /// Packages are always 0. Images are the sum of every byte.
    pub(crate) checksum: WzInt,

    /// Position of the content within the WZ file
    pub(crate) offset: WzOffset,
}

impl RawContentRef {
    fn dereference_name<R, D>(offset: i32, reader: &mut WzReader<R, D>) -> Result<(u8, WzString)>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        if offset.is_negative() {
            // sanity check
            return Err(Error::InvalidOffset(offset).into());
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
            t => Err(Error::InvalidContentType(t).into()),
        }
    }
}

impl Decode for RawContentRef {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let tag = reader.read_byte()?;
        match tag {
            2 => {
                // A tag of 2 indicates a reference elsewhere. This is probably used as a form of
                // compression so duplicate strings are only written once to the WZ file. This just
                // de-references the strings. Writing duplicate strings to in a WZ pacakge should
                // not be invalid when parsing. Maybe in the future, I can implement this form of
                // compression but I'd rather not waste the time now.

                // Read the offset of where the name and "real" tag are located
                let off = i32::decode(reader)?;

                // Read the rest while the content is still in the buffer
                let size = WzInt::decode(reader)?;
                let checksum = WzInt::decode(reader)?;
                let offset = WzOffset::decode(reader)?;
                let (tag, name) = RawContentRef::dereference_name(off, reader)?;

                Ok(Self {
                    tag,
                    name,
                    size,
                    checksum,
                    offset,
                })
            }
            3 | 4 => Ok(Self {
                tag,
                name: WzString::decode(reader)?,
                size: WzInt::decode(reader)?,
                checksum: WzInt::decode(reader)?,
                offset: WzOffset::decode(reader)?,
            }),
            t => return Err(Error::InvalidContentType(t).into()),
        }
    }
}

impl Encode for RawContentRef {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<()>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        self.tag.encode(writer)?;
        self.name.encode(writer)?;
        self.size.encode(writer)?;
        self.checksum.encode(writer)?;
        self.offset.encode(writer)
    }
}
