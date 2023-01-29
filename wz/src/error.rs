use std::{fmt, io};

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
    InvalidImgObj,
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
            WzErrorType::InvalidImgObj => write!(f, "Invalid WZ image object"),
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
}

impl fmt::Display for WzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WzError::Wz(err) => write!(f, "Wz: {}", err),
            WzError::Io(err) => write!(f, "Io: {}", err),
            WzError::Parse(err) => write!(f, "Parse: {}", err),
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
