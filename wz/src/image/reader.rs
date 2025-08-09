//! WZ Image Reader

use crate::{
    decode::{self, Decoder},
    image::Error,
};
use crypto::{Decryptor, DummyKeyStream};
use std::{
    fs::File,
    io::{self, BufReader, Read, Seek, SeekFrom, Write},
    path::Path,
};

/// WZ Image Reader
#[derive(Debug)]
pub struct Reader<D>
where
    D: Decryptor,
{
    file: BufReader<File>,
    decryptor: D,
    start: u32,
    end: u32,
}

impl<D> Decoder for Reader<D>
where
    D: Decryptor,
{
    fn decode_bytes(&mut self, bytes: &mut [u8]) -> Result<(), decode::Error> {
        Ok(self.file.read_exact(bytes)?)
    }

    fn decrypt_bytes(&mut self, bytes: &mut [u8]) {
        self.decryptor.decrypt(bytes)
    }

    fn position(&mut self) -> Result<u32, decode::Error> {
        let position = self.file.stream_position()?;
        let position: u32 = position
            .try_into()
            .map_err(|_| decode::Error::position(&format!("position is greater than u32::MAX")))?;
        if position > self.end || position < self.start {
            Err(decode::Error::position(&format!(
                "position is outside of image bounds"
            )))
        } else {
            Ok(position - self.start)
        }
    }

    fn seek(&mut self, position: u32) -> Result<(), decode::Error> {
        let position = self.start + position;
        if position > self.end {
            return Err(decode::Error::position(&format!(
                "position is outside of image bounds"
            )));
        }
        let new_position = self.file.seek(SeekFrom::Start(position as u64))?;
        if new_position != position as u64 {
            Err(decode::Error::position(&format!(
                "tried to seek to {:08x} but currently at {:08x}",
                position, new_position
            )))
        } else {
            Ok(())
        }
    }
}

impl Reader<DummyKeyStream> {
    /// Opens an unencrypted image Reader
    pub fn unencrypted<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        Reader::new(path, DummyKeyStream)
    }

    /// Opens an unencrypted image Reader
    pub fn unencrypted_from_archive<P>(path: P, offset: u32, size: u32) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        Reader::from_archive(path, offset, size, DummyKeyStream)
    }
}

impl<D> Reader<D>
where
    D: Decryptor,
{
    /// Opens an image Reader
    pub fn new<P>(path: P, decryptor: D) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        D: Decryptor,
    {
        let size = path.as_ref().metadata()?.len();
        let size: u32 = size
            .try_into()
            .map_err(|_| decode::Error::size(&format!("image size is too large")))?;
        Ok(Self {
            file: BufReader::new(File::open(path)?),
            decryptor,
            start: 0,
            end: size,
        })
    }

    /// Opens a WZ Reader as a specific version
    pub fn from_archive<P>(path: P, offset: u32, size: u32, decryptor: D) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        D: Decryptor,
    {
        let mut ret = Self {
            file: BufReader::new(File::open(path)?),
            decryptor,
            start: offset,
            end: offset + size,
        };
        ret.seek(0)?;
        Ok(ret)
    }

    /// Copies content at an offset to a `Write`
    pub fn copy_to<W>(&mut self, write: &mut W, offset: u32, size: u32) -> Result<(), Error>
    where
        W: Write,
    {
        self.seek(offset)?;
        let mut buf = [0u8; 8192];
        let mut remaining = size as usize;
        while remaining > 0 {
            let bytes_read = self.file.read(&mut buf)?;
            if bytes_read == 0 {
                return Err(io::Error::from(io::ErrorKind::UnexpectedEof))?;
            }
            let bytes_copied = if bytes_read > remaining {
                remaining
            } else {
                bytes_read
            };
            write.write_all(&buf[0..bytes_copied])?;
            remaining = remaining - bytes_copied;
        }
        Ok(())
    }
}
