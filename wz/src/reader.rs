use std::{
    fs,
    io::{self, prelude::*, SeekFrom},
};

pub struct WzReader {
    file: fs::File,
    buf: Vec<u8>,
    abs_pos: u64,
    buf_pos: u64,
}

impl WzReader {
    pub fn new(filename: &str) -> io::Result<Self> {
        Ok(WzReader {
            file: fs::File::open(filename)?,
            buf: Vec::new(),
            abs_pos: 0,
            buf_pos: 0,
        })
    }

    fn buffer(&mut self, size: usize) -> io::Result<usize> {
        let mut buf = Vec::with_capacity(size);
        let n = self.file.take(size as u64).read_to_end(&mut buf)?;
        self.buf.extend(buf);
        Ok(n)
    }

    pub fn seek_to(&mut self, pos: u64) -> io::Result<u64> {
        self.file.seek(SeekFrom::Start(pos))
    }

    pub fn read_short(&mut self) -> io::Result<[u8; 2]> {
        let mut buf = [0; 2];
        self.file.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_word(&mut self) -> io::Result<[u8; 4]> {
        let mut buf = [0; 4];
        self.file.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_long(&mut self) -> io::Result<[u8; 8]> {
        let mut buf = [0; 8];
        self.file.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_to_null(&mut self, buf_size: usize) -> io::Result<Vec<u8>> {
        let pos = self.file.stream_position()?;
        self.buffer(4096);
    }

    pub fn read_header(&mut self) -> io::Result<(String, u64, i32, String)> {
        Ok((
            String::from_utf8(Vec::from(self.read_word()?)).unwrap(),
            u64::from_le_bytes(self.read_long()?),
            i32::from_le_bytes(self.read_word()?),
            String::from("testing"),
        ))
    }
}
