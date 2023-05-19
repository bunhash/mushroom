//! Audio Formats

use crate::{
    error::Result,
    io::{Decode, Encode, SizeHint, WzRead, WzWrite},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Pcm,
    Mp3,
    Unknown(u16),
}

impl From<AudioFormat> for u16 {
    fn from(other: AudioFormat) -> Self {
        match other {
            AudioFormat::Pcm => 1,
            AudioFormat::Mp3 => 85,
            AudioFormat::Unknown(t) => t,
        }
    }
}

impl From<u16> for AudioFormat {
    fn from(other: u16) -> Self {
        match other {
            1 => Self::Pcm,
            85 => Self::Mp3,
            t => Self::Unknown(t),
        }
    }
}

impl Decode for AudioFormat {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead,
    {
        Ok(Self::from(u16::decode(reader)?))
    }
}

impl Encode for AudioFormat {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite + ?Sized,
    {
        match self {
            Self::Pcm => 1u16.encode(writer),
            Self::Mp3 => 85u16.encode(writer),
            Self::Unknown(b) => b.encode(writer),
        }
    }
}

impl SizeHint for AudioFormat {
    fn size_hint(&self) -> u32 {
        2
    }
}
