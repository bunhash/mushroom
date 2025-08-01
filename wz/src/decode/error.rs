//! Decode Errors

use std::{fmt, io, string};

/// Decode errors
#[derive(Debug)]
pub enum Error {
    /// IO Errors
    Io(io::Error),

    /// Invalid lengths
    Length(i32),

    /// Invalid position (stream is before content)
    Position {
        /// Content start
        start: u32,

        /// Current position
        position: u64,
    },

    /// Invalid tag
    Tag(u8),

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
    pub fn position(start: u32, position: u64) -> Self {
        Self::Position { start, position }
    }

    /// Builds new `Error::Tag`
    pub fn tag(tag: u8) -> Self {
        Self::Tag(tag)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io: {}", e),
            Self::Length(l) => write!(f, "invalid length: {}", l),
            Self::Position { start, position } => write!(
                f,
                "stream before content (start={:08x}, position={:08x})",
                start, position
            ),
            Self::Tag(t) => write!(f, "invalid tag: 0x{:02x}", t),
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
