//! WZ File Reader

use crate::{error::Result, types::WzOffset, Metadata, Reader};
use crypto::KeyStream;
use std::io::{Read, Seek, SeekFrom};

/// Reads WZ files with encrypted strings
pub struct EncryptedReader<R>
where
    R: Read + Seek,
{
    reader: R,
    metadata: Metadata,
    stream: KeyStream,
}

impl<R> EncryptedReader<R>
where
    R: Read + Seek,
{
    /// Creates an [`EncryptedReader`] that handles string decryption
    pub fn new(reader: R, metadata: Metadata, keystream: KeyStream) -> Self {
        Self {
            reader,
            metadata,
            stream: keystream,
        }
    }

    /// Creates an [`EncryptedReader`] from a file
    pub fn from_reader(reader: R, keystream: KeyStream) -> Result<Self> {
        let mut reader = reader;
        let metadata = Metadata::from_reader(&mut reader)?;
        Ok(Self::new(reader, metadata, keystream))
    }
}

impl<R> Reader for EncryptedReader<R>
where
    R: Read + Seek,
{
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn position(&mut self) -> Result<WzOffset> {
        Ok(WzOffset::from(self.reader.stream_position()?))
    }

    fn seek(&mut self, pos: WzOffset) -> Result<WzOffset> {
        Ok(WzOffset::from(
            self.reader.seek(SeekFrom::Start(*pos as u64))?,
        ))
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Ok(self.reader.read(buf)?)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        Ok(self.reader.read_exact(buf)?)
    }

    fn decrypt(&mut self, bytes: &mut Vec<u8>) {
        self.stream.decrypt(bytes);
    }
}
