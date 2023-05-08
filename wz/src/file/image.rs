//! WZ Image

use crate::{
    error::{Result, WzError},
    io::{decode, Decode, WzReader},
    map::{CursorMut, Map},
    types::{WzInt, WzLong, WzOffset, WzString},
};
use crypto::Decryptor;
use std::io::{Read, Seek};

mod canvas;
mod node;
mod sound;
mod uol;
mod vector;

pub use canvas::Canvas;
pub use node::Node;
pub use sound::Sound;
pub use uol::{UolObject, UolString};
pub use vector::Vector2D;

pub mod raw;

#[derive(Debug)]
pub struct Image {
    map: Map<Node>,
}

impl Image {
    pub fn parse<R, D>(name: &str, reader: &mut WzReader<R, D>) -> Result<Self>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let mut map = Map::new(WzString::from(name), Node::Property);
        let name = UolString::decode(reader)?;
        if name != "Property" {
            return Err(WzError::InvalidImage.into());
        }
        let property = raw::Property::decode(reader)?;
        map_property_to(&property, reader, &mut map.cursor_mut())?;
        Ok(Self { map })
    }

    pub fn map(&self) -> &Map<Node> {
        &self.map
    }
}

fn map_property_to<R, D>(
    property: &raw::Property,
    reader: &mut WzReader<R, D>,
    cursor: &mut CursorMut<Node>,
) -> Result<()>
where
    R: Read + Seek,
    D: Decryptor,
{
    for content in property.contents() {
        match &content {
            raw::ContentRef::Null { name } => {
                cursor.create(WzString::from(name.as_ref()), Node::Null)?;
            }
            raw::ContentRef::Short { name, value } => {
                cursor.create(WzString::from(name.as_ref()), Node::Short(*value))?;
            }
            raw::ContentRef::Int { name, value } => {
                cursor.create(WzString::from(name.as_ref()), Node::Int(*value))?;
            }
            raw::ContentRef::Long { name, value } => {
                cursor.create(WzString::from(name.as_ref()), Node::Long(*value))?;
            }
            raw::ContentRef::Float { name, value } => {
                cursor.create(WzString::from(name.as_ref()), Node::Float(*value))?;
            }
            raw::ContentRef::Double { name, value } => {
                cursor.create(WzString::from(name.as_ref()), Node::Double(*value))?;
            }
            raw::ContentRef::String { name, value } => {
                cursor.create(WzString::from(name.as_ref()), Node::String(value.clone()))?;
            }
            raw::ContentRef::Object { name, offset, .. } => {
                map_object_to(name.as_ref(), *offset, reader, cursor)?;
            }
        }
    }
    Ok(())
}

fn map_object_to<R, D>(
    name: &str,
    offset: WzOffset,
    reader: &mut WzReader<R, D>,
    cursor: &mut CursorMut<Node>,
) -> Result<()>
where
    R: Read + Seek,
    D: Decryptor,
{
    reader.seek(offset)?;
    let object = raw::Object::decode(reader)?;
    match &object {
        raw::Object::Property(p) => {
            cursor.create(WzString::from(name), Node::Property)?;
            cursor.move_to(name)?;
            map_property_to(p, reader, cursor)?;
            cursor.parent()?;
        }
        raw::Object::Canvas(c) => {
            cursor.create(
                WzString::from(name),
                Node::Canvas(Canvas::new(
                    c.width(),
                    c.height(),
                    c.format(),
                    c.mag_level(),
                    Vec::from(c.data()),
                )),
            )?;
            if let Some(p) = c.property() {
                cursor.move_to(name)?;
                map_property_to(p, reader, cursor)?;
                cursor.parent()?;
            }
        }
        raw::Object::Convex2D => {
            cursor.create(WzString::from(name), Node::Convex2D)?;
            cursor.move_to(name)?;
            let num_objects = WzInt::decode(reader)?;
            if num_objects.is_negative() {
                return Err(decode::Error::InvalidLength(*num_objects).into());
            }
            let num_objects = *num_objects as usize;
            for i in 0..num_objects {
                map_object_to(i.to_string().as_ref(), reader.position()?, reader, cursor)?;
            }
            cursor.parent()?;
        }
        raw::Object::Vector2D(v) => {
            cursor.create(WzString::from(name), Node::Vector2D(*v))?;
        }
        raw::Object::Uol(u) => {
            cursor.create(WzString::from(name), Node::Uol(u.clone()))?;
        }
        raw::Object::Sound(s) => {
            cursor.create(WzString::from(name), Node::Sound(s.clone()))?;
        }
    }
    Ok(())
}
