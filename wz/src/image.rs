use crate::{WzError, WzErrorType, WzHeader, WzNode, WzObject, WzProperty, WzRead, WzResult};
use indextree::{Ancestors, Arena, Children, Node, NodeId, Traverse};
use std::{convert::TryFrom, io::SeekFrom, slice::Iter};

const LUA: u8 = 0x1;
const WITHOUT_OFFSET: u8 = 0x73;
const WITH_OFFSET: u8 = 0x1B;

/// A structure representing a WZ image
pub struct WzImage {
    name: String,
    size: u64,
    checksum: Option<u32>,
    offset: Option<u64>,
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
            offset: None,
            arena: arena,
            root: root,
            loaded: false,
        }
    }

    /// Loads an image
    pub fn load(&mut self, reader: &mut dyn WzRead) -> WzResult<()> {
        if !self.loaded {
            // Go to offset
            let offset = match self.offset {
                Some(offset) => offset,
                None => return Err(WzError::from(WzErrorType::InvalidImg)),
            };
            reader.seek(SeekFrom::Start(offset))?;

            // Read the header byte
            let header = reader.read_byte()?;
            match header {
                LUA => {}
                WITHOUT_OFFSET => {
                    let prop = reader.read_wz_string()?;
                    let val = u16::from_le_bytes(reader.read_short()?);
                    if prop != "Property" || val != 0 {
                        return Err(WzError::from(WzErrorType::InvalidImg));
                    }
                }
                _ => return Err(WzError::from(WzErrorType::InvalidImg)),
            }

            // Load the properties
            let num_properties = reader.read_wz_int()?;
            for i in 0..num_properties {
                let obj = self.load_property(offset, reader)?;
            }

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
    pub fn offset(&self) -> Option<u64> {
        self.offset
    }

    /// Returns the immutable arena with all of the WzImage contents
    pub fn arena(&self) -> &Arena<WzObject> {
        &self.arena
    }
}

impl WzImage {
    fn load_property(&mut self, offset: u64, reader: &mut dyn WzRead) -> WzResult<WzObject> {
        let name = reader.read_wz_uol(offset)?;
        let prop_type = reader.read_byte()?;
        Ok(match prop_type {
            0 => WzObject::Property(WzProperty::Null),
            2 | 11 => {
                let val = i16::from_le_bytes(reader.read_short()?);
                WzObject::Property(WzProperty::Short(val))
            }
            3 | 19 => {
                let val = reader.read_wz_int()?;
                WzObject::Property(WzProperty::Int(val))
            }
            20 => {
                let val = reader.read_wz_long()?;
                WzObject::Property(WzProperty::Long(val))
            }
            4 => {
                let t = reader.read_byte()?;
                WzObject::Property(WzProperty::Float(match t {
                    0 => 0.0,
                    0x80 => f32::from_le_bytes(reader.read_word()?),
                    _ => return Err(WzError::from(WzErrorType::InvalidProp)),
                }))
            }
            5 => {
                let val = f64::from_le_bytes(reader.read_long()?);
                WzObject::Property(WzProperty::Double(val))
            }
            8 => {
                let val = reader.read_wz_uol(offset)?;
                WzObject::Property(WzProperty::String(val))
            }
            9 => self.load_extended_property(reader)?,
            _ => return Err(WzError::from(WzErrorType::InvalidProp)),
        })
    }

    fn load_extended_property(&mut self, reader: &mut dyn WzRead) -> WzResult<WzObject> {
        Ok(WzObject::Property(WzProperty::Extended))
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
        Ok(WzImage {
            name: String::from(value.name()),
            size: value.size(),
            checksum: value.checksum(),
            offset: value.offset(),
            arena: arena,
            root: root,
            loaded: false,
        })
    }
}
