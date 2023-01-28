use crate::{WzError, WzErrorType, WzNode, WzObject, WzProperty, WzRead, WzResult};
use indextree::{Ancestors, Arena, Children, Node, NodeId, Traverse};
use std::{convert::TryFrom, io::SeekFrom, slice::Iter};

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
        let root = arena.new_node(WzObject::Directory);
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

            let name = self.name.clone();
            let type_name = reader.read_wz_uol(self.offset)?;
            self.load_object(&name, &type_name, self.offset, reader)?;

            // Mark parsed
            self.loaded = true;
        }
        Ok(())
    }

    /// Unloads an image
    pub fn unload(&mut self) -> WzResult<()> {
        if self.loaded {
            let mut arena = Arena::new();
            let root = arena.new_node(WzObject::Directory);
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
}

impl WzImage {
    fn load_object(
        &mut self,
        name: &str,
        type_name: &str,
        offset: u64,
        reader: &mut dyn WzRead,
    ) -> WzResult<()> {
        match type_name {
            "Property" => self.load_property(offset, reader)?,
            "Canvas" => {}
            "Shape2D#Convex2D" => {}
            "Shape2D#Vector2D" => {}
            "UOL" => {}
            "Sound_DX8" => {}
            _ => return Err(WzError::from(WzErrorType::InvalidImgObj)),
        }
        Ok(())
    }

    fn load_property(&mut self, offset: u64, reader: &mut dyn WzRead) -> WzResult<()> {
        let _ = reader.read_short()?; // ???
        let num_properties = reader.read_wz_int()?;
        println!("{:?}", num_properties);

        for i in 0..num_properties {
            let name = reader.read_wz_uol(offset)?;
            let prop_type = reader.read_byte()?;
            match prop_type {
                0 => {
                    let _obj = WzObject::Property(WzProperty::Null);
                }
                2 | 11 => {
                    let val = i16::from_le_bytes(reader.read_short()?);
                    let _obj = WzObject::Property(WzProperty::Short(val));
                }
                3 | 19 => {
                    let val = reader.read_wz_int()?;
                    let _obj = WzObject::Property(WzProperty::Int(val));
                }
                20 => {
                    let val = reader.read_wz_long()?;
                    let _obj = WzObject::Property(WzProperty::Long(val));
                }
                4 => {
                    let t = reader.read_byte()?;
                    let _obj = WzObject::Property(WzProperty::Float(match t {
                        0x80 => f32::from_le_bytes(reader.read_word()?),
                        _ => 0.0,
                    }));
                }
                5 => {
                    let val = f64::from_le_bytes(reader.read_long()?);
                    let _obj = WzObject::Property(WzProperty::Double(val));
                }
                8 => {
                    let val = reader.read_wz_uol(offset)?;
                    let _obj = WzObject::Property(WzProperty::String(val));
                }
                9 => {
                    let size = u32::from_le_bytes(reader.read_word()?) as u64;
                    let block_end = reader.stream_position()? + size;
                    let type_name = reader.read_wz_uol(offset)?;
                    let _obj = self.load_object(&name, &type_name, offset, reader)?;
                    if reader.stream_position()? != block_end {
                        reader.seek(SeekFrom::Start(block_end))?;
                    }
                }
                _ => {
                    println!("{:?}", prop_type);
                    return Err(WzError::from(WzErrorType::InvalidProp));
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
        let root = arena.new_node(WzObject::Directory);
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
