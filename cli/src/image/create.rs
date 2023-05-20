//! Image builder

use crate::{utils, Key};
use crypto::{KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use std::{
    fs,
    io::BufReader,
    path::{Path, PathBuf},
    str::FromStr,
};
use wz::{
    error::{ImageError, Result},
    image::Writer,
    io::{
        xml::{
            attribute::OwnedAttribute,
            reader::{EventReader, XmlEvent},
        },
        DummyEncryptor,
    },
    map::Map,
    types::{Canvas, CanvasFormat, Property, Sound, UolObject, UolString, Vector, WzInt, WzLong},
};

pub(crate) fn do_create(path: &PathBuf, directory: &str, verbose: bool, key: Key) -> Result<()> {
    // Remove the WZ archive if it exists
    utils::remove_file(path)?;
    let target = utils::file_name(path)?;
    utils::verbose!(verbose, "{}", target);
    let mut writer = Writer::from_map(map_image_from_xml(target, directory, verbose)?);
    match key {
        Key::Gms => writer.save(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV)),
        Key::Kms => writer.save(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV)),
        Key::None => writer.save(path, DummyEncryptor),
    }
}

fn map_image_from_xml<S>(img_name: &str, xml_path: S, verbose: bool) -> Result<Map<Property>>
where
    S: AsRef<Path>,
{
    let parent = utils::parent(&xml_path)?.to_path_buf();
    let mut parser = EventReader::new(BufReader::new(fs::File::open(xml_path)?));
    let mut map = Map::new(img_name.into(), Property::ImgDir);
    let mut cursor = map.cursor_mut();

    // Check to make sure the root of the image is as expected
    loop {
        match parser.next()? {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                let (name, prop) = read_start_element(&name.local_name, &attributes, &parent)?;
                if name != img_name {
                    return Err(ImageError::Name(img_name.into(), name).into());
                }
                match &prop {
                    Property::ImgDir => {}
                    _ => return Err(ImageError::ImageRoot.into()),
                }
                break;
            }
            XmlEvent::EndElement { .. } | XmlEvent::EndDocument => {
                return Err(ImageError::ImageRoot.into())
            }
            _ => {}
        }
    }

    // Do the rest of the image
    loop {
        let event = parser.next()?;
        match event {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                let (name, property) = read_start_element(&name.local_name, &attributes, &parent)?;
                cursor.create(name.clone(), property)?;
                cursor.move_to(&name)?;
                utils::verbose!(verbose, "{}", cursor.pwd());
            }
            XmlEvent::EndElement { .. } => {
                let _ = cursor.parent();
            }
            XmlEvent::EndDocument => break,
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
        $(
            let $container = $container.ok_or_else(|| ImageError::Property($name.into()))?;
         )*
    };
}

fn read_start_element<S>(
    name: &str,
    attributes: &[OwnedAttribute],
    directory: S,
) -> Result<(String, Property)>
where
    S: AsRef<Path>,
{
    match name {
        "null" => {
            map_attributes!(attributes, "name", name);
            Ok((name.into(), Property::Null))
        }
        "short" => {
            map_attributes!(attributes, "name", name, "value", value);
            let value = i16::from_str(value).map_err(|_| ImageError::Value(value.into()))?;
            Ok((name.into(), Property::Short(value)))
        }
        "int" => {
            map_attributes!(attributes, "name", name, "value", value);
            let value =
                WzInt::from(i32::from_str(value).map_err(|_| ImageError::Value(value.into()))?);
            Ok((name.into(), Property::Int(value)))
        }
        "long" => {
            map_attributes!(attributes, "name", name, "value", value);
            let value =
                WzLong::from(i64::from_str(value).map_err(|_| ImageError::Value(value.into()))?);
            Ok((name.into(), Property::Long(value)))
        }
        "float" => {
            map_attributes!(attributes, "name", name, "value", value);
            let value = f32::from_str(value).map_err(|_| ImageError::Value(value.into()))?;
            Ok((name.into(), Property::Float(value)))
        }
        "double" => {
            map_attributes!(attributes, "name", name, "value", value);
            let value = f64::from_str(value).map_err(|_| ImageError::Value(value.into()))?;
            Ok((name.into(), Property::Double(value)))
        }
        "string" => {
            map_attributes!(attributes, "name", name, "value", value);
            Ok((
                name.into(),
                Property::String(UolString::from(value.to_string())),
            ))
        }
        "imgdir" => {
            map_attributes!(attributes, "name", name);
            Ok((name.into(), Property::ImgDir))
        }
        "canvas" => {
            map_attributes!(attributes, "name", name, "format", format, "src", src);
            let format = CanvasFormat::from_int(WzInt::from(
                i32::from_str(format).map_err(|_| ImageError::Value(format.into()))?,
            ))?;
            let mut path = directory.as_ref().to_path_buf();
            path.push(src);
            let canvas = Canvas::from_image(&path, format)?;
            Ok((name.into(), Property::Canvas(canvas)))
        }
        "extended" => {
            map_attributes!(attributes, "name", name);
            Ok((name.into(), Property::Convex))
        }
        "vector" => {
            map_attributes!(attributes, "name", name, "x", x, "y", y);
            let x = WzInt::from(i32::from_str(x).map_err(|_| ImageError::Value(x.into()))?);
            let y = WzInt::from(i32::from_str(y).map_err(|_| ImageError::Value(y.into()))?);
            Ok((name.into(), Property::Vector(Vector::new(x, y))))
        }
        "uol" => {
            map_attributes!(attributes, "name", name, "value", value);
            Ok((
                name.into(),
                Property::Uol(UolObject::from(value.to_string())),
            ))
        }
        "sound" => {
            map_attributes!(attributes, "name", name, "src", src, "duration", duration);
            let duration = WzInt::from(
                i32::from_str(duration).map_err(|_| ImageError::Value(duration.into()))?,
            );
            let mut path = directory.as_ref().to_path_buf();
            path.push(src);
            let sound = Sound::from_wav(&path, duration)?;
            Ok((name.into(), Property::Sound(sound)))
        }
        n => panic!("Invalid name: `{}`", n),
    }
}
