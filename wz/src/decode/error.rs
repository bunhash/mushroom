//! Decode Errors

use std::{fmt, io, string};

/// Decode errors
#[derive(Debug)]
pub enum Error {
    /// IO Errors
    Io(io::Error),

    /// Invalid lengths
    Length(i32),

    /// Invalid position
    Position(u64),

    /// Unable to decode UTF-8
    Utf8(string::FromUtf8Error),

    /// Unable to decode Unicode
    Unicode(string::FromUtf16Error),
}

impl Error {
    /// Builds new `Error::Length`
    pub fn length(length: i32) -> Self {
        Self::Length(length)
    }

    /// Builds new `Error::Position`
    pub fn position(position: u64) -> Self {
        Self::Position(position)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io: {}", e),
            Self::Length(l) => write!(f, "invalid length: {}", l),
            Self::Position(p) => write!(f, "invalid position: {}", p),
            Self::Utf8(e) => write!(f, "utf-8: {}", e),
            Self::Unicode(e) => write!(f, "unicode: {}", e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(other: string::FromUtf8Error) -> Self {
        Error::Utf8(other)
    }
}

impl From<string::FromUtf16Error> for Error {
    fn from(other: string::FromUtf16Error) -> Self {
        Error::Unicode(other)
    }
}
