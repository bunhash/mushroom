#[derive(Clone, Debug, PartialEq)]
pub enum WzObjectValue {
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
    Variant,
    Long(i64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WzObject {
    name: String,
    value: WzObjectValue,
}

impl WzObject {
    pub(crate) fn new(name: &str, value: WzObjectValue) -> Self {
        WzObject {
            name: String::from(name),
            value: value,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn value(&self) -> &WzObjectValue {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut WzObjectValue {
        &mut self.value
    }
}
