//! Archive Errors

use crate::{decode, encode};
use std::{fmt, io};

/// Decode errors
#[derive(Debug)]
pub enum Error {
    /// IO Errors
    Io(io::Error),

    /// Decode Errors
    Decode(decode::Error),

    /// Encode Errors
    Encode(encode::Error),

    /// Invalid header
    Header,

    /// Bruteforce failure
    Bruteforce,

    /// Invalid version
    Version,

    /// Invalid length
    Length(i32),

    /// Invalid size
    Size(i32),

    /// Invalid offset
    Offset(i32),

    /// Invalid content tag
    Tag(u8),

    /// Catch all
    Other(Box<str>),
}

impl Error {
    /// Custom error message
    pub fn other(msg: &str) -> Self {
        Self::Other(msg.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io: {}", e),
            Self::Decode(e) => write!(f, "decode: {}", e),
            Self::Encode(e) => write!(f, "encode: {}", e),
            Self::Header => f.write_str("invalid archive header"),
            Self::Bruteforce => f.write_str("failed to bruteforce version"),
            Self::Version => f.write_str("invalid WZ archive version"),
            Self::Length(l) => write!(f, "invalid package length: {}", l),
            Self::Size(s) => write!(f, "invalid content size: {}", s),
            Self::Offset(o) => write!(f, "invalid offset: {}", o),
            Self::Tag(t) => write!(f, "invalid content tag: 0x{:02x}", t),
            Self::Other(s) => f.write_str(s),
        }
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

impl From<decode::Error> for Error {
    fn from(other: decode::Error) -> Self {
        Error::Decode(other)
    }
}

impl From<encode::Error> for Error {
    fn from(other: encode::Error) -> Self {
        Error::Encode(other)
    }
}
