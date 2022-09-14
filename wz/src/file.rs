use crate::{
    WzDirectory, WzError, WzErrorType, WzHeader, WzNode, WzObject, WzRead, WzResult, WzUnparsed,
};
use crypto::checksum;
use std::{
    io::{Seek, SeekFrom},
    slice::Iter,
};

pub struct WzFile {
    name: String,
    header: WzHeader,
    version: u16,
    version_checksum: u32,
    objects: Vec<WzObject>,
    root: Option<usize>,
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
            objects: Vec::new(),
            root: None,
        }
    }

    pub fn from_reader(name: &str, reader: &mut dyn WzRead) -> WzResult<Self> {
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
                    objects: vec![WzObject::Directory(WzDirectory::with_offset(
                        name,
                        reader.stream_position()?,
                    ))],
                    root: Some(0),
                };
                wz.load_directory(0, reader)?;
                return Ok(wz);
            }
        }

        // Brute force failed
        Err(WzError::from(WzErrorType::InvalidWz))
    }

    fn load_directory(&mut self, index: usize, reader: &mut dyn WzRead) -> WzResult<()> {
        // Get the WZ offset
        let offset = match self.objects.get(index) {
            Some(WzObject::Directory(dir)) => match dir.offset() {
                Some(offset) => offset,
                None => return Err(WzError::from(WzErrorType::InvalidDir)),
            },
            _ => return Err(WzError::from(WzErrorType::InvalidDir)),
        };

        reader.seek(SeekFrom::Start(offset))?;
        let length = reader.read_wz_int()?;
        if length.is_negative() {
            return Err(WzError::from(WzErrorType::InvalidDir));
        }
        for i in 0..length {
            let item_type = reader.read_byte()?;
            let name = match item_type {
                // ???
                1 => {
                    // Unknown 10 bytes
                    let mut buf = Vec::new();
                    _ = reader.read_nbytes(10, &mut buf)?;
                    String::new()
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
                    String::from("uol")
                }
                // Directory
                3 | 4 => reader.read_wz_string()?,
                _ => return Err(WzError::from(WzErrorType::InvalidType)),
            };
            let child = self.objects.len();
            let size = reader.read_wz_int()?;
            let checksum = reader.read_wz_int()?;
            let offset =
                reader.read_wz_offset(self.header.content_start(), self.version_checksum)?;
            self.objects.push(WzObject::Unparsed(WzUnparsed {
                directory: item_type == 3,
                name: name,
                size: size,
                checksum: checksum,
                offset: offset,
            }));
            self.add_child(index, child, size as u64)?;
        }
        Ok(())
    }

    fn add_child(&mut self, directory: usize, child: usize, size: u64) -> WzResult<()> {
        match self.objects.get_mut(directory) {
            Some(WzObject::Directory(dir)) => {
                dir.add_child(child, size);
                Ok(())
            }
            _ => Err(WzError::from(WzErrorType::InvalidDir)),
        }
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

    pub fn root(&self) -> Option<&WzObject> {
        match self.root {
            Some(i) => Some(&self.objects[i]),
            None => None,
        }
    }

    pub fn root_mut(&mut self) -> Option<&mut WzObject> {
        match self.root {
            Some(i) => Some(&mut self.objects[i]),
            None => None,
        }
    }

    pub fn iter(&self) -> Iter<'_, WzObject> {
        self.objects.iter()
    }
}
