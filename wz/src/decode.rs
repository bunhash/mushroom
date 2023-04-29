//! Decoder Trait

use crate::WzReader;
use crypto::Decryptor;
use std::{
    fmt,
    io::{self, Read, Seek},
    string,
};

/// Trait for decoding objects
pub trait Decode {
    /// Decodes objects
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
        Self: Sized;
}

#[derive(Debug)]
pub enum Error {
    InvalidContentType(u8),
    InvalidLength(i32),
    InvalidOffset(i32),
    Utf8(string::FromUtf8Error),
    Unicode(string::FromUtf16Error),
    Io(io::ErrorKind),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidContentType(t) => {
                write!(f, "Package ContentType is invalid `{}`", t)
            }
            Error::InvalidLength(l) => write!(f, "Invalid length: `{}`", l),
            Error::InvalidOffset(o) => write!(f, "Invalid offset: `{}`", o),
            Error::Utf8(e) => write!(f, "UTF-8: {}", e),
            Error::Unicode(e) => write!(f, "Unicode: {}", e),
            Error::Io(kind) => write!(f, "IO: {}", kind),
        }
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
