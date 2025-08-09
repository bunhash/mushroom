//! WZ "Any" type

use crate::{
    decode::{Decode, Decoder},
    encode::{Encode, Encoder, SizeHint},
    image::{Error, Object, UolString},
    Int32, Int64,
};

/// The value of a property
#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

    /// Nested `Object` at offset
    Object(u32),
}

impl Decode for Value {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        Ok(match u8::decode(decoder)? {
            0 => Self::Null,
            2 | 11 => Self::Short(i16::decode(decoder)?),
            3 | 19 => Self::Int(Int32::decode(decoder)?),
            20 => Self::Long(Int64::decode(decoder)?),
            4 => Self::Float(f32::decode(decoder)?),
            5 => Self::Double(f64::decode(decoder)?),
            8 => Self::String(UolString::decode(decoder)?),
            9 => Self::Object(Object::decode(decoder)?),
            t => Err(Error::tag(&format!("invalid Value tag {:02x}", t)))?,
        })
    }
}

impl Encode for Value {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        match self {
            Self::Null => 0u8.encode(encoder)?,
            Self::Short(value) => {
                2u8.encode(encoder)?;
                value.encode(encoder)?;
            }
            Self::Int(value) => {
                3u8.encode(encoder)?;
                value.encode(encoder)?;
            }
            Self::Long(value) => {
                20u8.encode(encoder)?;
                value.encode(encoder)?;
            }
            Self::Float(value) => {
                4u8.encode(encoder)?;
                value.encode(encoder)?;
            }
            Self::Double(value) => {
                5u8.encode(encoder)?;
                value.encode(encoder)?;
            }
            Self::String(value) => {
                8u8.encode(encoder)?;
                value.encode(encoder)?;
            }
            Self::Object(value) => {
                9u8.encode(encoder)?;
                value.encode(encoder)?;
            }
        }
        Ok(())
    }
}

impl SizeHint for Value {
    fn size_hint(&self) -> u64 {
        match self {
            Self::Null => 1,
            Self::Short(value) => 1 + value.size_hint(),
            Self::Int(value) => 1 + value.size_hint(),
            Self::Long(value) => 1 + value.size_hint(),
            Self::Float(value) => 1 + value.size_hint(),
            Self::Double(value) => 1 + value.size_hint(),
            Self::String(value) => 1 + value.size_hint(),
            Self::Object(value) => 1 + value.size_hint(),
        }
    }
}
