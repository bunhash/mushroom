//! WZ String Formats

use crate::{
    error::{Error, Result},
    {Decode, Reader},
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
            return Err(Error::InvalidLength(length));
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
