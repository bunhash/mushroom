//! Parsed Canvas type

use crate::{io::xml::writer::ToXml, types::WzInt};
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct Canvas {
    width: WzInt,
    height: WzInt,
    format: WzInt,
    mag_level: u8,
    data: Vec<u8>,
}

impl Canvas {
    pub fn new(width: WzInt, height: WzInt, format: WzInt, mag_level: u8, data: Vec<u8>) -> Self {
        Self {
            width,
            height,
            format,
            mag_level,
            data,
        }
    }

    pub fn width(&self) -> WzInt {
        self.width
    }

    pub fn height(&self) -> WzInt {
        self.height
    }

    pub fn format(&self) -> WzInt {
        self.format
    }

    pub fn mag_level(&self) -> u8 {
        self.mag_level
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl fmt::Debug for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Canvas {{ width: {:?}, height: {:?}, format: {:?}, mag_level: {:?}, data: [..] }}",
            self.width, self.height, self.format, self.mag_level,
        )
    }
}

impl ToXml for Canvas {
    fn tag(&self) -> &'static str {
        "canvas"
    }

    fn attributes(&self, name: &str) -> Vec<(String, String)> {
        vec![
            (String::from("name"), name.to_string()),
            (String::from("width"), self.width.to_string()),
            (String::from("height"), self.height.to_string()),
        ]
    }
}
