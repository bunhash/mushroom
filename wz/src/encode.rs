//! Encoder Trait

use crate::WzWriter;
use crypto::Encryptor;
use std::{
    fmt,
    io::{self, Seek, Write},
};

/// Trait for encoding objects
pub trait Encode {
    /// Encodes objects
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), Error>
    where
        W: Write + Seek,
        E: Encryptor,
        Self: Sized;
}

/// Encode Error Types
#[derive(Debug)]
pub enum Error {
    /// IO error
    Io(io::ErrorKind),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(kind) => write!(f, "IO: {}", kind),
        }
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

/// Internal trait for quicker size estimation
pub(crate) trait SizeHint {
    fn size_hint(&self) -> i32;
}
