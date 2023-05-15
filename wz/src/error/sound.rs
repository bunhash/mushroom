//! Sound Errors

use std::fmt;

/// Possible sound errors
#[derive(Debug)]
pub enum SoundError {
    /// Unknown audio format
    AudioFormat(u16),

    /// Extra header bytes do not add up
    ExtraLength(usize),

    /// The WAV header length is invalid
    WavHeaderLength(usize),

    /// Sound Header is invalid
    SoundHeader(Vec<u8>),
}

impl fmt::Display for SoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AudioFormat(t) => write!(f, "Unknown audio format: `{}`", t),
            Self::ExtraLength(l) => write!(f, "Extra bytes length does not add up: `{}`", l),
            Self::SoundHeader(b) => write!(f, "Unknown sound header: {:?}", b),
            Self::WavHeaderLength(l) => write!(f, "Invalid header length: `{}`", l),
        }
    }
}
