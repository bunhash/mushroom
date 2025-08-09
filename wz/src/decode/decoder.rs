//! WZ Decoder

use crate::{
    archive::Offset,
    decode::{Decode, Error},
};
use crypto::{Decryptor, DummyKeyStream};
use std::io::{Cursor, Read, Seek, SeekFrom};

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

    /// Allow the decoder to perform a function on a decoded value
    fn visit<T>(&mut self, value: T) -> T {
        value
    }
}

/// Decoder for slices
pub struct SliceDecoder<'a, D>
where
    D: Decryptor,
{
    content_start: u32,
    version_checksum: u32,
    cursor: Cursor<&'a [u8]>,
    decryptor: D,
}

impl<'a> SliceDecoder<'a, DummyKeyStream> {
    /// Creates a new unencrypted `SliceDecoder`
    pub fn unencrypted(content_start: u32, version_checksum: u32, slice: &'a [u8]) -> Self {
        Self::new(content_start, version_checksum, slice, DummyKeyStream)
    }
}

impl<'a, D> SliceDecoder<'a, D>
where
    D: Decryptor,
{
    /// Creates a new `SliceDecoder`
    pub fn new(content_start: u32, version_checksum: u32, slice: &'a [u8], decryptor: D) -> Self {
        Self {
            content_start,
            version_checksum,
            cursor: Cursor::new(slice),
            decryptor,
        }
    }
}

impl<'a, D> Decoder for SliceDecoder<'a, D>
where
    D: Decryptor,
{
    fn decode_bytes(&mut self, bytes: &mut [u8]) -> Result<(), Error> {
        Ok(self.cursor.read_exact(bytes)?)
    }

    fn decrypt_bytes(&mut self, bytes: &mut [u8]) {
        self.decryptor.decrypt(bytes)
    }

    fn position(&mut self) -> Result<u32, Error> {
        let position = self.cursor.stream_position()?;
        Ok(position
            .try_into()
            .map_err(|_| Error::position(&format!("position is greater than u32::MAX")))?)
    }

    fn seek(&mut self, position: u32) -> Result<(), Error> {
        let new_position = self.cursor.seek(SeekFrom::Start(position as u64))?;
        if new_position != position as u64 {
            Err(Error::position(&format!(
                "tried to seek to {:08x} but currently at {:08x}",
                position, new_position
            )))
        } else {
            Ok(())
        }
    }

    fn decode_offset(&mut self) -> Result<Offset, Error> {
        let position = self.position()? as u32;
        Ok(Offset::decode_with(
            u32::decode(self)?,
            position,
            self.content_start,
            self.version_checksum,
        ))
    }
}
