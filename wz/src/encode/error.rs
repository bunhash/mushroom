//! Encode Errors

use std::{fmt, io};

/// Encode error codes
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
pub enum ErrorCode {
    /// IO Errors
    Io,

    /// Invalid UolString
    UolString,

    /// Invalid position
    Position,

    /// Content is too large
    TooLarge,

    /// Catch-all other
    Other,
}

/// Encode errors
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

    /// Builds new UOL string `Error`
    pub fn uol_string(msg: &str) -> Self {
        Self::new(ErrorCode::UolString, msg)
    }

    /// Builds new position `Error`
    pub fn position(msg: &str) -> Self {
        Self::new(ErrorCode::Position, msg)
    }

    /// Builds new TooLarge `Error`
    pub fn too_large(msg: &str) -> Self {
        Self::new(ErrorCode::TooLarge, msg)
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
            ErrorCode::UolString => write!(f, "UOL string: {}", self.msg),
            ErrorCode::Position => write!(f, "position: {}", self.msg),
            ErrorCode::TooLarge => write!(f, "too large: {}", self.msg),
            ErrorCode::Other => write!(f, "other: {}", self.msg),
        }
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::io(&format!("{}", other))
    }
}
