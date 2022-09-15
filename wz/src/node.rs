pub struct WzNodeRef {
    pub(crate) index: usize,
}

impl From<WzNodeType> for WzNodeRef {
    fn from(other: WzNodeType) -> Self {
        WzNodeRef {
            index: match other {
                WzNodeType::Directory(dir) => dir.index,
                WzNodeType::Image(img) => img.index,
            },
        }
    }
}

impl From<&WzNodeType> for WzNodeRef {
    fn from(other: &WzNodeType) -> Self {
        WzNodeRef {
            index: match other {
                WzNodeType::Directory(dir) => dir.index,
                WzNodeType::Image(img) => img.index,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WzNodeType {
    Directory(WzNode),
    Image(WzNode),
}

impl WzNodeType {
    pub fn name(&self) -> &str {
        match self {
            WzNodeType::Directory(dir) => dir.name(),
            WzNodeType::Image(img) => img.name(),
        }
    }

    pub fn size(&self) -> u64 {
        match self {
            WzNodeType::Directory(dir) => dir.size(),
            WzNodeType::Image(img) => img.size(),
        }
    }

    pub fn offset(&self) -> Option<u64> {
        match self {
            WzNodeType::Directory(dir) => dir.offset(),
            WzNodeType::Image(img) => img.offset(),
        }
    }

    pub fn checksum(&self) -> Option<u32> {
        match self {
            WzNodeType::Directory(dir) => dir.checksum(),
            WzNodeType::Image(img) => img.checksum(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            WzNodeType::Directory(dir) => dir.len(),
            WzNodeType::Image(img) => img.len(),
        }
    }

    pub fn parent(&self) -> Option<WzNodeRef> {
        match self {
            WzNodeType::Directory(dir) => dir.parent(),
            WzNodeType::Image(img) => img.parent(),
        }
    }

    pub fn children<'a>(&'a self) -> WzNodeIterator<'a> {
        match self {
            WzNodeType::Directory(dir) => dir.children(),
            WzNodeType::Image(img) => img.children(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WzNode {
    index: usize,
    name: String,
    size: u64,
    checksum: Option<u32>,
    offset: Option<u64>,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl WzNode {
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

    pub fn parent(&self) -> Option<WzNodeRef> {
        match self.parent {
            Some(p) => Some(WzNodeRef { index: p }),
            None => None,
        }
    }

    pub fn children<'a>(&'a self) -> WzNodeIterator<'a> {
        WzNodeIterator {
            children: &self.children,
            position: 0,
        }
    }
}

impl WzNode {
    pub(crate) fn new(index: usize, name: &str) -> Self {
        WzNode {
            index: index,
            name: String::from(name),
            size: 0,
            checksum: None,
            offset: None,
            parent: None,
            children: Vec::new(),
        }
    }

    pub(crate) fn from_reader(
        index: usize,
        name: &str,
        size: u64,
        checksum: u32,
        offset: u64,
        parent: Option<usize>,
    ) -> Self {
        WzNode {
            index: index,
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
}

pub struct WzNodeIterator<'a> {
    children: &'a Vec<usize>,
    position: usize,
}

impl<'a> Iterator for WzNodeIterator<'a> {
    type Item = WzNodeRef;
    fn next(&mut self) -> Option<Self::Item> {
        match self.children.get(self.position) {
            Some(child) => {
                self.position = self.position + 1;
                Some(WzNodeRef {
                    index: child.clone(),
                })
            }
            None => None,
        }
    }
}
