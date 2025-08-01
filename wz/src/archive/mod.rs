//! WZ Archive Structures

use crate::decode::{Decode, Reader};
use crypto::{checksum, Decryptor, DummyKeyStream};
use std::{fs::File, io::BufReader, path::Path};

mod error;
mod header;
mod map;
mod offset;
mod package;

pub use error::Error;
pub use header::Header;
pub use map::{ContentMap, ContentPathIter};
pub use offset::Offset;
pub use package::{Content, ContentType, Package};

/// WZ Archive structure
#[derive(Debug)]
pub struct Archive {
    /// WZ Archive header
    pub header: Header,

    /// Tree
    pub map: ContentMap,
}

impl Archive {
    /// Opens a WZ Archive
    pub fn parse<P, D>(path: P, decryptor: D) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        D: Decryptor,
    {
        let mut read = BufReader::new(File::open(path)?);
        let header = Header::from_read(&mut read)?;
        let mut reader = bruteforce_version(&header, read, decryptor)?;
        let map = ContentMap::parse(&header, &mut reader)?;
        Ok(Archive { header, map })
    }

    /// Opens a WZ Archive as a specific version
    pub fn parse_as_version<P, D>(path: P, version: u16, decryptor: D) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        D: Decryptor,
    {
        let mut read = BufReader::new(File::open(path)?);
        let header = Header::from_read(&mut read)?;
        let (version_hash, version_checksum) = checksum(&version.to_string());
        if header.version_hash != version_hash {
            return Err(Error::Version);
        }
        let mut reader = Reader::new(header.content_start, version_checksum, read, decryptor);
        let map = ContentMap::parse(&header, &mut reader)?;
        Ok(Archive { header, map })
    }

    /// Opens an unencrypted WZ Archive
    pub fn parse_unencrypted<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        Archive::parse(path, DummyKeyStream)
    }

    /// Opens an unencrypted WZ Archive
    pub fn parse_unencrypted_as_version<P>(path: P, version: u16) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        Archive::parse_as_version(path, version, DummyKeyStream)
    }
}

fn bruteforce_version<D>(
    header: &Header,
    buf: BufReader<File>,
    decryptor: D,
) -> Result<Reader<BufReader<File>, D>, Error>
where
    D: Decryptor,
{
    let lower_bound = Offset::from(header.content_start);
    let upper_bound = Offset::from(header.content_start + header.size as u32);
    let mut reader = Reader::new(header.content_start, 0u32, buf, decryptor);
    for version_checksum in Header::possible_versions(header.version_hash) {
        reader.version_checksum = version_checksum;
        reader.rewind()?;

        // Decodes the top-level directory contents. If all contents lie within the lower and
        // upper bounds, we can assume the version checksum is good.
        let package = Package::decode(&mut reader)?;
        let filtered_len = package
            .contents
            .iter()
            .map(|content| content.offset)
            .filter(|off| *off >= lower_bound && *off < upper_bound)
            .count();
        if package.contents.len() == filtered_len {
            return Ok(reader);
        }
    }
    Err(Error::Bruteforce)
}
