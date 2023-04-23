//! WZ File Reader

use crate::{
    error::{Error, Result},
    Metadata, Reader,
};
use crypto::{generic_array::ArrayLength, KeyStream, System};
use std::{
    fs::File,
    io::{BufRead, BufReader, ErrorKind, Read, Seek, SeekFrom},
};

/// Reads WZ files with unencrypted strings
pub struct EncryptedWzReader<'a, B, S>
where
    B: ArrayLength<u8>,
    S: System<B>,
{
    buf: BufReader<File>,
    metadata: Metadata,
    to_consume: usize,
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
            to_consume: 0,
            stream: KeyStream::new(system),
        })
    }

    fn consume(&mut self) {
        self.buf.consume(self.to_consume);
        self.to_consume = 0;
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
        self.consume();
        Ok(self.buf.read(buf)?)
    }

    fn read_slice(&mut self, len: usize) -> Result<&[u8]> {
        self.consume();
        if len > self.buf.capacity() {
            return Err(Error::SliceTooBig(len));
        }
        let buf = self.buf.buffer();
        if buf.len() < len {
            let position = self.position()?;
            self.seek(SeekFrom::Start(position))?;
            let buf = self.buf.fill_buf()?;
            if buf.len() < len {
                return Err(Error::Io(ErrorKind::UnexpectedEof));
            }
        }
        let slice = &self.buf.buffer()[0..len];
        self.to_consume = len;
        Ok(slice)
    }

    fn decrypt(&mut self, bytes: &mut Vec<u8>) {
        self.stream.decrypt(bytes);
    }
}
