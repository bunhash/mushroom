use crate::WzResult;
use crypto::{generic_array::ArrayLength, KeyStream, System};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, ErrorKind, Read, Seek, SeekFrom},
};

#[derive(Debug)]
pub struct WzReader {
    buf: BufReader<File>,
}

// WZ definitions
impl WzReader {
    pub fn read_wz_string(&mut self) -> WzResult<String> {
        Ok(String::from("ok"))
    }

    pub fn read_encrypted_wz_string(&mut self) -> WzResult<String> {
        Ok(String::from("ok"))
    }
}

// Lower-level definitions
impl WzReader {
    pub fn open(filename: &str) -> WzResult<Self> {
        Ok(WzReader {
            buf: BufReader::new(File::open(filename)?),
        })
    }

    pub fn with_capacity(capacity: usize, filename: &str) -> WzResult<Self> {
        Ok(WzReader {
            buf: BufReader::with_capacity(capacity, File::open(filename)?),
        })
    }

    pub fn read_byte(&mut self) -> WzResult<u8> {
        match self.buf.by_ref().bytes().next() {
            Some(b) => Ok(b?),
            None => Err(io::Error::from(ErrorKind::UnexpectedEof).into()),
        }
    }

    pub fn read_short(&mut self) -> WzResult<[u8; 2]> {
        let mut buf = [0; 2];
        self.buf.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_word(&mut self) -> WzResult<[u8; 4]> {
        let mut buf = [0; 4];
        self.buf.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_long(&mut self) -> WzResult<[u8; 8]> {
        let mut buf = [0; 8];
        self.buf.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_nbytes(&mut self, length: usize, buf: &mut Vec<u8>) -> WzResult<()> {
        let n = io::Read::by_ref(&mut self.buf)
            .take(length as u64)
            .read_to_end(buf)?;
        if n < length {
            Err(io::Error::from(ErrorKind::UnexpectedEof).into())
        } else {
            Ok(())
        }
    }

    pub fn read_until(&mut self, delim: u8, buf: &mut Vec<u8>) -> WzResult<usize> {
        Ok(self.buf.read_until(delim, buf)?)
    }

    pub fn read_zstring(&mut self) -> WzResult<String> {
        let mut buf = Vec::new();
        self.read_until(0, &mut buf)?;
        if let Some(b) = buf.pop() {
            if b == 0 {
                return Ok(String::from_utf8(buf)?);
            }
        }
        Err(io::Error::from(ErrorKind::UnexpectedEof).into())
    }

    pub fn read_nstring(&mut self, size: usize) -> WzResult<String> {
        let mut buf = Vec::with_capacity(size);
        self.read_nbytes(size, &mut buf)?;
        Ok(String::from_utf8(buf)?)
    }

    pub fn read_encrypted_nstring<B: ArrayLength<u8>, S: System<B>>(
        &mut self,
        size: usize,
        stream: &mut KeyStream<B, S>,
    ) -> WzResult<String> {
        let mut buf = Vec::with_capacity(size);
        self.read_nbytes(size, &mut buf)?;
        stream.decrypt(&mut buf);
        Ok(String::from_utf8(buf)?)
    }
}

// Raising BufReader's Seek trait
impl Seek for WzReader {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.buf.seek(pos)
    }

    fn rewind(&mut self) -> io::Result<()> {
        self.buf.rewind()
    }

    fn stream_position(&mut self) -> io::Result<u64> {
        self.buf.stream_position()
    }
}
