//! WZ Package types

use crate::error::{DecodeError, Result};
use crate::io::{Decode, Encode, WzRead, WzWrite};
use crate::types::WzInt;

mod content;

pub(crate) use content::{ContentRef, Metadata};

/// Packages can hold other packages or images. The structure is as follows:
///
/// ```no_build
/// [ num_contents: WzInt ]
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
pub(crate) struct Package {
    pub(crate) contents: Vec<ContentRef>,
}

impl Decode for Package {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead + ?Sized,
    {
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

impl Encode for Package {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite + ?Sized,
    {
        let num_contents = WzInt::from(self.contents.len() as i32);
        num_contents.encode(writer)?;
        for content in &self.contents {
            content.encode(writer)?;
        }
        Ok(())
    }
}
