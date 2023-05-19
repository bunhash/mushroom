//! Content Metadata

use crate::{
    error::{DecodeError, PackageError, Result},
    io::{Decode, Encode, SizeHint, WzRead, WzWrite},
    types::{WzInt, WzOffset},
};

/// Content Types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentRef {
    Package(Metadata),
    Image(Metadata),
}

impl ContentRef {
    pub fn offset(&self) -> WzOffset {
        match &self {
            ContentRef::Package(ref data) => data.offset(),
            ContentRef::Image(ref data) => data.offset(),
        }
    }
}

impl Decode for ContentRef {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead + ?Sized,
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
                String::decode(reader)?,
                WzInt::decode(reader)?,
                WzInt::decode(reader)?,
                WzOffset::decode(reader)?,
            ),
            t => return Err(PackageError::ContentType(t).into()),
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
            t => Err(PackageError::ContentType(t).into()),
        }
    }
}

impl Encode for ContentRef {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite + ?Sized,
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

impl SizeHint for ContentRef {
    #[inline]
    fn size_hint(&self) -> u32 {
        match &self {
            ContentRef::Package(ref data) => 3u8.size_hint() + data.size_hint(),
            ContentRef::Image(ref data) => 4u8.size_hint() + data.size_hint(),
        }
    }
}

/// Content metadata
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    /// Name of the content
    pub(crate) name: String,

    /// Size of the content
    pub(crate) size: WzInt,

    /// Checksum of the content. Sum of all the bytes
    pub(crate) checksum: WzInt,

    /// Position of the content within the WZ archive
    pub(crate) offset: WzOffset,
}

impl Metadata {
    pub fn new(name: String, size: WzInt, checksum: WzInt, offset: WzOffset) -> Self {
        Self {
            name,
            size,
            checksum,
            offset,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn size(&self) -> WzInt {
        self.size
    }

    pub fn offset(&self) -> WzOffset {
        self.offset
    }

    fn dereference_name<R>(offset: i32, reader: &mut R) -> Result<(u8, String)>
    where
        R: WzRead + ?Sized,
    {
        if offset.is_negative() {
            // sanity check
            return Err(DecodeError::Offset(offset).into());
        }

        // Get current position
        let pos = reader.position()?;

        // Move to the referenced content--this empties the buffer
        reader.seek_from_start(offset as u32)?;

        // Read the "real" tag and name
        let tag = reader.read_byte()?;
        let name = String::decode(reader)?;

        // Seek back
        reader.seek(pos)?;

        // Sanity check to ensure the tag is valid
        match tag {
            3 | 4 => Ok((tag, name)),
            t => Err(PackageError::ContentType(t).into()),
        }
    }
}

impl Encode for Metadata {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite + ?Sized,
    {
        self.name.encode(writer)?;
        self.size.encode(writer)?;
        self.checksum.encode(writer)?;
        self.offset.encode(writer)
    }
}

impl SizeHint for Metadata {
    #[inline]
    fn size_hint(&self) -> u32 {
        self.name.size_hint()
            + self.size.size_hint()
            + self.checksum.size_hint()
            + self.offset.size_hint()
    }
}
