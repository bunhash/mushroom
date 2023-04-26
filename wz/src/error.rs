//! Errors

use crate::{map, package};
use std::{fmt, io, string};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::ErrorKind),
    Map(map::Error),
    Package(package::Error),
    Utf8(string::FromUtf8Error),
    Unicode(string::FromUtf16Error),
    Wz(WzError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(kind) => write!(f, "IO: {}", kind),
            Error::Map(e) => write!(f, "Map: {}", e),
            Error::Package(e) => write!(f, "Package: {}", e),
            Error::Utf8(e) => write!(f, "UTF8: {}", e),
            Error::Unicode(e) => write!(f, "Unicode: {}", e),
            Error::Wz(e) => write!(f, "WZ: {}", e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other.kind())
    }
}

impl From<io::ErrorKind> for Error {
    fn from(other: io::ErrorKind) -> Self {
        Error::Io(other)
    }
}

impl From<map::Error> for Error {
    fn from(other: map::Error) -> Self {
        Error::Map(other)
    }
}

impl From<package::Error> for Error {
    fn from(other: package::Error) -> Self {
        Error::Package(other)
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

impl From<WzError> for Error {
    fn from(other: WzError) -> Self {
        Error::Wz(other)
    }
}

#[derive(Debug)]
pub enum WzError {
    BruteForceChecksum,
    CreateError,
    DirectoryName(String),
    InvalidPackage,
    InvalidContentType(u8),
    InvalidLength(i32),
}

impl fmt::Display for WzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WzError::BruteForceChecksum => write!(f, "Brute force of the checksum failed"),
            WzError::CreateError => write!(f, "Failed to create image or directory"),
            WzError::DirectoryName(name) => {
                write!(f, "Invalid directory name: `{}`", name)
            }
            WzError::InvalidPackage => write!(f, "Invalid Package"),
            WzError::InvalidContentType(t) => {
                write!(f, "Package ContentType is invalid `{}`", t)
            }
            WzError::InvalidLength(l) => write!(f, "Invalid length: `{}`", l),
        }
    }
}
