//! WZ Archive Package types

use crate::{
    Int32,
    archive::Error,
    decode::{self, Decode, Decoder},
    encode::{Encode, Encoder, SizeHint},
};

mod content;

pub use content::{Content, ContentType};

/// Packages can hold other packages or images. The structure is as follows:
///
/// ```no_build
/// [ num_contents: Int32 ]
/// [metadata for content1]
/// [         ...         ]
/// [metadata for contentN]
/// [      content 1      ]
/// [         ...         ]
/// [      content N      ]
/// ```
///
/// Packages are allowed to be empty. Empty packages are used in Base.wz to probably to signify what
/// other WZ archives exist. It's best to treat each of the package contents as binary blobs.
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct Package {
    /// List of package contents
    pub contents: Vec<Content>,
}

impl Decode for Package {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        let num_contents = Int32::decode(decoder)?;
        if num_contents.is_negative() {
            return Err(decode::Error::length(*num_contents).into());
        }
        let num_contents = *num_contents as usize;
        let mut contents = Vec::with_capacity(num_contents);
        for _ in 0..num_contents {
            contents.push(Content::decode(decoder)?);
        }
        Ok(Self { contents })
    }
}

impl Encode for Package {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        let num_contents = Int32::from(self.contents.len() as i32);
        num_contents.encode(encoder)?;
        for content in &self.contents {
            content.encode(encoder)?;
        }
        Ok(())
    }
}

impl SizeHint for Package {
    fn size_hint(&self) -> u64 {
        Int32::from(self.contents.len() as i32).size_hint()
            + self.contents.iter().map(|c| c.size_hint()).sum::<u64>()
    }
}
