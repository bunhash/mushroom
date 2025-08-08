//! WZ Encoder

use crate::{
    archive::Offset,
    encode::{Encode, Error},
};
use crypto::{DummyKeyStream, Encryptor};
use std::io::{Cursor, Seek, Write};

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

/// Encodes to a `Vec`
pub struct VecEncoder<E>
where
    E: Encryptor,
{
    content_start: u32,
    version_checksum: u32,
    cursor: Cursor<Vec<u8>>,
    encryptor: E,
}

impl VecEncoder<DummyKeyStream> {
    /// Creates a new unencrypted `VecEncoder`
    pub fn unencrypted(content_start: u32, version_checksum: u32) -> Self {
        Self::new(content_start, version_checksum, DummyKeyStream)
    }
}

impl<E> VecEncoder<E>
where
    E: Encryptor,
{
    /// Creates a new `VecEncoder`
    pub fn new(content_start: u32, version_checksum: u32, encryptor: E) -> Self {
        Self {
            content_start,
            version_checksum,
            cursor: Cursor::new(Vec::new()),
            encryptor,
        }
    }

    /// Returns a reference to the written bytes
    pub fn as_slice(&self) -> &[u8] {
        &self.cursor.get_ref()[..]
    }

    /// Clones the written bytes
    pub fn to_vec(&self) -> Vec<u8> {
        Vec::from(self.as_slice())
    }

    /// Consumes itself and returns the written bytes
    pub fn into_vec(self) -> Vec<u8> {
        self.cursor.into_inner()
    }
}

impl<E> Encoder for VecEncoder<E>
where
    E: Encryptor,
{
    fn encode_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.cursor.write(bytes)?;
        Ok(())
    }

    fn encrypt_bytes(&mut self, bytes: &mut [u8]) {
        self.encryptor.encrypt(bytes);
    }

    fn position(&mut self) -> Result<u32, Error> {
        let position = self.cursor.stream_position()?;
        Ok(position
            .try_into()
            .map_err(|_| Error::position(&format!("position is greater than u32::MAX")))?)
    }

    fn encode_offset(&mut self, offset: Offset) -> Result<(), Error> {
        let position = self.position()? as u32;
        offset
            .encode_with(position, self.content_start, self.version_checksum)
            .encode(self)?;
        Ok(())
    }
}
