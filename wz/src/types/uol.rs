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
///
/// UOLs are a URI formatted path specifyig where the "borrowed" content actually resides. This is
/// probably the most useful form of compression WZ files make use of. It can be utilized in
/// memory. I would not the same about duplicated strings.
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
            // The URI directly follows this byte
            0 | 0x73 => Ok(Uol::from(WzString::decode(reader)?.as_ref())),

            // The URI is duplicated somewhere in the image. So instead of rewriting it, this
            // points to the offset of where the first (?) occurance is. Another compression
            // mechanism.
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
        0u8.encode(writer)?;
        WzString::from(self.as_ref()).encode(writer)
    }
}

impl encode::SizeHint for Uol {
    #[inline]
    fn size_hint(&self) -> i32 {
        1 + WzString::from(self.as_ref()).size_hint()
    }
}
