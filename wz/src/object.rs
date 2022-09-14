use crate::{WzDirectory, WzError, WzErrorType, WzNode, WzReader, WzResult};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WzObjectType {
    Directory,
    Property,
    Canvas,
    ShapeConvex,
    ShapeVector,
    Uol,
    Sound,
    Unparsed,
}

impl TryFrom<&str> for WzObjectType {
    type Error = WzError;
    fn try_from(other: &str) -> Result<Self, Self::Error> {
        match other {
            "Property" => Ok(WzObjectType::Property),
            "Canvas" => Ok(WzObjectType::Canvas),
            "Shape2D#Convex2D" => Ok(WzObjectType::ShapeConvex),
            "Shape2D#Vector2D" => Ok(WzObjectType::ShapeVector),
            "UOL" => Ok(WzObjectType::Uol),
            "Sound_DX8" => Ok(WzObjectType::Sound),
            _ => Err(WzError::from(WzErrorType::InvalidType)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WzUnparsed {
    pub(crate) directory: bool,
    pub(crate) name: String,
    pub(crate) size: i32,
    pub(crate) checksum: i32,
    pub(crate) offset: u32,
}

impl WzNode for WzUnparsed {
    fn is_dir(&self) -> bool {
        self.directory
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> Option<u64> {
        Some(self.size as u64)
    }

    fn offset(&self) -> Option<u64> {
        Some(self.offset as u64)
    }

    fn load(&mut self, reader: &mut WzReader, abs_pos: u64) -> WzResult<()> {
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WzObject {
    Directory(WzDirectory),
    /*
    Property,
    Canvas,
    ShapeConvex,
    ShapeVector,
    Uol,
    Sound,
    */
    Unparsed(WzUnparsed),
}

impl WzObject {}
