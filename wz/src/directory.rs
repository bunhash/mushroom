use crate::{WzError, WzErrorType, WzNode, WzObject, WzReader, WzResult};
use std::io::{Seek, SeekFrom};

pub struct WzDirectory {
    name: String,
    size: Option<u64>,
    offset: Option<u64>,
    length: usize,
    children: Vec<WzObject>,
}

impl WzDirectory {
    pub fn new(name: &str) -> Self {
        WzDirectory {
            name: String::from(name),
            size: None,
            offset: None,
            length: 0,
            children: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl WzNode for WzDirectory {
    fn is_dir(&self) -> bool {
        true
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> Option<u64> {
        self.size
    }

    fn offset(&self) -> Option<u64> {
        self.offset
    }

    fn load(&mut self, reader: &mut WzReader, abs_pos: u64) -> WzResult<()> {
        self.offset = Some(abs_pos);
        reader.seek(SeekFrom::Start(abs_pos))?;
        let length = reader.read_wz_int()?;
        if length.is_negative() {
            return Err(WzError::from(WzErrorType::InvalidDir));
        }
        self.length = length as usize;
        for i in 0..self.length {
            match reader.read_byte()? {
                // ???
                1 => {
                    // Unknown 10 bytes
                    let mut buf = Vec::new();
                    _ = reader.read_nbytes(10, &mut buf)?;
                    /*
                    _ = reader.read_word()?; // unknown
                    _ = reader.read_short()?; // padding
                    let offset = reader.read_offset()?;
                    */
                }
                // UOL
                2 => {
                    let uol_offset = i32::from_le_bytes(reader.read_word()?);
                    // TODO
                }
                // Directory
                3 => {
                    self.children.push(WzObject {
                        dir: true,
                        name: reader.read_wz_string()?,
                        size: reader.read_wz_int()?,
                        checksum: reader.read_wz_int()?,
                        offset: u32::from_le_bytes(reader.read_word()?),
                    });
                }
                // Object
                4 => {
                    self.children.push(WzObject {
                        dir: false,
                        name: reader.read_wz_string()?,
                        size: reader.read_wz_int()?,
                        checksum: reader.read_wz_int()?,
                        offset: u32::from_le_bytes(reader.read_word()?),
                    });
                }
                _ => return Err(WzError::from(WzErrorType::InvalidType)),
            }
        }
        Ok(())
    }
}
