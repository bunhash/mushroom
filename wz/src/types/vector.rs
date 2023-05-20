//! Parsed Vector type

use crate::{
    error::Result,
    io::{xml::writer::ToXml, Decode, Encode, SizeHint, WzRead, WzWrite},
    types::WzInt,
};

/// Vector property found in WZ images.
///
/// This is just a `(x, y)` coordinate. Typically used in Canvas objects.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector {
    pub x: WzInt,
    pub y: WzInt,
}

impl Vector {
    pub fn new(x: WzInt, y: WzInt) -> Self {
        Self { x, y }
    }
}

impl Decode for Vector {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead + ?Sized,
    {
        Ok(Self {
            x: WzInt::decode(reader)?,
            y: WzInt::decode(reader)?,
        })
    }
}

impl Encode for Vector {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite + ?Sized,
    {
        self.x.encode(writer)?;
        self.y.encode(writer)
    }
}

impl SizeHint for Vector {
    fn size_hint(&self) -> u32 {
        self.x.size_hint() + self.y.size_hint()
    }
}

impl ToXml for Vector {
    fn tag(&self) -> &'static str {
        "vector"
    }

    fn attributes(&self, name: &str) -> Vec<(String, String)> {
        vec![
            (String::from("name"), name.to_string()),
            (String::from("x"), self.x.to_string()),
            (String::from("y"), self.y.to_string()),
        ]
    }
}
