//! UOL

use crate::{
    io::{decode, encode, Decode, Encode, WzReader, WzWriter},
    types::{WzOffset, WzString},
};
use crypto::{Decryptor, Encryptor};
use std::{
    io::{Read, Seek, Write},
    ops::{Deref, DerefMut},
};

/// Defines a UOL structure and how to encode/decode it
///
/// UOLs are a URI formatted path specifyig where the "borrowed" content actually resides. This is
/// probably the most useful form of compression WZ files make use of. It can be utilized in
/// memory. I would not the same about duplicated strings.
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Uol {
    uri: WzString,
}

impl Uol {
    /// Consumes the UOL and returns the inner WzString
    pub fn into_wz_string(self) -> WzString {
        self.uri
    }
}

impl From<String> for Uol {
    fn from(other: String) -> Self {
        Self {
            uri: WzString::from(other),
        }
    }
}

impl From<WzString> for Uol {
    fn from(other: WzString) -> Self {
        Self { uri: other }
    }
}

impl Deref for Uol {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.uri.as_ref()
    }
}

impl DerefMut for Uol {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.uri.as_mut()
    }
}

impl AsRef<str> for Uol {
    fn as_ref(&self) -> &str {
        self.deref().as_ref()
    }
}

impl AsMut<str> for Uol {
    fn as_mut(&mut self) -> &mut str {
        self.deref_mut().as_mut()
    }
}

impl From<&str> for Uol {
    fn from(other: &str) -> Self {
        Self {
            uri: WzString::from(other),
        }
    }
}

impl PartialEq<&str> for Uol {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.as_ref(), &other[..])
    }
    #[inline]
    fn ne(&self, other: &&str) -> bool {
        PartialEq::ne(self.as_ref(), &other[..])
    }
}

impl PartialEq<Uol> for &str {
    #[inline]
    fn eq(&self, other: &Uol) -> bool {
        PartialEq::eq(&self[..], other.as_ref())
    }
    #[inline]
    fn ne(&self, other: &Uol) -> bool {
        PartialEq::ne(&self[..], other.as_ref())
    }
}

impl Decode for Uol {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let check = u8::decode(reader)?;
        match check {
            // The URI directly follows this byte
            0 | 0x73 => Ok(Self {
                uri: WzString::decode(reader)?,
            }),

            // The URI is duplicated somewhere in the image. So instead of rewriting it, this
            // points to the offset of where the first (?) occurance is. Another compression
            // mechanism.
            1 | 0x1b => {
                let offset = WzOffset::from(u32::decode(reader)?);
                let pos = reader.position()?;
                reader.seek(offset)?;
                let uri = WzString::decode(reader)?;
                reader.seek(pos)?;
                Ok(Self { uri })
            }
            u => Err(decode::Error::InvalidUol(u)),
        }
    }
}

impl Encode for Uol {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        0u8.encode(writer)?;
        self.uri.encode(writer)
    }
}

impl encode::SizeHint for Uol {
    #[inline]
    fn size_hint(&self) -> u32 {
        1 + self.uri.size_hint()
    }
}
