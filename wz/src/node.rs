#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WzNodeType {
    Directory,
    Image,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WzNode {
    node_type: WzNodeType,
    name: String,
    size: u64,
    checksum: Option<u32>,
    offset: Option<u64>,
}

impl WzNode {
    pub fn is_directory(&self) -> bool {
        self.node_type == WzNodeType::Directory
    }

    pub fn is_image(&self) -> bool {
        self.node_type == WzNodeType::Image
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn checksum(&self) -> Option<u32> {
        self.checksum
    }

    pub fn offset(&self) -> Option<u64> {
        self.offset
    }
}

impl WzNode {
    pub(crate) fn new(node_type: WzNodeType, name: &str) -> Self {
        WzNode {
            node_type: node_type,
            name: String::from(name),
            size: 0,
            checksum: None,
            offset: None,
        }
    }

    pub(crate) fn from_reader(
        node_type: WzNodeType,
        name: &str,
        size: u64,
        checksum: u32,
        offset: u64,
        ) -> Self {
        WzNode {
            node_type: node_type,
            name: String::from(name),
            size: size,
            offset: Some(offset),
            checksum: Some(checksum),
        }
    }
}
