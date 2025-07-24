//! WZ Readers

use crate::{Error, Offset};
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
    /// Wraps an underlying Read
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
