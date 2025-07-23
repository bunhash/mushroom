//! Decoder Trait

use crate::{error::Result, io::WzRead};

/// Trait for decoding objects
pub trait Decode {
    /// Decodes objects
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead + ?Sized,
        Self: Sized;
}
