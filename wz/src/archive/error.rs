//! Archive Errors

use crate::decode;
use std::{fmt, io};

/// Decode errors
#[derive(Debug)]
pub enum Error {
    /// Decode Errors
    Decode(decode::Error),

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
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode(e) => write!(f, "decode: {}", e),
            Self::Header => f.write_str("invalid archive header"),
            Self::Bruteforce => f.write_str("failed to bruteforce version"),
            Self::Version => f.write_str("invalid WZ archive version"),
            Self::Length(l) => write!(f, "invalid package length: {}", l),
            Self::Size(s) => write!(f, "invalid content size: {}", s),
            Self::Offset(o) => write!(f, "invalid offset: {}", o),
            Self::Tag(t) => write!(f, "invalid content tag: 0x{:02x}", t),
        }
    }
}

impl From<decode::Error> for Error {
    fn from(other: decode::Error) -> Self {
        Error::Decode(other)
    }
}
impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Decode(other.into())
    }
}
