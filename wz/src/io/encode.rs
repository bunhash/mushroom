//! Encoder Trait

use crate::{error::Result, io::WzWriter};
use crypto::Encryptor;
use std::io::{Seek, Write};

/// Trait for encoding objects
pub trait Encode {
    /// Encodes objects
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<()>
    where
        W: Write + Seek,
        E: Encryptor,
        Self: Sized;
}

/// Internal trait for quicker size estimation
pub(crate) trait SizeHint {
    fn size_hint(&self) -> u32;
}
