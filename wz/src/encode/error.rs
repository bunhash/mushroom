//! Encode Errors

use std::{fmt, io};

/// Encode errors
#[derive(Debug)]
pub enum Error {
    /// IO Errors
    Io(io::Error),

    /// Invalid UolString
    UolString,

    /// Invalid position
    Position(u64),

    /// Content is too large
    TooLarge(u64),
}

impl Error {
    /// Builds new `Error::Position`
    pub fn position(position: u64) -> Self {
        Self::Position(position)
    }

    /// Builds new `Error::TooLarge`
    pub fn too_large(size: u64) -> Self {
        Self::TooLarge(size)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io: {}", e),
            Self::UolString => f.write_str("UOL string must not be referenced"),
            Self::Position(p) => write!(f, "invalid position ({:08x})", p),
            Self::TooLarge(s) => write!(f, "content is too large: {}", s),
        }
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}
