//! WZ Archive Package types

use crate::{
    archive::{Content, Error},
    decode::{self, Decode, Decoder},
    Int32,
};

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
            return Err(decode::Error::length("negative length decoding package").into());
        }
        let num_contents = *num_contents as usize;
        let mut contents = Vec::with_capacity(num_contents);
        for _ in 0..num_contents {
            contents.push(Content::decode(decoder)?);
        }
        Ok(Self { contents })
    }
}
