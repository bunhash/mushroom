//! WZ Property Object

use crate::{macros, offset::Offset, string::UolString, Decode, Error, Reader};
use crypto::Decryptor;
use std::io::{Read, Seek};

/// Object types
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct Object {
    pub tag: ObjectTag,
    pub offset: u32,
}

/// Uol Object Tags
pub enum UolObjectTag {
    Placed(ObjectTag),
    Referenced(u32),
}

impl Decode for UolObjectTag {
    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Self::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let check = u8::decode(self)?;
        match check {
            0x73 => ObjectTag::decode(self),
            0x1b => {
                let offset = WzOffset::from(u32::decode(self)?);
                let pos = self.position()?;
                self.seek(offset)?;
                let string = String::decode(self)?;
                self.seek(pos)?;
                Ok(string)
            }
            u => Err(ImageError::UolType(u).into()),
        }
    }
}

/// Object Tags
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum ObjectTag {
    /// List of properties
    PropertyList,

    /// Canvas
    Canvas,

    /// Shape2D#Convex2D
    Convex,

    /// Shape2D#Vector2D
    Vector,

    /// UOL object
    Uol,

    /// Sound_DX8
    Sound,
}

impl Decode for ObjectTag {
    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Self::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let tag = String::decode(reader)?;
        Ok(match tag.as_ref() {
            "Property" => Self::Property,
            "Canvas" => Self::Canvas,
            "Shape2D#Convex2D" => Self::Convex,
            "Shape2D#Vector2D" => Self::Vector,
            "UOL" => Self::Uol,
            "Sound_DX8" => Self::Sound,
            t => Err(ImageError::ObjectType(String::from(t)).into())?,
        })
    }
}
