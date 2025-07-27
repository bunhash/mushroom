//! Decoder Trait

use crypto::Decryptor;
use std::io::{Read, Seek};

mod error;
mod reader;

pub use error::Error;
pub use reader::Reader;

/// Trait for decoding objects
pub trait Decode: Sized {
    /// Type of error thrown. Must implement `From<decode::Error>`
    type Error;

    /// Decodes objects
    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Self::Error>
    where
        R: Read + Seek,
        D: Decryptor,
        Self::Error: From<Error>;
}

impl Decode for i8 {
    type Error = Error;

    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        Ok(reader.read_byte()? as i8)
    }
}

impl Decode for i16 {
    type Error = Error;

    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(i16::from_le_bytes(buf))
    }
}

impl Decode for i32 {
    type Error = Error;

    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }
}

impl Decode for i64 {
    type Error = Error;

    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }
}

impl Decode for u8 {
    type Error = Error;

    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        reader.read_byte()
    }
}

impl Decode for u16 {
    type Error = Error;

    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }
}

impl Decode for u32 {
    type Error = Error;

    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
}

impl Decode for u64 {
    type Error = Error;

    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
}

impl Decode for f32 {
    type Error = Error;

    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        Ok(match u8::decode(reader)? {
            0x80 => {
                let mut buf = [0u8; 4];
                reader.read_exact(&mut buf)?;
                f32::from_le_bytes(buf)
            }
            _ => 0f32,
        })
    }
}

impl Decode for f64 {
    type Error = Error;

    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }
}

impl Decode for String {
    type Error = Error;

    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let check = i8::decode(reader)?;
        let length = match check {
            i8::MIN | i8::MAX => i32::decode(reader)?,
            0 => return Ok(String::from("")),
            _ => (check as i32).wrapping_abs(),
        };
        // Sanity check
        if length <= 0 {
            return Err(Error::length(length));
        }
        Ok(if check < 0 {
            // UTF-8
            String::from_utf8(read_utf8_bytes(reader, length as usize)?)?
        } else {
            // Unicode
            String::from_utf16(&read_unicode_bytes(reader, length as usize)?)?
        })
    }
}

fn read_utf8_bytes<R, D>(reader: &mut Reader<R, D>, len: usize) -> Result<Vec<u8>, Error>
where
    R: Read + Seek,
    D: Decryptor,
{
    let mut buf = reader.read_to_vec(len)?;
    reader.decrypt(&mut buf);
    let mut mask = 0xaa;
    Ok(buf
        .iter()
        .map(|b| {
            let c = b ^ mask;
            mask = mask.checked_add(1).unwrap_or(0);
            c
        })
        .collect())
}

fn read_unicode_bytes<R, D>(reader: &mut Reader<R, D>, len: usize) -> Result<Vec<u16>, Error>
where
    R: Read + Seek,
    D: Decryptor,
{
    let mut buf = reader.read_to_vec(len * 2)?;
    reader.decrypt(&mut buf);
    let mut mask: u16 = 0xaaaa;
    Ok(buf
        .chunks(2)
        .map(|c| {
            let wchar = u16::from_le_bytes([c[0], c[1]]);
            let wchar = wchar ^ mask;
            mask = mask.checked_add(1).unwrap_or(0);
            wchar
        })
        .collect())
}
