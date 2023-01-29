use crate::{WzError, WzErrorType, WzNode, WzObject, WzObjectValue, WzProperty, WzRead, WzResult};
use indextree::{Arena, Node, NodeId, Traverse};
use std::{convert::TryFrom, io::SeekFrom};

/// A structure representing a WZ image
pub struct WzImage {
    name: String,
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
        let root = arena.new_node(WzObject::new(
            name,
            WzObjectValue::Property(WzProperty::Variant),
        ));
        WzImage {
            name: String::from(name),
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
            let mut arena = Arena::new();
            let root = arena.new_node(WzObject::new(
                self.name.as_str(),
                WzObjectValue::Property(WzProperty::Variant),
            ));
            self.arena = arena;
            self.root = root;
            self.loaded = false;
        }
        Ok(())
    }

    /// Returns the name of the WzImage
    pub fn name(&self) -> &str {
        &self.name
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
    pub fn root(&self) -> Option<&Node<WzObject>> {
        self.get(self.root)
    }

    /// Returns a mutable reference to the root "Variant" of the WZ image
    pub fn root_mut(&mut self) -> Option<&mut Node<WzObject>> {
        self.get_mut(self.root)
    }

    /// Returns an iterator of the WZ content. The order of the data is not guaranteed. It is also
    /// possible some of the content has been detached from the WZ image structure.
    pub fn iter(&self) -> impl Iterator<Item = &Node<WzObject>> {
        self.arena.iter()
    }

    /// Traverses the WZ content depth-first
    pub fn traverse(&self) -> Traverse<'_, WzObject> {
        self.root.traverse(&self.arena)
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
            println!("{}", name);
            let prop_type = reader.read_byte()?;
            let prop = match prop_type {
                0 => WzProperty::Null,
                2 | 11 => {
                    let val = i16::from_le_bytes(reader.read_short()?);
                    WzProperty::Short(val)
                }
                3 | 19 => {
                    let val = reader.read_wz_int()?;
                    WzProperty::Int(val)
                }
                20 => {
                    let val = reader.read_wz_long()?;
                    WzProperty::Long(val)
                }
                4 => {
                    let t = reader.read_byte()?;
                    WzProperty::Float(match t {
                        0x80 => f32::from_le_bytes(reader.read_word()?),
                        _ => 0.0,
                    })
                }
                5 => {
                    let val = f64::from_le_bytes(reader.read_long()?);
                    WzProperty::Double(val)
                }
                8 => {
                    let val = reader.read_wz_uol(offset)?;
                    WzProperty::String(val)
                }
                9 => WzProperty::Variant,
                _ => return Err(WzError::from(WzErrorType::InvalidProp)),
            };

            let child_index = self
                .arena
                .new_node(WzObject::new(name.as_str(), WzObjectValue::Property(prop)));
            index.append(child_index, &mut self.arena);

            if prop_type == 9 {
                // Variant
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
}

impl TryFrom<WzNode> for WzImage {
    type Error = WzError;

    fn try_from(value: WzNode) -> Result<Self, Self::Error> {
        if !value.is_image() {
            return Err(WzError::from(WzErrorType::InvalidImg));
        }
        let mut arena = Arena::new();
        let root = arena.new_node(WzObject::new(
            value.name(),
            WzObjectValue::Property(WzProperty::Variant),
        ));
        let offset = match value.offset() {
            Some(o) => o,
            None => return Err(WzError::from(WzErrorType::InvalidImg)),
        };
        Ok(WzImage {
            name: String::from(value.name()),
            size: value.size(),
            checksum: value.checksum(),
            offset: offset,
            arena: arena,
            root: root,
            loaded: false,
        })
    }
}

impl TryFrom<&WzNode> for WzImage {
    type Error = WzError;

    fn try_from(value: &WzNode) -> Result<Self, Self::Error> {
        if !value.is_image() {
            return Err(WzError::from(WzErrorType::InvalidImg));
        }
        let mut arena = Arena::new();
        let root = arena.new_node(WzObject::new(
            value.name(),
            WzObjectValue::Property(WzProperty::Variant),
        ));
        let offset = match value.offset() {
            Some(o) => o,
            None => return Err(WzError::from(WzErrorType::InvalidImg)),
        };
        Ok(WzImage {
            name: String::from(value.name()),
            size: value.size(),
            checksum: value.checksum(),
            offset: offset,
            arena: arena,
            root: root,
            loaded: false,
        })
    }
}
