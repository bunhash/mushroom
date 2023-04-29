//! WZ Reader

use crate::{error::Result, file::Metadata, types::WzOffset};
use crypto::{Decryptor, KeyStream};
use std::io::{Read, Seek, SeekFrom};

mod dummy_decryptor;

pub use self::dummy_decryptor::DummyDecryptor;

/// Wraps a reader into a WZ decoder. Used in [`Decode`](crate::Decode) trait
///
/// ```no_run
/// use std::{io::BufReader, fs::File};
/// use wz::{file::Metadata, WzReader};
///
/// let file = File::open("Base.wz").unwrap();
/// let metadata = Metadata::from_reader(&file).unwrap();
/// let reader = WzReader::unencrypted(&metadata, BufReader::new(file));
/// ```
///
/// ```no_run
/// use crypto::{KeyStream, TRIMMED_KEY, GMS_IV};
/// use std::{io::BufReader, fs::File};
/// use wz::{file::Metadata, WzReader};
///
/// let file = File::open("Base.wz").unwrap();
/// let metadata = Metadata::from_reader(&file).unwrap();
/// let reader = WzReader::encrypted(
///     &metadata,
///     BufReader::new(file),
///     KeyStream::new(&TRIMMED_KEY, &GMS_IV)
/// );
/// ```
pub struct WzReader<R, D>
where
    R: Read + Seek,
    D: Decryptor,
{
    /// Points to the beginning of the file after the metadata. I just include the version checksum
    /// in the metadata while it technically isn't.
    absolute_position: i32,

    /// Version hash/checksum
    version_checksum: u32,

    /// Underlying reader
    reader: R,

    /// Some versions of WZ files have encrypted strings. A [`DummyDecryptor`] is provided for
    /// versions that do not.
    decryptor: D,
}

impl<R> WzReader<R, DummyDecryptor>
where
    R: Read + Seek,
{
    /// Creates an unencrypted reader
    pub fn unencrypted(metadata: &Metadata, reader: R) -> Self {
        WzReader::new(metadata, reader, DummyDecryptor)
    }
}

impl<R> WzReader<R, KeyStream>
where
    R: Read + Seek,
{
    /// Creates an encrypted reader
    pub fn encrypted(metadata: &Metadata, reader: R, decryptor: KeyStream) -> Self {
        WzReader::new(metadata, reader, decryptor)
    }
}

impl<R, D> WzReader<R, D>
where
    R: Read + Seek,
    D: Decryptor,
{
    /// Creates a new `WzReader`
    pub fn new(metadata: &Metadata, reader: R, decryptor: D) -> Self {
        Self {
            absolute_position: metadata.absolute_position,
            version_checksum: metadata.version_checksum,
            reader,
            decryptor,
        }
    }

    /// Returns the metadata of the WZ file
    pub fn absolute_position(&self) -> i32 {
        self.absolute_position
    }

    /// Returns the metadata of the WZ file
    pub fn version_checksum(&self) -> u32 {
        self.version_checksum
    }

    /// Get the position within the input
    pub fn position(&mut self) -> Result<WzOffset> {
        Ok(WzOffset::from(self.reader.stream_position()?))
    }

    /// Seek to position
    pub fn seek(&mut self, pos: WzOffset) -> Result<WzOffset> {
        Ok(WzOffset::from(
            self.reader.seek(SeekFrom::Start(*pos as u64))?,
        ))
    }

    /// Read into the buffer. Raises the underlying `Read` trait
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Ok(self.reader.read(buf)?)
    }

    /// Read exact into buffer. Raises the underlying `Read` trait
    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        Ok(self.reader.read_exact(buf)?)
    }

    /// Seek to start after the version checksum (absolute_position + 2)
    pub fn seek_to_start(&mut self) -> Result<WzOffset> {
        self.seek(WzOffset::from(self.absolute_position() + 2))
    }

    /// Seek from absolute position
    pub fn seek_from_start(&mut self, offset: u32) -> Result<WzOffset> {
        self.seek(WzOffset::from(self.absolute_position() as u32 + offset))
    }

    /// Reads a single byte and updates the cursor position
    pub fn read_byte(&mut self) -> Result<u8> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    /// Attempts to read input into a byte vecotr
    pub fn read_vec(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut data = vec![0u8; len];
        self.read_exact(&mut data)?;
        Ok(data)
    }

    /// Reads a string as if it were utf8. This function does not do UTF-8 conversion but will read
    /// the amount of bytes required to convert to utf8.
    pub fn read_utf8_bytes(&mut self, len: usize) -> Result<Vec<u8>> {
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
    pub fn read_unicode_bytes(&mut self, len: usize) -> Result<Vec<u16>> {
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

    use crate::{file::Metadata, WzReader};
    use crypto::{KeyStream, GMS_IV, TRIMMED_KEY};
    use std::{fs::File, io::BufReader};

    #[test]
    fn make_encrypted() {
        let file = File::open("testdata/v83-base.wz").expect("error opening file");
        let metadata = Metadata::from_reader(&file).expect("error reading metadata");
        let _ = WzReader::encrypted(
            &metadata,
            BufReader::new(file),
            KeyStream::new(&TRIMMED_KEY, &GMS_IV),
        );
    }

    #[test]
    fn make_unencrypted() {
        let file = File::open("testdata/v172-base.wz").expect("error opening file");
        let metadata = Metadata::from_reader(&file).expect("error reading metadata");
        let _ = WzReader::unencrypted(&metadata, BufReader::new(file));
    }
}
