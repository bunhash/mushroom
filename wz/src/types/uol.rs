//! WZ Image UOLs

use crate::error::Result;
use crate::io::{xml::writer::ToXml, Decode, Encode, SizeHint, WzRead, WzWrite};
use crate::types::{macros, VerboseDebug};
use std::{
    io,
    ops::{Deref, DerefMut},
};

/// This is just a deduplicated string.
///
/// WZ Images will use an offset to point to the string value instead of re-encoding it. This is
/// useful for compressing data. It is not entirely known when they decide to use a reference
/// instead of re-encoding it. I arbitrarily set this threshold to when the encoded size of the
/// string is >5 since that seems to match the behavior I've witnessed during decoding.
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct UolString(String);

macros::impl_debug!(UolString);

impl UolString {
    /// Consumes the UolString and returns the inner String
    pub fn into_string(self) -> String {
        self.0
    }
}

impl From<String> for UolString {
    fn from(other: String) -> Self {
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
        self.deref()
    }
}

impl AsMut<str> for UolString {
    fn as_mut(&mut self) -> &mut str {
        self.deref_mut()
    }
}

impl From<&str> for UolString {
    fn from(other: &str) -> Self {
        Self(String::from(other))
    }
}

impl PartialEq<&str> for UolString {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.as_ref(), &other[..])
    }
}

impl PartialEq<UolString> for &str {
    #[inline]
    fn eq(&self, other: &UolString) -> bool {
        PartialEq::eq(&self[..], other.as_ref())
    }
}

impl Decode for UolString {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead + ?Sized,
    {
        Ok(Self(reader.read_uol_string()?))
    }
}

impl Encode for UolString {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite + ?Sized,
    {
        writer.write_uol_string(&self.0)
    }
}

impl SizeHint for UolString {
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

macros::impl_debug!(UolObject);

impl UolObject {
    /// Consumes the UolObject and returns the inner String
    pub fn into_string(self) -> String {
        self.uri.into_string()
    }
}

impl From<String> for UolObject {
    fn from(other: String) -> Self {
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
        self.deref()
    }
}

impl AsMut<str> for UolObject {
    fn as_mut(&mut self) -> &mut str {
        self.deref_mut()
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
}

impl PartialEq<UolObject> for &str {
    #[inline]
    fn eq(&self, other: &UolObject) -> bool {
        PartialEq::eq(&self[..], other.as_ref())
    }
}

impl Decode for UolObject {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead + ?Sized,
    {
        u8::decode(reader)?;
        Ok(Self {
            uri: UolString::decode(reader)?,
        })
    }
}

impl Encode for UolObject {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite + ?Sized,
    {
        0u8.encode(writer)?;
        writer.write_uol_string(&self.uri)
    }
}

impl SizeHint for UolObject {
    #[inline]
    fn size_hint(&self) -> u32 {
        1 + self.uri.size_hint()
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
