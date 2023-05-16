//! Image Property

use crate::{
    io::xml::writer::ToXml,
    types::{Canvas, Sound, UolObject, UolString, Vector, WzInt, WzLong},
};

#[derive(Debug)]
pub enum Property {
    Null,
    Short(i16),
    Int(WzInt),
    Long(WzLong),
    Float(f32),
    Double(f64),
    String(UolString),
    ImgDir,
    Canvas(Canvas),
    Convex,
    Vector(Vector),
    Uol(UolObject),
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
