use crate::{WzError, WzErrorType, WzRead, WzResult};

pub struct WzHeader {
    identifier: String,
    size: u64,
    content_start: u64,
    description: String,
}

impl WzHeader {
    pub fn new() -> Self {
        WzHeader {
            identifier: String::from("PKG1"),
            size: 0,
            content_start: 60,
            description: String::from("Package file v1.0 Copyright 2002 Wizet, ZMS"),
        }
    }

    pub fn from_reader(reader: &mut dyn WzRead) -> WzResult<Self> {
        // Ensure the identifier is PKG1, otherwise, this isn't a WZ file
        let identifier = match String::from_utf8(Vec::from(reader.read_word()?)) {
            Ok(s) => {
                if s == "PKG1" {
                    s
                } else {
                    return Err(WzError::from(WzErrorType::InvalidWz));
                }
            }
            _ => return Err(WzError::from(WzErrorType::InvalidWz)),
        };

        // Grab the size of the WZ contents
        let size = u64::from_le_bytes(reader.read_long()?);

        // Grab the absolute starting position of the WZ contents
        let start = i32::from_le_bytes(reader.read_word()?);
        if start.is_negative() {
            return Err(WzError::from(WzErrorType::InvalidWz));
        }
        let content_start = start as u64;

        // Grab the description/copyright
        let description = reader.read_cstr()?;

        Ok(WzHeader {
            identifier,
            size,
            content_start,
            description,
        })
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn content_start(&self) -> u64 {
        self.content_start
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
