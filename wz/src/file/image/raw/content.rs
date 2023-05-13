//! Object in a WZ image

use crate::{
    error::{ImageError, Result},
    file::image::UolString,
    io::{Decode, Encode, SizeHint, WzReader, WzWriter},
    types::{WzInt, WzLong, WzOffset},
};
use crypto::{Decryptor, Encryptor};
use std::io::{Read, Seek, Write};

/// Represents the contents of a [`Property`](crate::file::image::Property)
#[derive(Debug)]
pub enum ContentRef {
    /// Primitive Null type
    Null { name: UolString },

    /// Primitive short
    Short { name: UolString, value: i16 },

    /// Primitive int
    Int { name: UolString, value: WzInt },

    /// Primitive WZ-LONG
    Long { name: UolString, value: WzLong },

    /// Primitive float
    Float { name: UolString, value: f32 },

    /// Primitive double
    Double { name: UolString, value: f64 },

    /// UOL
    String { name: UolString, value: UolString },

    /// Complex object
    Object {
        name: UolString,
        size: u32,
        offset: WzOffset,
    },
}

impl Decode for ContentRef {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let name = UolString::decode(reader)?;
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
                value: UolString::decode(reader)?,
            }),
            9 => {
                let size = u32::decode(reader)?;
                let offset = reader.position()?;
                reader.seek(offset + size.into())?;
                Ok(Self::Object { name, size, offset })
            }
            t => Err(ImageError::PropertyType(t).into()),
        }
    }
}

impl Encode for ContentRef {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<()>
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

impl SizeHint for ContentRef {
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
            ContentRef::Object { name, size, .. } => name.size_hint() + size,
        }
    }
}
