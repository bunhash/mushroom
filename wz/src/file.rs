use crate::{WzError, WzErrorType, WzHeader, WzNode, WzNodeType, WzRead, WzResult};
use crypto::checksum;
use std::{io::SeekFrom, slice::Iter};

pub struct WzFile {
    name: String,
    header: WzHeader,
    version: u16,
    version_checksum: u32,
    objects: Vec<WzNodeType>,
    root: usize,
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
            objects: vec![WzNodeType::Directory(WzNode::new(name))],
            root: 0,
        }
    }

    pub fn from_reader(name: &str, reader: &mut dyn WzRead) -> WzResult<Self> {
        let header = WzHeader::from_reader(reader)?;
        let file_size = header.size();

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
                    objects: vec![WzNodeType::Directory(WzNode::from_reader(
                        name,
                        file_size,
                        0,
                        reader.stream_position()?,
                        None,
                    ))],
                    root: 0,
                };
                wz.load_directory(0, reader)?;
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

    pub fn root(&self) -> Option<&WzNodeType> {
        self.objects.get(self.root)
    }

    pub fn root_mut(&mut self) -> Option<&mut WzNodeType> {
        self.objects.get_mut(self.root)
    }

    pub fn iter(&self) -> Iter<'_, WzNodeType> {
        self.objects.iter()
    }

    pub fn get_path(&self, path: &str) -> Option<&WzNodeType> {
        let path: Vec<&str> = path.split('/').collect();
        // Don't both if the first node isn't right
        if path[0] == self.name() {
            match self.get_path_internal(&path, 1, self.root) {
                Some(i) => self.objects.get(i),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn get_path_mut(&mut self, path: &str) -> Option<&mut WzNodeType> {
        let path: Vec<&str> = path.split('/').collect();
        // Don't both if the first node isn't right
        if path[0] == self.name() {
            match self.get_path_internal(&path, 1, self.root) {
                Some(i) => self.objects.get_mut(i),
                None => None,
            }
        } else {
            None
        }
    }
}

impl WzFile {
    fn load_directory(&mut self, index: usize, reader: &mut dyn WzRead) -> WzResult<()> {
        // Get the WZ offset
        let offset = match self.objects.get(index) {
            Some(WzNodeType::Directory(dir)) => match dir.offset() {
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
        for _ in 0..length {
            let item_type = reader.read_byte()?;
            let child_name = match item_type {
                // ???
                1 => {
                    // Unknown 10 bytes
                    let mut buf = Vec::new();
                    _ = reader.read_nbytes(10, &mut buf)?;
                    continue;
                }
                // UOL
                2 => {
                    // TODO: Grab name from other location
                    let _ = i32::from_le_bytes(reader.read_word()?);
                    String::from("uol")
                }
                // Directory
                3 | 4 => reader.read_wz_string()?,
                _ => {
                    return Err(WzError::from(WzErrorType::InvalidType));
                }
            };
            let child_index = self.objects.len();
            let child_size = reader.read_wz_int()? as u64;
            let child_checksum = reader.read_wz_int()? as u32;
            let child_offset =
                reader.read_wz_offset(self.header.content_start(), self.version_checksum)?;
            if item_type == 3 {
                self.objects.push(WzNodeType::Directory(WzNode::from_reader(
                    &child_name,
                    child_size,
                    child_checksum,
                    child_offset as u64,
                    Some(index),
                )));
                self.add_child(index, child_index)?;
                if child_checksum != 0 {
                    self.load_directory(child_index, reader)?;
                }
            } else {
                self.objects.push(WzNodeType::Image(WzNode::from_reader(
                    &child_name,
                    child_size,
                    child_checksum,
                    child_offset as u64,
                    Some(index),
                )));
                self.add_child(index, child_index)?;
            }
        }
        Ok(())
    }

    fn add_child(&mut self, directory: usize, child: usize) -> WzResult<()> {
        match self.objects.get_mut(directory) {
            Some(WzNodeType::Directory(dir)) => {
                dir.add_child(child);
                Ok(())
            }
            _ => Err(WzError::from(WzErrorType::InvalidDir)),
        }
    }

    // Searches the tree recursively and returns Option<usize> of the index it finds
    fn get_path_internal(
        &self,
        path: &Vec<&str>,
        search_index: usize,
        obj_index: usize,
    ) -> Option<usize> {
        // Return the result if the end of the path was reached
        if search_index >= path.len() {
            return Some(obj_index);
        }

        // Get the object name to look for
        let search: &str = match path.get(search_index) {
            Some(s) => s,
            None => return None,
        };

        // Collect the index of all the children
        let children: Vec<usize> = match self.objects.get(obj_index) {
            Some(WzNodeType::Directory(dir)) => dir.children().map(|c| c.clone()).collect(),
            _ => return None,
        };

        // Search for the correct child
        let mut found = None;
        for child_index in children {
            let name = match self.objects.get(child_index) {
                Some(WzNodeType::Directory(dir)) => dir.name(),
                Some(WzNodeType::Image(img)) => img.name(),
                None => continue,
            };
            if name == search {
                found = Some(child_index);
                break;
            }
        }

        // Return if found or not
        match found {
            Some(i) => self.get_path_internal(path, search_index + 1, i),
            None => None,
        }
    }
}
