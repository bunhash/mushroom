//! Encoder Trait

use crate::{error::Result, Writer};

/// Trait for encoding objects
pub trait Encode {
    /// Get the size
    fn encode_size(&self) -> u64;

    /// Encodes objects
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
        Self: Sized;
}
