//! UOL

use crate::{
    io::{decode, encode, Decode, Encode, WzReader, WzWriter},
    types::{WzOffset, WzString},
};
use crypto::{Decryptor, Encryptor};
use std::{
    io::{Read, Seek, Write},
    ops::{Deref, DerefMut},
};

/// Defines a UOL structure and how to encode/decode it
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Uol(String);

impl_str!(Uol);

impl_str_eq!(Uol, &'a String, String, &'a str, str);

impl Decode for Uol {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let check = u8::decode(reader)?;
        match check {
            0 | 0x73 => Ok(Uol::from(WzString::decode(reader)?.as_ref())),
            1 | 0x1b => {
                let pos = reader.position()?;
                let offset = WzOffset::from(u32::decode(reader)?);
                reader.seek(offset)?;
                let string = Uol::from(WzString::decode(reader)?.as_ref());
                reader.seek(pos)?;
                Ok(string)
            }
            u => Err(decode::Error::InvalidUol(u)),
        }
    }
}

impl Encode for Uol {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        unimplemented!()
    }
}

impl encode::SizeHint for Uol {
    #[inline]
    fn size_hint(&self) -> i32 {
        unimplemented!()
    }
}
