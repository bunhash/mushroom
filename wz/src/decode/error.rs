//! Decode Errors

use std::{fmt, io, string};

/// Decode error codes
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
pub enum ErrorCode {
    /// IO Errors
    Io,

    /// Length Errors
    Length,

    /// Position Errors
    Position,

    /// String encoding Errors
    StringEncoding,

    /// Catch-all
    Other,
}

/// Decode errors
#[derive(Debug)]
pub struct Error {
    code: ErrorCode,
    msg: Box<str>,
}

impl Error {
    /// Builds new `Error`
    pub fn new(code: ErrorCode, msg: &str) -> Self {
        Self {
            code,
            msg: msg.into(),
        }
    }

    /// Builds new IO `Error`
    pub fn io(msg: &str) -> Self {
        Self::new(ErrorCode::Io, msg)
    }

    /// Builds new length `Error`
    pub fn length(msg: &str) -> Self {
        Self::new(ErrorCode::Length, msg)
    }

    /// Builds new position `Error`
    pub fn position(msg: &str) -> Self {
        Self::new(ErrorCode::Position, msg)
    }

    /// Builds new string encoding `Error`
    pub fn string_encoding(msg: &str) -> Self {
        Self::new(ErrorCode::StringEncoding, msg)
    }

    /// Builds new other `Error`
    pub fn other(msg: &str) -> Self {
        Self::new(ErrorCode::Other, msg)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            ErrorCode::Io => write!(f, "io: {}", self.msg),
            ErrorCode::Length => write!(f, "length: {}", self.msg),
            ErrorCode::Position => write!(f, "position: {}", self.msg),
            ErrorCode::StringEncoding => write!(f, "string encoding: {}", self.msg),
            ErrorCode::Other => write!(f, "other: {}", self.msg),
        }
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::io(&format!("{}", other))
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(other: string::FromUtf8Error) -> Self {
        Error::string_encoding(&format!("{}", other))
    }
}

impl From<string::FromUtf16Error> for Error {
    fn from(other: string::FromUtf16Error) -> Self {
        Error::string_encoding(&format!("{}", other))
    }
}
