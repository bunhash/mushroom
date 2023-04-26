//! WZ String Formats

use crate::{
    error::{Result, WzError},
    {Decode, Encode, Reader, Writer},
};
use core::ops::{Deref, DerefMut};

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
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
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
    #[inline]
    fn encode_size(&self) -> u64 {
        self.0.len() as u64 + 1
    }

    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        writer.write_all(self.as_bytes())?;
        writer.write_byte(0)
    }
}

/// Defines a WZ-STRING structure and how to encode/decode it
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct WzString(String);

impl_str!(WzString);

impl_eq!(WzString, &'a String, String, &'a str, str);

impl Decode for WzString {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
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
    #[inline]
    fn encode_size(&self) -> u64 {
        let length = self.0.len() as i32;
        if length == 0 {
            1
        } else {
            if self.0.is_ascii() {
                if length > (i8::MAX as i32) {
                    5 + (length as u64)
                } else {
                    1 + (length as u64)
                }
            } else {
                if length >= (i8::MAX as i32) {
                    5 + (length as u64 * 2)
                } else {
                    1 + (length as u64 * 2)
                }
            }
        }
    }

    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
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
