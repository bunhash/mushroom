//! XML Writer

use crate::error::Result;
use crate::io::xml::{attribute::Attribute, namespace::Namespace};
use crate::map::Cursor;
use std::{borrow::Cow, io::Write};

pub use xml::writer::*;

/// Tells the [`XmlWriter`] how to write the object
pub trait ToXml {
    /// Returns the tag name
    fn tag(&self) -> &'static str;

    /// List of key-value pairs to be used as attributes
    fn attributes(&self, name: &str) -> Vec<(String, String)>;

    /// Any raw text to be appended as a child
    fn text(&self) -> Option<&str> {
        None
    }
}

/// A very basic XML writer that does almost no validation. Use with caution. Children are written
/// as they were inserted into the [`Map`](crate::map::Map). Text is written as the first XML
/// child.
pub struct XmlWriter<W>
where
    W: Write,
{
    writer: EventWriter<W>,
}

impl<W> XmlWriter<W>
where
    W: Write,
{
    /// Creates a new [`XmlWriter`] that wraps a primitive writer.
    pub fn new(writer: W) -> Self {
        Self {
            writer: EmitterConfig::new()
                .perform_indent(true)
                .create_writer(writer),
        }
    }

    /// Creates a new [`XmlWriter`] with the privided config.
    pub fn from_config(writer: EventWriter<W>) -> Self {
        Self { writer }
    }

    pub fn into_inner(self) -> W {
        self.writer.into_inner()
    }

    /// Writes from a cursor
    pub fn write<E>(&mut self, cursor: &mut Cursor<E>) -> Result<()>
    where
        E: ToXml,
    {
        let data = cursor.get();
        let attributes = data.attributes(cursor.name());
        self.writer.write(XmlEvent::StartElement {
            name: data.tag().into(),
            attributes: Cow::Owned(
                attributes
                    .iter()
                    .map(|(key, value)| Attribute::new(key.as_str().into(), value))
                    .collect::<Vec<Attribute<'_>>>(),
            ),
            namespace: Cow::Owned(Namespace::empty()),
        })?;
        if let Some(text) = data.text() {
            self.writer.write(XmlEvent::characters(text))?;
        }
        let mut num_children = cursor.children().count();
        if num_children > 0 {
            cursor.first_child()?;
            loop {
                self.write(cursor)?;
                num_children -= 1;
                if num_children == 0 {
                    break;
                }
                cursor.next_sibling()?;
            }
            cursor.parent()?;
        }
        self.writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        io::xml::writer::{EmitterConfig, ToXml, XmlWriter},
        map::Map,
    };
    use std::io::Cursor;

    impl ToXml for i32 {
        fn tag(&self) -> &'static str {
            "int"
        }

        fn attributes(&self, name: &str) -> Vec<(String, String)> {
            vec![
                (String::from("name"), name.to_string()),
                (String::from("value"), self.to_string()),
            ]
        }
    }

    fn make_map() -> Map<i32> {
        let mut map = Map::new(String::from("n1"), 100);
        let mut cursor = map.cursor_mut();
        cursor
            .create(String::from("n1_1"), 150)
            .expect("error creating n1_1")
            .move_to("n1_1")
            .expect("error moving into n1_1")
            .create(String::from("n1_1_1"), 155)
            .expect("error creating n1_1_1")
            .create(String::from("n1_1_2"), 175)
            .expect("error creating n1_1_1")
            .move_to("n1_1_1")
            .expect("error moving into n1_1_1")
            .create(String::from("n1_1_1_1"), 255)
            .expect("error creating n1_1_1_1")
            .move_to("n1_1_1_1")
            .expect("error moving into n1_1_1_1");
        map
    }

    #[test]
    fn write_xml() {
        let map = make_map();
        let mut writer = XmlWriter::from_config(
            EmitterConfig::new()
                .write_document_declaration(false)
                .line_separator("")
                .create_writer(Cursor::new(Vec::new())),
        );
        writer.write(&mut map.cursor()).expect("error writing XML");
        let data =
            String::from_utf8(writer.into_inner().into_inner()).expect("invalid UTF-8 encoding");
        assert_eq!(
            data.as_str(),
            r#"<int name="n1" value="100"><int name="n1_1" value="150"><int name="n1_1_1" value="155"><int name="n1_1_1_1" value="255" /></int><int name="n1_1_2" value="175" /></int></int>"#
        )
    }
}
