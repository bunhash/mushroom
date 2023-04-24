//! WZ File Reader

use crate::{error::Result, Metadata, Reader};
use std::io::{BufReader, Read, Seek, SeekFrom};

/// Reads WZ files with unencrypted strings
pub struct WzReader<R>
where
    R: Read + Seek,
{
    buf: BufReader<R>,
    metadata: Metadata,
}

impl<R> WzReader<R>
where
    R: Read + Seek,
{
    /// Creates a [`WzReader`] that does not need string decryption
    pub fn new(reader: R, metadata: Metadata) -> Self {
        Self {
            buf: BufReader::new(reader),
            metadata,
        }
    }

    /// Creates a [`WzReader`] from a file
    pub fn from_reader(reader: R) -> Result<Self> {
        let mut reader = reader;
        let metadata = Metadata::from_reader(&mut reader)?;
        Ok(Self::new(reader, metadata))
    }
}

impl<R> Reader for WzReader<R>
where
    R: Read + Seek,
{
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn position(&mut self) -> Result<u64> {
        Ok(self.buf.stream_position()?)
    }

    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        Ok(self.buf.seek(pos)?)
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Ok(self.buf.read(buf)?)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        Ok(self.buf.read_exact(buf)?)
    }
}
