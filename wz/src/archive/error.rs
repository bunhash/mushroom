//! Archive Errors

use crate::{decode, encode};
use std::{fmt, io};

/// Archive error codes
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
pub enum ErrorCode {
    /// IO Errors
    Io,

    /// Decode Errors
    Decode,

    /// Encode Errors
    Encode,

    /// Invalid header
    Header,

    /// Bruteforce failure
    Bruteforce,

    /// Invalid version
    Version,

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

/// Archive errors
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

    /// Header `Error`
    pub fn header(msg: &str) -> Self {
        Self::new(ErrorCode::Header, msg)
    }

    /// Bruteforce `Error`
    pub fn bruteforce(msg: &str) -> Self {
        Self::new(ErrorCode::Bruteforce, msg)
    }

    /// Version `Error`
    pub fn version(msg: &str) -> Self {
        Self::new(ErrorCode::Version, msg)
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
            ErrorCode::Io => write!(f, "io: {}", self.msg),
            ErrorCode::Decode => write!(f, "decode: {}", self.msg),
            ErrorCode::Encode => write!(f, "encode: {}", self.msg),
            ErrorCode::Header => write!(f, "header: {}", self.msg),
            ErrorCode::Bruteforce => write!(f, "bruteforce: {}", self.msg),
            ErrorCode::Version => write!(f, "version: {}", self.msg),
            ErrorCode::Length => write!(f, "length: {}", self.msg),
            ErrorCode::Size => write!(f, "size: {}", self.msg),
            ErrorCode::Offset => write!(f, "offset: {}", self.msg),
            ErrorCode::Tag => write!(f, "tag: {}", self.msg),
            ErrorCode::Other => write!(f, "other: {}", self.msg),
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
