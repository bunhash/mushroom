//! Decoder Trait

use crate::{error::Result, Reader};

/// Trait for decoding objects that do not require metadata
pub trait Decode {
    /// Decodes objects without file metadata
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
        Self: Sized;
}
