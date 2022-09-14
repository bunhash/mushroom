use crate::WzDirectory;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WzUnparsed {
    pub(crate) name: String,
    pub(crate) size: u64,
    pub(crate) checksum: u32,
    pub(crate) offset: u64,
}

impl WzUnparsed {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn checksum(&self) -> u32 {
        self.checksum
    }
}

/*
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
*/
