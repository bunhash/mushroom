use crate::{WzError, WzReader, WzResult};

pub struct WzFile {
    file: WzReader,
    pub identifier: String,
    pub size: u64,
    pub start: i32,
    pub description: String,
}

impl WzFile {
    pub fn open(filename: &str) -> WzResult<Self> {
        let mut wz = WzFile {
            file: WzReader::new(filename)?,
            identifier: String::new(),
            size: 0,
            start: 0,
            description: String::new(),
        };
        wz.identifier = match String::from_utf8(Vec::from(wz.file.read_word()?)) {
            Ok(s) => {
                if s == "PKG1" {
                    s
                } else {
                    return Err(WzError::InvalidWz);
                }
            }
            _ => return Err(WzError::InvalidWz),
        };
        wz.size = u64::from_le_bytes(wz.file.read_long()?);
        wz.start = i32::from_le_bytes(wz.file.read_word()?);
        wz.description = String::from_utf8(wz.file.read_string()?)?;
        Ok(wz)
    }
}
