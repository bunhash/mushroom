//! Decode Error types

use std::{fmt, string};

/// Possible decoding errors
#[derive(Debug)]
pub enum DecodeError {
    /// The length is invalid (likely negative)
    Length(i32),

    /// The offset is invalid (likely negative)
    Offset(i32),

    /// Unable to decode UTF-8
    Utf8(string::FromUtf8Error),

    /// Unable to decode Unicode
    Unicode(string::FromUtf16Error),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Length(l) => write!(f, "Invalid length: `{}`", l),
            Self::Offset(o) => write!(f, "Invalid offset: `{}`", o),
            Self::Utf8(e) => write!(f, "UTF-8: {}", e),
            Self::Unicode(e) => write!(f, "Unicode: {}", e),
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
