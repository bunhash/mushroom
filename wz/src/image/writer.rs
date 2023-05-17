//! WZ Image Writer

use crate::{
    error::{ImageError, Result},
    io::{Encode, WzWrite, WzWriter},
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

    pub fn add_property<S>(&mut self, path: S, property: Property) -> Result<()>
    where
        S: AsRef<Path>,
    {
        let parent = match path.as_ref().parent() {
            Some(p) => p,
            None => return Err(ImageError::Path(path.as_ref().to_string_lossy().into()).into()),
        };
        let name = match path.as_ref().file_name() {
            Some(name) => match name.to_str() {
                Some(name) => name,
                None => return Err(ImageError::Path(path.as_ref().to_string_lossy().into()).into()),
            },
            None => return Err(ImageError::Path(path.as_ref().to_string_lossy().into()).into()),
        };
        let mut cursor = self.map.cursor_mut_at(parent)?;
        cursor.create(String::from(name), property)?;
        Ok(())
    }

    pub fn save<S, E>(&mut self, path: S, encryptor: E) -> Result<()>
    where
        S: AsRef<Path>,
        E: Encryptor,
    {
        let mut writer = WzWriter::new(0, 0, BufWriter::new(File::create(path)?), encryptor);
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
        recursive_write(writer, &mut self.map.cursor())
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
        | Property::Sound(_) => encode_object(writer, Some(UolString::from(cursor.name())), cursor),
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

fn encode_object<W>(
    writer: &mut W,
    name: Option<UolString>,
    cursor: &mut Cursor<Property>,
) -> Result<()>
where
    W: WzWrite,
{
    if let Some(name) = name {
        name.encode(writer)?;
    }
    9u8.encode(writer)?;

    // Remember this position to write the size later
    let size_position = writer.position()?;
    0u32.encode(writer)?;

    // Write the objects
    match cursor.get() {
        Property::ImgDir => {
            0x73u8.encode(writer)?;
            UolString::from("Property").encode(writer)?;
            WzInt::from(cursor.children().count()).encode(writer)?;
            encode_object_children(writer, cursor)?;
        }
        Property::Canvas(val) => {
            0x73u8.encode(writer)?;
            UolString::from("Canvas").encode(writer)?;
            0u8.encode(writer)?;
            let canvas = val.clone(); // We lose the cursor when encoding children
            let num_children = cursor.children().count();
            if num_children > 0 {
                1u8.encode(writer)?;
                encode_object_children(writer, cursor)?;
            } else {
                0u8.encode(writer)?;
            }
            canvas.encode(writer)?;
        }
        Property::Convex => {
            0x73u8.encode(writer)?;
            UolString::from("Shape2D#Convex2D").encode(writer)?;
            let mut num_children = cursor.children().count();
            if num_children > 0 {
                cursor.first_child()?;
                loop {
                    // Do not write the names here
                    encode_object(writer, None, cursor)?;
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
            0x73u8.encode(writer)?;
            UolString::from("Shape2D#Vector2D").encode(writer)?;
            val.encode(writer)?;
        }
        Property::Uol(val) => {
            0x73u8.encode(writer)?;
            UolString::from("UOL").encode(writer)?;
            val.encode(writer)?;
        }
        Property::Sound(val) => {
            0x73u8.encode(writer)?;
            UolString::from("Sound_DX8").encode(writer)?;
            val.encode(writer)?;
        }
        _ => panic!("should not get here"),
    }

    // Go back and write the size
    let current_position = writer.position()?;
    writer.seek(size_position)?;
    (*current_position - *size_position).encode(writer)?;
    writer.seek(current_position)?;
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
