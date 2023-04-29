//! WZ String Formats

use crate::{
    error::{Result, WzError},
    map::SizeHint,
    types::WzInt,
    {Decode, Encode, WzReader, WzWriter},
};
use core::ops::{Deref, DerefMut};
use crypto::{Decryptor, Encryptor};
use std::io::{Read, Seek, Write};

macro_rules! impl_str {
    ( $type:ty ) => {
        impl From<String> for $type {
            fn from(other: String) -> Self {
                Self(other)
            }
        }

        impl From<&str> for $type {
            fn from(other: &str) -> Self {
                Self(String::from(other))
            }
        }

        impl Deref for $type {
            type Target = str;
            fn deref(&self) -> &Self::Target {
                self.0.as_str()
            }
        }

        impl DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.0.as_mut_str()
            }
        }

        impl AsRef<str> for $type {
            fn as_ref(&self) -> &str {
                self.deref().as_ref()
            }
        }

        impl AsMut<str> for $type {
            fn as_mut(&mut self) -> &mut str {
                self.deref_mut().as_mut()
            }
        }
    };
}

macro_rules! impl_eq {
    ($lhs:ty, $( $rhs:ty ),+ ) => {
        $(
            #[allow(unused_lifetimes)]
            impl<'a, 'b> PartialEq<$rhs> for $lhs {
                #[inline]
                fn eq(&self, other: &$rhs) -> bool {
                    PartialEq::eq(&self.0[..], &other[..])
                }
                #[inline]
                fn ne(&self, other: &$rhs) -> bool {
                    PartialEq::ne(&self.0[..], &other[..])
                }
            }

            #[allow(unused_lifetimes)]
            impl<'a, 'b> PartialEq<$lhs> for $rhs {
                #[inline]
                fn eq(&self, other: &$lhs) -> bool {
                    PartialEq::eq(&self[..], &other.0[..])
                }
                #[inline]
                fn ne(&self, other: &$lhs) -> bool {
                    PartialEq::ne(&self[..], &other.0[..])
                }
            }
        )+
    };
}

/// Defines how to encode/decode a C-style string
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct CString(String);

impl_str!(CString);

impl_eq!(CString, &'a String, String, &'a str, str);

impl Decode for CString {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self>
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
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<()>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        writer.write_all(self.as_bytes())?;
        writer.write_byte(0)
    }
}

impl SizeHint for CString {
    fn size_hint(&self) -> WzInt {
        WzInt::from(self.0.len() as i32 + 1)
    }
}

/// Defines a WZ-STRING structure and how to encode/decode it
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct WzString(String);

impl_str!(WzString);

impl_eq!(WzString, &'a String, String, &'a str, str);

impl Decode for WzString {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self>
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
            return Err(WzError::InvalidLength(length).into());
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
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<()>
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

impl SizeHint for WzString {
    fn size_hint(&self) -> WzInt {
        let length = self.0.len() as i32;
        if self.0.is_ascii() {
            if length > (i8::MAX as i32) {
                WzInt::from(5 + length)
            } else {
                WzInt::from(1 + length)
            }
        } else {
            if length >= (i8::MAX as i32) {
                WzInt::from(5 + (length * 2))
            } else {
                WzInt::from(1 + (length * 2))
            }
        }
    }
}
