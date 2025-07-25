//! WZ String Format

use crate::{Decode, DecodeError, Error, Reader};
use crypto::Decryptor;
use std::io::{Read, Seek};

/// WZ String - can be UTF-8 or Unicode
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum WzString {
    /// UTF-8 String
    Utf8(Vec<u8>),

    /// Unicode String
    Unicode(Vec<u8>),
}

impl WzString {
    /// Converts the WzString to an encoded string
    pub fn to_string(&self) -> Result<String, Error> {
        Ok(match self {
            WzString::Utf8(bytes) => String::from_utf8(bytes.clone()).map_err(DecodeError::from)?,
            WzString::Unicode(bytes) => {
                let wchars = bytes
                    .chunks(2)
                    .map(|c| u16::from_le_bytes([c[0], c[1]]))
                    .collect::<Vec<_>>();
                String::from_utf16(wchars.as_slice()).map_err(DecodeError::from)?
            }
        })
    }

    /// Decrypts the WzString in place
    pub fn decrypt<D>(&mut self, decryptor: &mut D) -> &mut Self
    where
        D: Decryptor,
    {
        match self {
            WzString::Utf8(bytes) => decryptor.decrypt(bytes),
            WzString::Unicode(bytes) => decryptor.decrypt(bytes),
        }
        self
    }

    fn decode_utf8<R>(reader: &mut Reader<R>, len: usize) -> Result<Self, Error>
    where
        R: Read + Seek,
    {
        let buf = reader.read_to_vec(len)?;
        let mut mask = 0xaa;
        Ok(WzString::Utf8(
            buf.iter()
                .map(|b| {
                    let c = b ^ mask;
                    mask = mask.checked_add(1).unwrap_or(0);
                    c
                })
                .collect(),
        ))
    }

    fn decode_unicode<R>(reader: &mut Reader<R>, len: usize) -> Result<Self, Error>
    where
        R: Read + Seek,
    {
        let mut buf = reader.read_to_vec(len * 2)?;
        let mut mask: u16 = 0xaaaa;
        buf.chunks_mut(2).for_each(|c| {
            let m = mask.to_le_bytes();
            mask = mask.checked_add(1).unwrap_or(0);
            c[0] = c[0] ^ m[0];
            c[1] = c[1] ^ m[1];
        });
        Ok(WzString::Unicode(buf))
    }
}

impl Decode for WzString {
    fn decode<R>(reader: &mut Reader<R>) -> Result<Self, Error>
    where
        R: Read + Seek,
    {
        let check = i8::decode(reader)?;
        let length = match check {
            i8::MIN | i8::MAX => i32::decode(reader)?,
            0 => return Ok(WzString::Utf8(Vec::new())),
            _ => (check as i32).wrapping_abs(),
        };
        // Sanity check
        if length <= 0 {
            return Err(DecodeError::Length(length).into());
        }
        if check < 0 {
            // UTF-8
            WzString::decode_utf8(reader, length as usize)
        } else {
            // Unicode
            WzString::decode_unicode(reader, length as usize)
        }
    }
}
