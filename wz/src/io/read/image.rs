//! WZ Image Reader

use crate::{
    error::{ImageError, Result},
    io::{Decode, WzRead},
    types::{WzInt, WzOffset},
};
use std::{collections::HashMap, io::Write};

/// WZ Image Reader
///
/// This just wraps a WzRead so the seeking offsets align properly. This is not needed unless the
/// image resides within a WZ archive. It also tracks cached strings so it may slightly speed up
/// parsing but hog more memory. Make sure to let this object die after reading is complete to
/// clear the cache from memory.
#[derive(Debug)]
pub struct WzImageReader<'a, R>
where
    R: WzRead + ?Sized,
{
    inner: &'a mut R,
    offset: WzOffset,
    cache: HashMap<u32, String>,
}

impl<'a, R> WzImageReader<'a, R>
where
    R: WzRead + ?Sized,
{
    /// Creates a new [`WzImageReader`] with a starting offset of 0
    pub fn new(inner: &'a mut R) -> Self {
        Self {
            inner,
            offset: WzOffset::from(0),
            cache: HashMap::new(),
        }
    }

    /// Creates a new [`WzImageReader`] starting at `offset`
    pub fn with_offset(inner: &'a mut R, offset: WzOffset) -> Self {
        Self {
            inner,
            offset,
            cache: HashMap::new(),
        }
    }
}

impl<'a, R> WzRead for WzImageReader<'a, R>
where
    R: WzRead + ?Sized,
{
    fn absolute_position(&self) -> i32 {
        self.inner.absolute_position()
    }

    fn version_checksum(&self) -> u32 {
        self.inner.version_checksum()
    }

    fn set_version_checksum(&mut self, version_checksum: u32) {
        self.inner.set_version_checksum(version_checksum)
    }

    fn position(&mut self) -> Result<WzOffset> {
        Ok(self.inner.position()? - self.offset)
    }

    fn seek(&mut self, pos: WzOffset) -> Result<WzOffset> {
        self.inner.seek(self.offset + pos)
    }

    fn seek_to_start(&mut self) -> Result<WzOffset> {
        self.inner.seek(self.offset)
    }

    fn seek_from_start(&mut self, offset: u32) -> Result<WzOffset> {
        self.inner.seek(self.offset + offset.into())
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.inner.read(buf)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        self.inner.read_exact(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        self.inner.read_to_end(buf)
    }

    fn copy_to<W>(&mut self, dest: &mut W, offset: WzOffset, size: WzInt) -> Result<()>
    where
        W: Write,
    {
        self.inner.copy_to(dest, self.offset + offset, size)
    }

    fn decrypt(&mut self, bytes: &mut Vec<u8>) {
        self.inner.decrypt(bytes)
    }

    fn read_uol_string(&mut self) -> Result<String> {
        let check = u8::decode(self)?;
        match check {
            0 => {
                let position = self.position()?;
                let string = String::decode(self)?;
                self.cache.insert(*position, string.clone());
                Ok(string)
            }
            1 => {
                let offset = u32::decode(self)?;
                Ok(match self.cache.get(&offset) {
                    Some(string) => string.to_string(),
                    None => {
                        let pos = self.position()?;
                        self.seek(offset.into())?;
                        let string = String::decode(self)?;
                        self.seek(pos)?;
                        string
                    }
                })
            }
            u => Err(ImageError::UolType(u).into()),
        }
    }

    fn read_object_tag(&mut self) -> Result<String> {
        let check = u8::decode(self)?;
        match check {
            0x73 => {
                let position = self.position()?;
                let string = String::decode(self)?;
                self.cache.insert(*position, string.clone());
                Ok(string)
            }
            0x1b => {
                let offset = u32::decode(self)?;
                Ok(match self.cache.get(&offset) {
                    Some(string) => string.to_string(),
                    None => {
                        let pos = self.position()?;
                        self.seek(offset.into())?;
                        let string = String::decode(self)?;
                        self.seek(pos)?;
                        string
                    }
                })
            }
            u => Err(ImageError::UolType(u).into()),
        }
    }
}
