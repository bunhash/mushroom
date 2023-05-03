//! WZ Metadata

use crate::{
    encode,
    error::{self, WzError},
    types::CString,
    Encode, WzWriter,
};
use crypto::{checksum, Encryptor};
use std::io::{Read, Seek, Write};

/// Metadata of the WZ file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    /// Constant value. ASCII for "PKG1"
    pub identifier: [u8; 4],

    /// Total size of the WZ contents
    pub size: u64,

    /// Position of where the content starts. However, this technically points to the version
    /// checksum which isn't very useful for me. So you should add 2 to get to the top-level
    /// `Package`. However, this value is important for calculating offsets so it isn't modified
    /// here. The actual value used for WZ files is signed but it probably shouldn't be.
    pub absolute_position: i32,

    /// Description of the WZ package. Should be "Package file v1.0 Copyright 2002 Wizet, ZMS"
    pub description: CString,

    /// Encrypted version (not really encrypted since it is irreversable. More like a checksum or
    /// non-cryptographic hash.
    pub encrypted_version: u16,
}

impl Metadata {
    /// Creates new metadata with default values.
    pub fn new(version: u16) -> Self {
        let (encrypted_version, _) = checksum(&version.to_string());
        Self {
            identifier: [0x50, 0x4b, 0x47, 0x31],
            size: 0,
            absolute_position: 60,
            description: CString::from("Package file v1.0 Copyright 2002 Wizet, ZMS"),
            encrypted_version,
        }
    }

    /// Reads the metadata at the beginning of the WZ file
    pub fn from_reader<R>(mut reader: R) -> error::Result<Metadata>
    where
        R: Read,
    {
        // Fill the readerfer and ensure there are at least 16 bytes
        let mut data = [0u8; 16];
        reader.read_exact(&mut data)?;

        // Read the identifier
        let mut identifier = [0u8; 4];
        identifier.copy_from_slice(&data[0..4]);
        if &identifier != &[0x50, 0x4b, 0x47, 0x31] {
            return Err(WzError::InvalidMetadata.into());
        }

        // Read the size
        let mut size = [0u8; 8];
        size.copy_from_slice(&data[4..12]);
        let size = u64::from_le_bytes(size);

        // Read the absolute position
        let mut absolute_position = [0u8; 4];
        absolute_position.copy_from_slice(&data[12..16]);
        let absolute_position = i32::from_le_bytes(absolute_position);
        if absolute_position.is_negative() {
            return Err(WzError::InvalidMetadata.into());
        }

        // Read the description
        let mut description = vec![0u8; (absolute_position as usize) - 17];
        reader.read_exact(&mut description)?;
        let description = CString::from(match String::from_utf8(description) {
            Ok(s) => s,
            Err(_) => return Err(WzError::InvalidMetadata.into()),
        });

        // Skip the null
        let mut skip = [0];
        reader.read_exact(&mut skip)?;

        // Read the encrypted version and bruteforce the checksum
        let mut encrypted_version = [0u8; 2];
        reader.read_exact(&mut encrypted_version)?;
        let encrypted_version = u16::from_le_bytes(encrypted_version);

        Ok(Metadata {
            identifier,
            size,
            absolute_position,
            description,
            encrypted_version,
        })
    }

    pub(crate) fn possible_versions(encrypted_version: u16) -> Vec<(u16, u32)> {
        let mut versions = Vec::new();
        for version in 1..1000 {
            let (calc_version, version_checksum) = checksum(&version.to_string());
            if calc_version == encrypted_version {
                versions.push((version, version_checksum));
            }
        }
        versions
    }
}

impl Encode for Metadata {
    /// Encodes objects
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        writer.write_all(&self.identifier)?;
        self.size.encode(writer)?;
        self.absolute_position.encode(writer)?;
        self.description.encode(writer)?;
        self.encrypted_version.encode(writer)
    }
}

#[cfg(test)]
mod tests {

    use crate::file::Metadata;
    use std::fs::File;

    #[test]
    fn v83_metadata() {
        let file = File::open("testdata/v83-base.wz").expect("error opening file");
        let metadata = Metadata::from_reader(file).expect("error reading metadata");
        assert_eq!(&metadata.identifier, &[0x50, 0x4b, 0x47, 0x31]);
        assert_eq!(metadata.size, 6480);
        assert_eq!(metadata.absolute_position, 60);
        assert_eq!(
            &metadata.description,
            "Package file v1.0 Copyright 2002 Wizet, ZMS"
        );
        assert_eq!(metadata.encrypted_version, 172);
    }

    #[test]
    fn v172_metadata() {
        let file = File::open("testdata/v172-base.wz").expect("error opening file");
        let metadata = Metadata::from_reader(file).expect("error reading metadata");
        assert_eq!(&metadata.identifier, &[0x50, 0x4b, 0x47, 0x31]);
        assert_eq!(metadata.size, 6705);
        assert_eq!(metadata.absolute_position, 60);
        assert_eq!(
            &metadata.description,
            "Package file v1.0 Copyright 2002 Wizet, ZMS"
        );
        assert_eq!(metadata.encrypted_version, 7);
    }
}
