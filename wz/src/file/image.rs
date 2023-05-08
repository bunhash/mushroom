//! WZ Image

use crate::{
    error::{Result, WzError},
    io::{decode, encode, Decode, Encode, WzReader, WzWriter},
    map::{CursorMut, Map},
    types::{WzInt, WzLong, WzOffset, WzString},
};
use crypto::Decryptor;
use std::io::{Read, Seek};

mod canvas;
mod content;
mod object;
mod property;
mod uol;

pub use canvas::Canvas;
pub use content::ContentRef;
pub use object::Object;
pub use property::Property;
pub use uol::Uol;

#[derive(Debug)]
pub enum Node {
    Null,
    Short(i16),
    Int(WzInt),
    Long(WzLong),
    Float(f32),
    Double(f64),
    String(Uol),
    Property,
    Canvas {
        width: WzInt,
        height: WzInt,
        format: WzInt,
        mag_level: u8,
        data: Vec<u8>,
    },
    Convex2D,
    Vector2D {
        x: WzInt,
        y: WzInt,
    },
    Uol(Uol),
    Sound {
        duration: WzInt,
        header: Vec<u8>,
        data: Vec<u8>,
    },
}

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
        let name = Uol::decode(reader)?;
        if name != "Property" {
            return Err(WzError::InvalidImage.into());
        }
        let property = Property::decode(reader)?;
        map_property_to(&property, reader, &mut map.cursor_mut())?;
        Ok(Self { map })
    }

    pub fn map(&self) -> &Map<Node> {
        &self.map
    }
}

fn map_property_to<R, D>(
    property: &Property,
    reader: &mut WzReader<R, D>,
    cursor: &mut CursorMut<Node>,
) -> Result<()>
where
    R: Read + Seek,
    D: Decryptor,
{
    for content in property.contents() {
        match &content {
            ContentRef::Null { name } => {
                cursor.create(WzString::from(name.as_ref()), Node::Null)?;
            }
            ContentRef::Short { name, value } => {
                cursor.create(WzString::from(name.as_ref()), Node::Short(*value))?;
            }
            ContentRef::Int { name, value } => {
                cursor.create(WzString::from(name.as_ref()), Node::Int(*value))?;
            }
            ContentRef::Long { name, value } => {
                cursor.create(WzString::from(name.as_ref()), Node::Long(*value))?;
            }
            ContentRef::Float { name, value } => {
                cursor.create(WzString::from(name.as_ref()), Node::Float(*value))?;
            }
            ContentRef::Double { name, value } => {
                cursor.create(WzString::from(name.as_ref()), Node::Double(*value))?;
            }
            ContentRef::String { name, value } => {
                cursor.create(WzString::from(name.as_ref()), Node::String(name.clone()))?;
            }
            ContentRef::Object { name, offset, .. } => {
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
    let object = Object::decode(reader)?;
    match &object {
        Object::Property(p) => {
            cursor.create(WzString::from(name), Node::Property)?;
            cursor.move_to(name)?;
            map_property_to(p, reader, cursor)?;
            cursor.parent()?;
        }
        Object::Canvas(c) => {
            cursor.create(
                WzString::from(name),
                Node::Canvas {
                    width: c.width(),
                    height: c.height(),
                    format: c.format(),
                    mag_level: c.mag_level(),
                    data: Vec::from(c.data()),
                },
            )?;
            if let Some(p) = c.property() {
                cursor.move_to(name)?;
                map_property_to(p, reader, cursor)?;
                cursor.parent()?;
            }
        }
        Object::Convex2D => {
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
        Object::Vector2D { x, y } => {
            cursor.create(WzString::from(name), Node::Vector2D { x: *x, y: *y })?;
        }
        Object::Uol(u) => {
            cursor.create(WzString::from(name), Node::Uol(u.clone()))?;
        }
        Object::Sound {
            duration,
            header,
            data,
        } => {
            cursor.create(
                WzString::from(name),
                Node::Sound {
                    duration: *duration,
                    header: header.clone(),
                    data: data.clone(),
                },
            )?;
        }
    }
    Ok(())
}
