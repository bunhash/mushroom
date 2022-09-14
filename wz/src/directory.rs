use crate::{WzError, WzErrorType, WzNode, WzObject, WzReader, WzResult, WzUnparsed};
use std::io::{Seek, SeekFrom};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WzDirectory {
    name: String,
    size: Option<u64>,
    offset: Option<u64>,
    children: Vec<usize>,
}

impl WzDirectory {
    pub fn new(name: &str) -> Self {
        WzDirectory {
            name: String::from(name),
            size: None,
            offset: None,
            children: Vec::new(),
        }
    }

    pub(crate) fn with_offset(name: &str, offset: u64) -> Self {
        WzDirectory {
            name: String::from(name),
            size: None,
            offset: Some(offset),
            children: Vec::new(),
        }
    }

    pub(crate) fn add_child(&mut self, child: usize, size: u64) {
        self.children.push(child);
        match self.size {
            Some(s) => self.size = Some(s + size),
            None => self.size = Some(size),
        }
    }

    pub fn len(&self) -> usize {
        self.children.len()
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
        /*
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
        */
        Ok(())
    }
}
