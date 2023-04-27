//! Decoder Trait

use crate::{error::Result, Reader};
use std::io::{Read, Seek};

/// Trait for decoding objects
pub trait Decode {
    /// Decodes objects
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
        Self: Sized;
}

/// Decoder trait
pub trait Decoder<R>
where
    R: Read + Seek,
{
}
