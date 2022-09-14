use std::slice::Iter;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WzNodeType {
    Directory(WzNode),
    Image(WzNode),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WzNode {
    name: String,
    size: u64,
    checksum: Option<u32>,
    offset: Option<u64>,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl WzNode {
    pub fn new(name: &str) -> Self {
        WzNode {
            name: String::from(name),
            size: 0,
            checksum: None,
            offset: None,
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn offset(&self) -> Option<u64> {
        self.offset
    }

    pub fn checksum(&self) -> Option<u32> {
        self.checksum
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }
}

impl WzNode {
    pub(crate) fn from_reader(
        name: &str,
        size: u64,
        checksum: u32,
        offset: u64,
        parent: Option<usize>,
    ) -> Self {
        WzNode {
            name: String::from(name),
            size: size,
            checksum: Some(checksum),
            offset: Some(offset),
            parent: parent,
            children: Vec::new(),
        }
    }

    pub(crate) fn add_child(&mut self, child: usize) {
        self.children.push(child);
    }

    pub(crate) fn add_child_with_size(&mut self, child: usize, size: u64) {
        self.children.push(child);
        self.size = self.size + size;
    }

    pub(crate) fn children(&self) -> Iter<'_, usize> {
        self.children.iter()
    }
}
