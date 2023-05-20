//! Image Property

use crate::{
    io::xml::writer::ToXml,
    types::{Canvas, Sound, UolObject, UolString, Vector, WzInt, WzLong},
};

/// Possible WZ image contents.
///
/// This list has flattened to include both primitive properties and more complex objects.
#[derive(Debug)]
pub enum Property {
    /// Null value
    Null,

    /// Short
    Short(i16),

    /// Int
    Int(WzInt),

    /// Long
    Long(WzLong),

    /// Float
    Float(f32),

    /// Double
    Double(f64),

    /// String
    String(UolString),

    /// A package/directory within the Image that holds more data. Called ImgDir for HaRepacker
    /// compatability
    ImgDir,

    /// Canvas
    Canvas(Canvas),

    /// Holds a list of Vector properties. The vector children are within the Map instead of here.
    /// That decision was made to make compatability with HaRepacker easier. However, it makes more
    /// sense to have this a `Vec<Vector>`.
    Convex,

    /// Holds a `(x, y)` coordiate
    Vector(Vector),

    /// A UOL object just references another object in a URI pathspec format. Another form of WZ
    /// compression.
    Uol(UolObject),

    /// Holds WAV sound data
    Sound(Sound),
}

impl ToXml for Property {
    fn tag(&self) -> &'static str {
        match &self {
            Property::Null => "null",
            Property::Short(_) => "short",
            Property::Int(_) => "int",
            Property::Long(_) => "long",
            Property::Float(_) => "float",
            Property::Double(_) => "double",
            Property::String(v) => v.tag(),
            Property::ImgDir => "imgdir",
            Property::Canvas(v) => v.tag(),
            Property::Convex => "extended",
            Property::Vector(v) => v.tag(),
            Property::Uol(v) => v.tag(),
            Property::Sound(v) => v.tag(),
        }
    }

    fn attributes(&self, name: &str) -> Vec<(String, String)> {
        match &self {
            Property::Null => vec![(String::from("name"), name.to_string())],
            Property::Short(v) => vec![
                (String::from("name"), name.to_string()),
                (String::from("value"), v.to_string()),
            ],
            Property::Int(v) => vec![
                (String::from("name"), name.to_string()),
                (String::from("value"), v.to_string()),
            ],
            Property::Long(v) => vec![
                (String::from("name"), name.to_string()),
                (String::from("value"), v.to_string()),
            ],
            Property::Float(v) => vec![
                (String::from("name"), name.to_string()),
                (String::from("value"), v.to_string()),
            ],
            Property::Double(v) => vec![
                (String::from("name"), name.to_string()),
                (String::from("value"), v.to_string()),
            ],
            Property::String(v) => v.attributes(name),
            Property::ImgDir => vec![(String::from("name"), name.to_string())],
            Property::Canvas(v) => v.attributes(name),
            Property::Convex => vec![(String::from("name"), name.to_string())],
            Property::Vector(v) => v.attributes(name),
            Property::Uol(v) => v.attributes(name),
            Property::Sound(v) => v.attributes(name),
        }
    }
}
