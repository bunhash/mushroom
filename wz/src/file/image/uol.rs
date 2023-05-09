//! WZ Image UOLs

use crate::{
    io::{decode, encode, xml::writer::ToXml, Decode, Encode, WzReader, WzWriter},
    types::{WzOffset, WzString},
};
use crypto::{Decryptor, Encryptor};
use std::{
    io::{Read, Seek, Write},
    ops::{Deref, DerefMut},
};

/// This is just a deduplicated string. WZ Images will use an offset to point to the string value
/// instead of encoding the same string multiple times.
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct UolString(WzString);

impl UolString {
    /// Consumes the UolString and returns the inner WzString
    pub fn into_wz_string(self) -> WzString {
        self.0
    }
}

impl From<String> for UolString {
    fn from(other: String) -> Self {
        Self(WzString::from(other))
    }
}

impl From<WzString> for UolString {
    fn from(other: WzString) -> Self {
        Self(other)
    }
}

impl Deref for UolString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl DerefMut for UolString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut()
    }
}

impl AsRef<str> for UolString {
    fn as_ref(&self) -> &str {
        self.deref().as_ref()
    }
}

impl AsMut<str> for UolString {
    fn as_mut(&mut self) -> &mut str {
        self.deref_mut().as_mut()
    }
}

impl From<&str> for UolString {
    fn from(other: &str) -> Self {
        Self(WzString::from(other))
    }
}

impl PartialEq<&str> for UolString {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.as_ref(), &other[..])
    }
    #[inline]
    fn ne(&self, other: &&str) -> bool {
        PartialEq::ne(self.as_ref(), &other[..])
    }
}

impl PartialEq<UolString> for &str {
    #[inline]
    fn eq(&self, other: &UolString) -> bool {
        PartialEq::eq(&self[..], other.as_ref())
    }
    #[inline]
    fn ne(&self, other: &UolString) -> bool {
        PartialEq::ne(&self[..], other.as_ref())
    }
}

impl Decode for UolString {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let check = u8::decode(reader)?;
        match check {
            0 => Ok(Self(WzString::decode(reader)?)),
            1 => {
                let offset = WzOffset::from(u32::decode(reader)?);
                let pos = reader.position()?;
                reader.seek(offset)?;
                let string = WzString::decode(reader)?;
                reader.seek(pos)?;
                Ok(Self(string))
            }
            u => Err(decode::Error::InvalidUol(u)),
        }
    }
}

impl Encode for UolString {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        0u8.encode(writer)?;
        self.0.encode(writer)
    }
}

impl encode::SizeHint for UolString {
    #[inline]
    fn size_hint(&self) -> u32 {
        1 + self.0.size_hint()
    }
}

impl ToXml for UolString {
    fn tag(&self) -> &'static str {
        "string"
    }

    fn attributes(&self, name: &str) -> Vec<(String, String)> {
        vec![
            (String::from("name"), name.to_string()),
            (String::from("value"), self.as_ref().to_string()),
        ]
    }
}

/// Defines a UOL object and how to encode/decode it
///
/// UOLs are a URI formatted path specifyig where the "borrowed" content actually resides. This is
/// probably the most useful form of compression WZ files make use of. It can be utilized in
/// memory. I would not the same about duplicated strings.
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct UolObject {
    uri: UolString,
}

impl UolObject {
    /// Consumes the UolObject and returns the inner WzString
    pub fn into_wz_string(self) -> WzString {
        self.uri.into_wz_string()
    }
}

impl From<String> for UolObject {
    fn from(other: String) -> Self {
        Self {
            uri: UolString::from(other),
        }
    }
}

impl From<WzString> for UolObject {
    fn from(other: WzString) -> Self {
        Self {
            uri: UolString::from(other),
        }
    }
}

impl Deref for UolObject {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.uri.as_ref()
    }
}

impl DerefMut for UolObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.uri.as_mut()
    }
}

impl AsRef<str> for UolObject {
    fn as_ref(&self) -> &str {
        self.deref().as_ref()
    }
}

impl AsMut<str> for UolObject {
    fn as_mut(&mut self) -> &mut str {
        self.deref_mut().as_mut()
    }
}

impl From<&str> for UolObject {
    fn from(other: &str) -> Self {
        Self {
            uri: UolString::from(other),
        }
    }
}

impl PartialEq<&str> for UolObject {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.as_ref(), &other[..])
    }
    #[inline]
    fn ne(&self, other: &&str) -> bool {
        PartialEq::ne(self.as_ref(), &other[..])
    }
}

impl PartialEq<UolObject> for &str {
    #[inline]
    fn eq(&self, other: &UolObject) -> bool {
        PartialEq::eq(&self[..], other.as_ref())
    }
    #[inline]
    fn ne(&self, other: &UolObject) -> bool {
        PartialEq::ne(&self[..], other.as_ref())
    }
}

impl Decode for UolObject {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        u8::decode(reader)?;
        Ok(Self {
            uri: UolString::decode(reader)?,
        })
    }
}

impl Encode for UolObject {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        0u8.encode(writer)?;
        0x73u8.encode(writer)?;
        self.uri.encode(writer)
    }
}

impl encode::SizeHint for UolObject {
    #[inline]
    fn size_hint(&self) -> u32 {
        2 + self.uri.size_hint()
    }
}

impl ToXml for UolObject {
    fn tag(&self) -> &'static str {
        "uol"
    }

    fn attributes(&self, name: &str) -> Vec<(String, String)> {
        vec![
            (String::from("name"), name.to_string()),
            (String::from("value"), self.as_ref().to_string()),
        ]
    }
}
