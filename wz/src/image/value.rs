//! WZ "Any" type

use crate::{image::Object, macros, string::UolString, Decode, Error, Reader};
use crypto::Decryptor;
use std::io::{Read, Seek};

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Value {
    /// Null
    Null,

    /// Short
    Short(i16),

    /// Int
    Int(Int32),

    /// Long
    Long(Int64),

    /// Float
    Float(f32),

    /// Double
    Double(f64),

    /// String
    String(UolString),

    /// Nested Object
    Object(Object),
}

impl Decode for Value {
    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        Ok(match u8::decode(reader)? {
            0 => Self::Null,
            2 | 11 => Self::Short(i16::decode(reader)?),
            3 | 19 => Self::Int(Int32::decode(reader)?),
            20 => Self::Long(Int64::decode(reader)?),
            4 => Self::Float(f32::decode(reader)?),
            5 => Self::Double(f64::decode(reader)?),
            8 => Self::String(UolString::decode(reader)?),
            9 => Self::Object(Object::decode(reader)?),
            t => Err(ImageError::PropertyType(t).into()),
        })
    }
}
