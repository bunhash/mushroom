//! WZ File Reader

use crate::{error::Result, Metadata, Reader};
use crypto::{generic_array::ArrayLength, KeyStream, System};
use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
};

/// Reads WZ files with unencrypted strings
pub struct EncryptedWzReader<'a, B, S>
where
    B: ArrayLength<u8>,
    S: System<B>,
{
    buf: BufReader<File>,
    metadata: Metadata,
    stream: KeyStream<'a, B, S>,
}

impl<'a, B, S> EncryptedWzReader<'a, B, S>
where
    B: ArrayLength<u8>,
    S: System<B>,
{
    /// Creates a [`WzReader`] that does not need string decryption
    pub fn new(file: File, system: &'a S) -> Result<Self> {
        let mut buf = BufReader::new(file);
        let metadata = Metadata::from_reader(&mut buf)?;
        Ok(Self {
            buf,
            metadata,
            stream: KeyStream::new(system),
        })
    }
}

impl<'a, B, S> Reader for EncryptedWzReader<'a, B, S>
where
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
