//! Encoder Trait

use crate::{error::Result, Writer};

/// Trait for encoding objects
pub trait Encode {
    /// Encodes objects
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
        Self: Sized;
}
