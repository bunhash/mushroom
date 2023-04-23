//! Encoder Trait

use crate::{error::Result, Writer};

/// Trait for encoding objects
pub trait Encode {
    /// Encodes objects
    fn encode<W>(writer: &mut W) -> Result<Self>
    where
        W: Writer,
        Self: Sized;
}
