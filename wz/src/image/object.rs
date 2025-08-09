//! WZ Property Object

use crate::{
    decode::{Decode, Decoder},
    encode::{Encode, Encoder, SizeHint},
    image::Error,
};

/// Object types
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct Object {
    /// Tag specifying the type of object
    pub tag: UolObjectTag,

    /// The offset of the object in the WZ image
    pub offset: u32,
}

impl Decode for Object {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        Ok(Self {
            tag: UolObjectTag::decode(decoder)?,
            offset: u32::decode(decoder)?,
        })
    }
}

impl Encode for Object {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        self.tag.encode(encoder)?;
        self.offset.encode(encoder)?;
        Ok(())
    }
}

impl SizeHint for Object {
    fn size_hint(&self) -> u64 {
        self.tag.size_hint() + self.offset.size_hint()
    }
}

/// Uol Object Tags
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum UolObjectTag {
    /// Object tag
    Placed(ObjectTag),

    /// Reference to an object tag
    Referenced(u32),
}

impl Decode for UolObjectTag {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        let check = u8::decode(decoder)?;
        Ok(match check {
            0x73 => Self::Placed(ObjectTag::decode(decoder)?),
            0x1b => Self::Referenced(u32::decode(decoder)?),
            u => Err(Error::tag(&format!("invalid UolObjectTag {:02x}", u)))?,
        })
    }
}

impl Encode for UolObjectTag {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        match self {
            Self::Placed(value) => {
                0x73u8.encode(encoder)?;
                value.encode(encoder)?;
            }
            Self::Referenced(value) => {
                0x1bu8.encode(encoder)?;
                value.encode(encoder)?;
            }
        }
        Ok(())
    }
}

impl SizeHint for UolObjectTag {
    fn size_hint(&self) -> u64 {
        match self {
            Self::Placed(value) => 1 + value.size_hint(),
            Self::Referenced(value) => 1 + value.size_hint(),
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
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        let tag = String::decode(decoder)?;
        Ok(match tag.as_ref() {
            "Property" => Self::PropertyList,
            "Canvas" => Self::Canvas,
            "Shape2D#Convex2D" => Self::Convex,
            "Shape2D#Vector2D" => Self::Vector,
            "UOL" => Self::Uol,
            "Sound_DX8" => Self::Sound,
            t => Err(Error::tag(&format!("invalid ObjectTag {}", t)))?,
        })
    }
}

impl Encode for ObjectTag {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        match self {
            ObjectTag::PropertyList => "Property".encode(encoder)?,
            ObjectTag::Canvas => "Canvas".encode(encoder)?,
            ObjectTag::Convex => "Shape2D#Convex2D".encode(encoder)?,
            ObjectTag::Vector => "Shape2D#Vector2D".encode(encoder)?,
            ObjectTag::Uol => "UOL".encode(encoder)?,
            ObjectTag::Sound => "Sound_DX8".encode(encoder)?,
        }
        Ok(())
    }
}

impl SizeHint for ObjectTag {
    fn size_hint(&self) -> u64 {
        match self {
            ObjectTag::PropertyList => "Property".size_hint(),
            ObjectTag::Canvas => "Canvas".size_hint(),
            ObjectTag::Convex => "Shape2D#Convex2D".size_hint(),
            ObjectTag::Vector => "Shape2D#Vector2D".size_hint(),
            ObjectTag::Uol => "UOL".size_hint(),
            ObjectTag::Sound => "Sound_DX8".size_hint(),
        }
    }
}
