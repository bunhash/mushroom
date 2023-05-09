//! Parsed Sound type

use crate::{
    io::{decode, encode, xml::writer::ToXml, Decode, Encode, WzReader, WzWriter},
    types::WzInt,
};
use crypto::{Decryptor, Encryptor};
use std::{
    fmt,
    io::{Read, Seek, Write},
};

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
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
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
        Ok(Self {
            duration,
            header,
            data,
        })
    }
}

impl Encode for Sound {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        0u8.encode(writer)?;
        WzInt::from(self.data.len() as i32).encode(writer)?;
        self.duration.encode(writer)?;
        writer.write_all(&self.header)?;
        writer.write_all(&self.data)
    }
}

impl encode::SizeHint for Sound {
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
