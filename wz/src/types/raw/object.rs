//! WZ Property Object

use crate::{
    error::{ImageError, Result},
    io::{Decode, WzRead},
    types::{
        raw::{Canvas, Property},
        Sound, UolObject, Vector,
    },
};

/// These are just complex structures compared to the primitive values contained in WZ properties
#[derive(Debug)]
pub enum Object {
    /// Contains an embedded list of properties
    Property(Property),

    /// Canvas type
    Canvas(Canvas),

    /// Shape2D#Convex2D
    Convex,

    /// Shape2D#Vector2D
    Vector(Vector),

    /// UOL object
    Uol(UolObject),

    /// Sound_DX8
    Sound(Sound),
}

impl Decode for Object {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead + ?Sized,
    {
        let typename = reader.read_object_tag()?;
        match typename.as_ref() {
            "Property" => Ok(Self::Property(Property::decode(reader)?)),
            "Canvas" => Ok(Self::Canvas(Canvas::decode(reader)?)),
            "Shape2D#Convex2D" => Ok(Self::Convex),
            "Shape2D#Vector2D" => Ok(Self::Vector(Vector::decode(reader)?)),
            "UOL" => Ok(Self::Uol(UolObject::decode(reader)?)),
            "Sound_DX8" => Ok(Self::Sound(Sound::decode(reader)?)),
            t => Err(ImageError::ObjectType(String::from(t)).into()),
        }
    }
}
