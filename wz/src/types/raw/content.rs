//! Object in a WZ image

use crate::{
    error::{ImageError, Result},
    io::{Decode, Encode, WzRead, WzWrite},
    types::{UolString, WzInt, WzLong, WzOffset},
};

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
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead,
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
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite + ?Sized,
    {
        match &self {
            ContentRef::Null { name } => {
                name.encode(writer)?;
                0u8.encode(writer)
            }
            ContentRef::Short { name, value } => {
                name.encode(writer)?;
                2u8.encode(writer)?;
                value.encode(writer)
            }
            ContentRef::Int { name, value } => {
                name.encode(writer)?;
                3u8.encode(writer)?;
                value.encode(writer)
            }
            ContentRef::Long { name, value } => {
                name.encode(writer)?;
                20u8.encode(writer)?;
                value.encode(writer)
            }
            ContentRef::Float { name, value } => {
                name.encode(writer)?;
                4u8.encode(writer)?;
                value.encode(writer)
            }
            ContentRef::Double { name, value } => {
                name.encode(writer)?;
                5u8.encode(writer)?;
                value.encode(writer)
            }
            ContentRef::String { name, value } => {
                name.encode(writer)?;
                8u8.encode(writer)?;
                value.encode(writer)
            }
            ContentRef::Object { .. } => {
                unimplemented!()
            }
        }
    }
}
