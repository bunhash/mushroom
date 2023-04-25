//! WZ File Reader

use crate::{error::Result, Metadata, Reader};
use crypto::{generic_array::ArrayLength, KeyStream, System};
use std::io::{BufReader, Read, Seek, SeekFrom};

/// Reads WZ files with unencrypted strings
pub struct EncryptedReader<'a, R, B, S>
where
    R: Read + Seek,
    B: ArrayLength<u8>,
    S: System<B>,
{
    buf: BufReader<R>,
    metadata: Metadata,
    stream: KeyStream<'a, B, S>,
}

impl<'a, R, B, S> EncryptedReader<'a, R, B, S>
where
    R: Read + Seek,
    B: ArrayLength<u8>,
    S: System<B>,
{
    /// Creates an [`EncryptedReader`] that handles string decryption
    pub fn new(reader: R, metadata: Metadata, system: &'a S) -> Self {
        Self {
            buf: BufReader::new(reader),
            metadata,
            stream: KeyStream::new(system),
        }
    }

    /// Creates an [`EncryptedReader`] from a file
    pub fn from_reader(name: &str, reader: R, system: &'a S) -> Result<Self> {
        let mut reader = reader;
        let metadata = Metadata::from_reader(name, &mut reader)?;
        Ok(Self::new(reader, metadata, system))
    }
}

impl<'a, R, B, S> Reader for EncryptedReader<'a, R, B, S>
where
    R: Read + Seek,
    B: ArrayLength<u8>,
    S: System<B>,
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

    fn decrypt(&mut self, bytes: &mut Vec<u8>) {
        self.stream.decrypt(bytes);
    }
}
