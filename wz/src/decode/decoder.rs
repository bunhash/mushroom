//! WZ Decoder

use crate::{
    archive::Offset,
    decode::{Decode, Error},
};

/// WZ Decoder Trait
pub trait Decoder: Sized {
    /// Fill the buffer with bytes
    fn decode_bytes(&mut self, bytes: &mut [u8]) -> Result<(), Error>;

    /// Decrypt the slice of bytes
    fn decrypt_bytes(&mut self, bytes: &mut [u8]);

    /// Position in stream
    fn position(&mut self) -> Result<u32, Error>;

    /// Seek to position
    fn seek(&mut self, position: u32) -> Result<(), Error>;

    /// Decode offset
    fn decode_offset(&mut self) -> Result<Offset, Error> {
        Ok(Offset::from(0))
    }

    /// Decode at an offset and return to the previous position
    fn decode_at<T>(&mut self, position: u32) -> Result<T, <T as Decode>::Error>
    where
        T: Decode,
        <T as Decode>::Error: From<Error>,
    {
        let current = self.position()?;
        self.seek(position)?;
        let ret = <T as Decode>::decode(self)?;
        self.seek(current)?;
        Ok(ret)
    }
}
