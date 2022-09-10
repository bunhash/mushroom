use crate::{WzError, WzErrorType, WzReader, WzResult};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WzObjectType {
    Property,
    Canvas,
    ShapeConvex,
    ShapeVector,
    Uol,
    Sound,
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
pub struct WzObject {
    pub dir: bool,
    pub name: String,
    pub size: i32,
    pub checksum: i32,
    pub offset: u32,
}
