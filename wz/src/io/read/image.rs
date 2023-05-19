//! WZ Image Reader

use crate::{
    error::Result,
    io::WzRead,
    types::{WzInt, WzOffset},
};
use std::{collections::HashMap, io::Write};

/// WZ Image Reader
///
/// This just wraps a WzRead so the seeking offsets align properly. This is not needed unless the
/// image resides within a WZ archive.
#[derive(Debug)]
pub struct WzImageReader<'a, R>
where
    R: WzRead,
{
    inner: &'a mut R,
    offset: WzOffset,
    cache: HashMap<u32, String>,
}

impl<'a, R> WzImageReader<'a, R>
where
    R: WzRead,
{
    /// Creates a new [`WzImageReader`]
    pub fn new(inner: &'a mut R) -> Self {
        Self {
            inner,
            offset: WzOffset::from(0),
            cache: HashMap::new(),
        }
    }

    /// Creates a new [`WzImageReader`]
    pub fn with_offset(inner: &'a mut R, offset: WzOffset) -> Self {
        Self {
            inner,
            offset,
            cache: HashMap::new(),
        }
    }
}

impl<'a, R> WzRead for WzImageReader<'a, R>
where
    R: WzRead,
{
    fn absolute_position(&self) -> i32 {
        self.inner.absolute_position()
    }

    fn version_checksum(&self) -> u32 {
        self.inner.version_checksum()
    }

    fn set_version_checksum(&mut self, version_checksum: u32) {
        self.inner.set_version_checksum(version_checksum)
    }

    fn position(&mut self) -> Result<WzOffset> {
        Ok(self.inner.position()? - self.offset)
    }

    fn seek(&mut self, pos: WzOffset) -> Result<WzOffset> {
        self.inner.seek(self.offset + pos)
    }

    fn seek_to_start(&mut self) -> Result<WzOffset> {
        self.inner.seek(self.offset)
    }

    fn seek_from_start(&mut self, offset: u32) -> Result<WzOffset> {
        self.inner.seek(self.offset + offset.into())
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.inner.read(buf)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        self.inner.read_exact(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        self.inner.read_to_end(buf)
    }

    fn copy_to<W>(&mut self, dest: &mut W, offset: WzOffset, size: WzInt) -> Result<()>
    where
        W: Write,
    {
        self.inner.copy_to(dest, self.offset + offset, size)
    }

    fn decrypt(&mut self, bytes: &mut Vec<u8>) {
        self.inner.decrypt(bytes)
    }

    fn cache(&mut self, offset: u32, string: &str) {
        self.cache.insert(offset, string.to_string());
    }

    fn from_cache(&self, offset: u32) -> Option<&str> {
        Some(self.cache.get(&offset)?.as_str())
    }
}
