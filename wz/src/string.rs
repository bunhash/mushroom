//! WZ String Format

use crate::decode::{Decode, Error, Reader};
use crypto::Decryptor;
use std::io::{Read, Seek};

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
    Referenced(u64),
}

impl Decode for UolString {
    type Error = Error;

    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let tag = u8::decode(reader)?;
        Ok(match tag {
            0 => UolString::Placed(String::decode(reader)?),
            1 => UolString::Referenced(u32::decode(reader)? as u64),
            u => Err(Error::Tag(u))?,
        })
    }
}
