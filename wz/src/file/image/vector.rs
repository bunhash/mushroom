//! Parsed Vector type

use crate::{
    io::{decode, encode, xml::writer::ToXml, Decode, Encode, WzReader, WzWriter},
    types::WzInt,
};
use crypto::{Decryptor, Encryptor};
use std::io::{Read, Seek, Write};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector {
    x: WzInt,
    y: WzInt,
}

impl Vector {
    pub fn new(x: WzInt, y: WzInt) -> Self {
        Self { x, y }
    }
}

impl Decode for Vector {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        Ok(Self {
            x: WzInt::decode(reader)?,
            y: WzInt::decode(reader)?,
        })
    }
}

impl Encode for Vector {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        self.x.encode(writer)?;
        self.y.encode(writer)
    }
}

impl encode::SizeHint for Vector {
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
