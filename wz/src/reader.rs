//! WZ Readers

use crate::{Decode, Error, Offset};
use std::io::{Read, Seek, SeekFrom};

/// WZ structure reader
pub struct Reader<R>
where
    R: Read + Seek,
{
    content_start: i32,
    version_checksum: u32,
    inner: R,
}

impl<R> Reader<R>
where
    R: Read + Seek,
{
    /// Wraps an underlying `Read` + `Seek`
    pub fn new(content_start: i32, version_checksum: u32, inner: R) -> Self {
        Self {
            content_start,
            version_checksum,
            inner,
        }
    }

    /// Returns the content start of the reader
    pub fn content_start(&self) -> i32 {
        self.content_start
    }

    /// Returns the version checksum of the WZ archive
    pub fn version_checksum(&self) -> u32 {
        self.version_checksum
    }

    /// Wrap `Seek::seek` so it functions with `Offset`
    pub fn seek(&mut self, pos: Offset) -> Result<Offset, Error> {
        Ok(Offset::from(self.inner.seek(SeekFrom::Start(*pos as u64))?))
    }

    /// Seek to start after the version "hash"
    pub fn seek_to_start(&mut self) -> Result<Offset, Error> {
        self.seek(Offset::from(self.content_start() as u32 + 2))
    }

    /// Seek from the content start position
    pub fn seek_from_start(&mut self, offset: u32) -> Result<Offset, Error> {
        self.seek(Offset::from(self.content_start() as u32 + offset))
    }

    /// Raises underlying `Read::read_exact`
    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        Ok(self.inner.read_exact(buf)?)
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

    /// Decode object
    pub fn decode<D>(&mut self) -> Result<D, Error>
    where
        D: Decode,
    {
        D::decode(self)
    }

    /// Get a reference to the underlying `Read`
    pub fn get(&self) -> &R {
        &self.inner
    }

    /// Get a mutable reference to the underlying `Read`
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    /// Consumes itself and returns the underlying `Read`
    pub fn into_inner(self) -> R {
        self.inner
    }
}
