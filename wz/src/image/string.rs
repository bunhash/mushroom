//! WZ String Format

use crate::{decode, encode, Decode, Encode, Reader, Writer};
use crypto::{Decryptor, Encryptor};
use std::{
    fmt,
    io::{Read, Seek, Write},
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
    type Error = decode::Error;

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
            u => Err(decode::Error::Tag(u))?,
        })
    }
}

impl Encode for UolString {
    type Error = encode::Error;

    fn encode<W, E>(&self, writer: &mut Writer<W, E>) -> Result<(), Self::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        let position = match self {
            Self::Placed(value) => {
                if str_size(&value) > 5 {
                    None
                } else {
                    writer.ref_string(&value)
                }
            }
            Self::Referenced(value) => Some(value.clone()),
        };
        match position {
            Some(position) => {
                1u8.encode(writer)?;
                position.encode(writer)?;
            }
            None => {
                0u8.encode(writer)?;
                let position = writer.position()?;
                match self {
                    Self::Placed(value) => {
                        value.encode(writer)?;
                        writer.insert_string(&value, position);
                    }
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }
}

fn str_size(string: &str) -> usize {
    if string.len() == 0 {
        return 1;
    }
    if string.is_ascii() {
        if string.len() > (i8::MAX as usize) {
            5 + string.len()
        } else {
            1 + string.len()
        }
    } else {
        if string.len() >= (i8::MAX as usize) {
            5 + (string.len() * 2)
        } else {
            1 + (string.len() * 2)
        }
    }
}
