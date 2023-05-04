//! WZ Writer

use crate::{encode::Error, types::WzOffset};
use crypto::{Encryptor, KeyStream};
use std::io::{Seek, SeekFrom, Write};

mod dummy_encryptor;

pub use self::dummy_encryptor::DummyEncryptor;

/// Wraps a writer into a WZ encoder. Used in [`Encode`](crate::Encode) trait
///
/// ```no_run
/// use crypto::checksum;
/// use std::{io::BufWriter, fs::File};
/// use wz::{file::Header, WzWriter};
///
/// let header = Header::new(172);
/// let file = File::create("Base.wz").unwrap();
/// let (_, version_checksum) = checksum("172");
/// let reader = WzWriter::unencrypted(
///     header.absolute_position,
///     version_checksum,
///     BufWriter::new(file),
/// );
/// ```
///
/// ```no_run
/// use crypto::{checksum, KeyStream, TRIMMED_KEY, GMS_IV};
/// use std::{io::BufWriter, fs::File};
/// use wz::{file::Header, WzWriter};
///
/// let header = Header::new(83);
/// let file = File::open("Base.wz").unwrap();
/// let (_, version_checksum) = checksum("83");
/// let reader = WzWriter::encrypted(
///     header.absolute_position,
///     version_checksum,
///     BufWriter::new(file),
///     KeyStream::new(&TRIMMED_KEY, &GMS_IV)
/// );
/// ```
#[derive(Debug)]
pub struct WzWriter<W, E>
where
    W: Write + Seek,
    E: Encryptor,
{
    /// Points to the beginning of the file after the header. I just include the version checksum
    /// in the header while it technically isn't.
    absolute_position: i32,

    /// Version hash/checksum
    version_checksum: u32,

    /// Underlying writer
    writer: W,

    /// Some versions of WZ archives have encrypted strings. A [`DummyEncryptor`] is provided for
    /// versions that do not.
    encryptor: E,
}

impl<W> WzWriter<W, DummyEncryptor>
where
    W: Write + Seek,
{
    /// Creates an unencrypted writer
    pub fn unencrypted(absolute_position: i32, version_checksum: u32, writer: W) -> Self {
        WzWriter::new(absolute_position, version_checksum, writer, DummyEncryptor)
    }
}

impl<W> WzWriter<W, KeyStream>
where
    W: Write + Seek,
{
    /// Creates an encrypted writer
    pub fn encrypted(
        absolute_position: i32,
        version_checksum: u32,
        writer: W,
        encryptor: KeyStream,
    ) -> Self {
        WzWriter::new(absolute_position, version_checksum, writer, encryptor)
    }
}

impl<W, E> WzWriter<W, E>
where
    W: Write + Seek,
    E: Encryptor,
{
    /// Creates a new `WzWriter`
    pub fn new(absolute_position: i32, version_checksum: u32, writer: W, encryptor: E) -> Self {
        Self {
            absolute_position,
            version_checksum,
            writer,
            encryptor,
        }
    }

    /// Returns the absolution position of the WZ archive
    pub fn absolute_position(&self) -> i32 {
        self.absolute_position
    }

    /// Returns the version checksum of the WZ archive
    pub fn version_checksum(&self) -> u32 {
        self.version_checksum
    }

    /// Get the position within the input
    pub fn position(&mut self) -> Result<WzOffset, Error> {
        Ok(WzOffset::from(self.writer.stream_position()?))
    }

    /// Seek to position
    pub fn seek(&mut self, pos: WzOffset) -> Result<WzOffset, Error> {
        Ok(WzOffset::from(
            self.writer.seek(SeekFrom::Start(*pos as u64))?,
        ))
    }

    /// Write the buffer. Raises the underlying `Write` trait
    pub fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        Ok(self.writer.write(buf)?)
    }

    /// Write all of the buffer. Raises the underlying `Write` trait
    pub fn write_all(&mut self, buf: &[u8]) -> Result<(), Error> {
        Ok(self.writer.write_all(buf)?)
    }

    /// Writes a single byte
    pub fn write_byte(&mut self, byte: u8) -> Result<(), Error> {
        self.write_all(&[byte])
    }

    /// Writes a UTF-8 string. This function does not do UTF-8 conversion but will write the proper
    /// WZ encoding of the bytes.
    pub fn write_utf8_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
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
        self.encryptor.encrypt(&mut buf);
        self.write_all(&buf)
    }

    /// Writes a unicode string. This function does not do Unicode conversion but will write the
    /// proper WZ encoding of the bytes.
    pub fn write_unicode_bytes(&mut self, bytes: &[u16]) -> Result<(), Error> {
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
        self.encryptor.encrypt(&mut buf);
        self.write_all(&buf)
    }
}

#[cfg(test)]
mod tests {

    use crate::{file::Header, WzWriter};
    use crypto::{checksum, KeyStream, GMS_IV, TRIMMED_KEY};
    use std::io::Cursor;

    #[test]
    fn make_encrypted() {
        let header = Header::new(83);
        let (_, version_checksum) = checksum("83");
        let _ = WzWriter::encrypted(
            header.absolute_position,
            version_checksum,
            Cursor::new(vec![0u8; 60]),
            KeyStream::new(&TRIMMED_KEY, &GMS_IV),
        );
    }

    #[test]
    fn make_unencrypted() {
        let header = Header::new(176);
        let (_, version_checksum) = checksum("176");
        let _ = WzWriter::unencrypted(
            header.absolute_position,
            version_checksum,
            Cursor::new(vec![0u8; 60]),
        );
    }
}
