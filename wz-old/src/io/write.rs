//! WZ Writer

use crate::error::Result;
use crate::io::Encode;
use crate::types::{WzInt, WzOffset};
use std::io::Read;

mod dummy_encryptor;
mod image;
mod writer;

pub use self::image::WzImageWriter;
pub use dummy_encryptor::DummyEncryptor;
pub use writer::WzWriter;

pub trait WzWrite {
    /// Returns the absolution position of the WZ archive
    fn absolute_position(&self) -> i32;

    /// Returns the version checksum of the WZ archive
    fn version_checksum(&self) -> u32;

    /// Get the position within the input
    fn position(&mut self) -> Result<WzOffset>;

    /// Seek to position
    fn seek(&mut self, pos: WzOffset) -> Result<WzOffset>;

    /// Write the buffer. Raises the underlying `Write` trait
    fn write(&mut self, buf: &[u8]) -> Result<usize>;

    /// Write all of the buffer. Raises the underlying `Write` trait
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;

    /// Copies `size` bytes from `src` to this writer
    fn copy_from<R>(&mut self, src: &mut R, size: WzInt) -> Result<()>
    where
        R: Read;

    /// Encrypts a vector of bytes
    fn encrypt(&mut self, bytes: &mut Vec<u8>);

    /// Writes a [`UolString`](crate::types::UolString) (images only)
    fn write_uol_string(&mut self, string: &str) -> Result<()> {
        0u8.encode(self)?;
        string.encode(self)
    }

    /// Writes the object tag string (images only)
    fn write_object_tag(&mut self, tag: &str) -> Result<()> {
        0x73u8.encode(self)?;
        tag.encode(self)
    }

    /// Seek to start after the version checksum (absolute_position + 2)
    fn seek_to_start(&mut self) -> Result<WzOffset> {
        self.seek(WzOffset::from(self.absolute_position() + 2))
    }

    /// Seek from absolute position
    fn seek_from_start(&mut self, offset: u32) -> Result<WzOffset> {
        self.seek(WzOffset::from(self.absolute_position() as u32 + offset))
    }

    /// Writes a single byte
    fn write_byte(&mut self, byte: u8) -> Result<()> {
        self.write_all(&[byte])
    }

    /// Writes a UTF-8 string. This function does not do UTF-8 conversion but will write the proper
    /// WZ encoding of the bytes.
    fn write_utf8_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        let mut mask = 0xaa;
        let mut buf = bytes
            .iter()
            .map(|b| {
                let c = b ^ mask;
                mask = mask.checked_add(1).unwrap_or(0);
                c
            })
            .collect();
        self.encrypt(&mut buf);
        self.write_all(&buf)
    }

    /// Writes a unicode string. This function does not do Unicode conversion but will write the
    /// proper WZ encoding of the bytes.
    fn write_unicode_bytes(&mut self, bytes: &[u16]) -> Result<()> {
        let mut mask: u16 = 0xaaaa;
        let mut buf = bytes
            .iter()
            .flat_map(|c| {
                let wchar = c ^ mask;
                mask = mask.checked_add(1).unwrap_or(0);
                wchar.to_le_bytes()
            })
            .collect();
        self.encrypt(&mut buf);
        self.write_all(&buf)
    }
}
