//! WZ Encoder

use crate::{
    archive::Offset,
    encode::{Encode, Error},
};

/// WZ Encoder Trait
pub trait Encoder {
    /// Fill the buffer with bytes
    fn encode_bytes(&mut self, bytes: &[u8]) -> Result<(), Error>;

    /// Encrypt the slice of bytes
    fn encrypt_bytes(&mut self, bytes: &mut [u8]);

    /// Position in stream
    fn position(&mut self) -> Result<u32, Error>;

    /// Encode offset
    fn encode_offset(&mut self, _offset: Offset) -> Result<(), Error> {
        Ok(())
    }
}
