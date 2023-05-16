//! Parsed Sound type
//!
//! Sound objects do not always adhere to the size constraint in the Property. Maybe this size is
//! the decoded size? The size should be ignored when parsing this. It is quite annoying.

use crate::{
    error::{DecodeError, Result},
    io::{xml::writer::ToXml, Decode, Encode, SizeHint, WzRead, WzWrite},
    types::WzInt,
};
use std::{fmt, fs, io::Write, path::Path};

mod format;
mod header;

use header::HEADER;

pub use format::AudioFormat;
pub use header::{SoundHeader, WavHeader};

#[derive(Clone, PartialEq, Eq)]
pub struct Sound {
    duration: WzInt,
    header: SoundHeader,
    data: Vec<u8>,
}

impl Sound {
    pub fn new(duration: WzInt, header: SoundHeader, data: Vec<u8>) -> Self {
        Self {
            duration,
            header,
            data,
        }
    }

    /// Constructs a Sound object from a wav file. The duration is probably in the metadata but I
    /// do not want to parse it here.
    pub fn from_wav<S>(path: S, duration: WzInt) -> Result<Self>
    where
        S: AsRef<Path>,
    {
        let data = fs::read(path)?;
        let header = SoundHeader::from_slice(&data)?;
        let data = data.as_slice()[HEADER.len() + 1 + header.as_bytes().len()..].to_vec();
        Ok(Self {
            duration,
            header,
            data,
        })
    }

    pub fn duration(&self) -> WzInt {
        self.duration
    }

    pub fn header(&self) -> &SoundHeader {
        &self.header
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn save_to_file<S>(&self, path: S) -> Result<()>
    where
        S: AsRef<Path>,
    {
        let bytes = self.header.as_bytes();
        let mut file = fs::File::create(path)?;
        file.write_all(HEADER)?;
        file.write_all(&[bytes.len() as u8])?;
        file.write_all(bytes)?;
        Ok(file.write_all(&self.data)?)
    }
}

impl fmt::Debug for Sound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Sound {{ duration: {:?}, header: {:?}, data: [..] }}",
            self.duration, self.header,
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

        // Decode the wav_header. The len is probably a WzInt but the size should always be 16-34
        // bytes.
        let header = SoundHeader::decode(reader)?;

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
        self.header.encode(writer)?;
        writer.write_all(&self.data)
    }
}

impl SizeHint for Sound {
    fn size_hint(&self) -> u32 {
        2 + WzInt::from(self.data.len() as i32).size_hint()
            + self.duration.size_hint()
            + self.header.size_hint()
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
