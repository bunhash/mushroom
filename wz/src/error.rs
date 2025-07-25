//! Errors

use std::{fmt, io, string};

/// IO and decoding errors
pub struct Error {
    inner: Box<ErrorImpl>,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &*self.inner)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &*self.inner)
    }
}

impl From<DecodeError> for Error {
    fn from(other: DecodeError) -> Self {
        Self {
            inner: Box::new(ErrorImpl::Decode(other)),
        }
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Self {
            inner: Box::new(ErrorImpl::Io(other)),
        }
    }
}

#[derive(Debug)]
pub enum ErrorImpl {
    /// Decode Errors
    Decode(DecodeError),

    /// IO Errors
    Io(io::Error),
}

impl fmt::Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode(e) => write!(f, "{}", e),
            Self::Io(e) => write!(f, "{}", e),
        }
    }
}

/// Decode errors
#[derive(Debug)]
pub enum DecodeError {
    /// Invalid WZ Header
    Header,

    /// The length is invalid (likely negative)
    Length(i32),

    /// The offset is invalid (likely negative)
    Offset(i32),

    /// Invalid tag
    Tag(u8),

    /// Unable to decode UTF-8
    Utf8(string::FromUtf8Error),

    /// Unable to decode Unicode
    Unicode(string::FromUtf16Error),
}

impl DecodeError {
    /// New header error
    pub fn header() -> Self {
        DecodeError::Header
    }

    /// New invalid length error
    pub fn length(length: i32) -> Self {
        DecodeError::Length(length)
    }

    /// New invalid offset error
    pub fn offset(offset: i32) -> Self {
        DecodeError::Offset(offset)
    }

    /// New invalid tag error
    pub fn tag(tag: u8) -> Self {
        DecodeError::Tag(tag)
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Header => write!(f, "invalid header"),
            Self::Length(l) => write!(f, "invalid length: {}", l),
            Self::Offset(o) => write!(f, "invalid offset: {}", o),
            Self::Tag(t) => write!(f, "invalid tag: 0x{:02x}", t),
            Self::Utf8(e) => write!(f, "{}", e),
            Self::Unicode(e) => write!(f, "{}", e),
        }
    }
}

impl From<string::FromUtf8Error> for DecodeError {
    fn from(other: string::FromUtf8Error) -> Self {
        Self::Utf8(other)
    }
}

impl From<string::FromUtf16Error> for DecodeError {
    fn from(other: string::FromUtf16Error) -> Self {
        Self::Unicode(other)
    }
}
