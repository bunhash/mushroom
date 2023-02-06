use crate::{WzError, WzErrorType, WzNode, WzObject, WzProperty, WzRead, WzResult};
use std::io::SeekFrom;
use tree::{Arena, Node, NodeError, NodeId, Traverse};

/// A structure representing a WZ image
pub struct WzImage {
    size: u64,
    checksum: Option<u32>,
    offset: u64,
    arena: Arena<WzObject>,
    root: NodeId,
    loaded: bool,
}

impl WzImage {
    /// Creates an empty WzFile with the provided name and version
    pub fn new(name: &str) -> Self {
        let mut arena = Arena::new();
        let root = arena.new_node(String::from(name), WzObject::Property(WzProperty::Variant));
        WzImage {
            size: 0,
            checksum: None,
            offset: 0,
            arena: arena,
            root: root,
            loaded: false,
        }
    }

    /// Reads from a WZ Image file
    pub fn from_reader(name: &str, reader: &mut dyn WzRead) -> WzResult<Self> {
        let mut img = WzImage::new(name);
        img.load(reader)?;
        Ok(img)
    }

    /// Creates an Image from a WzNode
    pub fn from_node(name: &str, node: &WzNode) -> WzResult<Self> {
        if !node.is_image() {
            return Err(WzError::from(WzErrorType::InvalidImg));
        }
        let mut arena = Arena::new();
        let root = arena.new_node(String::from(name), WzObject::Property(WzProperty::Variant));
        let offset = match node.offset() {
            Some(o) => o,
            None => return Err(WzError::from(WzErrorType::InvalidImg)),
        };
        Ok(WzImage {
            size: node.size(),
            checksum: node.checksum(),
            offset: offset,
            arena: arena,
            root: root,
            loaded: false,
        })
    }

    /// Loads an image
    pub fn load(&mut self, reader: &mut dyn WzRead) -> WzResult<()> {
        if !self.loaded {
            // Go to offset
            reader.seek(SeekFrom::Start(self.offset))?;

            self.load_object(self.root, self.offset, reader)?;

            // Mark parsed
            self.loaded = true;
        }
        Ok(())
    }

    /// Unloads an image
    pub fn unload(&mut self) -> WzResult<()> {
        if self.loaded {
            let name = match self.root.name(&self.arena) {
                Some(n) => n,
                None => return Err(WzError::from(NodeError::NotNamed)),
            };
            let mut arena = Arena::new();
            let root = arena.new_node(String::from(name), WzObject::Property(WzProperty::Variant));
            self.arena = arena;
            self.root = root;
            self.loaded = false;
        }
        Ok(())
    }

    /// Returns the size of the WzImage
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Returns the checksum of the WzImage
    pub fn checksum(&self) -> Option<u32> {
        self.checksum
    }

    /// Returns the offset where WzImage exists in the WZ file
    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// Returns the immutable arena with all of the WzImage contents
    pub fn arena(&self) -> &Arena<WzObject> {
        &self.arena
    }

    /// Returns True if the image has been loaded
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    /// Returns a reference to the root "Variant" of the WZ image
    pub fn root_id(&self) -> NodeId {
        self.root
    }

    /// Returns a reference to the root "Variant" of the WZ image
    pub fn root_name(&self) -> Option<&str> {
        self.root.name(&self.arena)
    }

    /// Returns a reference to the root "Variant" of the WZ image
    pub fn root(&self) -> Option<&Node<WzObject>> {
        self.get(self.root)
    }

    /// Returns a mutable reference to the root "Variant" of the WZ image
    pub fn root_mut(&mut self) -> Option<&mut Node<WzObject>> {
        self.get_mut(self.root)
    }

    /// Traverses the WZ content depth-first
    pub fn traverse(&self) -> Traverse<'_, WzObject> {
        self.root.depth_first(&self.arena)
    }

    /// Returns a Node of the WZ tree structure given the NodeId
    pub fn get(&self, index: NodeId) -> Option<&Node<WzObject>> {
        self.arena.get(index)
    }

    /// Returns a mutable Node of the WZ tree structure given the NodeId
    pub fn get_mut(&mut self, index: NodeId) -> Option<&mut Node<WzObject>> {
        self.arena.get_mut(index)
    }

    /// Given a Node, returns the NodeId
    pub fn get_node_id(&self, object: &Node<WzObject>) -> Option<NodeId> {
        self.arena.get_node_id(object)
    }
}

impl WzImage {
    fn load_object(&mut self, index: NodeId, offset: u64, reader: &mut dyn WzRead) -> WzResult<()> {
        let type_name = reader.read_wz_uol(self.offset)?;
        match type_name.as_str() {
            "Property" => self.load_property(index, offset, reader)?,
            "Canvas" => {}
            "Shape2D#Convex2D" => {}
            "Shape2D#Vector2D" => {}
            "UOL" => {}
            "Sound_DX8" => {}
            _ => return Err(WzError::from(WzErrorType::InvalidImgObj)),
        }
        Ok(())
    }

    fn load_property(
        &mut self,
        index: NodeId,
        offset: u64,
        reader: &mut dyn WzRead,
    ) -> WzResult<()> {
        let _ = reader.read_short()?; // ???
        let num_properties = reader.read_wz_int()?;

        for _ in 0..num_properties {
            let name = reader.read_wz_uol(offset)?;
            let prop_type = reader.read_byte()?;
            let prop = WzProperty::from_reader(prop_type, offset, reader)?;

            let child_index = self.arena.new_node(name, WzObject::Property(prop));
            index.insert(child_index, &mut self.arena);

            // Variant
            if prop_type == 9 {
                let size = u32::from_le_bytes(reader.read_word()?) as u64;
                let block_end = reader.stream_position()? + size;
                self.load_object(child_index, offset, reader)?;
                if reader.stream_position()? != block_end {
                    reader.seek(SeekFrom::Start(block_end))?;
                }
            }
        }
        Ok(())
    }

    fn load_canvas(&mut self, index: NodeId, offset: u64, reader: &mut dyn WzRead) -> WzResult<()> {
        Ok(())
    }
}
