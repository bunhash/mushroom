use crate::{WzError, WzResult};
use crypto::{generic_array::ArrayLength, KeyStream, System};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, ErrorKind, Read, Seek, SeekFrom},
    num::Wrapping,
};

/// Trait that lists functions for reading WZ files
pub trait WzRead: Seek {
    /// Reads an ASCII-encoded string
    fn read_ascii(&mut self, length: usize) -> WzResult<Vec<u8>>;

    /// Reads a unicode-encoded string
    fn read_unicode(&mut self, length: usize) -> WzResult<Vec<u16>>;

    /// Reads a WZ-INT
    fn read_wz_int(&mut self) -> WzResult<i32>;

    /// Reads a WZ-LONG
    fn read_wz_long(&mut self) -> WzResult<i64>;

    /// Reads a WZ-OFFSET
    fn read_wz_offset(&mut self, abs_pos: u64, version_checksum: u32) -> WzResult<u32>;

    /// Reads a WZ-STRING
    fn read_wz_string(&mut self) -> WzResult<String>;

    /// Reads a single byte
    fn read_byte(&mut self) -> WzResult<u8>;

    /// Reads 2 bytes
    fn read_short(&mut self) -> WzResult<[u8; 2]>;

    /// Reads 4 bytes
    fn read_word(&mut self) -> WzResult<[u8; 4]>;

    /// Reads 8 bytes
    fn read_long(&mut self) -> WzResult<[u8; 8]>;

    /// Reads n bytes into the provided vector
    fn read_nbytes(&mut self, length: usize, buf: &mut Vec<u8>) -> WzResult<()>;

    /// Reads until `delim` or EOF
    fn read_until(&mut self, delim: u8, buf: &mut Vec<u8>) -> WzResult<usize>;

    /// Reads until `\0` or EOF
    fn read_zstring(&mut self) -> WzResult<String>;
}

#[derive(Debug)]
pub struct WzReader {
    buf: BufReader<File>,
}

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
}

impl WzRead for WzReader {
    fn read_ascii(&mut self, length: usize) -> WzResult<Vec<u8>> {
        let mut buf = Vec::with_capacity(length);
        self.read_nbytes(length, &mut buf)?;
        for i in 0..buf.len() {
            buf[i] = buf[i] ^ (0xaa + i as u8);
        }
        Ok(buf)
    }

    fn read_unicode(&mut self, length: usize) -> WzResult<Vec<u16>> {
        let mut buf = Vec::with_capacity(length * 2);
        self.read_nbytes(length * 2, &mut buf)?;
        let mut mask: u16 = 0xaaaa;
        Ok(buf
            .chunks(2)
            .map(|c| {
                let wchar = (c[1] as u16) << 8;
                let wchar = wchar | (c[0] as u16);
                let wchar = wchar ^ mask;
                mask = mask + 1;
                wchar
            })
            .collect())
    }

    fn read_wz_int(&mut self) -> WzResult<i32> {
        let check = self.read_byte()? as i8;
        Ok(match check {
            i8::MIN => i32::from_le_bytes(self.read_word()?),
            _ => check as i32,
        })
    }

    fn read_wz_long(&mut self) -> WzResult<i64> {
        let check = self.read_byte()? as i8;
        Ok(match check {
            i8::MIN => i64::from_le_bytes(self.read_long()?),
            _ => check as i64,
        })
    }

    fn read_wz_offset(&mut self, abs_pos: u64, version_checksum: u32) -> WzResult<u32> {
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

    fn read_wz_string(&mut self) -> WzResult<String> {
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

        // Return the proper string
        Ok(if check < 0 {
            String::from_utf8(self.read_ascii(length as usize)?)?
        } else {
            String::from_utf16(&self.read_unicode(length as usize)?)?
        })
    }

    fn read_byte(&mut self) -> WzResult<u8> {
        match self.buf.by_ref().bytes().next() {
            Some(b) => Ok(b?),
            None => Err(io::Error::from(ErrorKind::UnexpectedEof).into()),
        }
    }

    fn read_short(&mut self) -> WzResult<[u8; 2]> {
        let mut buf = [0; 2];
        self.buf.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_word(&mut self) -> WzResult<[u8; 4]> {
        let mut buf = [0; 4];
        self.buf.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_long(&mut self) -> WzResult<[u8; 8]> {
        let mut buf = [0; 8];
        self.buf.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_nbytes(&mut self, length: usize, buf: &mut Vec<u8>) -> WzResult<()> {
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

    fn read_until(&mut self, delim: u8, buf: &mut Vec<u8>) -> WzResult<usize> {
        Ok(self.buf.read_until(delim, buf)?)
    }

    fn read_zstring(&mut self) -> WzResult<String> {
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

#[derive(Debug)]
pub struct WzEncryptedReader<'a, B: ArrayLength<u8>, S: System<B>> {
    reader: WzReader,
    stream: KeyStream<'a, B, S>,
}

impl<'a, B: ArrayLength<u8>, S: System<B>> WzEncryptedReader<'a, B, S> {
    pub fn open(filename: &str, system: &'a S) -> WzResult<Self> {
        Ok(WzEncryptedReader {
            reader: WzReader::open(filename)?,
            stream: KeyStream::new(system),
        })
    }

    pub fn with_capacity(capacity: usize, filename: &str, system: &'a S) -> WzResult<Self> {
        Ok(WzEncryptedReader {
            reader: WzReader::with_capacity(capacity, filename)?,
            stream: KeyStream::new(system),
        })
    }
}

impl<'a, B: ArrayLength<u8>, S: System<B>> WzRead for WzEncryptedReader<'a, B, S> {
    fn read_ascii(&mut self, length: usize) -> WzResult<Vec<u8>> {
        let mut buf = Vec::with_capacity(length);
        self.read_nbytes(length, &mut buf)?;
        self.stream.decrypt(&mut buf);
        let mut mask = 0xaa;
        Ok(buf
            .iter()
            .map(|b| {
                let c = b ^ mask;
                mask = mask + 1;
                c
            })
            .collect())
    }

    fn read_unicode(&mut self, length: usize) -> WzResult<Vec<u16>> {
        let mut buf = Vec::with_capacity(length * 2);
        self.read_nbytes(length * 2, &mut buf)?;
        self.stream.decrypt(&mut buf);
        let mut mask: u16 = 0xaaaa;
        Ok(buf
            .chunks(2)
            .map(|c| {
                let wchar = (c[1] as u16) << 8;
                let wchar = wchar | (c[0] as u16);
                let wchar = wchar ^ mask;
                mask = mask + 1;
                wchar
            })
            .collect())
    }

    fn read_wz_int(&mut self) -> WzResult<i32> {
        self.reader.read_wz_int()
    }

    fn read_wz_long(&mut self) -> WzResult<i64> {
        self.reader.read_wz_long()
    }

    fn read_wz_offset(&mut self, abs_pos: u64, version_checksum: u32) -> WzResult<u32> {
        self.reader.read_wz_offset(abs_pos, version_checksum)
    }

    fn read_wz_string(&mut self) -> WzResult<String> {
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

        // Return the proper string
        Ok(if check < 0 {
            String::from_utf8(self.read_ascii(length as usize)?)?
        } else {
            String::from_utf16(&self.read_unicode(length as usize)?)?
        })
    }

    fn read_byte(&mut self) -> WzResult<u8> {
        self.reader.read_byte()
    }

    fn read_short(&mut self) -> WzResult<[u8; 2]> {
        self.reader.read_short()
    }

    fn read_word(&mut self) -> WzResult<[u8; 4]> {
        self.reader.read_word()
    }

    fn read_long(&mut self) -> WzResult<[u8; 8]> {
        self.reader.read_long()
    }

    fn read_nbytes(&mut self, length: usize, buf: &mut Vec<u8>) -> WzResult<()> {
        self.reader.read_nbytes(length, buf)
    }

    fn read_until(&mut self, delim: u8, buf: &mut Vec<u8>) -> WzResult<usize> {
        self.reader.read_until(delim, buf)
    }

    fn read_zstring(&mut self) -> WzResult<String> {
        self.reader.read_zstring()
    }
}

// Raising BufReader's Seek trait
impl<'a, B: ArrayLength<u8>, S: System<B>> Seek for WzEncryptedReader<'a, B, S> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.reader.seek(pos)
    }

    fn rewind(&mut self) -> io::Result<()> {
        self.reader.rewind()
    }

    fn stream_position(&mut self) -> io::Result<u64> {
        self.reader.stream_position()
    }
}
