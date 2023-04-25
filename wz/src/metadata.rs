//! WZ Metadata

use crate::{
    error::{Error, Result, PackageError},
    types::CString,
    Encode, Writer,
};
use crypto::checksum;
use std::{
    fs::File,
    io::{BufReader, ErrorKind, Read},
    path::Path,
};

/// Metadata of the WZ file
pub struct Metadata {
    /// Name of the file
    pub name: String,

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

    /// Version of the WZ file
    pub version: u16,

    /// Checksum of the WZ file version. The computation of this is defined in the [`crypto`]
    /// package.
    pub version_checksum: u32,
}

impl Metadata {
    /// Creates new metadata with default values.
    pub fn new(name: &str, version: u16) -> Self {
        let (_, version_checksum) = checksum(&version.to_string());
        Self {
            name: String::from(name),
            identifier: [0x50, 0x4b, 0x47, 0x31],
            size: 0,
            absolute_position: 60,
            description: CString::from("Package file v1.0 Copyright 2002 Wizet, ZMS"),
            version,
            version_checksum,
        }
    }

    pub fn from_file(filename: &str) -> Result<Metadata> {
        let path = Path::new(filename);
        let name = match path.file_name() {
            Some(file_name) => file_name.to_str().unwrap(), // it passed during file_name()
            None => return Err(Error::Io(ErrorKind::NotFound)),
        };
        let mut reader = BufReader::new(File::open(path)?);
        Metadata::from_reader(name, &mut reader)
    }

    /// Reads the metadata at the beginning of the WZ file
    pub(crate) fn from_reader<R>(name: &str, buf: &mut R) -> Result<Metadata>
    where
        R: Read,
    {
        // Fill the buffer and ensure there are at least 16 bytes
        let mut data = [0u8; 16];
        buf.read_exact(&mut data)?;

        // Read the identifier
        let mut identifier = [0u8; 4];
        identifier.copy_from_slice(&data[0..4]);
        if &identifier != &[0x50, 0x4b, 0x47, 0x31] {
            return Err(PackageError::InvalidPackage.into());
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
            return Err(PackageError::InvalidPackage.into());
        }

        // Read the description
        let mut description = vec![0u8; (absolute_position as usize) - 17];
        buf.read_exact(&mut description)?;
        let description = CString::from(String::from_utf8(description)?);

        // Skip the null
        let mut skip = [0];
        buf.read_exact(&mut skip)?;

        // Read the encrypted version and bruteforce the checksum
        let mut encrypted_version = [0u8; 2];
        buf.read_exact(&mut encrypted_version)?;
        let encrypted_version = u16::from_le_bytes(encrypted_version);
        let (version, version_checksum) = Metadata::bruteforce_version(encrypted_version)?;

        Ok(Metadata {
            name: String::from(name),
            identifier,
            size,
            absolute_position,
            description,
            version,
            version_checksum,
        })
    }

    pub(crate) fn bruteforce_version(encrypted_version: u16) -> Result<(u16, u32)> {
        for version in 1..1000 {
            let (calc_version, version_checksum) = checksum(&version.to_string());
            if calc_version == encrypted_version {
                return Ok((version, version_checksum));
            }
        }
        Err(PackageError::BruteForceChecksum.into())
    }
}

impl Encode for Metadata {
    /// Get the size
    fn encode_size(&self) -> u64 {
        19 + self.description.len() as u64
    }

    /// Encodes objects
    fn encode<W>(&self, _writer: &mut W) -> Result<()>
    where
        W: Writer,
        Self: Sized,
    {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {

    use crate::Metadata;

    #[test]
    fn v83_metadata() {
        let metadata = Metadata::from_file("testdata/v83-base.wz").expect("error reading metadata");
        assert_eq!(&metadata.name, "v83-base.wz");
        assert_eq!(&metadata.identifier, &[0x50, 0x4b, 0x47, 0x31]);
        assert_eq!(metadata.size, 6480);
        assert_eq!(metadata.absolute_position, 60);
        assert_eq!(
            &metadata.description,
            "Package file v1.0 Copyright 2002 Wizet, ZMS"
        );
        assert_eq!(metadata.version, 83);
        assert_eq!(metadata.version_checksum, 1876);
    }

    #[test]
    fn v172_metadata() {
        let metadata =
            Metadata::from_file("testdata/v172-base.wz").expect("error reading metadata");
        assert_eq!(&metadata.name, "v172-base.wz");
        assert_eq!(&metadata.identifier, &[0x50, 0x4b, 0x47, 0x31]);
        assert_eq!(metadata.size, 6705);
        assert_eq!(metadata.absolute_position, 60);
        assert_eq!(
            &metadata.description,
            "Package file v1.0 Copyright 2002 Wizet, ZMS"
        );
        assert_eq!(metadata.version, 176);
        assert_eq!(metadata.version_checksum, 53047);
    }
}
