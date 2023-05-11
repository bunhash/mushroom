//! WZ Image Canvas

use crate::{
    error::{CanvasError, DecodeError, Result},
    file::image::raw::Property,
    io::{Decode, WzReader},
    types::WzInt,
};
use crypto::Decryptor;
use std::io::{Read, Seek};

#[derive(Debug)]
pub struct Canvas {
    width: WzInt,
    height: WzInt,
    format: WzInt,
    format2: u8,
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
    pub fn format(&self) -> WzInt {
        self.format
    }

    /// Returns the MagLevel
    pub fn format2(&self) -> u8 {
        self.format2
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
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self>
    where
        R: Read + Seek,
        D: Decryptor,
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
        let format = WzInt::decode(reader)?;
        let format2 = u8::decode(reader)?;
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
            format2,
            data,
            property,
        })
    }
}
