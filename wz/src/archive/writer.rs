//! WZ Archive Writer

use crate::{
    archive::{builder::Image, Error, Header, Offset},
    encode::{self, Encode, Encoder},
};
use crypto::{checksum, DummyKeyStream, Encryptor};
use std::{
    fs::File,
    io::{self, BufWriter, Read, Seek, Write},
    path::Path,
};

/// WZ Archive Writer structure
#[derive(Debug)]
pub struct Writer<E>
where
    E: Encryptor,
{
    /// The WZ archive header
    pub header: Header,
    file: BufWriter<File>,
    encryptor: E,
    version_checksum: u32,
}

impl<E> Encoder for Writer<E>
where
    E: Encryptor,
{
    fn encode_bytes(&mut self, bytes: &[u8]) -> Result<(), encode::Error> {
        self.file.write(bytes)?;
        Ok(())
    }

    fn encrypt_bytes(&mut self, bytes: &mut [u8]) {
        self.encryptor.encrypt(bytes);
    }

    fn position(&mut self) -> Result<u32, encode::Error> {
        let position = self.file.stream_position()?;
        Ok(position
            .try_into()
            .map_err(|_| encode::Error::position(position))?)
    }

    fn encode_offset(&mut self, offset: Offset) -> Result<(), encode::Error> {
        let position = self.position()? as u32;
        offset
            .encode_with(position, self.header.content_start, self.version_checksum)
            .encode(self)?;
        Ok(())
    }
}

impl Writer<DummyKeyStream> {
    /// Opens an unencrypted WZ Reader
    pub fn unencrypted<P>(path: P, version: u16) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        Writer::new(path, DummyKeyStream, version)
    }
}

impl<E> Writer<E>
where
    E: Encryptor,
{
    /// Opens a WZ Reader
    pub fn new<P>(path: P, encryptor: E, version: u16) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        E: Encryptor,
    {
        let header = Header::new(version);
        let file = BufWriter::new(File::create(path)?);
        let (_, version_checksum) = checksum(&version.to_string());
        Ok(Self {
            header,
            file,
            encryptor,
            version_checksum,
        })
    }

    /// Write the header
    pub fn write_header(&mut self) -> Result<(), Error> {
        self.header.to_write(&mut self.file)?;
        Ok(())
    }

    /// Copies content at an offset to a `Write`
    pub fn write_image<I: Image>(&mut self, image: &I) -> Result<(), Error> {
        let mut read = image.to_reader()?;
        let mut buf = [0u8; 8192];
        let mut remaining = image.size() as usize;
        while remaining > 0 {
            let bytes_read = read.read(&mut buf)?;
            if bytes_read == 0 {
                return Err(io::Error::from(io::ErrorKind::UnexpectedEof))?;
            }
            let bytes_copied = if bytes_read > remaining {
                remaining
            } else {
                bytes_read
            };
            self.encode_bytes(&buf[0..bytes_copied])?;
            remaining = remaining - bytes_copied;
        }
        Ok(())
    }
}
