use std::{
    fmt, io,
    string::{FromUtf16Error, FromUtf8Error},
};
use thiserror::Error;

pub type WzResult<T> = Result<T, WzError>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WzErrorType {
    InvalidWz,
    InvalidDir,
    InvalidType,
}

impl fmt::Display for WzErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WzErrorType::InvalidWz => write!(f, "Invalid WZ file"),
            WzErrorType::InvalidDir => write!(f, "Invalid directory"),
            WzErrorType::InvalidType => write!(f, "Invalid WZ object type"),
        }
    }
}

#[derive(Debug, Error)]
pub enum WzError {
    #[error("Invalid WZ file")]
    Wz(WzErrorType),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] FromUtf8Error),

    #[error("UTF-8 error: {0}")]
    Utf16(#[from] FromUtf16Error),
}

impl From<WzErrorType> for WzError {
    fn from(other: WzErrorType) -> Self {
        WzError::Wz(other)
    }
}
