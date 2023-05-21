//! WZ Image Property Type

use crate::error::{DecodeError, Result};
use crate::io::{Decode, WzRead};
use crate::types::raw::ContentRef;
use crate::types::WzInt;
use std::slice::Iter;

/// A property contains a list of contents--similar to package.
#[derive(Debug)]
pub struct Property {
    contents: Vec<ContentRef>,
}

impl Property {
    /// Returns an iterator over the contents
    pub fn contents(&self) -> Iter<'_, ContentRef> {
        self.contents.iter()
    }
}

impl Decode for Property {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead + ?Sized,
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
