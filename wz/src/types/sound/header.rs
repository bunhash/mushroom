//! Sound Header

use crate::{
    error::{Error, Result, SoundError},
    io::{Decode, Encode, SizeHint, WzRead, WzWrite},
    types::sound::AudioFormat,
};
use std::fmt;

pub(crate) const HEADER: &[u8] = &[
    0x02, 0x83, 0xEB, 0x36, 0xE4, 0x4F, 0x52, 0xCE, 0x11, 0x9F, 0x53, 0x00, 0x20, 0xAF, 0x0B, 0xA7,
    0x70, 0x8B, 0xEB, 0x36, 0xE4, 0x4F, 0x52, 0xCE, 0x11, 0x9F, 0x53, 0x00, 0x20, 0xAF, 0x0B, 0xA7,
    0x70, 0x00, 0x01, 0x81, 0x9F, 0x58, 0x05, 0x56, 0xC3, 0xCE, 0x11, 0xBF, 0x01, 0x00, 0xAA, 0x00,
    0x55, 0x59, 0x5A,
];

#[derive(Clone, PartialEq, Eq)]
pub struct SoundHeader {
    header: Vec<u8>,
}

impl SoundHeader {
    pub fn as_bytes(&self) -> &[u8] {
        self.header.as_slice()
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        // If the size is too small, just error here
        if slice.len() < HEADER.len() + 17 {
            return Err(SoundError::SoundHeader(slice.to_vec()).into());
        }

        // Ensure the header is correct
        if &slice[0..HEADER.len()] != HEADER {
            return Err(SoundError::SoundHeader(slice[0..HEADER.len()].to_vec()).into());
        }

        // Decode the wav_header
        let wav_header_len = slice[HEADER.len()] as usize;
        if wav_header_len < 16 || wav_header_len == 17 {
            return Err(SoundError::WavHeaderLength(wav_header_len).into());
        }
        let header = slice[HEADER.len() + 1..HEADER.len() + 1 + wav_header_len].to_vec();

        Ok(Self { header })
    }
}

impl From<WavHeader> for SoundHeader {
    fn from(mut other: WavHeader) -> Self {
        let mut header = Vec::with_capacity(other.size_hint() as usize);
        header.extend_from_slice(&u16::from(other.audio_format).to_le_bytes());
        header.extend_from_slice(&other.channel_count.to_le_bytes());
        header.extend_from_slice(&other.sampling_rate.to_le_bytes());
        header.extend_from_slice(&other.bytes_per_second.to_le_bytes());
        header.extend_from_slice(&other.bytes_per_sample.to_le_bytes());
        header.extend_from_slice(&other.bits_per_sample.to_le_bytes());
        if !other.extra.is_empty() {
            header.append(&mut other.extra);
        }
        Self { header }
    }
}

impl fmt::Debug for SoundHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SoundHeader {{ data: [..] }}")
    }
}

impl Decode for SoundHeader {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead,
    {
        // Decode static header
        let mut sound_header = vec![0u8; HEADER.len()];
        reader.read_exact(&mut sound_header)?;
        if sound_header != HEADER {
            return Err(SoundError::SoundHeader(sound_header).into());
        }

        // Decode the wav_header
        let wav_header_len = u8::decode(reader)? as usize;
        if wav_header_len < 16 || wav_header_len == 17 {
            return Err(SoundError::WavHeaderLength(wav_header_len).into());
        }
        let mut header = vec![0u8; wav_header_len];
        reader.read_exact(&mut header)?;

        // Decrypt it
        reader.decrypt(&mut header);

        Ok(Self { header })
    }
}

impl Encode for SoundHeader {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite,
    {
        writer.write_all(HEADER)?;
        (self.header.len() as u8).encode(writer)?;
        writer.write_all(&self.header)
    }
}

impl SizeHint for SoundHeader {
    fn size_hint(&self) -> u32 {
        1 + HEADER.len() as u32 + self.header.len() as u32
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WavHeader {
    pub audio_format: AudioFormat,
    pub channel_count: u16,
    pub sampling_rate: u32,
    pub bytes_per_second: u32,
    pub bytes_per_sample: u16,
    pub bits_per_sample: u16,
    pub extra: Vec<u8>,
}

impl WavHeader {
    pub fn from_slice(header: &[u8]) -> Result<Self> {
        let audio_format = AudioFormat::from(u16::from_le_bytes([header[0], header[1]]));
        let channel_count = u16::from_le_bytes([header[2], header[3]]);
        let sampling_rate = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);
        let bytes_per_second = u32::from_le_bytes([header[8], header[9], header[10], header[11]]);
        let bytes_per_sample = u16::from_le_bytes([header[12], header[13]]);
        let bits_per_sample = u16::from_le_bytes([header[14], header[15]]);
        let (extra_len, extra) = if header.len() > 16 {
            (
                u16::from_le_bytes([header[16], header[17]]) as usize,
                Vec::from(&header[18..]),
            )
        } else {
            (0, Vec::new())
        };

        // Sanity check
        if header.len() != 18 + extra_len {
            return Err(SoundError::ExtraLength(extra_len).into());
        }

        Ok(Self {
            audio_format,
            channel_count,
            sampling_rate,
            bytes_per_second,
            bytes_per_sample,
            bits_per_sample,
            extra,
        })
    }
}

impl TryFrom<SoundHeader> for WavHeader {
    type Error = Error;

    fn try_from(other: SoundHeader) -> Result<Self> {
        Self::from_slice(other.header.as_slice())
    }
}

impl Encode for WavHeader {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite,
    {
        ((self.size_hint() - 1) as u8).encode(writer)?;
        self.audio_format.encode(writer)?;
        self.channel_count.encode(writer)?;
        self.sampling_rate.encode(writer)?;
        self.bytes_per_second.encode(writer)?;
        self.bytes_per_sample.encode(writer)?;
        self.bits_per_sample.encode(writer)?;
        if !self.extra.is_empty() {
            writer.write_all(&self.extra)?;
        }
        Ok(())
    }
}

impl SizeHint for WavHeader {
    fn size_hint(&self) -> u32 {
        let size = 1
            + self.audio_format.size_hint()
            + self.channel_count.size_hint()
            + self.sampling_rate.size_hint()
            + self.bytes_per_second.size_hint()
            + self.bytes_per_sample.size_hint()
            + self.bits_per_sample.size_hint();
        if self.extra.is_empty() {
            size
        } else {
            size + 2 + self.extra.len() as u32
        }
    }
}
