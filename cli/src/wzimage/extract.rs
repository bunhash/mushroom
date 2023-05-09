//! Custom Image exporter

use std::{borrow::Cow, fs, io::Write, path::Path};
use wz::{
    error::Result,
    file::image::Node,
    io::xml::{
        attribute::Attribute,
        namespace::Namespace,
        writer::{EmitterConfig, EventWriter, ToXml, XmlEvent},
    },
    map::{Cursor, Map},
};

pub(crate) fn extract_image_from_map(map: &Map<Node>) -> Result<()> {
    let mut cursor = map.cursor();
    let mut num_children = cursor.children().count();

    if num_children > 0 {
        // Create the directory
        let image_dir = cursor.name().replace(".img", "");
        if !Path::new(&image_dir).is_dir() {
            fs::create_dir(&image_dir)?;
        }

        // Extract immediate children
        cursor.first_child()?;
        loop {
            let path = format!("{}/{}.xml", &image_dir, &cursor.name());
            if Path::new(&path).is_file() {
                fs::remove_file(&path)?;
            }
            let mut writer = EmitterConfig::new()
                .perform_indent(true)
                .create_writer(fs::File::create(&path)?);
            recursive_extract(&image_dir, &mut writer, &mut cursor)?;
            num_children = num_children - 1;
            if num_children <= 0 {
                break;
            }
            cursor.next_sibling()?;
        }
    }
    Ok(())
}

fn recursive_extract<W>(
    image_dir: &str,
    writer: &mut EventWriter<W>,
    cursor: &mut Cursor<Node>,
) -> Result<()>
where
    W: Write,
{
    let data = cursor.get();
    match &data {
        Node::Canvas(v) => {
            let res_dir = format!("{}/res", &image_dir);
            if !Path::new(&res_dir).is_dir() {
                fs::create_dir(&res_dir)?;
            }
            let res_path = format!("res/{}.png", cursor.pwd()[1..].join("-"));
            writer.write(
                XmlEvent::start_element("canvas")
                    .attr("name", cursor.name())
                    .attr("src", &res_path)
                    .attr("format", &v.format().to_string()),
            )?;
            fs::write(&format!("{}/{}", &image_dir, &res_path), v.data())?;
        }
        Node::Sound(v) => {
            let res_dir = format!("{}/res", &image_dir);
            if !Path::new(&res_dir).is_dir() {
                fs::create_dir(&res_dir)?;
            }
            let header = format!("res/{}-header.bin", cursor.pwd()[1..].join("-"));
            let data = format!("res/{}-data.bin", cursor.pwd()[1..].join("-"));
            writer.write(
                XmlEvent::start_element("sound")
                    .attr("name", cursor.name())
                    .attr("duration", &v.duration().to_string())
                    .attr("header", &header)
                    .attr("data", &data),
            )?;
            fs::write(&format!("{}/{}", &image_dir, &header), v.header())?;
            fs::write(&format!("{}/{}", &image_dir, &data), v.data())?;
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
            recursive_extract(image_dir, writer, cursor)?;
            num_children = num_children - 1;
            if num_children <= 0 {
                break;
            }
            cursor.next_sibling()?;
        }
        cursor.parent()?;
    }
    writer.write(XmlEvent::end_element())?;
    Ok(())
}
