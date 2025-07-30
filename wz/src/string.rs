//! WZ String Format

use crate::decode::{Decode, Error, Reader};
use crypto::Decryptor;
use std::{
    fmt,
    io::{Read, Seek},
};

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

    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Self::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let tag = u8::decode(reader)?;
        Ok(match tag {
            0 => {
                let position = reader.position()?;
                let value = String::decode(reader)?;
                reader.string_map.insert(position, value.clone());
                UolString::Placed(value)
            }
            1 => {
                let position = u32::decode(reader)?;
                match reader.deref_string(position) {
                    Some(v) => UolString::Placed(v.to_string()),
                    None => UolString::Referenced(position),
                }
            }
            u => Err(Error::Tag(u))?,
        })
    }
}
