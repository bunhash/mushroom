//! WZ String Format

use crate::{
    decode::{Decode, Decoder},
    encode::{Encode, Encoder, SizeHint},
    image::Error,
};
use std::fmt;

/// Defines a WZ string structure and how to encode/decode it.
///
/// The archives are compressed in a way where duplicate strings are referenced instead of defined
/// multiple times. It complicates encoding/decoding. The client can function with or without these
/// de-duplicated strings.
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub enum UolString {
    /// Un-cached string
    Placed(String),

    /// Reference to cached string
    Referenced(u32),
}

impl fmt::Display for UolString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Placed(s) => f.write_str(s),
            Self::Referenced(pos) => write!(f, "Ref({:08x})", pos),
        }
    }
}

impl Decode for UolString {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        let tag = u8::decode(decoder)?;
        Ok(match tag {
            0 => UolString::Placed(String::decode(decoder)?),
            1 => UolString::Referenced(u32::decode(decoder)?),
            u => Err(Error::tag(&format!("invalid UolString tag: {:02x}", u)))?,
        })
    }
}

impl Encode for UolString {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        match self {
            Self::Placed(value) => {
                0u8.encode(encoder)?;
                value.encode(encoder)?;
            }
            Self::Referenced(value) => {
                1u8.encode(encoder)?;
                value.encode(encoder)?;
            }
        }
        Ok(())
    }
}

impl SizeHint for UolString {
    fn size_hint(&self) -> u64 {
        match self {
            Self::Placed(value) => 1 + value.size_hint(),
            Self::Referenced(value) => 1 + value.size_hint(),
        }
    }
}
