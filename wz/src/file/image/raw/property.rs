//! WZ Image Property Type

use crate::{
    error::{DecodeError, Result},
    file::image::raw::ContentRef,
    io::{Decode, Encode, WzRead, WzWrite},
    types::WzInt,
};
use std::slice::Iter;

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
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead,
    {
        u16::decode(reader)?;
        let num_contents = WzInt::decode(reader)?;
        if num_contents.is_negative() {
            return Err(DecodeError::Length(*num_contents).into());
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
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite,
    {
        let num_contents = WzInt::from(self.contents.len() as i32);
        num_contents.encode(writer)?;
        for content in &self.contents {
            content.encode(writer)?;
        }
        Ok(())
    }
}
