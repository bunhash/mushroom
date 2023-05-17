//! Image builder

use crate::file_name;
use std::{fs, io::BufReader, path::Path};
use wz::{
    error::Result,
    io::xml::{
        attribute::OwnedAttribute,
        name::OwnedName,
        reader::{EventReader, XmlEvent},
    },
    map::{CursorMut, Map},
    types::Property,
};

pub(crate) fn map_image_from_xml<S>(name: &str, xml_path: S, verbose: bool) -> Result<Map<Property>>
where
    S: AsRef<Path>,
{
    let mut parser = EventReader::new(BufReader::new(fs::File::open(xml_path)?));
    let mut map = Map::new(name.into(), Property::ImgDir);
    let mut cursor = map.cursor_mut();
    /*
    let init = match parser.next()? {
    XmlEvent::StartElement {
    name, attributes, ..
    } => {
    let prop = read_start_element(&name.local_name, &attributes)?;
    }
    _ => panic!("invalid root"),
    };
    */
    for event in parser {
        match event? {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                read_start_element(&name.local_name, &attributes)?;
            }
            XmlEvent::EndElement { name } => {}
            _ => {}
        }
    }
    Ok(map)
}

macro_rules! map_attributes {
    ( $attrs:ident, $( $name:expr, $container:ident ),* ) => {
        $(
            let mut $container = None;
         )*
            for attr in $attrs {
                $(
                    if $name == &attr.name.local_name {
                        $container = Some(&attr.value);
                    }
                 )*
            }
    };
}

fn read_start_element(name: &str, attributes: &[OwnedAttribute]) -> Result<(String, Property)> {
    match name {
        "null" => {
            map_attributes!(attributes, "name", name);
            println!("null: {}", name.unwrap());
        }
        "short" => {
            map_attributes!(attributes, "name", name, "value", value);
            println!("short: {}({})", name.unwrap(), value.unwrap());
        }
        "int" => {
            map_attributes!(attributes, "name", name, "value", value);
            println!("int: {}({})", name.unwrap(), value.unwrap());
        }
        "long" => {
            map_attributes!(attributes, "name", name, "value", value);
            println!("long: {}({})", name.unwrap(), value.unwrap());
        }
        "float" => {
            map_attributes!(attributes, "name", name, "value", value);
            println!("float: {}({})", name.unwrap(), value.unwrap());
        }
        "double" => {
            map_attributes!(attributes, "name", name, "value", value);
            println!("double: {}({})", name.unwrap(), value.unwrap());
        }
        "string" => {
            map_attributes!(attributes, "name", name, "value", value);
            println!("string: {}({})", name.unwrap(), value.unwrap());
        }
        "imgdir" => {
            map_attributes!(attributes, "name", name);
            println!("imgdir: {}", name.unwrap());
        }
        "canvas" => {
            map_attributes!(attributes, "name", name);
            println!("canvas: {}", name.unwrap());
        }
        "extended" => {
            map_attributes!(attributes, "name", name);
            println!("extended: {}", name.unwrap());
        }
        "vector" => {
            map_attributes!(attributes, "name", name);
            println!("vector: {}", name.unwrap());
        }
        "uol" => {
            map_attributes!(attributes, "name", name);
            println!("uol: {}", name.unwrap());
        }
        "sound" => {
            map_attributes!(attributes, "name", name);
            println!("sound: {}", name.unwrap());
        }
        n => panic!("Invalid name: `{}`", n),
    }
    Ok(("".into(), Property::ImgDir))
}
