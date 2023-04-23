//! WZ File Reader

use crate::{error::Result, Metadata, Reader};
use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
};

/// Reads WZ files with unencrypted strings
pub struct WzReader {
    buf: BufReader<File>,
    metadata: Metadata,
}

impl WzReader {
    /// Creates a [`WzReader`] that does not need string decryption
    pub fn new(file: File) -> Result<Self> {
        let mut buf = BufReader::new(file);
        let metadata = Metadata::from_reader(&mut buf)?;
        Ok(Self { buf, metadata })
    }
}

impl Reader for WzReader {
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
