//! Seekable Chunk Reader

use std::cmp::min;
use std::io::{self, Read, Seek, SeekFrom};

/// A wrapper to a pimitive [`Read`] that limits the bytes it can read.
pub struct ChunkReader<R>
where
    R: Read + Seek,
{
    inner: R,
    lower: u64,
    upper: u64,
    pos: u64,
}

impl<R> ChunkReader<R>
where
    R: Read + Seek,
{
    /// Creates a [`ChunkReader`] that can only read `size` bytes starting as `offset`.
    pub fn new(read: R, offset: u64, size: u64) -> Self {
        Self {
            inner: read,
            lower: offset,
            upper: offset + size,
            pos: offset,
        }
    }
}

impl<R> Read for ChunkReader<R>
where
    R: Read + Seek,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.pos < self.lower || self.pos >= self.upper {
            return Ok(0);
        }
        let num_bytes = min((self.upper - self.pos) as usize, buf.len());
        let bytes_read = self.inner.read(&mut buf[0..num_bytes])?;
        self.pos = self.pos + bytes_read as u64;
        Ok(bytes_read)
    }
}

impl<R> Seek for ChunkReader<R>
where
    R: Read + Seek,
{
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.inner.seek(pos)?;
        Ok(self.pos)
    }

    fn rewind(&mut self) -> io::Result<()> {
        self.pos = self.inner.seek(SeekFrom::Start(self.lower))?;
        Ok(())
    }

    fn stream_position(&mut self) -> io::Result<u64> {
        Ok(self.pos)
    }
}
