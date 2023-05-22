//! WZ Image Canvas

use crate::error::{CanvasError, DecodeError, Result};
use crate::io::{Decode, WzRead};
use crate::types::{raw::Property, CanvasFormat, WzInt};

#[derive(Debug)]
pub(crate) struct Canvas {
    pub(crate) width: WzInt,
    pub(crate) height: WzInt,
    pub(crate) format: CanvasFormat,
    pub(crate) data: Vec<u8>,
    pub(crate) property: Option<Property>,
}

impl Decode for Canvas {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead + ?Sized,
    {
        u8::decode(reader)?;
        let property = match u8::decode(reader)? {
            1 => Some(Property::decode(reader)?),
            _ => None,
        };
        let width = WzInt::decode(reader)?;
        let height = WzInt::decode(reader)?;
        if width > 0x10000 || height > 0x10000 {
            return Err(CanvasError::TooBig(*width as u32, *height as u32).into());
        }
        let format = CanvasFormat::decode(reader)?;
        i32::decode(reader)?;
        let length = i32::decode(reader)?;
        if length.is_negative() {
            return Err(DecodeError::Length(length).into());
        }
        let length = length as usize - 1;
        u8::decode(reader)?;
        let mut data = vec![0u8; length];
        reader.read_exact(&mut data)?;

        /*
        // Check if the compressed data is encrypted
        if data[0] != 0x78
            || (data[1] != 0x01 && data[1] != 0x5e && data[1] != 0x9c && data[1] != 0xda)
        {
            reader.decrypt(&mut data);
        }
        */
        Ok(Self {
            width,
            height,
            format,
            data,
            property,
        })
    }
}
