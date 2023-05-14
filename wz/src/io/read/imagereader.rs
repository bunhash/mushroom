//! WZ Image Reader

use crate::{
    error::Result,
    io::{WzRead, WzReader},
    types::{WzInt, WzOffset},
};
use crypto::Decryptor;
use std::io::{Read, Seek, Write};

/// WZ Image Reader
///
/// This just wraps a WzReader so the seeking offsets align properly. This is not needed unless the
/// image resides within a WZ archive.
#[derive(Debug)]
pub struct WzImageReader<'a, R, D>
where
    R: Read + Seek,
    D: Decryptor,
{
    inner: &'a mut WzReader<R, D>,
    offset: WzOffset,
}

impl<'a, R, D> WzImageReader<'a, R, D>
where
    R: Read + Seek,
    D: Decryptor,
{
    pub fn new(inner: &'a mut WzReader<R, D>, offset: WzOffset) -> Self {
        Self { inner, offset }
    }
}

impl<'a, R, D> WzRead for WzImageReader<'a, R, D>
where
    R: Read + Seek,
    D: Decryptor,
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
}
