//! WZ Image Reader

use crate::{
    error::{DecodeError, ImageError, Result},
    io::{Decode, WzRead, WzReader},
    map::{CursorMut, Map},
    types::{
        image::{raw, Canvas, Node},
        WzInt, WzOffset,
    },
};
use crypto::Decryptor;
use std::{fs::File, io::BufReader, path::Path};

/// Reads a WZ image.
#[derive(Debug)]
pub struct Reader<D>
where
    D: Decryptor,
{
    inner: WzReader<BufReader<File>, D>,
}

impl<D> Reader<D>
where
    D: Decryptor,
{
    /// Opens a WZ image
    pub fn open<S>(path: S, decryptor: D) -> Result<Self>
    where
        S: AsRef<Path>,
    {
        let buf = BufReader::new(File::open(path)?);
        let inner = WzReader::new(0, 0, buf, decryptor);
        Ok(Self { inner })
    }

    /// Maps the archive contents. The root will be named `name`
    pub fn map(&mut self, name: &str) -> Result<Map<Node>> {
        let mut map = Map::new(String::from(name), Node::Property);
        let object = raw::Object::decode(&mut self.inner)?;
        match &object {
            raw::Object::Property(p) => {
                map_property_to(p, &mut self.inner, &mut map.cursor_mut())?;
                Ok(map)
            }
            _ => Err(ImageError::ImageRoot.into()),
        }
    }

    /// Consumes the archive and returns the inner reader
    pub fn into_inner(self) -> WzReader<BufReader<File>, D> {
        self.inner
    }
}

fn map_property_to<D>(
    property: &raw::Property,
    reader: &mut WzReader<BufReader<File>, D>,
    cursor: &mut CursorMut<Node>,
) -> Result<()>
where
    D: Decryptor,
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

fn map_object_to<D>(
    name: &str,
    offset: WzOffset,
    reader: &mut WzReader<BufReader<File>, D>,
    cursor: &mut CursorMut<Node>,
) -> Result<()>
where
    D: Decryptor,
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
