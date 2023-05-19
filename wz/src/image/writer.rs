//! WZ Image Writer

use crate::{
    error::{ImageError, Result},
    io::{Encode, WzImageWriter, WzWrite, WzWriter},
    map::{Cursor, Map},
    types::{Property, UolString, WzInt},
};
use crypto::Encryptor;
use std::{fs::File, io::BufWriter, path::Path};

/// Reads a WZ image.
#[derive(Debug)]
pub struct Writer {
    map: Map<Property>,
}

impl Writer {
    /// Creates a new image writer given the extracted XML (not server XML).
    pub fn new(name: &str) -> Self {
        Self {
            map: Map::new(String::from(name), Property::ImgDir),
        }
    }

    /// Creates a new image writer from an already constructed map
    pub fn from_map(map: Map<Property>) -> Self {
        Self { map }
    }

    pub fn add_property<S>(&mut self, path: S, property: Property) -> Result<()>
    where
        S: AsRef<Path>,
    {
        let parent = path
            .as_ref()
            .parent()
            .ok_or_else(|| ImageError::Path(path.as_ref().to_string_lossy().into()))?;
        let name = path
            .as_ref()
            .file_name()
            .ok_or_else(|| ImageError::Path(path.as_ref().to_string_lossy().into()))?
            .to_str()
            .ok_or_else(|| ImageError::Path(path.as_ref().to_string_lossy().into()))?;
        let mut cursor = self.map.cursor_mut_at(parent)?;
        cursor.create(String::from(name), property)?;
        Ok(())
    }

    pub fn save<S, E>(&mut self, path: S, encryptor: E) -> Result<()>
    where
        S: AsRef<Path>,
        E: Encryptor,
    {
        let mut inner = WzWriter::new(0, 0, BufWriter::new(File::create(path)?), encryptor);
        let mut writer = WzImageWriter::new(&mut inner);
        self.write_to(&mut writer)
    }

    pub fn write_to<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite,
    {
        // Images don't really need an intermediary object storage like archives do. They are far
        // easier to encode since there are no checksums to calculate and the size is always 4
        // bytes long which makes it possible to retroactively fill in. So most of the complex
        // structure encoding is done here.
        encode_object(writer, &mut self.map.cursor())
    }
}

fn recursive_write<W>(writer: &mut W, cursor: &mut Cursor<Property>) -> Result<()>
where
    W: WzWrite,
{
    let prop = cursor.get();
    match prop {
        Property::Null
        | Property::Short(_)
        | Property::Int(_)
        | Property::Long(_)
        | Property::Float(_)
        | Property::Double(_)
        | Property::String(_) => encode_property(writer, cursor.name(), prop),
        Property::ImgDir
        | Property::Canvas(_)
        | Property::Convex
        | Property::Vector(_)
        | Property::Uol(_)
        | Property::Sound(_) => {
            UolString::from(cursor.name()).encode(writer)?;
            9u8.encode(writer)?;

            // Save the size position and write 0 for now
            let size_position = writer.position()?;
            0u32.encode(writer)?;

            // Encode the object
            encode_object(writer, cursor)?;

            // Go back and write the size
            let current_position = writer.position()?;
            writer.seek(size_position)?;
            (*current_position - *size_position - 4).encode(writer)?;
            writer.seek(current_position)?;
            Ok(())
        }
    }
}

fn encode_property<W>(writer: &mut W, name: &str, property: &Property) -> Result<()>
where
    W: WzWrite,
{
    UolString::from(name).encode(writer)?;
    match property {
        Property::Null => 0u8.encode(writer),
        Property::Short(val) => {
            2u8.encode(writer)?;
            val.encode(writer)
        }
        Property::Int(val) => {
            3u8.encode(writer)?;
            val.encode(writer)
        }
        Property::Long(val) => {
            20u8.encode(writer)?;
            val.encode(writer)
        }
        Property::Float(val) => {
            4u8.encode(writer)?;
            val.encode(writer)
        }
        Property::Double(val) => {
            5u8.encode(writer)?;
            val.encode(writer)
        }
        Property::String(val) => {
            8u8.encode(writer)?;
            val.encode(writer)
        }
        _ => panic!("should not get here"),
    }
}

fn encode_object<W>(writer: &mut W, cursor: &mut Cursor<Property>) -> Result<()>
where
    W: WzWrite,
{
    match cursor.get() {
        Property::ImgDir => {
            writer.write_object_tag("Property")?;
            0u16.encode(writer)?;
            WzInt::from(cursor.children().count()).encode(writer)?;
            encode_object_children(writer, cursor)?;
        }
        Property::Canvas(val) => {
            writer.write_object_tag("Canvas")?;
            0u8.encode(writer)?;
            let canvas = val.clone(); // We lose the cursor when encoding children
            let num_children = cursor.children().count();
            if num_children > 0 {
                1u8.encode(writer)?;
                0u16.encode(writer)?;
                WzInt::from(num_children as i32).encode(writer)?;
                encode_object_children(writer, cursor)?;
            } else {
                0u8.encode(writer)?;
            }
            canvas.encode(writer)?;
        }
        Property::Convex => {
            writer.write_object_tag("Shape2D#Convex2D")?;
            let mut num_children = cursor.children().count();
            if num_children > 0 {
                cursor.first_child()?;
                loop {
                    encode_object(writer, cursor)?;
                    num_children -= 1;
                    if num_children == 0 {
                        break;
                    }
                    cursor.next_sibling()?;
                }
                cursor.parent()?;
            }
        }
        Property::Vector(val) => {
            writer.write_object_tag("Shape2D#Vector2D")?;
            val.encode(writer)?;
        }
        Property::Uol(val) => {
            writer.write_object_tag("UOL")?;
            val.encode(writer)?;
        }
        Property::Sound(val) => {
            writer.write_object_tag("Sound_DX8")?;
            val.encode(writer)?;
        }
        _ => panic!("should not get here"),
    }
    Ok(())
}

#[inline]
fn encode_object_children<W>(writer: &mut W, cursor: &mut Cursor<Property>) -> Result<()>
where
    W: WzWrite,
{
    let mut num_children = cursor.children().count();
    if num_children > 0 {
        cursor.first_child()?;
        loop {
            recursive_write(writer, cursor)?;
            num_children -= 1;
            if num_children == 0 {
                break;
            }
            cursor.next_sibling()?;
        }
        cursor.parent()?;
    }
    Ok(())
}
