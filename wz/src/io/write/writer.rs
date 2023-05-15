//! WZ Writer

use crate::{
    error::Result,
    io::{DummyEncryptor, WzWrite},
    types::{WzInt, WzOffset},
};
use crypto::{Encryptor, KeyStream};
use std::io::{Read, Seek, SeekFrom, Write};

/// Wraps a writer into a WZ encoder. Used in [`Encode`](crate::io::Encode) trait
///
/// ```no_run
/// use crypto::checksum;
/// use std::{io::BufWriter, fs::File};
/// use wz::{io::WzWriter, types::package::Header};
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
/// use wz::{io::WzWriter, types::package::Header};
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

    /// Consumes the WzWriter and returns the underlying writer
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W, E> WzWrite for WzWriter<W, E>
where
    W: Write + Seek,
    E: Encryptor,
{
    fn absolute_position(&self) -> i32 {
        self.absolute_position
    }

    fn version_checksum(&self) -> u32 {
        self.version_checksum
    }

    fn position(&mut self) -> Result<WzOffset> {
        Ok(WzOffset::from(self.writer.stream_position()?))
    }

    fn seek(&mut self, pos: WzOffset) -> Result<WzOffset> {
        Ok(WzOffset::from(
            self.writer.seek(SeekFrom::Start(*pos as u64))?,
        ))
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(self.writer.write(buf)?)
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        Ok(self.writer.write_all(buf)?)
    }

    fn copy_from<R>(&mut self, src: &mut R, size: WzInt) -> Result<()>
    where
        R: Read,
    {
        let mut buf = [0u8; 8192];
        let mut remaining = *size as usize;
        while remaining > 0 {
            let to_read = if remaining > buf.len() {
                buf.len()
            } else {
                remaining
            };
            src.read_exact(&mut buf[0..to_read])?;
            self.write_all(&buf[0..to_read])?;
            remaining -= to_read;
        }
        Ok(())
    }

    fn encrypt(&mut self, bytes: &mut Vec<u8>) {
        self.encryptor.encrypt(bytes);
    }
}

#[cfg(test)]
mod tests {

    use crate::{io::WzWriter, types::package::Header};
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
