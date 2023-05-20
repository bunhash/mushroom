//! Image extractor

use crate::{utils, Key};
use crypto::{KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use image::ImageFormat;
use std::{borrow::Cow, fs, io::Write, path::PathBuf};
use wz::{
    error::{ImageError, Result},
    image::Reader,
    io::{
        xml::{
            attribute::Attribute,
            namespace::Namespace,
            writer::{EmitterConfig, EventWriter, ToXml, XmlEvent},
        },
        DummyDecryptor, WzRead,
    },
    map::Cursor,
    types::Property,
};

pub(crate) fn do_extract(path: &PathBuf, verbose: bool, key: Key) -> Result<()> {
    let name = utils::file_name(path)?;
    let result = match key {
        Key::Gms => extract(
            name,
            Reader::open(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
            verbose,
        ),
        Key::Kms => extract(
            name,
            Reader::open(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
            verbose,
        ),
        Key::None => extract(name, Reader::open(path, DummyDecryptor)?, verbose),
    };
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{} failed: {:?}", path.display(), e);
            Ok(())
        }
    }
}

fn extract<R>(name: &str, mut reader: Reader<R>, verbose: bool) -> Result<()>
where
    R: WzRead,
{
    let map = reader.map(name)?;
    let mut cursor = map.cursor();

    // Create the directory
    let image_dir = cursor.name().replace(".img", "");
    utils::create_dir(&image_dir)?;

    // Create the XML
    let path = format!("{}/{}.xml", &image_dir, &cursor.name());
    utils::remove_file(&path)?;
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(fs::File::create(&path)?);
    recursive_extract(&image_dir, &mut writer, &mut cursor, verbose)
}

fn recursive_extract<W>(
    image_dir: &str,
    writer: &mut EventWriter<W>,
    cursor: &mut Cursor<Property>,
    verbose: bool,
) -> Result<()>
where
    W: Write,
{
    let data = cursor.get();
    match &data {
        Property::Canvas(v) => {
            let res_dir = format!("{}/res", &image_dir);
            utils::create_dir(&res_dir)?;
            let res_path = format!(
                "res/{}.png",
                cursor
                    .pwd()
                    .strip_prefix(image_dir)
                    .ok_or_else(|| ImageError::Path(image_dir.into()))?
                    .strip_prefix(".img/")
                    .ok_or_else(|| ImageError::Path(".img/".into()))?
                    .replace('/', "-")
            );
            writer.write(
                XmlEvent::start_element("canvas")
                    .attr("name", cursor.name())
                    .attr("src", &res_path)
                    .attr("format", &v.format().to_int().to_string()),
            )?;
            let png_out = format!("{}/{}", &image_dir, &res_path);
            utils::verbose!(verbose, "{}", &png_out);
            utils::remove_file(&png_out)?;
            v.save_to_file(&png_out, ImageFormat::Png)?;
        }
        Property::Sound(v) => {
            let res_dir = format!("{}/res", &image_dir);
            utils::create_dir(&res_dir)?;
            let res_path = format!(
                "res/{}.wav",
                cursor
                    .pwd()
                    .strip_prefix(image_dir)
                    .ok_or_else(|| ImageError::Path(image_dir.into()))?
                    .strip_prefix(".img/")
                    .ok_or_else(|| ImageError::Path(".img/".into()))?
                    .replace('/', "-")
            );
            writer.write(
                XmlEvent::start_element("sound")
                    .attr("name", cursor.name())
                    .attr("src", &res_path)
                    .attr("duration", &v.duration().to_string()),
            )?;
            let wav_out = format!("{}/{}", &image_dir, &res_path);
            utils::verbose!(verbose, "{}", &wav_out);
            utils::remove_file(&wav_out)?;
            v.save_to_file(&wav_out)?;
        }
        _ => {
            let attributes = data.attributes(cursor.name());
            writer.write(XmlEvent::StartElement {
                name: data.tag().into(),
                attributes: Cow::Owned(
                    attributes
                        .iter()
                        .map(|(key, value)| Attribute::new(key.as_str().into(), value))
                        .collect::<Vec<Attribute<'_>>>(),
                ),
                namespace: Cow::Owned(Namespace::empty()),
            })?;
        }
    }
    let mut num_children = cursor.children().count();
    if num_children > 0 {
        cursor.first_child()?;
        loop {
            recursive_extract(image_dir, writer, cursor, verbose)?;
            num_children -= 1;
            if num_children == 0 {
                break;
            }
            cursor.next_sibling()?;
        }
        cursor.parent()?;
    }
    writer.write(XmlEvent::end_element())?;
    Ok(())
}
