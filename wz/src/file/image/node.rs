//! Image Node

use crate::{
    file::image::{Canvas, Sound, UolObject, UolString, Vector},
    io::xml::writer::ToXml,
    types::{WzInt, WzLong},
};

#[derive(Debug)]
pub enum Node {
    Null,
    Short(i16),
    Int(WzInt),
    Long(WzLong),
    Float(f32),
    Double(f64),
    String(UolString),
    Property,
    Canvas(Canvas),
    Convex,
    Vector(Vector),
    Uol(UolObject),
    Sound(Sound),
}

impl ToXml for Node {
    fn tag(&self) -> &'static str {
        match &self {
            Node::Null => "null",
            Node::Short(_) => "short",
            Node::Int(_) => "int",
            Node::Long(_) => "long",
            Node::Float(_) => "float",
            Node::Double(_) => "double",
            Node::String(v) => v.tag(),
            Node::Property => "imgdir",
            Node::Canvas(v) => v.tag(),
            Node::Convex => "extended",
            Node::Vector(v) => v.tag(),
            Node::Uol(v) => v.tag(),
            Node::Sound(v) => v.tag(),
        }
    }

    fn attributes(&self, name: &str) -> Vec<(String, String)> {
        match &self {
            Node::Null => vec![(String::from("name"), name.to_string())],
            Node::Short(v) => vec![
                (String::from("name"), name.to_string()),
                (String::from("value"), v.to_string()),
            ],
            Node::Int(v) => vec![
                (String::from("name"), name.to_string()),
                (String::from("value"), v.to_string()),
            ],
            Node::Long(v) => vec![
                (String::from("name"), name.to_string()),
                (String::from("value"), v.to_string()),
            ],
            Node::Float(v) => vec![
                (String::from("name"), name.to_string()),
                (String::from("value"), v.to_string()),
            ],
            Node::Double(v) => vec![
                (String::from("name"), name.to_string()),
                (String::from("value"), v.to_string()),
            ],
            Node::String(v) => v.attributes(name),
            Node::Property => vec![(String::from("name"), name.to_string())],
            Node::Canvas(v) => v.attributes(name),
            Node::Convex => vec![(String::from("name"), name.to_string())],
            Node::Vector(v) => v.attributes(name),
            Node::Uol(v) => v.attributes(name),
            Node::Sound(v) => v.attributes(name),
        }
    }
}
