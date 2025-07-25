//! Decoder Trait

use crate::{Error, Reader};
use std::io::{Read, Seek};

/// Trait for decoding objects
pub trait Decode {
    /// Decodes objects
    fn decode<R>(reader: &mut Reader<R>) -> Result<Self, Error>
    where
        R: Read + Seek,
        Self: Sized;
}
