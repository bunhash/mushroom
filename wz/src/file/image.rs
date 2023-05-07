//! WZ Image

use crate::{
    error::Result,
    io::{decode, encode, Decode, Encode, WzReader, WzWriter},
    map::{CursorMut, Map},
    types::{WzInt, WzLong, WzString},
};
use crypto::Decryptor;
use std::io::{Read, Seek};

mod content;
mod object;
mod property;
mod uol;

pub use content::ContentRef;
pub use object::Object;
pub use property::Property;
pub use uol::Uol;

pub enum Node {
    Null,
    Short(i16),
    Int(WzInt),
    Long(WzLong),
    Float(f32),
    Double(f64),
    String(Uol),
    Object(Object),
}

pub struct Image {
    map: Map<Node>,
}

impl Image {
    pub fn parse<R, D>(name: &str, reader: &mut WzReader<R, D>) -> Result<Self>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let mut map = Map::new(WzString::from(name), Node::Object(Object::Property));
        map_property_to(reader, &mut map.cursor_mut())?;
        Ok(Self { map })
    }
}

fn map_property_to<R, D>(reader: &mut WzReader<R, D>, cursor: &mut CursorMut<Node>) -> Result<()>
where
    R: Read + Seek,
    D: Decryptor,
{
    let property = Property::decode(reader)?;
    for content in property.contents() {
        let (name, node) = match &content {
            ContentRef::Null { name } => (WzString::from(name.as_ref()), Node::Null),
            ContentRef::Short { name, value } => {
                (WzString::from(name.as_ref()), Node::Short(*value))
            }
            ContentRef::Int { name, value } => (WzString::from(name.as_ref()), Node::Int(*value)),
            ContentRef::Long { name, value } => (WzString::from(name.as_ref()), Node::Long(*value)),
            ContentRef::Float { name, value } => {
                (WzString::from(name.as_ref()), Node::Float(*value))
            }
            ContentRef::Double { name, value } => {
                (WzString::from(name.as_ref()), Node::Double(*value))
            }
            ContentRef::String { name, value } => (
                WzString::from(name.as_ref()),
                Node::String(Uol::from(value.as_ref())),
            ),
            ContentRef::Object { name, size, offset } => {
                let name = WzString::from(name.as_ref());
                reader.seek(*offset)?;
                let object = Object::decode_with_size(reader, *size as usize)?;
                (name, Node::Object(object))
            }
        };
        cursor.create(name, node)?;
    }
    Ok(())
}
