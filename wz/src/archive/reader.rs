//! WZ Archive Reader

use crate::{
    archive::{Archive, Error, Header, Offset, Package},
    decode::{self, Decode, Decoder},
    Int32,
};
use crypto::{checksum, Decryptor, DummyKeyStream};
use std::{
    fs::File,
    io::{self, BufReader, Read, Seek, SeekFrom, Write},
    path::Path,
};

/// WZ Archive Reader structure
#[derive(Debug)]
pub struct Reader<D>
where
    D: Decryptor,
{
    header: Header,
    file: BufReader<File>,
    decryptor: D,
    version_checksum: u32,
}

impl<D> Decoder for Reader<D>
where
    D: Decryptor,
{
    fn decode_bytes(&mut self, bytes: &mut [u8]) -> Result<(), decode::Error> {
        Ok(self.file.read_exact(bytes)?)
    }

    fn decrypt_bytes(&mut self, bytes: &mut [u8]) {
        self.decryptor.decrypt(bytes)
    }

    fn position(&mut self) -> Result<u32, decode::Error> {
        let position = self.file.stream_position()?;
        Ok(position
            .try_into()
            .map_err(|_| decode::Error::position(&format!("position is greater than u32::MAX")))?)
    }

    fn seek(&mut self, position: u32) -> Result<(), decode::Error> {
        let new_position = self.file.seek(SeekFrom::Start(position as u64))?;
        if new_position != position as u64 {
            Err(decode::Error::position(&format!(
                "tried to seek to {:08x} but currently at {:08x}",
                position, new_position
            )))
        } else {
            Ok(())
        }
    }

    fn decode_offset(&mut self) -> Result<Offset, decode::Error> {
        let position = self.position()? as u32;
        Ok(Offset::decode_with(
            u32::decode(self)?,
            position,
            self.header.content_start,
            self.version_checksum,
        ))
    }
}

impl Reader<DummyKeyStream> {
    /// Opens an unencrypted WZ Reader
    pub fn unencrypted<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        Reader::new(path, DummyKeyStream)
    }

    /// Opens an unencrypted WZ Reader
    pub fn unencrypted_as_version<P>(path: P, version: u16) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        Reader::as_version(path, version, DummyKeyStream)
    }
}

impl<D> Reader<D>
where
    D: Decryptor,
{
    /// Opens a WZ Reader
    pub fn new<P>(path: P, decryptor: D) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        D: Decryptor,
    {
        let mut file = BufReader::new(File::open(path)?);
        let header = Header::from_read(&mut file)?;
        let mut ret = Self {
            header,
            file,
            decryptor,
            version_checksum: 0,
        };
        ret.bruteforce_version()?;
        Ok(ret)
    }

    /// Opens a WZ Reader as a specific version
    pub fn as_version<P>(path: P, version: u16, decryptor: D) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        D: Decryptor,
    {
        let mut file = BufReader::new(File::open(path)?);
        let header = Header::from_read(&mut file)?;
        let (version_hash, version_checksum) = checksum(&version.to_string());
        if header.version_hash != version_hash {
            return Err(Error::version(
                "WZ archive does not match expected version hash",
            ));
        }
        Ok(Self {
            header,
            file,
            decryptor,
            version_checksum,
        })
    }

    /// Parses the WZ archive
    pub fn parse(&mut self) -> Result<Archive, Error> {
        Archive::parse(self.header.clone(), self)
    }

    /// Copies content at an offset to a `Write`
    pub fn copy_to<W>(&mut self, write: &mut W, offset: Offset, size: Int32) -> Result<(), Error>
    where
        W: Write,
    {
        self.seek(*offset)?;
        let mut buf = [0u8; 8192];
        let mut remaining = *size as usize;
        while remaining > 0 {
            let bytes_read = self.file.read(&mut buf)?;
            if bytes_read == 0 {
                return Err(io::Error::from(io::ErrorKind::UnexpectedEof))?;
            }
            let bytes_copied = if bytes_read > remaining {
                remaining
            } else {
                bytes_read
            };
            write.write_all(&buf[0..bytes_copied])?;
            remaining = remaining - bytes_copied;
        }
        Ok(())
    }

    fn bruteforce_version(&mut self) -> Result<(), Error> {
        let lower_bound = Offset::from(self.header.content_start);
        let upper_bound = Offset::from(self.header.content_start + self.header.size as u32);
        for version_checksum in Header::possible_versions(self.header.version_hash) {
            self.version_checksum = version_checksum;
            self.seek(self.header.content_start + 2)?;

            // Decodes the top-level directory contents. If all contents lie within the lower and
            // upper bounds, we can assume the version checksum is good.
            let package = Package::decode(self)?;
            let filtered_len = package
                .contents
                .iter()
                .map(|content| content.offset)
                .filter(|off| *off >= lower_bound && *off < upper_bound)
                .count();
            if package.contents.len() == filtered_len {
                return Ok(());
            }
        }
        Err(Error::bruteforce("bruteforce failed"))
    }
}
