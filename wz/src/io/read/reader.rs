//! WZ Reader

use crate::{
    error::Result,
    io::{DummyDecryptor, WzRead},
    types::{WzInt, WzOffset},
};
use crypto::{Decryptor, KeyStream};
use std::io::{Read, Seek, SeekFrom, Write};

/// Wraps a reader into a WZ decoder. Used in [`Decode`](crate::io::Decode) trait
///
/// ```no_run
/// use crypto::checksum;
/// use std::{io::BufReader, fs::File};
/// use wz::{file::Header, io::WzReader};
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
/// use wz::{file::Header, io::WzReader};
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

    /// Consumes the WzReader and returns the underlying reader
    pub fn into_inner(self) -> R {
        self.reader
    }
}

impl<R, D> WzRead for WzReader<R, D>
where
    R: Read + Seek,
    D: Decryptor,
{
    fn absolute_position(&self) -> i32 {
        self.absolute_position
    }

    fn version_checksum(&self) -> u32 {
        self.version_checksum
    }

    fn set_version_checksum(&mut self, version_checksum: u32) {
        self.version_checksum = version_checksum;
    }

    fn position(&mut self) -> Result<WzOffset> {
        Ok(WzOffset::from(self.reader.stream_position()?))
    }

    fn seek(&mut self, pos: WzOffset) -> Result<WzOffset> {
        Ok(WzOffset::from(
            self.reader.seek(SeekFrom::Start(*pos as u64))?,
        ))
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Ok(self.reader.read(buf)?)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        Ok(self.reader.read_exact(buf)?)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        Ok(self.reader.read_to_end(buf)?)
    }

    fn copy_to<W>(&mut self, dest: &mut W, offset: WzOffset, size: WzInt) -> Result<()>
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
            remaining -= to_read;
        }
        Ok(())
    }

    fn decrypt(&mut self, bytes: &mut Vec<u8>) {
        self.decryptor.decrypt(bytes)
    }
}

#[cfg(test)]
mod tests {

    use crate::{file::Header, io::WzReader};
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
