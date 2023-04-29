//! WZ Writer

use crate::{encode::Error, file::Metadata, types::WzOffset};
use crypto::{Encryptor, KeyStream};
use std::io::{Seek, SeekFrom, Write};

mod dummy_encryptor;

pub use self::dummy_encryptor::DummyEncryptor;

/// Wraps a writer into a WZ encoder. Used in [`Encode`](crate::Encode) trait
///
/// ```no_run
/// use std::{io::BufWriter, fs::File};
/// use wz::{file::Metadata, WzWriter};
///
/// let metadata = Metadata::new(172);
/// let file = File::create("Base.wz").unwrap();
/// let reader = WzWriter::unencrypted(&metadata, BufWriter::new(file));
/// ```
///
/// ```no_run
/// use crypto::{KeyStream, TRIMMED_KEY, GMS_IV};
/// use std::{io::BufWriter, fs::File};
/// use wz::{file::Metadata, WzWriter};
///
/// let metadata = Metadata::new(83);
/// let file = File::open("Base.wz").unwrap();
/// let reader = WzWriter::encrypted(
///     &metadata,
///     BufWriter::new(file),
///     KeyStream::new(&TRIMMED_KEY, &GMS_IV)
/// );
/// ```
pub struct WzWriter<W, E>
where
    W: Write + Seek,
    E: Encryptor,
{
    /// Points to the beginning of the file after the metadata. I just include the version checksum
    /// in the metadata while it technically isn't.
    absolute_position: i32,

    /// Version hash/checksum
    version_checksum: u32,

    /// Underlying writer
    writer: W,

    /// Some versions of WZ files have encrypted strings. A [`DummyEncryptor`] is provided for
    /// versions that do not.
    encryptor: E,
}

impl<W> WzWriter<W, DummyEncryptor>
where
    W: Write + Seek,
{
    /// Creates an unencrypted writer
    pub fn unencrypted(metadata: &Metadata, writer: W) -> Self {
        WzWriter::new(metadata, writer, DummyEncryptor)
    }
}

impl<W> WzWriter<W, KeyStream>
where
    W: Write + Seek,
{
    /// Creates an encrypted writer
    pub fn encrypted(metadata: &Metadata, writer: W, encryptor: KeyStream) -> Self {
        WzWriter::new(metadata, writer, encryptor)
    }
}

impl<W, E> WzWriter<W, E>
where
    W: Write + Seek,
    E: Encryptor,
{
    /// Creates a new `WzWriter`
    pub fn new(metadata: &Metadata, writer: W, encryptor: E) -> Self {
        Self {
            absolute_position: metadata.absolute_position,
            version_checksum: metadata.version_checksum,
            writer,
            encryptor,
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

    use crate::{file::Metadata, WzWriter};
    use crypto::{KeyStream, GMS_IV, TRIMMED_KEY};
    use std::io::Cursor;

    #[test]
    fn make_encrypted() {
        let metadata = Metadata::new(83);
        let _ = WzWriter::encrypted(
            &metadata,
            Cursor::new(vec![0u8; 60]),
            KeyStream::new(&TRIMMED_KEY, &GMS_IV),
        );
    }

    #[test]
    fn make_unencrypted() {
        let metadata = Metadata::new(176);
        let _ = WzWriter::unencrypted(&metadata, Cursor::new(vec![0u8; 60]));
    }
}
