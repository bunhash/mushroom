//! WZ Archive Header

use crate::archive::Error;
use crypto::checksum;
use std::io::{Read, Write};

/// Header of the WZ archive
///
/// The `Header` object contains 2 important pieces of information used in parsing WZ archives:
/// * `content_start`
/// * `version_hash`
///
/// These values are referenced to calculate WZ offset values within the archive.
#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord)]
pub struct Header {
    /// Constant value. ASCII for "PKG1"
    pub identifier: [u8; 4],

    /// Total size of the WZ contents
    pub size: u64,

    /// Position of where the content starts. This technically points to the version checksum which
    /// isn't very useful. The first `Package` is `content_start + 2`. However, this value is
    /// important for calculating offsets so it is not modified here. The actual value used for WZ
    /// archives is a signed 32-bit integer but it probably shouldn't be.
    pub content_start: u32,

    /// Description of the WZ package. Should be "Package file v1.0 Copyright 2002 Wizet, ZMS"
    pub description: String,

    /// Some obfuscated version checksum.
    pub version_hash: u16,
}

impl Header {
    /// Creates new header with default values.
    pub fn new(version: u16) -> Self {
        let (version_hash, _) = checksum(&version.to_string());
        Self {
            identifier: [0x50, 0x4b, 0x47, 0x31],
            size: 0,
            content_start: 60,
            description: String::from("Package file v1.0 Copyright 2002 Wizet, ZMS"),
            version_hash,
        }
    }

    /// Reads the header at the beginning of the WZ archive
    pub fn from_read<R>(read: &mut R) -> Result<Header, Error>
    where
        R: Read,
    {
        // Fill the readfer and ensure there are at least 16 bytes
        let mut data = [0u8; 16];
        read.read_exact(&mut data)?;

        // Read the identifier
        let mut identifier = [0u8; 4];
        identifier.copy_from_slice(&data[0..4]);
        if identifier != [0x50, 0x4b, 0x47, 0x31] {
            return Err(Error::Header);
        }

        // Read the size
        let mut size = [0u8; 8];
        size.copy_from_slice(&data[4..12]);
        let size = u64::from_le_bytes(size);

        // Read the absolute position
        let mut content_start = [0u8; 4];
        content_start.copy_from_slice(&data[12..16]);
        let content_start = i32::from_le_bytes(content_start);
        if content_start.is_negative() {
            return Err(Error::Header);
        }
        let content_start = content_start as u32;

        // Read the description
        let mut description = vec![0u8; (content_start as usize) - 17];
        read.read_exact(&mut description)?;
        let description = String::from_utf8(description).map_err(|_| Error::Header)?;

        // Skip the null
        let mut skip = [0];
        read.read_exact(&mut skip)?;

        // Read the encrypted version and bruteforce the checksum
        let mut version_hash = [0u8; 2];
        read.read_exact(&mut version_hash)?;
        let version_hash = u16::from_le_bytes(version_hash);

        Ok(Header {
            identifier,
            size,
            content_start,
            description,
            version_hash,
        })
    }

    pub fn to_write<W>(&self, write: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        write.write_all(&self.identifier)?;
        write.write_all(&self.size.to_le_bytes())?;
        write.write_all(&self.content_start.to_le_bytes())?;
        write.write_all(self.description.as_bytes())?;
        write.write_all(&[0u8])?;
        write.write_all(&self.version_hash.to_le_bytes())?;
        Ok(())
    }

    /// Gathers a list of possible versions that match the `version_hash`
    pub(crate) fn possible_versions(version_hash: u16) -> Vec<u32> {
        let mut versions = Vec::new();
        for version in 1..1000 {
            let (calc_version_hash, version_checksum) = checksum(&version.to_string());
            if calc_version_hash == version_hash {
                versions.push(version_checksum);
            }
        }
        versions
    }
}

#[cfg(test)]
mod tests {

    use crate::archive::Header;
    use std::fs::File;

    #[test]
    fn v83_header() {
        let mut file = File::open("testdata/v83-base.wz").expect("error opening file");
        let header = Header::from_read(&mut file).expect("error reading header");
        assert_eq!(&header.identifier, &[0x50, 0x4b, 0x47, 0x31]);
        assert_eq!(header.size, 6480);
        assert_eq!(header.content_start, 60);
        assert_eq!(
            &header.description,
            "Package file v1.0 Copyright 2002 Wizet, ZMS"
        );
        assert_eq!(header.version_hash, 172);
    }

    #[test]
    fn v172_header() {
        let mut file = File::open("testdata/v172-base.wz").expect("error opening file");
        let header = Header::from_read(&mut file).expect("error reading header");
        assert_eq!(&header.identifier, &[0x50, 0x4b, 0x47, 0x31]);
        assert_eq!(header.size, 6705);
        assert_eq!(header.content_start, 60);
        assert_eq!(
            &header.description,
            "Package file v1.0 Copyright 2002 Wizet, ZMS"
        );
        assert_eq!(header.version_hash, 7);
    }
}
