//! WZ Package

use crate::{decode, encode, types::WzInt, Decode, Encode, WzReader, WzWriter};
use crypto::{Decryptor, Encryptor};
use std::{
    io::{Read, Seek, Write},
    slice::Iter,
};

mod content;

pub use content::{ContentRef, Metadata};

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
pub struct Package {
    contents: Vec<ContentRef>,
}

impl Package {
    /// Returns the number of contents
    pub fn num_contents(&self) -> usize {
        self.contents.len()
    }

    /// Returns an iterator over the contents
    pub fn contents(&self) -> Iter<'_, ContentRef> {
        self.contents.iter()
    }
}

impl Decode for Package {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
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

impl Encode for Package {
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
