//! WZ Image Canvas

use crate::{
    error::{CanvasError, DecodeError, Result},
    file::image::{raw::Property, CanvasFormat},
    io::{Decode, WzRead},
    types::WzInt,
};

#[derive(Debug)]
pub struct Canvas {
    width: WzInt,
    height: WzInt,
    format: CanvasFormat,
    data: Vec<u8>,
    property: Option<Property>,
}

impl Canvas {
    /// Returns the width
    pub fn width(&self) -> WzInt {
        self.width
    }

    /// Returns the height
    pub fn height(&self) -> WzInt {
        self.height
    }

    /// Returns the format
    pub fn format(&self) -> CanvasFormat {
        self.format
    }

    /// Returns the data
    pub fn data(&self) -> &[u8] {
        &self.data[..]
    }

    /// Returns the property
    pub fn property(&self) -> &Option<Property> {
        &self.property
    }
}

impl Decode for Canvas {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead,
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

        // Check if the compressed data is encrypted
        if data[0] != 0x78
            || (data[1] != 0x01 && data[1] != 0x5e && data[1] != 0x9c && data[1] != 0xda)
        {
            reader.decrypt(&mut data);
        }
        Ok(Self {
            width,
            height,
            format,
            data,
            property,
        })
    }
}
