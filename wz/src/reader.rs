//! WZ Reader

use crate::{error::Result, Metadata};
use std::io::SeekFrom;

mod encrypted;
mod unencrypted;

pub use self::{encrypted::EncryptedReader, unencrypted::UnencryptedReader};

/// Trait for reading WZ files
pub trait Reader: Sized {
    /// Returns the metadata of the WZ file
    fn metadata(&self) -> &Metadata;

    /// Get the position within the input
    fn position(&mut self) -> Result<u64>;

    /// Seek to position
    fn seek(&mut self, pos: SeekFrom) -> Result<u64>;

    /// Read into the buffer. Raises the underlying `Read` trait
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    /// Read exact into buffer. Raises the underlying `Read` trait
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()>;

    /// Some versions of WZ files have encrypted strings. This function is used internally to
    /// decrypt them. If the version of WZ file you are reading does not need to decrypt strings,
    /// this function does not need to be implemented.
    fn decrypt(&mut self, _bytes: &mut Vec<u8>) {}

    /// Seek to start
    fn seek_to_start(&mut self) -> Result<u64> {
        self.seek(SeekFrom::Start(
            self.metadata().absolute_position as u64 + 2,
        ))
    }

    /// Seek from absolute position
    fn seek_from_start(&mut self, offset: u64) -> Result<u64> {
        self.seek(SeekFrom::Start(
            self.metadata().absolute_position as u64 + offset,
        ))
    }

    /// Reads a single byte and updates the cursor position
    fn read_byte(&mut self) -> Result<u8> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    /// Attempts to read input into a byte vecotr
    fn read_vec(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut data = vec![0u8; len];
        self.read_exact(&mut data)?;
        Ok(data)
    }

    /// Reads a string as if it were utf8. This function does not do UTF-8 conversion but will read
    /// the amount of bytes required to convert to utf8.
    fn read_utf8_bytes(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut buf = self.read_vec(len)?;
        self.decrypt(&mut buf);
        let mut mask = 0xaa;
        Ok(buf
            .iter()
            .map(|b| {
                let c = b ^ mask;
                mask = match mask.checked_add(1) {
                    Some(v) => v,
                    None => 0,
                };
                c
            })
            .collect())
    }

    /// Reads a string as if it were unicode (or wchar). This function does not do unicode
    /// conversion but will read the amount of bytes required to convert to unicode.
    fn read_unicode_bytes(&mut self, len: usize) -> Result<Vec<u16>> {
        let mut buf = self.read_vec(len * 2)?;
        self.decrypt(&mut buf);
        let mut mask: u16 = 0xaaaa;
        Ok(buf
            .chunks(2)
            .map(|c| {
                let wchar = u16::from_le_bytes([c[0], c[1]]);
                let wchar = wchar ^ mask;
                mask = match mask.checked_add(1) {
                    Some(v) => v,
                    None => 0,
                };
                wchar
            })
            .collect())
    }
}
