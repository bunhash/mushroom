//! WZ Property Object

use crate::{
    file::image::{Canvas, Property, Uol},
    io::{decode, encode, Decode, Encode, WzReader, WzWriter},
    types::WzInt,
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
    Vector2D { x: WzInt, y: WzInt },

    /// UOL object
    Uol(Uol),

    /// Sound_DX8
    Sound {
        duration: WzInt,
        header: Vec<u8>,
        data: Vec<u8>,
    },
}

impl Decode for Object {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let typename = Uol::decode(reader)?;
        match typename.as_ref() {
            "Property" => Ok(Self::Property(Property::decode(reader)?)),
            "Canvas" => Ok(Self::Canvas(Canvas::decode(reader)?)),
            "Shape2D#Convex2D" => Ok(Self::Convex2D),
            "Shape2D#Vector2D" => Ok(Self::Vector2D {
                x: WzInt::decode(reader)?,
                y: WzInt::decode(reader)?,
            }),
            "UOL" => {
                u8::decode(reader)?;
                Ok(Self::Uol(Uol::decode(reader)?))
            }
            "Sound_DX8" => {
                u8::decode(reader)?;
                let data_length = WzInt::decode(reader)?;
                if data_length.is_negative() {
                    return Err(decode::Error::InvalidLength(*data_length));
                }
                let data_length = *data_length as usize;
                let duration = WzInt::decode(reader)?;
                let mut header = vec![0u8; 82];
                reader.read_exact(&mut header)?;
                let mut data = vec![0u8; data_length];
                reader.read_exact(&mut data)?;
                Ok(Self::Sound {
                    duration,
                    header,
                    data,
                })
            }
            t => Err(decode::Error::InvalidObjectType(String::from(t))),
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
