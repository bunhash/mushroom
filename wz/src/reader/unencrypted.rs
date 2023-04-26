//! WZ File Reader

use crate::{error::Result, types::WzOffset, Metadata, Reader};
use std::io::{BufReader, Read, Seek, SeekFrom};

/// Reads WZ files with unencrypted strings
pub struct UnencryptedReader<R>
where
    R: Read + Seek,
{
    buf: BufReader<R>,
    metadata: Metadata,
}

impl<R> UnencryptedReader<R>
where
    R: Read + Seek,
{
    /// Creates a [`UnencryptedReader`] that does not need string decryption
    pub fn new(reader: R, metadata: Metadata) -> Self {
        Self {
            buf: BufReader::new(reader),
            metadata,
        }
    }

    /// Creates a [`UnencryptedReader`] from a file
    pub fn from_reader(reader: R) -> Result<Self> {
        let mut reader = reader;
        let metadata = Metadata::from_reader(&mut reader)?;
        Ok(Self::new(reader, metadata))
    }
}

impl<R> Reader for UnencryptedReader<R>
where
    R: Read + Seek,
{
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn position(&mut self) -> Result<WzOffset> {
        Ok(WzOffset::from(self.buf.stream_position()?))
    }

    fn seek(&mut self, pos: WzOffset) -> Result<WzOffset> {
        Ok(WzOffset::from(self.buf.seek(SeekFrom::Start(*pos as u64))?))
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Ok(self.buf.read(buf)?)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        Ok(self.buf.read_exact(buf)?)
    }
}
