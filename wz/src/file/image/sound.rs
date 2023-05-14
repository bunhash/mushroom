//! Parsed Sound type
//!
//! Sound objects do not always adhere to the size constraint in the Property. Maybe this size is
//! the decoded size? The size should be ignored when parsing this. It is quite annoying.

use crate::{
    error::{DecodeError, Result},
    io::{xml::writer::ToXml, Decode, Encode, SizeHint, WzRead, WzWrite},
    types::WzInt,
};
use std::fmt;

const HEADER: &[u8] = &[
    0x02, 0x83, 0xEB, 0x36, 0xE4, 0x4F, 0x52, 0xCE, 0x11, 0x9F, 0x53, 0x00, 0x20, 0xAF, 0x0B, 0xA7,
    0x70, 0x8B, 0xEB, 0x36, 0xE4, 0x4F, 0x52, 0xCE, 0x11, 0x9F, 0x53, 0x00, 0x20, 0xAF, 0x0B, 0xA7,
    0x70, 0x00, 0x01, 0x81, 0x9F, 0x58, 0x05, 0x56, 0xC3, 0xCE, 0x11, 0xBF, 0x01, 0x00, 0xAA, 0x00,
    0x55, 0x59, 0x5A,
];

#[derive(Clone, PartialEq, Eq)]
pub struct Sound {
    duration: WzInt,
    header: Vec<u8>,
    data: Vec<u8>,
}

impl Sound {
    pub fn new(duration: WzInt, header: Vec<u8>, data: Vec<u8>) -> Self {
        Self {
            duration,
            header,
            data,
        }
    }

    pub fn duration(&self) -> WzInt {
        self.duration
    }

    pub fn header(&self) -> &[u8] {
        &self.header
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl fmt::Debug for Sound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Sound {{ duration: {:?}, header: [..], data: [..] }}",
            self.duration
        )
    }
}

impl Decode for Sound {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead,
    {
        u8::decode(reader)?; // garbage byte?
        let data_len = WzInt::decode(reader)?;
        if data_len.is_negative() {
            return Err(DecodeError::Length(*data_len).into());
        }
        let data_len = *data_len as usize;
        let duration = WzInt::decode(reader)?;

        // Decode static header
        let mut header = vec![0u8; HEADER.len()];
        reader.read_exact(&mut header)?;

        // Decode the format specs
        let format_spec_len = u8::decode(reader)?;
        let mut format_spec = vec![0u8; format_spec_len as usize];
        reader.read_exact(&mut format_spec)?;

        // Concatenate the header info
        header.push(format_spec_len);
        header.append(&mut format_spec);

        // Decode data
        let mut data = vec![0u8; data_len];
        reader.read_exact(&mut data)?;

        Ok(Self {
            duration,
            header,
            data,
        })
    }
}

impl Encode for Sound {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite,
    {
        0u8.encode(writer)?;
        WzInt::from(self.data.len() as i32).encode(writer)?;
        self.duration.encode(writer)?;
        writer.write_all(&self.header)?;
        writer.write_all(&self.data)
    }
}

impl SizeHint for Sound {
    fn size_hint(&self) -> u32 {
        1 + WzInt::from(self.data.len() as i32).size_hint()
            + self.duration.size_hint()
            + self.header.len() as u32
            + self.data.len() as u32
    }
}

impl ToXml for Sound {
    fn tag(&self) -> &'static str {
        "sound"
    }

    fn attributes(&self, name: &str) -> Vec<(String, String)> {
        vec![(String::from("name"), name.to_string())]
    }
}
