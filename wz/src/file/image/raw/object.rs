//! WZ Property Object

use crate::{
    file::image::{
        raw::{Canvas, Property},
        Sound, UolObject, UolString, Vector2D,
    },
    io::{decode, encode, Decode, Encode, WzReader, WzWriter},
    types::{WzOffset, WzString},
};
use crypto::{Decryptor, Encryptor};
use std::io::{Read, Seek, Write};

/// These are just complex structures compared to the primitive values contained in WZ properties
#[derive(Debug)]
pub enum Object {
    /// Contains an embedded list of properties
    Property(Property),

    /// Canvas type
    Canvas(Canvas),

    /// Shape2D#Convex2D
    Convex2D,

    /// Shape2D#Vector2D
    Vector2D(Vector2D),

    /// UOL object
    Uol(UolObject),

    /// Sound_DX8
    Sound(Sound),
}

impl Decode for Object {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let typename = match u8::decode(reader)? {
            0x73 => WzString::decode(reader)?,
            0x1b => {
                let offset = WzOffset::from(u32::decode(reader)?);
                let pos = reader.position()?;
                reader.seek(offset)?;
                let typename = WzString::decode(reader)?;
                reader.seek(pos)?;
                typename
            }
            t => return Err(decode::Error::InvalidUol(t)),
        };
        match typename.as_ref() {
            "Property" => Ok(Self::Property(Property::decode(reader)?)),
            "Canvas" => Ok(Self::Canvas(Canvas::decode(reader)?)),
            "Shape2D#Convex2D" => Ok(Self::Convex2D),
            "Shape2D#Vector2D" => Ok(Self::Vector2D(Vector2D::decode(reader)?)),
            "UOL" => Ok(Self::Uol(UolObject::decode(reader)?)),
            "Sound_DX8" => Ok(Self::Sound(Sound::decode(reader)?)),
            t => Err(decode::Error::InvalidObjectType(String::from(t))),
        }
    }
}

impl Encode for Object {
    fn encode<W, E>(&self, _writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        unimplemented!()
    }
}