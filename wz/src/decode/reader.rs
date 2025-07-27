//! WZ Readers

use crate::decode::Error;
use crypto::{Decryptor, DummyKeyStream};
use std::io::{Read, Seek, SeekFrom};

/// WZ structure reader
pub struct Reader<R, D>
where
    R: Read + Seek,
    D: Decryptor,
{
    content_start: u64,
    version_checksum: u32,
    read: R,
    decryptor: D,
}

impl<R> Reader<R, DummyKeyStream>
where
    R: Read + Seek,
{
    /// Builds an unencrypted `Decoder`
    pub fn unencrypted(content_start: u64, version_checksum: u32, read: R) -> Self {
        Self {
            content_start,
            version_checksum,
            read,
            decryptor: DummyKeyStream,
        }
    }
}

impl<R, D> Reader<R, D>
where
    R: Read + Seek,
    D: Decryptor,
{
    /// Wraps an underlying `Read` + `Seek`
    pub fn encrypted(content_start: u64, version_checksum: u32, read: R, decryptor: D) -> Self {
        Self {
            content_start,
            version_checksum,
            read,
            decryptor,
        }
    }

    /// Returns the content start of the reader
    pub fn content_start(&self) -> u64 {
        self.content_start
    }

    /// Returns the version checksum of the WZ archive
    pub fn version_checksum(&self) -> u32 {
        self.version_checksum
    }

    /// Decrypts a slice of bytes
    pub fn decrypt(&mut self, bytes: &mut [u8]) {
        self.decryptor.decrypt(bytes)
    }

    /// Wrap `Seek::stream_position` to ignore the header
    pub fn position(&mut self) -> Result<u64, Error> {
        let pos = self.read.stream_position()?;
        pos.checked_sub(self.content_start())
            .ok_or(Error::position(self.content_start, pos))
    }

    /// Wrap `Seek::seek` so it ignores the header
    pub fn seek(&mut self, pos: u64) -> Result<(), Error> {
        let pos = self.content_start() + pos;
        let new_pos = self.read.seek(SeekFrom::Start(pos))?;
        if pos == new_pos {
            Ok(())
        } else {
            Ok(())
        }
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

    /// Get a reference to the underlying `Read`
    pub fn get(&self) -> &R {
        &self.read
    }

    /// Get a mutable reference to the underlying `Read`
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.read
    }
}
