//! Decoder Trait

use crate::{error::Result, Reader};

/// Trait for decoding objects
pub trait Decode {
    /// Decodes objects
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
        Self: Sized;
}
