use crate::WzResult;
use std::{
    fs,
    io::{self, prelude::*, Error, ErrorKind, SeekFrom},
};

#[derive(Debug)]
pub struct WzReader {
    file: fs::File,
}

impl WzReader {
    pub fn new(filename: &str) -> WzResult<Self> {
        Ok(WzReader {
            file: fs::File::open(filename)?,
        })
    }

    pub fn pos(&mut self) -> WzResult<u64> {
        Ok(self.file.stream_position()?)
    }

    pub fn seek_to(&mut self, pos: u64) -> WzResult<u64> {
        Ok(self.file.seek(SeekFrom::Start(pos))?)
    }

    pub fn read_short(&mut self) -> WzResult<[u8; 2]> {
        let mut buf = [0; 2];
        self.file.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_word(&mut self) -> WzResult<[u8; 4]> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_long(&mut self) -> WzResult<[u8; 8]> {
        let mut buf = [0; 8];
        self.file.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_string(&mut self) -> WzResult<Vec<u8>> {
        let pos = self.pos()?;
        let mut ret = Vec::new();
        loop {
            let mut buf = [0; 256];
            let bytes_read = self.file.read(&mut buf)?;
            for i in 0..bytes_read {
                if buf[i] == 0 {
                    self.seek_to(pos + (ret.len() as u64) + 1)?;
                    return Ok(ret);
                }
                ret.push(buf[i]);
            }
        }
    }

    pub fn read_nstring(&mut self, size: usize) -> WzResult<String> {
        let mut buf = Vec::with_capacity(size);
        let n = io::Read::by_ref(&mut self.file)
            .take(size as u64)
            .read_to_end(&mut buf)?;
        if n < size {
            Err(Error::from(ErrorKind::UnexpectedEof).into())
        } else {
            Ok(String::from_utf8(buf)?)
        }
    }

    pub fn read_header(&mut self) -> WzResult<(String, u64, i32, String)> {
        Ok((
            String::from_utf8(Vec::from(self.read_word()?)).unwrap(),
            u64::from_le_bytes(self.read_long()?),
            i32::from_le_bytes(self.read_word()?),
            String::from_utf8(self.read_string()?).unwrap(),
        ))
    }
}
