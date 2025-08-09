//! Image Errors

use crate::{decode, encode};
use std::{fmt, io};

/// Image error codes
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
pub enum ErrorCode {
    /// IO Errors
    Io,

    /// Decode Errors
    Decode,

    /// Encode Errors
    Encode,

    /// Invalid length
    Length,

    /// Invalid size
    Size,

    /// Invalid offset
    Offset,

    /// Invalid content tag
    Tag,

    /// Catch all
    Other,
}

/// Image errors
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

    /// IO `Error`
    pub fn io(msg: &str) -> Self {
        Self::new(ErrorCode::Io, msg)
    }

    /// Decode `Error`
    pub fn decode(msg: &str) -> Self {
        Self::new(ErrorCode::Decode, msg)
    }

    /// Encode `Error`
    pub fn encode(msg: &str) -> Self {
        Self::new(ErrorCode::Encode, msg)
    }

    /// Length `Error`
    pub fn length(msg: &str) -> Self {
        Self::new(ErrorCode::Length, msg)
    }

    /// Size `Error`
    pub fn size(msg: &str) -> Self {
        Self::new(ErrorCode::Size, msg)
    }

    /// Offset `Error`
    pub fn offset(msg: &str) -> Self {
        Self::new(ErrorCode::Offset, msg)
    }

    /// Tag `Error`
    pub fn tag(msg: &str) -> Self {
        Self::new(ErrorCode::Tag, msg)
    }

    /// Custom error message
    pub fn other(msg: &str) -> Self {
        Self::new(ErrorCode::Other, msg)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            ErrorCode::Io => write!(f, "{}", self.msg),
            ErrorCode::Decode => write!(f, "{}", self.msg),
            ErrorCode::Encode => write!(f, "{}", self.msg),
            ErrorCode::Length => write!(f, "{}", self.msg),
            ErrorCode::Size => write!(f, "{}", self.msg),
            ErrorCode::Offset => write!(f, "{}", self.msg),
            ErrorCode::Tag => write!(f, "{}", self.msg),
            ErrorCode::Other => write!(f, "{}", self.msg),
        }
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Self::io(&format!("{}", other))
    }
}

impl From<decode::Error> for Error {
    fn from(other: decode::Error) -> Self {
        Self::decode(&format!("{}", other))
    }
}

impl From<encode::Error> for Error {
    fn from(other: encode::Error) -> Self {
        Self::encode(&format!("{}", other))
    }
}
