#[derive(Clone, Debug, PartialEq)]
pub enum WzObject {
    Directory,
    Property(WzProperty),
    Canvas,
    ShapeConvex,
    ShapeVector,
    Uol,
    Sound,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WzProperty {
    Null,
    Short(i16),
    Int(i32),
    Float(f32),
    Double(f64),
    String(String),
    Extended,
    Long(i64),
}
