//! Parsed Vector2D type

use crate::{
    io::{decode, encode, Decode, Encode, WzReader, WzWriter},
    types::WzInt,
};
use crypto::{Decryptor, Encryptor};
use std::io::{Read, Seek, Write};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector2D {
    x: WzInt,
    y: WzInt,
}

impl Vector2D {
    pub fn new(x: WzInt, y: WzInt) -> Self {
        Self { x, y }
    }
}

impl Decode for Vector2D {
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

impl Encode for Vector2D {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        self.x.encode(writer)?;
        self.y.encode(writer)
    }
}

impl encode::SizeHint for Vector2D {
    fn size_hint(&self) -> u32 {
        self.x.size_hint() + self.y.size_hint()
    }
}
