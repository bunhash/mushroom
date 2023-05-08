//! XML Writer

use crate::{error::Result, map::Cursor};
use std::io::Write;
use xml::writer::{EmitterConfig, EventWriter, XmlEvent};

/// Tells the [`XmlWriter`] how to write the object
pub trait XmlElement {
    /// Creates a start element `<foo>`. The writer keeps track of it and automates the closing
    /// tag.
    fn start_element(&self) -> XmlEvent;

    /// Any raw text to be appended as a child
    fn text(&self) -> Option<&str>;
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

    /// Writes from a cursor
    pub fn write<E>(&mut self, cursor: &mut Cursor<E>) -> Result<()>
    where
        E: XmlElement,
    {
        let data = cursor.get();
        self.writer.write(data.start_element())?;
        if let Some(text) = data.text() {
            self.writer.write(XmlEvent::characters(text))?;
        }
        let mut num_children = cursor.children().count();
        if num_children > 0 {
            cursor.first_child()?;
            loop {
                self.write(cursor)?;
                num_children = num_children - 1;
                if num_children <= 0 {
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
