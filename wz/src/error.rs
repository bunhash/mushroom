//! Errors

use crate::{decode, encode, map};
use std::{fmt, io};

pub type Result<T> = core::result::Result<T, Error>;

/// Overall Error catcher
#[derive(Debug)]
pub enum Error {
    /// Decoding error
    Decode(decode::Error),

    /// Decoding error
    Encode(encode::Error),

    /// IO error
    Io(io::ErrorKind),

    /// Map error
    Map(map::Error),

    /// WZ error
    Wz(WzError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Decode(e) => write!(f, "Decode: {}", e),
            Error::Encode(e) => write!(f, "Encode: {}", e),
            Error::Io(kind) => write!(f, "IO: {}", kind),
            Error::Map(e) => write!(f, "Map: {}", e),
            Error::Wz(e) => write!(f, "WZ: {}", e),
        }
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

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other.kind())
    }
}

impl From<io::ErrorKind> for Error {
    fn from(other: io::ErrorKind) -> Self {
        Error::Io(other)
    }
}

impl From<map::Error> for Error {
    fn from(other: map::Error) -> Self {
        Error::Map(other)
    }
}

impl From<WzError> for Error {
    fn from(other: WzError) -> Self {
        Error::Wz(other)
    }
}

/// Various WZ-specific errors
#[derive(Debug)]
pub enum WzError {
    /// Brute forcing the checksum failed
    BruteForceChecksum,

    /// Invalid checksum
    InvalidChecksum,

    /// The WZ package is invalid
    InvalidImage,

    /// The WZ archive header is invalid or cannot be parsed
    InvalidHeader,

    /// The WZ package is invalid
    InvalidPackage,

    /// Read-only
    ReadOnly,

    /// Write-only
    WriteOnly,
}

impl fmt::Display for WzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WzError::BruteForceChecksum => write!(f, "Brute force of the checksum failed"),
            WzError::InvalidChecksum => {
                write!(f, "Invalid version checksum. Maybe try guess_version()?")
            }
            WzError::InvalidImage => write!(f, "Invalid WZ image"),
            WzError::InvalidHeader => write!(f, "Invalid WZ header"),
            WzError::InvalidPackage => write!(f, "Invalid WZ pacakge"),
            WzError::ReadOnly => write!(f, "WZ file is read-only"),
            WzError::WriteOnly => write!(f, "WZ file is write-only"),
        }
    }
}
