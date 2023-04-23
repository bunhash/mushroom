//! WZ Writer

use crate::{error::Result, Metadata};
use std::io::SeekFrom;

/// Trait for writing WZ files
pub trait Writer: Sized {
    /// Returns the metadata of the WZ file
    fn metadata(&self) -> &Metadata;

    /// Get the position within the input
    fn position(&mut self) -> Result<u64>;

    /// Seek to position
    fn seek(&mut self, pos: SeekFrom) -> Result<u64>;

    /// Write the buffer. Raises the underlying [`Write`] trait
    fn write(&mut self, buf: &[u8]) -> Result<usize>;

    /// Write all of the buffer. Raises the underlying [`Write`] trait
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;

    /// Some versions of WZ files have encrypted strings. This function is used internally to
    /// encrypt them. If the version of WZ file you are writing does not need to encrypt strings,
    /// this function does not need to be implemented.
    fn encrypt(&mut self, _bytes: &mut Vec<u8>) {}

    /// Writes a single byte
    fn write_byte(&mut self, byte: u8) -> Result<()> {
        self.write_all(&[byte])
    }

    /// Writes a UTF-8 string. This function does not do UTF-8 conversion but will write the proper
    /// WZ encoding of the bytes.
    fn write_utf8_bytes(&mut self, bytes: &Vec<u8>) -> Result<()> {
        let mut mask = 0xaa;
        let mut buf = bytes
            .iter()
            .map(|b| {
                let c = b ^ mask;
                mask = match mask.checked_add(1) {
                    Some(v) => v,
                    None => 0,
                };
                c
            })
            .collect();
        self.encrypt(&mut buf);
        self.write_all(&buf)
    }

    /// Writes a unicode string. This function does not do Unicode conversion but will write the
    /// proper WZ encoding of the bytes.
    fn read_unicode_bytes(&mut self, bytes: &Vec<u16>) -> Result<()> {
        let mut mask: u16 = 0xaaaa;
        let mut buf = bytes
            .iter()
            .map(|c| {
                let wchar = c ^ mask;
                mask = match mask.checked_add(1) {
                    Some(v) => v,
                    None => 0,
                };
                wchar.to_le_bytes()
            })
            .flatten()
            .collect();
        self.encrypt(&mut buf);
        self.write_all(&buf)
    }
}
