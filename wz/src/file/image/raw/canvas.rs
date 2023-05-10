//! WZ Image Canvas

use crate::{
    file::image::raw::Property,
    io::{decode, Decode, WzReader},
    types::WzInt,
};
use crypto::Decryptor;
use std::io::{Read, Seek};

#[derive(Debug)]
pub struct Canvas {
    width: WzInt,
    height: WzInt,
    format: WzInt,
    mag_level: u8,
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
    pub fn mag_level(&self) -> u8 {
        self.mag_level
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
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
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
            return Err(decode::Error::InvalidObject);
        }
        let format = WzInt::decode(reader)?;
        let mag_level = u8::decode(reader)?;
        if i32::decode(reader)? != 0 {
            return Err(decode::Error::InvalidObject);
        }
        let length = i32::decode(reader)?;
        if length.is_negative() {
            return Err(decode::Error::InvalidLength(length));
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
            mag_level,
            data,
            property,
        })
    }
}
