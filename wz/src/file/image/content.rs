//! Object in a WZ image

use crate::{
    file::image::Uol,
    io::{decode, encode, Decode, Encode, WzReader, WzWriter},
    types::{WzInt, WzLong, WzOffset},
};
use crypto::{Decryptor, Encryptor};
use std::io::{Read, Seek, Write};

/// Represents the contents of a [`Property`](crate::file::image::Property)
#[derive(Debug)]
pub enum ContentRef {
    /// Primitive Null type
    Null { name: Uol },

    /// Primitive short
    Short { name: Uol, value: i16 },

    /// Primitive int
    Int { name: Uol, value: WzInt },

    /// Primitive WZ-LONG
    Long { name: Uol, value: WzLong },

    /// Primitive float
    Float { name: Uol, value: f32 },

    /// Primitive double
    Double { name: Uol, value: f64 },

    /// UOL
    String { name: Uol, value: Uol },

    /// Complex object
    Object {
        name: Uol,
        size: u32,
        offset: WzOffset,
    },
}

impl Decode for ContentRef {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let name = Uol::decode(reader)?;
        match u8::decode(reader)? {
            0 => Ok(Self::Null { name }),
            2 | 11 => Ok(Self::Short {
                name,
                value: i16::decode(reader)?,
            }),
            3 | 19 => Ok(Self::Int {
                name,
                value: WzInt::decode(reader)?,
            }),
            20 => Ok(Self::Long {
                name,
                value: WzLong::decode(reader)?,
            }),
            4 => Ok(Self::Float {
                name,
                value: f32::decode(reader)?,
            }),
            5 => Ok(Self::Double {
                name,
                value: f64::decode(reader)?,
            }),
            8 => Ok(Self::String {
                name,
                value: Uol::decode(reader)?,
            }),
            9 => {
                let size = u32::decode(reader)?;
                let offset = reader.position()?;
                reader.seek(offset + size)?;
                Ok(Self::Object { name, size, offset })
            }
            t => Err(decode::Error::InvalidContentType(t)),
        }
    }
}

impl Encode for ContentRef {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        match self {
            ContentRef::Null { name } => name.encode(writer),
            ContentRef::Short { name, value } => {
                name.encode(writer)?;
                value.encode(writer)
            }
            ContentRef::Int { name, value } => {
                name.encode(writer)?;
                value.encode(writer)
            }
            ContentRef::Long { name, value } => {
                name.encode(writer)?;
                value.encode(writer)
            }
            ContentRef::Float { name, value } => {
                name.encode(writer)?;
                value.encode(writer)
            }
            ContentRef::Double { name, value } => {
                name.encode(writer)?;
                value.encode(writer)
            }
            ContentRef::String { name, value } => {
                name.encode(writer)?;
                value.encode(writer)
            }
            ContentRef::Object { name, size, .. } => {
                name.encode(writer)?;
                size.encode(writer)
            }
        }
    }
}

impl encode::SizeHint for ContentRef {
    #[inline]
    fn size_hint(&self) -> u32 {
        match self {
            ContentRef::Null { name } => name.size_hint(),
            ContentRef::Short { name, value } => name.size_hint() + value.size_hint(),
            ContentRef::Int { name, value } => name.size_hint() + value.size_hint(),
            ContentRef::Long { name, value } => name.size_hint() + value.size_hint(),
            ContentRef::Float { name, value } => name.size_hint() + value.size_hint(),
            ContentRef::Double { name, value } => name.size_hint() + value.size_hint(),
            ContentRef::String { name, value } => name.size_hint() + value.size_hint(),
            ContentRef::Object { name, size, .. } => name.size_hint() as u32 + size,
        }
    }
}
