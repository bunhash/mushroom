//! WZ Image package

use crate::{
    decode::{self, Decode, Decoder},
    encode::{Encode, Encoder, SizeHint},
    image::{Content, Error},
    Int32,
};

/// Length prefixed list of content
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Package {
    contents: Vec<Content>,
}

impl Decode for Package {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        u16::decode(decoder)?;
        let num_contents = Int32::decode(decoder)?;
        if num_contents.is_negative() {
            return Err(decode::Error::length("negative length decoding package"))?;
        }
        let num_contents = *num_contents as usize;
        let mut contents = Vec::with_capacity(num_contents);
        for _ in 0..num_contents {
            contents.push(Content::decode(decoder)?);
        }
        Ok(Self { contents })
    }
}
