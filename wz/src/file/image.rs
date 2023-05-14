//! WZ Image

use crate::{
    error::{DecodeError, ImageError, Result},
    io::{Decode, WzRead},
    map::{CursorMut, Map},
    types::{WzInt, WzOffset},
};

pub mod canvas;
mod node;
mod sound;
mod uol;
mod vector;

pub use canvas::{Canvas, CanvasFormat};
pub use node::Node;
pub use sound::Sound;
pub use uol::{UolObject, UolString};
pub use vector::Vector;

pub mod raw;

#[derive(Debug)]
pub struct Image {
    map: Map<Node>,
}

impl Image {
    pub fn parse<R>(name: &str, reader: &mut R) -> Result<Self>
    where
        R: WzRead,
    {
        let mut map = Map::new(String::from(name), Node::Property);
        let object = raw::Object::decode(reader)?;
        match &object {
            raw::Object::Property(p) => {
                map_property_to(p, reader, &mut map.cursor_mut())?;
                Ok(Self { map })
            }
            _ => Err(ImageError::ImageRoot.into()),
        }
    }

    pub fn map(&self) -> &Map<Node> {
        &self.map
    }
}

fn map_property_to<R>(
    property: &raw::Property,
    reader: &mut R,
    cursor: &mut CursorMut<Node>,
) -> Result<()>
where
    R: WzRead,
{
    for content in property.contents() {
        match &content {
            raw::ContentRef::Null { name } => {
                cursor.create(String::from(name.as_ref()), Node::Null)?;
            }
            raw::ContentRef::Short { name, value } => {
                cursor.create(String::from(name.as_ref()), Node::Short(*value))?;
            }
            raw::ContentRef::Int { name, value } => {
                cursor.create(String::from(name.as_ref()), Node::Int(*value))?;
            }
            raw::ContentRef::Long { name, value } => {
                cursor.create(String::from(name.as_ref()), Node::Long(*value))?;
            }
            raw::ContentRef::Float { name, value } => {
                cursor.create(String::from(name.as_ref()), Node::Float(*value))?;
            }
            raw::ContentRef::Double { name, value } => {
                cursor.create(String::from(name.as_ref()), Node::Double(*value))?;
            }
            raw::ContentRef::String { name, value } => {
                cursor.create(String::from(name.as_ref()), Node::String(value.clone()))?;
            }
            raw::ContentRef::Object { name, offset, .. } => {
                map_object_to(name.as_ref(), *offset, reader, cursor)?;
            }
        }
    }
    Ok(())
}

fn map_object_to<R>(
    name: &str,
    offset: WzOffset,
    reader: &mut R,
    cursor: &mut CursorMut<Node>,
) -> Result<()>
where
    R: WzRead,
{
    reader.seek(offset)?;
    let object = raw::Object::decode(reader)?;
    match &object {
        raw::Object::Property(p) => {
            cursor.create(String::from(name), Node::Property)?;
            cursor.move_to(name)?;
            map_property_to(p, reader, cursor)?;
            cursor.parent()?;
        }
        raw::Object::Canvas(c) => {
            cursor.create(
                String::from(name),
                Node::Canvas(Canvas::new(
                    c.width(),
                    c.height(),
                    c.format(),
                    Vec::from(c.data()),
                )),
            )?;
            if let Some(p) = c.property() {
                cursor.move_to(name)?;
                map_property_to(p, reader, cursor)?;
                cursor.parent()?;
            }
        }
        raw::Object::Convex => {
            cursor.create(String::from(name), Node::Convex)?;
            cursor.move_to(name)?;
            let num_objects = WzInt::decode(reader)?;
            if num_objects.is_negative() {
                return Err(DecodeError::Length(*num_objects).into());
            }
            let num_objects = *num_objects as usize;
            for i in 0..num_objects {
                map_object_to(i.to_string().as_ref(), reader.position()?, reader, cursor)?;
            }
            cursor.parent()?;
        }
        raw::Object::Vector(v) => {
            cursor.create(String::from(name), Node::Vector(*v))?;
        }
        raw::Object::Uol(u) => {
            cursor.create(String::from(name), Node::Uol(u.clone()))?;
        }
        raw::Object::Sound(s) => {
            cursor.create(String::from(name), Node::Sound(s.clone()))?;
        }
    }
    Ok(())
}
