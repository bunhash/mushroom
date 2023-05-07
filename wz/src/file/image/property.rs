//! WZ Image Property Type

use crate::{
    file::image::ContentRef,
    io::{decode, encode, Decode, Encode, WzReader, WzWriter},
    types::WzInt,
};
use crypto::{Decryptor, Encryptor};
use std::{
    io::{Read, Seek, Write},
    slice::Iter,
};

/// A property contains a list of contents--similar to package.
#[derive(Debug)]
pub struct Property {
    contents: Vec<ContentRef>,
}

impl Property {
    /// Returns the number of contents
    pub fn num_contents(&self) -> usize {
        self.contents.len()
    }

    /// Returns an iterator over the contents
    pub fn contents(&self) -> Iter<'_, ContentRef> {
        self.contents.iter()
    }
}

impl Decode for Property {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let val = u16::decode(reader)?;
        println!("property val: {:x}", val);
        let num_contents = WzInt::decode(reader)?;
        if num_contents.is_negative() {
            return Err(decode::Error::InvalidLength(*num_contents));
        }
        let num_contents = *num_contents as usize;
        let mut contents = Vec::with_capacity(num_contents);
        for _ in 0..num_contents {
            contents.push(ContentRef::decode(reader)?);
        }
        Ok(Self { contents })
    }
}

impl Encode for Property {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        let num_contents = WzInt::from(self.contents.len() as i32);
        num_contents.encode(writer)?;
        for content in &self.contents {
            content.encode(writer)?;
        }
        Ok(())
    }
}
