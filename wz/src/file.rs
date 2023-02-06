use crate::{WzError, WzErrorType, WzHeader, WzNode, WzNodeType, WzRead, WzResult};
use crypto::checksum;
use std::io::SeekFrom;
use tree::{Arena, Node, NodeError, NodeId, Traverse};

/// A structure representing a WZ file
pub struct WzFile {
    header: WzHeader,
    version: u16,
    version_checksum: u32,
    arena: Arena<WzNode>,
    root: NodeId,
}

impl WzFile {
    /// Creates an empty WzFile with the provided name and version
    pub fn new(name: &str, version: u16) -> Self {
        WzFile::with_header(name, version, WzHeader::new())
    }

    /// Creates an empty WzFile with the provided header. The size specified in the header will be
    /// overwritten.
    pub fn with_header(name: &str, version: u16, header: WzHeader) -> Self {
        let (_, version_checksum) = checksum(&version.to_string());
        let mut arena = Arena::new();
        let root = arena.new_node(String::from(name), WzNode::new(WzNodeType::Directory));
        WzFile {
            header: header,
            version: version,
            version_checksum: version_checksum,
            arena: arena,
            root: root,
        }
    }

    /// Reads from a WZ file
    pub fn from_reader(name: &str, reader: &mut dyn WzRead) -> WzResult<Self> {
        let header = WzHeader::from_reader(reader)?;
        let file_size = header.size();

        // Brute force the version
        let encrypted_version = u16::from_le_bytes(reader.read_short()?);
        for real_version in 1..1000 {
            let (calc_version, version_checksum) = checksum(&real_version.to_string());
            if calc_version == encrypted_version {
                let mut arena = Arena::new();
                let root = arena.new_node(
                    String::from(name),
                    WzNode::from_reader(
                        WzNodeType::Directory,
                        file_size,
                        0,
                        reader.stream_position()?,
                    ),
                );
                let mut wz = WzFile {
                    header: header,
                    version: real_version,
                    version_checksum: version_checksum,
                    arena: arena,
                    root: root,
                };
                wz.load_directory(root, reader)?;
                return Ok(wz);
            }
        }

        // Brute force failed
        Err(WzError::from(WzErrorType::InvalidWz))
    }

    /// Returns the header of the WzFile
    pub fn header(&self) -> &WzHeader {
        &self.header
    }

    /// Returns the version of the WzFile
    pub fn version(&self) -> u16 {
        self.version
    }

    /// Returns the version checksum of the WzFile
    pub fn version_checksum(&self) -> u32 {
        self.version_checksum
    }

    /// Returns an immutable Arena of the WZ content
    pub fn arena(&self) -> &Arena<WzNode> {
        &self.arena
    }

    /// Returns a reference to the root directory of the WZ file
    pub fn root_id(&self) -> NodeId {
        self.root
    }

    /// Returns a reference to the root directory of the WZ file
    pub fn root_name(&self) -> Option<&str> {
        self.root.name(&self.arena)
    }

    /// Returns a reference to the root directory of the WZ file
    pub fn root(&self) -> Option<&Node<WzNode>> {
        self.get(self.root)
    }

    /// Returns a mutable reference to the root directory of the WZ file
    pub fn root_mut(&mut self) -> Option<&mut Node<WzNode>> {
        self.get_mut(self.root)
    }

    /// Traverses the WZ content depth-first
    pub fn traverse(&self) -> Traverse<'_, WzNode> {
        self.root.depth_first(&self.arena)
    }

    /// Returns a Node of the WZ tree structure given the NodeId
    pub fn get(&self, index: NodeId) -> Option<&Node<WzNode>> {
        self.arena.get(index)
    }

    /// Returns a mutable Node of the WZ tree structure given the NodeId
    pub fn get_mut(&mut self, index: NodeId) -> Option<&mut Node<WzNode>> {
        self.arena.get_mut(index)
    }

    /// Given a Node, returns the NodeId
    pub fn get_node_id(&self, object: &Node<WzNode>) -> Option<NodeId> {
        self.arena.get_node_id(object)
    }

    /// Return the directory path of the NodeId
    pub fn to_path(&self, index: NodeId) -> WzResult<String> {
        let mut ancestors = Vec::new();
        for (name, _) in index.ancestors(&self.arena) {
            ancestors.insert(
                0,
                match name {
                    Some(n) => n,
                    None => return Err(WzError::from(NodeError::NotNamed)),
                },
            );
        }
        Ok(ancestors.join("/"))
    }

    /// Return a Node given the directory path
    pub fn get_from_path(&self, path: &str) -> Option<&Node<WzNode>> {
        self.arena.get(self.get_id_from_path(path)?)
    }

    /// Return a mutable Node given the directory path
    pub fn get_from_path_mut(&mut self, path: &str) -> Option<&mut Node<WzNode>> {
        self.arena.get_mut(self.get_id_from_path(path)?)
    }
}

impl WzFile {
    fn load_directory(&mut self, index: NodeId, reader: &mut dyn WzRead) -> WzResult<()> {
        // Get the node
        let node = match self.get(index) {
            Some(node) => node,
            _ => return Err(WzError::from(WzErrorType::InvalidDir)),
        };
        if !node.get()?.is_directory() {
            // The NodeId references an Image
            return Err(WzError::from(WzErrorType::InvalidDir));
        }

        // Get the WZ offset
        let offset = match node.get()?.offset() {
            Some(offset) => offset,
            None => return Err(WzError::from(WzErrorType::InvalidDir)),
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
                    let _ = i32::from_le_bytes(reader.read_word()?);
                    todo!("Grab name from other location")
                }
                // Directory or Image
                3 | 4 => reader.read_wz_string()?,
                _ => {
                    return Err(WzError::from(WzErrorType::InvalidType));
                }
            };
            let child_size = reader.read_wz_int()? as u64;
            let child_checksum = reader.read_wz_int()? as u32;
            let child_offset =
                reader.read_wz_offset(self.header.content_start(), self.version_checksum)?;
            let child_index = self.arena.new_node(
                child_name,
                WzNode::from_reader(
                    match item_type {
                        3 => WzNodeType::Directory,
                        4 => WzNodeType::Image,
                        // Should not happen
                        _ => return Err(WzError::from(WzErrorType::Unknown)),
                    },
                    child_size,
                    child_checksum,
                    child_offset as u64,
                ),
            );
            index.insert(child_index, &mut self.arena);
            if item_type == 3 && child_checksum != 0 {
                self.load_directory(child_index, reader)?;
            }
        }
        Ok(())
    }

    /// Return a Node given the directory path
    fn get_id_from_path(&self, path: &str) -> Option<NodeId> {
        let path: Vec<&str> = path.split('/').collect();
        // Don't bother if the first node isn't right
        if path[0] == self.root.name(&self.arena)? {
            let mut node = self.root;
            for name in &path[1..] {
                node = match node.get_child(name, &self.arena) {
                    Some(child) => child,
                    None => return None,
                };
            }
            return Some(node);
        } else {
            None
        }
    }
}
