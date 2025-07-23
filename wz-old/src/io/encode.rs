//! Encoder Trait

use crate::{error::Result, io::WzWrite};

/// Trait for encoding objects
pub trait Encode {
    /// Encodes objects
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite + ?Sized,
        Self: Sized;
}

/// Internal trait for quicker size estimation
pub(crate) trait SizeHint {
    fn size_hint(&self) -> u32;
}
