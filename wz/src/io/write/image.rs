//! WZ Image Writer

use crate::error::Result;
use crate::io::{encode::SizeHint, Encode, WzWrite};
use crate::types::{WzInt, WzOffset};
use std::{collections::HashMap, io::Read};

/// WZ Image Writer
///
/// This just wraps a WzWrite to do the following:
/// * ensure the seeking offsets align properly
/// * keep track of the string cache for writing UOLs
#[derive(Debug)]
pub struct WzImageWriter<'a, W>
where
    W: WzWrite + ?Sized,
{
    inner: &'a mut W,
    offset: WzOffset,
    cache: HashMap<String, u32>,
}

impl<'a, W> WzImageWriter<'a, W>
where
    W: WzWrite + ?Sized,
{
    /// Creates a new [`WzImageWriter`] with a starting offset of 0
    pub fn new(inner: &'a mut W) -> Self {
        Self {
            inner,
            offset: 0.into(),
            cache: HashMap::new(),
        }
    }

    /// Creates a new [`WzImageWriter`] starting at `offset`
    pub fn with_offset(inner: &'a mut W, offset: WzOffset) -> Self {
        Self {
            inner,
            offset,
            cache: HashMap::new(),
        }
    }

    #[inline]
    fn write_from_cache(&mut self, string: &str, not_cached: u8, cached: u8) -> Result<()> {
        // WZ images don't seem to bother with UOLs if the length is greater than 4. I assume there
        // is some calculation based on the encoded size and not the string length, but I could be
        // wrong. I just set threshold to size_hint() > 5 to match what I've witnessed decoding.
        if string.size_hint() > 5 && self.cache.contains_key(string) {
            cached.encode(self)?;
            let offset = *self.cache.get(string).expect("cache should have string");
            offset.encode(self)
        } else {
            not_cached.encode(self)?;
            let position = self.position()?;
            self.cache.insert(string.to_string(), *position);
            string.encode(self)
        }
    }
}

impl<'a, W> WzWrite for WzImageWriter<'a, W>
where
    W: WzWrite + ?Sized,
{
    fn absolute_position(&self) -> i32 {
        self.inner.absolute_position()
    }

    fn version_checksum(&self) -> u32 {
        self.inner.version_checksum()
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

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.inner.write(buf)
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.inner.write_all(buf)
    }

    fn copy_from<R>(&mut self, src: &mut R, size: WzInt) -> Result<()>
    where
        R: Read,
    {
        self.inner.copy_from(src, size)
    }

    fn encrypt(&mut self, bytes: &mut Vec<u8>) {
        self.inner.encrypt(bytes)
    }

    fn write_uol_string(&mut self, string: &str) -> Result<()> {
        self.write_from_cache(string, 0, 1)
    }

    fn write_object_tag(&mut self, tag: &str) -> Result<()> {
        self.write_from_cache(tag, 0x73, 0x1b)
    }
}
