//! WZ Image Reader

use crate::{
    error::{DecodeError, ImageError, Result},
    io::{Decode, WzImageReader, WzRead, WzReader},
    map::{CursorMut, Map},
    types::{raw, Canvas, Property, WzInt, WzOffset},
};
use crypto::Decryptor;
use std::{fs::File, io::BufReader, path::Path};

/// Reads a WZ image.
#[derive(Debug)]
pub struct Reader<R>
where
    R: WzRead,
{
    inner: R,
}

impl<D> Reader<WzReader<BufReader<File>, D>>
where
    D: Decryptor,
{
    pub fn open<S>(path: S, decryptor: D) -> Result<Self>
    where
        S: AsRef<Path>,
    {
        Ok(Self {
            inner: WzReader::new(0, 0, BufReader::new(File::open(path)?), decryptor),
        })
    }
}

impl<R> Reader<R>
where
    R: WzRead,
{
    /// Creates a new WZ image reader
    pub fn new(inner: R) -> Self {
        Self { inner }
    }

    /// Maps the archive contents. The root will be named `name`
    pub fn map(&mut self, name: &str) -> Result<Map<Property>> {
        let mut map = Map::new(String::from(name), Property::ImgDir);
        let mut reader = WzImageReader::new(&mut self.inner);
        let object = raw::Object::decode(&mut reader)?;
        match &object {
            raw::Object::Property(p) => {
                map_property_to(p, &mut reader, &mut map.cursor_mut())?;
                Ok(map)
            }
            _ => Err(ImageError::ImageRoot.into()),
        }
    }

    /// Consumes the archive and returns the inner reader
    pub fn into_inner(self) -> R {
        self.inner
    }
}

fn map_property_to<R>(
    property: &raw::Property,
    reader: &mut R,
    cursor: &mut CursorMut<Property>,
) -> Result<()>
where
    R: WzRead,
{
    for content in property.contents() {
        match &content {
            raw::ContentRef::Null { name } => {
                cursor.create(String::from(name.as_ref()), Property::Null)?;
            }
            raw::ContentRef::Short { name, value } => {
                cursor.create(String::from(name.as_ref()), Property::Short(*value))?;
            }
            raw::ContentRef::Int { name, value } => {
                cursor.create(String::from(name.as_ref()), Property::Int(*value))?;
            }
            raw::ContentRef::Long { name, value } => {
                cursor.create(String::from(name.as_ref()), Property::Long(*value))?;
            }
            raw::ContentRef::Float { name, value } => {
                cursor.create(String::from(name.as_ref()), Property::Float(*value))?;
            }
            raw::ContentRef::Double { name, value } => {
                cursor.create(String::from(name.as_ref()), Property::Double(*value))?;
            }
            raw::ContentRef::String { name, value } => {
                cursor.create(String::from(name.as_ref()), Property::String(value.clone()))?;
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
    cursor: &mut CursorMut<Property>,
) -> Result<()>
where
    R: WzRead,
{
    reader.seek(offset)?;
    let object = raw::Object::decode(reader)?;
    match &object {
        raw::Object::Property(p) => {
            cursor.create(String::from(name), Property::ImgDir)?;
            cursor.move_to(name)?;
            map_property_to(p, reader, cursor)?;
            cursor.parent()?;
        }
        raw::Object::Canvas(c) => {
            cursor.create(
                String::from(name),
                Property::Canvas(Canvas::new(
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
            cursor.create(String::from(name), Property::Convex)?;
            cursor.move_to(name)?;
            let num_objects = WzInt::decode(reader)?;
            if num_objects.is_negative() {
                return Err(DecodeError::Length(*num_objects).into());
            }
            let num_objects = *num_objects as usize;
            for i in 0..num_objects {
                map_object_to(&i.to_string(), reader.position()?, reader, cursor)?;
            }
            cursor.parent()?;
        }
        raw::Object::Vector(v) => {
            cursor.create(String::from(name), Property::Vector(*v))?;
        }
        raw::Object::Uol(u) => {
            cursor.create(String::from(name), Property::Uol(u.clone()))?;
        }
        raw::Object::Sound(s) => {
            cursor.create(String::from(name), Property::Sound(s.clone()))?;
        }
    }
    Ok(())
}
