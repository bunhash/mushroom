//! WZ Property Object

use crate::{
    file::image::Uol,
    io::{decode, encode, Decode, Encode, WzReader, WzWriter},
};
use crypto::{Decryptor, Encryptor};
use std::io::{Read, Seek, Write};

/// These are just complex structures compared to the primitive values contained in WZ properties
pub enum Object {
    /// Contains an embedded list of properties
    Property,

    /// Canvas type
    Canvas(Vec<u8>),

    /// Shape2D#Convex2D
    Convex2D(Vec<u8>),

    /// Shape2D#Vector2D
    Vector2D(Vec<u8>),

    /// UOL object
    Uol(Uol),

    /// Sound_DX8
    Sound(Vec<u8>),
}

impl Object {
    pub fn decode_with_size<R, D>(
        reader: &mut WzReader<R, D>,
        size: usize,
    ) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let typename = Uol::decode(reader)?;
        match typename.as_ref() {
            "Property" => Ok(Self::Property),
            "Canvas" => {
                let mut buf = vec![0u8; size];
                reader.read_exact(&mut buf)?;
                Ok(Self::Canvas(buf))
            }
            "Shape2D#Convex2D" => {
                let mut buf = vec![0u8; size];
                reader.read_exact(&mut buf)?;
                Ok(Self::Convex2D(buf))
            }
            "Shape2D#Vector2D" => {
                let mut buf = vec![0u8; size];
                reader.read_exact(&mut buf)?;
                Ok(Self::Vector2D(buf))
            }
            "UOL" => Ok(Self::Uol(Uol::decode(reader)?)),
            "Sound_DX8" => {
                let mut buf = vec![0u8; size];
                reader.read_exact(&mut buf)?;
                Ok(Self::Sound(buf))
            }
            _ => Err(decode::Error::InvalidObjectType(String::from(
                typename.as_ref(),
            ))),
        }
    }
}

impl Encode for Object {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        unimplemented!()
    }
}
