/// WZ Node Type
///
/// A WzNode can either be a Directory and an Image
///
/// A Directory is primitive and only holds other directories or images.
///
/// Images hold different WZ-OBJECTs and can be quite large. Therefore, Images are left unparsed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WzNodeType {
    Directory,
    Image,
}

/// A WZ Node
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WzNode {
    node_type: WzNodeType,
    size: u64,
    checksum: Option<u32>,
    offset: Option<u64>,
}

impl WzNode {
    /// Returns true if the WzNode is a directory
    pub fn is_directory(&self) -> bool {
        self.node_type == WzNodeType::Directory
    }

    /// Returns true if the WzNode is an image
    pub fn is_image(&self) -> bool {
        self.node_type == WzNodeType::Image
    }

    /// Returns the size of the WzNode
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Returns the checksum of the WzNode
    pub fn checksum(&self) -> Option<u32> {
        self.checksum
    }

    /// Returns the offset where WzNode exists in the WZ file
    pub fn offset(&self) -> Option<u64> {
        self.offset
    }
}

impl WzNode {
    pub(crate) fn new(node_type: WzNodeType) -> Self {
        WzNode {
            node_type: node_type,
            size: 0,
            checksum: None,
            offset: None,
        }
    }

    pub(crate) fn from_reader(
        node_type: WzNodeType,
        size: u64,
        checksum: u32,
        offset: u64,
        ) -> Self {
        WzNode {
            node_type: node_type,
            size: size,
            offset: Some(offset),
            checksum: Some(checksum),
        }
    }

    pub(crate) fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub(crate) fn set_checksum(&mut self, checksum: u32) {
        self.checksum = Some(checksum);
    }

    pub(crate) fn set_offset(&mut self, offset: u64) {
        self.offset= Some(offset);
    }
}
