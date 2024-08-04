//! WZ Image Canvas

use crate::error::{CanvasError, DecodeError, Result};
use crate::io::{Decode, WzRead};
use crate::types::{raw::Property, CanvasFormat, WzInt, WzOffset};

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
        let data = read_raw_image_data(reader, length)?;

        Ok(Self {
            width,
            height,
            format,
            data,
            property,
        })
    }
}

fn read_raw_image_data<R>(reader: &mut R, length: usize) -> Result<Vec<u8>>
where
    R: WzRead + ?Sized,
{
    let position = reader.position()?;
    let header = u16::decode(reader)?;
    reader.seek(position)?;

    // The image is cleartext
    if header == 0x0178 || header == 0x5e78 || header == 0x9c78 || header == 0xda78 {
        let mut data = vec![0u8; length];
        reader.read_exact(&mut data)?;
        Ok(data)
    }
    // The image is obfuscated...
    else {
        let mut data = Vec::new();
        let end_position: WzOffset = position + length.into();
        while reader.position()? < end_position {
            let block_size = u32::decode(reader)? as usize;
            let mut buf = vec![0u8; block_size];
            reader.read_exact(&mut buf)?;
            reader.decrypt(&mut buf);
            data.append(&mut buf);
        }
        Ok(data)
    }
}
