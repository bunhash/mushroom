use std::{
    fmt, io,
    string::{FromUtf16Error, FromUtf8Error},
};

/// Result used within the wz crate
pub type WzResult<T> = Result<T, WzError>;

/// Different WZ error types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WzErrorType {
    InvalidWz,
    InvalidDir,
    InvalidPath,
    InvalidType,
    InvalidImg,
    InvalidProp,
    Unknown,
}

impl fmt::Display for WzErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WzErrorType::InvalidWz => write!(f, "Invalid WZ file"),
            WzErrorType::InvalidDir => write!(f, "Invalid directory"),
            WzErrorType::InvalidPath => write!(f, "Invalid node reference or corrupted tree"),
            WzErrorType::InvalidType => write!(f, "Invalid WZ object type"),
            WzErrorType::InvalidImg => write!(f, "Invalid WZ image"),
            WzErrorType::InvalidProp => write!(f, "Invalid WZ property"),
            WzErrorType::Unknown => write!(f, "Unexpected error occurred"),
        }
    }
}

/// Possible errors that may occur. This wraps everything into a WzError object
#[derive(Debug)]
pub enum WzError {
    Wz(WzErrorType),
    Io(io::Error),
    Parse(String),
    Utf8(FromUtf8Error),
    Utf16(FromUtf16Error),
}

impl fmt::Display for WzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WzError::Wz(err) => write!(f, "Wz: {}", err),
            WzError::Io(err) => write!(f, "Io: {}", err),
            WzError::Parse(err) => write!(f, "Parse: {}", err),
            WzError::Utf8(err) => write!(f, "Utf8: {}", err),
            WzError::Utf16(err) => write!(f, "Utf16: {}", err),
        }
    }
}

macro_rules! FromError {
    ( $from:ty, $enum:ident ) => {
        impl From<$from> for WzError {
            fn from(other: $from) -> Self {
                WzError::$enum(other)
            }
        }
    };
}
FromError!(WzErrorType, Wz);
FromError!(io::Error, Io);
FromError!(String, Parse);
FromError!(FromUtf8Error, Utf8);
FromError!(FromUtf16Error, Utf16);
