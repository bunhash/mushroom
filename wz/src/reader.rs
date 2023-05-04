//! WZ Reader

use crate::{
    decode::Error,
    types::{WzInt, WzOffset},
};
use crypto::{Decryptor, KeyStream};
use std::io::{Read, Seek, SeekFrom, Write};

mod chunk;
mod dummy_decryptor;

pub use self::{chunk::ChunkReader, dummy_decryptor::DummyDecryptor};

/// Wraps a reader into a WZ decoder. Used in [`Decode`](crate::Decode) trait
///
/// ```no_run
/// use crypto::checksum;
/// use std::{io::BufReader, fs::File};
/// use wz::{file::Header, WzReader};
///
/// let mut file = File::open("Base.wz").unwrap();
/// let header = Header::from_reader(&mut file).unwrap();
/// let (_, version_checksum) = checksum("176");
/// let reader = WzReader::unencrypted(
///     header.absolute_position,
///     version_checksum,
///     BufReader::new(file),
/// );
/// ```
///
/// ```no_run
/// use crypto::{checksum, KeyStream, TRIMMED_KEY, GMS_IV};
/// use std::{io::BufReader, fs::File};
/// use wz::{file::Header, WzReader};
///
/// let mut file = File::open("Base.wz").unwrap();
/// let header = Header::from_reader(&mut file).unwrap();
/// let (_, version_checksum) = checksum("83");
/// let reader = WzReader::encrypted(
///     header.absolute_position,
///     version_checksum,
///     BufReader::new(file),
///     KeyStream::new(&TRIMMED_KEY, &GMS_IV),
/// );
/// ```
#[derive(Debug)]
pub struct WzReader<R, D>
where
    R: Read + Seek,
    D: Decryptor,
{
    /// Points to the beginning of the file after the header. I just include the version checksum
    /// in the header while it technically isn't.
    absolute_position: i32,

    /// Version hash/checksum
    version_checksum: u32,

    /// Underlying reader
    reader: R,

    /// Some versions of WZ archives have encrypted strings. A [`DummyDecryptor`] is provided for
    /// versions that do not.
    decryptor: D,
}

impl<R> WzReader<R, DummyDecryptor>
where
    R: Read + Seek,
{
    /// Creates an unencrypted reader
    pub fn unencrypted(absolute_position: i32, version_checksum: u32, reader: R) -> Self {
        WzReader::new(absolute_position, version_checksum, reader, DummyDecryptor)
    }
}

impl<R> WzReader<R, KeyStream>
where
    R: Read + Seek,
{
    /// Creates an encrypted reader
    pub fn encrypted(
        absolute_position: i32,
        version_checksum: u32,
        reader: R,
        decryptor: KeyStream,
    ) -> Self {
        WzReader::new(absolute_position, version_checksum, reader, decryptor)
    }
}

impl<R, D> WzReader<R, D>
where
    R: Read + Seek,
    D: Decryptor,
{
    /// Creates a new `WzReader`
    pub fn new(absolute_position: i32, version_checksum: u32, reader: R, decryptor: D) -> Self {
        Self {
            absolute_position,
            version_checksum,
            reader,
            decryptor,
        }
    }

    /// Returns the header of the WZ archive
    pub fn absolute_position(&self) -> i32 {
        self.absolute_position
    }

    /// Returns the header of the WZ archive
    pub fn version_checksum(&self) -> u32 {
        self.version_checksum
    }

    /// Sets the version_checksum
    pub(crate) fn set_version_checksum(&mut self, version_checksum: u32) {
        self.version_checksum = version_checksum;
    }

    /// Get the position within the input
    pub fn position(&mut self) -> Result<WzOffset, Error> {
        Ok(WzOffset::from(self.reader.stream_position()?))
    }

    /// Seek to position
    pub fn seek(&mut self, pos: WzOffset) -> Result<WzOffset, Error> {
        Ok(WzOffset::from(
            self.reader.seek(SeekFrom::Start(*pos as u64))?,
        ))
    }

    /// Read into the buffer. Raises the underlying `Read` trait
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        Ok(self.reader.read(buf)?)
    }

    /// Read exact into buffer. Raises the underlying `Read` trait
    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        Ok(self.reader.read_exact(buf)?)
    }

    /// Reads until EOF. Raises the underlying `Read` trait
    pub fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        Ok(self.reader.read_to_end(buf)?)
    }

    /// Copies `size` bytes starting at `offset` to the destination
    pub fn copy_to<W>(&mut self, dest: &mut W, offset: WzOffset, size: WzInt) -> Result<(), Error>
    where
        W: Write,
    {
        self.seek(offset)?;
        let mut buf = [0u8; 8192];
        let mut remaining = *size as usize;
        while remaining > 0 {
            let to_read = if remaining > buf.len() {
                buf.len()
            } else {
                remaining
            };
            self.read_exact(&mut buf[0..to_read])?;
            dest.write_all(&buf[0..to_read])?;
            remaining = remaining - to_read;
        }
        Ok(())
    }

    /// Seek to start after the version checksum (absolute_position + 2)
    pub fn seek_to_start(&mut self) -> Result<WzOffset, Error> {
        self.seek(WzOffset::from(self.absolute_position() + 2))
    }

    /// Seek from absolute position
    pub fn seek_from_start(&mut self, offset: u32) -> Result<WzOffset, Error> {
        self.seek(WzOffset::from(self.absolute_position() as u32 + offset))
    }

    /// Reads a single byte and updates the cursor position
    pub fn read_byte(&mut self) -> Result<u8, Error> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    /// Attempts to read input into a byte vecotr
    pub fn read_vec(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        let mut data = vec![0u8; len];
        self.read_exact(&mut data)?;
        Ok(data)
    }

    /// Reads a string as if it were utf8. This function does not do UTF-8 conversion but will read
    /// the amount of bytes required to convert to utf8.
    pub fn read_utf8_bytes(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        let mut buf = self.read_vec(len)?;
        self.decryptor.decrypt(&mut buf);
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
    pub fn read_unicode_bytes(&mut self, len: usize) -> Result<Vec<u16>, Error> {
        let mut buf = self.read_vec(len * 2)?;
        self.decryptor.decrypt(&mut buf);
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

#[cfg(test)]
mod tests {

    use crate::{file::Header, WzReader};
    use crypto::{checksum, KeyStream, GMS_IV, TRIMMED_KEY};
    use std::{fs::File, io::BufReader};

    #[test]
    fn make_encrypted() {
        let mut file = File::open("testdata/v83-base.wz").expect("error opening file");
        let header = Header::from_reader(&mut file).expect("error reading header");
        let (_, version_checksum) = checksum("83");
        let _ = WzReader::encrypted(
            header.absolute_position,
            version_checksum,
            BufReader::new(file),
            KeyStream::new(&TRIMMED_KEY, &GMS_IV),
        );
    }

    #[test]
    fn make_unencrypted() {
        let mut file = File::open("testdata/v172-base.wz").expect("error opening file");
        let header = Header::from_reader(&mut file).expect("error reading header");
        let (_, version_checksum) = checksum("176");
        let _ = WzReader::unencrypted(
            header.absolute_position,
            version_checksum,
            BufReader::new(file),
        );
    }
}
