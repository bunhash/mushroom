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

    /// Seek to position
    fn seek(&mut self, position: u32) -> Result<(), Error>;

    /// Encode offset
    fn encode_offset(&mut self, _offset: Offset) -> Result<(), Error> {
        Ok(())
    }

    /// Encode at an offset and return to the previous position
    fn encode_at<T>(&mut self, position: u32, obj: &T) -> Result<(), <T as Encode>::Error>
    where
        T: Encode,
        <T as Encode>::Error: From<Error>,
        Self: Sized,
    {
        let current = self.position()?;
        self.seek(position)?;
        obj.encode(self)?;
        self.seek(current)?;
        Ok(())
    }
}
