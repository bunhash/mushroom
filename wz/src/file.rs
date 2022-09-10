use crate::{WzError, WzReader, WzResult};
use crypto::hash::checksum;

pub struct WzFile {
    identifier: String,
    size: u64,
    start: i32,
    description: String,
    version: u16,
}

impl WzFile {
    pub fn new(version: u16) -> Self {
        WzFile {
            identifier: String::from("PKG1"),
            size: 0,
            start: 60,
            description: String::from("Package file v1.0 Copyright 2002 Wizet, ZMS"),
            version: version,
        }
    }

    pub fn load(file: &mut WzReader) -> WzResult<Self> {
        let mut wz = WzFile {
            identifier: String::new(),
            size: 0,
            start: 0,
            description: String::new(),
            version: 0,
        };

        // Ensure the identifier is PKG1, otherwise, this isn't a WZ file
        wz.identifier = match String::from_utf8(Vec::from(file.read_word()?)) {
            Ok(s) => {
                if s == "PKG1" {
                    s
                } else {
                    return Err(WzError::InvalidWz);
                }
            }
            _ => return Err(WzError::InvalidWz),
        };

        // Grab the size of the WZ contents
        wz.size = u64::from_le_bytes(file.read_long()?);

        // Grab the absolute starting position of the WZ contents
        wz.start = i32::from_le_bytes(file.read_word()?);

        // Grab the description/copyright
        wz.description = file.read_zstring()?;

        // Brute force the version
        let encrypted_version = u16::from_le_bytes(file.read_short()?);
        for real_version in 1..1000 {
            let (calc_version, _) = checksum(&real_version.to_string());
            if calc_version == encrypted_version {
                wz.version = real_version as u16;
                return Ok(wz);
            }
        }

        // Brute force failed
        Err(WzError::InvalidWz)
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn start(&self) -> i32 {
        self.start
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn version(&self) -> u16 {
        self.version
    }
}
