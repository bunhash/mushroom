use crate::{WzDirectory, WzError, WzErrorType, WzHeader, WzNode, WzReader, WzResult};
use std::io::{Seek, SeekFrom};
use crypto::checksum;

pub struct WzFile {
    name: String,
    header: WzHeader,
    version: u16,
    version_checksum: u32,
    root: WzDirectory,
}

impl WzFile {
    pub fn new(name: &str, version: u16) -> Self {
        WzFile::with_header(name, version, WzHeader::new())
    }

    pub fn with_header(name: &str, version: u16, header: WzHeader) -> Self {
        let (_, version_checksum) = checksum(&version.to_string());
        WzFile {
            name: String::from(name),
            header: header,
            version: version,
            version_checksum: version_checksum,
            root: WzDirectory::new(name),
        }
    }

    pub fn from_reader(name: &str, reader: &mut WzReader) -> WzResult<Self> {
        let header = WzHeader::from_reader(reader)?;

        // Brute force the version
        let encrypted_version = u16::from_le_bytes(reader.read_short()?);
        for real_version in 1..1000 {
            let (calc_version, version_checksum) = checksum(&real_version.to_string());
            if calc_version == encrypted_version {
                let mut wz = WzFile {
                    name: String::from(name),
                    header: header,
                    version: real_version,
                    version_checksum: version_checksum,
                    root: WzDirectory::new(name),
                };
                wz.parse_root(reader)?;
                return Ok(wz);
            }
        }

        // Brute force failed
        Err(WzError::from(WzErrorType::InvalidWz))
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn header(&self) -> &WzHeader {
        &self.header
    }

    pub fn version(&self) -> u16 {
        self.version
    }

    pub fn version_checksum(&self) -> u32 {
        self.version_checksum
    }

    pub fn root(&self) -> &WzDirectory {
        &self.root
    }

    pub fn root_mut(&mut self) -> &mut WzDirectory {
        &mut self.root
    }

    fn parse_root(&mut self, reader: &mut WzReader) -> WzResult<()> {
        let abs_pos = reader.stream_position()?;
        self.root.load(reader, abs_pos)
    }
}
