//! WZ File Reader

use crate::{
    error::{Error, Result},
    Metadata, Reader,
};
use std::{
    fs::File,
    io::{BufRead, BufReader, ErrorKind, Read, Seek, SeekFrom},
};

/// Reads WZ files with unencrypted strings
pub struct WzReader {
    buf: BufReader<File>,
    metadata: Metadata,
    to_consume: usize,
}

impl WzReader {
    /// Creates a [`WzReader`] that does not need string decryption
    pub fn new(file: File) -> Result<Self> {
        let mut buf = BufReader::new(file);
        let metadata = Metadata::from_reader(&mut buf)?;
        Ok(Self {
            buf,
            metadata,
            to_consume: 0,
        })
    }

    fn consume(&mut self) {
        self.buf.consume(self.to_consume);
        self.to_consume = 0;
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
}
