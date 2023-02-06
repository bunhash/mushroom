use crate::{WzCanvas, WzProperty, WzSound, WzVector};

#[derive(Clone, Debug, PartialEq)]
pub enum WzObject {
    Property(WzProperty),
    Canvas(WzCanvas),
    ShapeConvex,
    ShapeVector(WzVector),
    Uol(String),
    Sound(WzSound),
}
