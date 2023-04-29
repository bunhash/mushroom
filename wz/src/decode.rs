//! Decoder Trait

use crate::{error::Result, WzReader};
use crypto::Decryptor;
use std::{
    fmt,
    io::{Read, Seek},
};

/// Trait for decoding objects
pub trait Decode {
    /// Decodes objects
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self>
    where
        R: Read + Seek,
        D: Decryptor,
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidContentType(u8),
    InvalidLength(i32),
    InvalidOffset(i32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidContentType(t) => {
                write!(f, "Package ContentType is invalid `{}`", t)
            }
            Error::InvalidLength(l) => write!(f, "Invalid length: `{}`", l),
            Error::InvalidOffset(o) => write!(f, "Invalid offset: `{}`", o),
        }
    }
}
