//! WZ String Formats

use crate::io::{decode, encode, Decode, Encode, WzReader, WzWriter};
use crypto::{Decryptor, Encryptor};
use std::{
    io::{Read, Seek, Write},
    ops::{Deref, DerefMut},
};

/// Defines how to encode/decode a C-style string
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct CString(String);

impl_str!(CString);

impl_str_eq!(CString, &'a String, String, &'a str, str);

impl Decode for CString {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let mut buf = Vec::new();
        loop {
            match reader.read_byte()? {
                0 => break,
                b => buf.push(b),
            }
        }
        Ok(Self(String::from_utf8(buf)?))
    }
}

impl Encode for CString {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        writer.write_all(self.as_bytes())?;
        writer.write_byte(0)
    }
}

impl encode::SizeHint for CString {
    #[inline]
    fn size_hint(&self) -> u32 {
        self.0.len() as u32 + 1
    }
}

/// Defines a WZ-STRING structure and how to encode/decode it
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct WzString(String);

impl_str!(WzString);

impl_str_eq!(WzString, &'a String, String, &'a str, str);

impl Decode for WzString {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let check = i8::decode(reader)?;
        let length = match check {
            i8::MIN | i8::MAX => i32::decode(reader)?,
            0 => return Ok(Self(String::from(""))),
            _ => (check as i32).wrapping_abs(),
        };
        // Sanity check
        if length <= 0 {
            return Err(decode::Error::InvalidLength(length).into());
        }
        Ok(Self(if check < 0 {
            // UTF-8
            String::from_utf8(reader.read_utf8_bytes(length as usize)?)?
        } else {
            // Unicode
            String::from_utf16(reader.read_unicode_bytes(length as usize)?.as_slice())?
        }))
    }
}

impl Encode for WzString {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        let length = self.0.len() as i32;

        // If length is 0 just write 0 and be done with it
        if length == 0 {
            return writer.write_byte(0);
        }

        // If everything is ASCII, encode as UTF-8, else Unicode
        if self.0.is_ascii() {
            // Write the length
            // length CAN equal i8::MAX here as the 2s compliment is not i8::MIN
            if length > (i8::MAX as i32) {
                writer.write_byte(i8::MIN as u8)?;
                length.encode(writer)?;
            } else {
                writer.write_byte((-length) as u8)?;
            }
            // Write the string
            writer.write_utf8_bytes(self.as_bytes())
        } else {
            // Write the length
            // If lenth is equal to i8::MAX it will be treated as a long-length marker
            if length >= (i8::MAX as i32) {
                writer.write_byte(i8::MAX as u8)?;
                length.encode(writer)?;
            } else {
                writer.write_byte(length as u8)?;
            }
            // Write the string
            let bytes = self.encode_utf16().collect::<Vec<u16>>();
            writer.write_unicode_bytes(&bytes)
        }
    }
}

impl encode::SizeHint for WzString {
    #[inline]
    fn size_hint(&self) -> u32 {
        let length = self.0.len() as u32;

        // If length is 0 just write 0 and be done with it
        if length == 0 {
            return 1;
        }

        // If everything is ASCII, encode as UTF-8, else Unicode
        if self.0.is_ascii() {
            // length CAN equal i8::MAX here as the 2s compliment is not i8::MIN
            if length > (i8::MAX as u32) {
                5 + length
            } else {
                1 + length
            }
        } else {
            // If lenth is equal to i8::MAX it will be treated as a long-length marker
            if length >= (i8::MAX as u32) {
                5 + (length * 2)
            } else {
                1 + (length * 2)
            }
        }
    }
}
