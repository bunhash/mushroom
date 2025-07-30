//! WZ Readers

use crate::decode::{Decode, Error};
use crypto::{Decryptor, DummyKeyStream};
use std::{
    collections::HashMap,
    io::{self, Read, Seek, SeekFrom},
};

/// WZ structure reader
#[derive(Debug)]
pub struct Reader<R, D>
where
    R: Read + Seek,
    D: Decryptor,
{
    /// Start of the content (before root package checksum)
    pub content_start: u32,

    /// Archive checksum
    pub version_checksum: u32,

    /// Underlying `Read + Seek`
    read: R,

    /// Decryptor
    decryptor: D,

    /// String map
    pub(crate) string_map: HashMap<u32, String>,
}

impl<R> Reader<R, DummyKeyStream>
where
    R: Read + Seek,
{
    /// Builds an unencrypted `Reader`
    pub fn unencrypted(content_start: u32, version_checksum: u32, read: R) -> Self {
        Reader::new(content_start, version_checksum, read, DummyKeyStream)
    }
}

impl<R, D> Reader<R, D>
where
    R: Read + Seek,
    D: Decryptor,
{
    /// Builds an new `Reader`
    pub fn new(content_start: u32, version_checksum: u32, read: R, decryptor: D) -> Self {
        Self {
            content_start,
            version_checksum,
            read,
            decryptor,
            string_map: HashMap::new(),
        }
    }

    /// Dereferences a UolString
    pub fn deref_string(&self, position: u32) -> Option<&str> {
        Some(self.string_map.get(&position)?.as_str())
    }

    /// Decrypts a slice of bytes
    pub fn decrypt(&mut self, bytes: &mut [u8]) {
        self.decryptor.decrypt(bytes)
    }

    /// Wrap `Seek::stream_position` to ignore the header
    pub fn position(&mut self) -> Result<u32, Error> {
        let position = self.read.stream_position()?;
        position
            .try_into()
            .map_err(|_| Error::position(self.content_start, position))
    }

    /// Wrap `Seek::seek` so it works with `u32`
    pub fn seek(&mut self, position: u32) -> Result<(), Error> {
        let new_position = self.read.seek(SeekFrom::Start(position as u64))?;
        if position as u64 == new_position {
            Ok(())
        } else {
            Err(io::Error::from(io::ErrorKind::UnexpectedEof).into())
        }
    }

    /// Rewind to the start of the content
    pub fn rewind(&mut self) -> Result<(), Error> {
        self.seek(self.content_start + 2)
    }

    /// Raises underlying `Read::read_exact`
    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        Ok(self.read.read_exact(buf)?)
    }

    /// Read a byte
    pub fn read_byte(&mut self) -> Result<u8, Error> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    /// Reads n bytes into a vec
    pub fn read_to_vec(&mut self, length: usize) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0u8; length];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// Decode at position while saving the current stream position
    pub fn decode_at<T>(&mut self, position: u32) -> Result<T, <T as Decode>::Error>
    where
        T: Decode,
        <T as Decode>::Error: From<Error>,
    {
        let current_position = self.position()?;
        self.seek(position)?;
        let ret = <T as Decode>::decode(self)?;
        self.seek(current_position)?;
        Ok(ret)
    }

    /// Get a reference to the underlying `Read`
    pub fn get(&self) -> &R {
        &self.read
    }

    /// Get a mutable reference to the underlying `Read`
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.read
    }

    /// Consumes the `Reader` and returns the string map
    pub fn into_string_map(self) -> HashMap<u32, String> {
        self.string_map
    }
}
