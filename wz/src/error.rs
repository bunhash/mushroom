//! Errors

use crate::{decode, map};
use std::{fmt, io};

pub type Result<T> = core::result::Result<T, Error>;

/// Overall Error catcher
#[derive(Debug)]
pub enum Error {
    /// Decoding error
    Decode(decode::Error),

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

    /// The Metadata is invalid or cannot be parsed
    InvalidMetadata,

    /// The WZ package is invalid
    InvalidPackage,
}

impl fmt::Display for WzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WzError::BruteForceChecksum => write!(f, "Brute force of the checksum failed"),
            WzError::InvalidMetadata => write!(f, "Invalid WZ file"),
            WzError::InvalidPackage => write!(f, "Invalid WZ pacakge"),
        }
    }
}
