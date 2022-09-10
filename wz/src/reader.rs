use crate::{WzError, WzResult};
use crypto::{generic_array::ArrayLength, KeyStream, System};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, ErrorKind, Read, Seek, SeekFrom},
    num::Wrapping,
};

#[derive(Debug)]
pub struct WzReader {
    buf: BufReader<File>,
}

// WZ definitions
impl WzReader {
    fn read_stringbytes(&mut self, length: usize, buf: &mut Vec<u8>) -> WzResult<()> {
        self.read_nbytes(length, buf)?;
        buf.iter_mut().for_each(|b| *b = *b ^ 0xaa);
        Ok(())
    }

    pub fn read_utf8(&mut self, length: usize) -> WzResult<String> {
        let mut buf = Vec::with_capacity(length);
        self.read_stringbytes(length, &mut buf)?;
        Ok(String::from_utf8(buf)?)
    }

    pub fn read_utf16(&mut self, length: usize) -> WzResult<String> {
        let mut buf = Vec::with_capacity(length);
        self.read_stringbytes(length * 2, &mut buf)?;

        // Crappy workaround to convert [u8] to [u16] but the compiler cannot guarantee [u8; 2]
        // even though length * 2 will always result in _ % 2 == 0
        let buf_u16: Vec<u16> = buf
            .chunks(2)
            .map(|c| ((c[1] as u16) << 8) | (c[0] as u16))
            .collect();
        Ok(String::from_utf16(&buf_u16)?)
    }

    pub fn read_encrypted_utf8<B: ArrayLength<u8>, S: System<B>>(
        &mut self,
        size: usize,
        stream: &mut KeyStream<B, S>,
    ) -> WzResult<String> {
        let mut buf = Vec::with_capacity(size);
        self.read_nbytes(size, &mut buf)?;
        stream.decrypt(&mut buf);
        Ok(String::from_utf8(buf)?)
    }

    pub fn read_encrypted_utf16<B: ArrayLength<u8>, S: System<B>>(
        &mut self,
        size: usize,
        stream: &mut KeyStream<B, S>,
    ) -> WzResult<String> {
        let mut buf = Vec::with_capacity(size);
        self.read_nbytes(size, &mut buf)?;
        stream.decrypt(&mut buf);
        let buf_u16: Vec<u16> = buf
            .chunks(2)
            .map(|c| ((c[1] as u16) << 8) | (c[0] as u16))
            .collect();
        Ok(String::from_utf16(&buf_u16)?)
    }

    pub fn read_wz_int(&mut self) -> WzResult<i32> {
        let check = self.read_byte()? as i8;
        Ok(match check {
            i8::MIN => i32::from_le_bytes(self.read_word()?),
            _ => check as i32,
        })
    }

    pub fn read_wz_long(&mut self) -> WzResult<i64> {
        let check = self.read_byte()? as i8;
        Ok(match check {
            i8::MIN => i64::from_le_bytes(self.read_long()?),
            _ => check as i64,
        })
    }

    pub fn read_wz_offset(&mut self, abs_pos: u64, version_checksum: u32) -> WzResult<u32> {
        // Wrap all of this so there are no overflow errors in debug. The wrapping should be
        // optimized out in release
        let cur = Wrapping(self.stream_position()? as u32);
        let abs_pos = Wrapping(abs_pos as u32);
        let version_checksum = Wrapping(version_checksum);

        // Make decoding key (?)
        let encoding = cur - abs_pos;
        let encoding = encoding ^ Wrapping(u32::MAX);
        let encoding = encoding * version_checksum;
        let encoding = encoding - Wrapping(0x581C3F6D);
        let encoding = encoding.0.rotate_left(encoding.0 & 0x1F); // unwrapped

        // Decode offset
        let encoded_offset = u32::from_le_bytes(self.read_word()?);
        let offset = Wrapping(encoded_offset ^ encoding);
        let offset = offset + (abs_pos * Wrapping(2));
        Ok(offset.0)
    }

    pub fn read_wz_string(&mut self) -> WzResult<String> {
        let check = self.read_byte()? as i8;
        let length = match check {
            i8::MAX => i32::from_le_bytes(self.read_word()?),
            i8::MIN => i32::from_le_bytes(self.read_word()?),
            _ => (check as i32).wrapping_abs(),
        };

        // Sanity
        if length <= 0 {
            return Err(WzError::Parse(String::from("Invalid string length")));
        }

        if check < 0 {
            Ok(self.read_utf16(length as usize)?)
        } else {
            Ok(self.read_utf8(length as usize)?)
        }
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
        if n != length {
            // We need this to error here to ensure future assumptions
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
