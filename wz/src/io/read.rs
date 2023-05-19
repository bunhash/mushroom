//! WZ Readers

use crate::{
    error::{ImageError, Result},
    io::Decode,
    types::{WzInt, WzOffset},
};
use std::io::Write;

mod dummy_decryptor;
mod image;
mod reader;

pub use self::image::WzImageReader;
pub use dummy_decryptor::DummyDecryptor;
pub use reader::WzReader;

pub trait WzRead {
    /// Returns the absolute position of the WZ archive
    fn absolute_position(&self) -> i32;

    /// Returns the version checksum of the WZ archive
    fn version_checksum(&self) -> u32;

    /// Sets the version_checksum
    fn set_version_checksum(&mut self, version_checksum: u32);

    /// Get the position within the input
    fn position(&mut self) -> Result<WzOffset>;

    /// Seek to position
    fn seek(&mut self, pos: WzOffset) -> Result<WzOffset>;

    /// Read into the buffer. Raises the underlying `Read` trait
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    /// Read exact into buffer. Raises the underlying `Read` trait
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()>;

    /// Reads until EOF. Raises the underlying `Read` trait
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>;

    /// Copies `size` bytes starting at `offset` to the destination
    fn copy_to<W>(&mut self, dest: &mut W, offset: WzOffset, size: WzInt) -> Result<()>
    where
        W: Write;

    /// Decrypts a vector of bytes
    fn decrypt(&mut self, bytes: &mut Vec<u8>);

    /// Reads a UOL string
    fn read_uol_string(&mut self) -> Result<String> {
        let check = u8::decode(self)?;
        match check {
            0 => String::decode(self),
            1 => {
                let offset = WzOffset::from(u32::decode(self)?);
                let pos = self.position()?;
                self.seek(offset)?;
                let string = String::decode(self)?;
                self.seek(pos)?;
                Ok(string)
            }
            u => Err(ImageError::UolType(u).into()),
        }
    }

    /// Reads an object tag
    fn read_object_tag(&mut self) -> Result<String> {
        let check = u8::decode(self)?;
        match check {
            0x73 => String::decode(self),
            0x1b => {
                let offset = WzOffset::from(u32::decode(self)?);
                let pos = self.position()?;
                self.seek(offset)?;
                let string = String::decode(self)?;
                self.seek(pos)?;
                Ok(string)
            }
            u => Err(ImageError::UolType(u).into()),
        }
    }

    /// Seek to start after the version checksum (absolute_position + 2)
    fn seek_to_start(&mut self) -> Result<WzOffset> {
        self.seek(WzOffset::from(self.absolute_position() + 2))
    }

    /// Seek from absolute position
    fn seek_from_start(&mut self, offset: u32) -> Result<WzOffset> {
        self.seek(WzOffset::from(self.absolute_position() as u32 + offset))
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
                mask = mask.checked_add(1).unwrap_or(0);
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
                mask = mask.checked_add(1).unwrap_or(0);
                wchar
            })
            .collect())
    }
}
