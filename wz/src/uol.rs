//! WZ Image UOLs

use crate::{Decode, DecodeError, Error, Offset, Reader, WzString};
use std::io::{Read, Seek};

/// This is just a deduplicated string.
///
/// WZ Images will use an offset to point to the string value instead of re-encoding it. This is
/// useful for compressing data. It is not entirely known when they decide to use a reference
/// instead of re-encoding it. I arbitrarily set this threshold to when the encoded size of the
/// string is >5 since that seems to match the behavior I've witnessed during decoding.
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub enum UolString {
    /// Placed string
    Placed(WzString),

    /// Referenced string
    Referenced(Offset),
}

impl Decode for UolString {
    fn decode<R>(reader: &mut Reader<R>) -> Result<Self, Error>
    where
        R: Read + Seek,
    {
        let check = u8::decode(reader)?;
        Ok(match check {
            0 => UolString::Placed(WzString::decode(reader)?),
            1 => UolString::Referenced(Offset::from(u32::decode(reader)?)),
            u => Err(DecodeError::Tag(u))?,
        })
    }
}

//impl Encode for UolString {
//    fn encode<W>(&self, writer: &mut W) -> Result<()>
//    where
//        W: WzWrite + ?Sized,
//    {
//        writer.write_uol_string(&self.0)
//    }
//}
//
//impl SizeHint for UolString {
//    #[inline]
//    fn size_hint(&self) -> u32 {
//        1 + self.0.size_hint()
//    }
//}
//
//impl ToXml for UolString {
//    fn tag(&self) -> &'static str {
//        "string"
//    }
//
//    fn attributes(&self, name: &str) -> Vec<(String, String)> {
//        vec![
//            (String::from("name"), name.to_string()),
//            (String::from("value"), self.as_ref().to_string()),
//        ]
//    }
//}

/// Defines a UOL object and how to encode/decode it
///
/// UOLs are a URI formatted path specifyig where the "borrowed" content actually resides. This is
/// probably the most useful form of compression WZ files make use of. It can be utilized in
/// memory. I would not the same about duplicated strings.
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct UolObject {
    uri: UolString,
}

impl Decode for UolObject {
    fn decode<R>(reader: &mut Reader<R>) -> Result<Self, Error>
    where
        R: Read + Seek,
    {
        u8::decode(reader)?;
        Ok(Self {
            uri: UolString::decode(reader)?,
        })
    }
}

//impl Encode for UolObject {
//    fn encode<W>(&self, writer: &mut W) -> Result<()>
//    where
//        W: WzWrite + ?Sized,
//    {
//        0u8.encode(writer)?;
//        writer.write_uol_string(&self.uri)
//    }
//}
//
//impl SizeHint for UolObject {
//    #[inline]
//    fn size_hint(&self) -> u32 {
//        1 + self.uri.size_hint()
//    }
//}
//
//impl ToXml for UolObject {
//    fn tag(&self) -> &'static str {
//        "uol"
//    }
//
//    fn attributes(&self, name: &str) -> Vec<(String, String)> {
//        vec![
//            (String::from("name"), name.to_string()),
//            (String::from("value"), self.as_ref().to_string()),
//        ]
//    }
//}
