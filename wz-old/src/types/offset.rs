//! WZ Offset Structure

use crate::error::Result;
use crate::io::{Decode, Encode, SizeHint, WzRead, WzWrite};
use crate::types::{macros, VerboseDebug};
use std::{
    io,
    ops::{Add, Deref, DerefMut, Div, Mul, Rem, Sub},
};

/// Defines a WZ offset structure and how to encode/decode it.
///
/// WZ offsets are annoying encoded offsets used in WZ archives. They do not exist within WZ images
/// unlike the other types. WZ offsets are an obfuscated position within the WZ archive. This
/// position is calculated based on the version hash and absolute position. This makes it
/// impossible to drop older WZ archives into the latest MS game data. This also means the version
/// must be known when reading or writing WZ archives. The `archive::Reader` structure offers a
/// method to bruteforce the version but it should not be relied on to work 100% of the time.
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct WzOffset(u32);

macros::impl_num!(WzOffset, u32);
macros::impl_from!(WzOffset, i8, u32);
macros::impl_from!(WzOffset, i16, u32);
macros::impl_from!(WzOffset, i32, u32);
macros::impl_from!(WzOffset, i64, u32);
macros::impl_from!(WzOffset, u8, u32);
macros::impl_from!(WzOffset, u16, u32);
macros::impl_from!(WzOffset, u32, u32);
macros::impl_from!(WzOffset, u64, u32);
macros::impl_from!(WzOffset, usize, u32);
macros::impl_debug!(WzOffset);

impl WzOffset {
    /// Creates a WZ-OFFSET given the relavent information
    pub fn new(value: u32, position: WzOffset, abs_pos: i32, version_checksum: u32) -> Self {
        Self(WzOffset::decode_from(
            value,
            position,
            abs_pos,
            version_checksum,
        ))
    }

    fn decode_from(value: u32, position: WzOffset, abs_pos: i32, version_checksum: u32) -> u32 {
        let enc_offset = *position;
        let abs_pos = abs_pos as u32;
        let magic = 0x581C3F6D;

        // Make decoding key (?)
        let enc_offset = enc_offset.wrapping_sub(abs_pos);
        let enc_offset = enc_offset ^ u32::MAX;
        let enc_offset = enc_offset.wrapping_mul(version_checksum);
        let enc_offset = enc_offset.wrapping_sub(magic);
        let enc_offset = enc_offset.rotate_left(enc_offset & 0x1F);

        // Decode offset
        let offset = value;
        let offset = offset ^ enc_offset;
        offset.wrapping_add(abs_pos.wrapping_mul(2))
    }

    fn encode_with(&self, position: WzOffset, abs_pos: i32, version_checksum: u32) -> u32 {
        let enc_offset = *position;
        let abs_pos = abs_pos as u32;
        let magic = 0x581C3F6D;

        // Make decoding key (?)
        let enc_offset = enc_offset.wrapping_sub(abs_pos);
        let enc_offset = enc_offset ^ u32::MAX;
        let enc_offset = enc_offset.wrapping_mul(version_checksum);
        let enc_offset = enc_offset.wrapping_sub(magic);
        let enc_offset = enc_offset.rotate_left(enc_offset & 0x1F);

        // Encode offset
        let offset = self.0;
        let offset = offset.wrapping_sub(abs_pos.wrapping_mul(2));
        offset ^ enc_offset
    }
}

impl Decode for WzOffset {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead + ?Sized,
    {
        let position = reader.position()?;
        let encoded = u32::decode(reader)?;
        Ok(WzOffset::new(
            encoded,
            position,
            reader.absolute_position(),
            reader.version_checksum(),
        ))
    }
}

impl Encode for WzOffset {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite + ?Sized,
    {
        let position = writer.position()?;
        let encoded = self.encode_with(
            position,
            writer.absolute_position(),
            writer.version_checksum(),
        );
        encoded.encode(writer)
    }
}

impl SizeHint for WzOffset {
    #[inline]
    fn size_hint(&self) -> u32 {
        4
    }
}

#[cfg(test)]
mod tests {

    use crate::types::WzOffset;

    #[test]
    fn wz_offset() {
        let test1: u8 = 5;
        let test2: u16 = 15;
        let test3: u32 = 25;
        let test4: u64 = u64::MAX;

        // Test conversions
        let wz_offset = WzOffset::from(test1);
        assert_eq!(wz_offset, WzOffset::from(test1));
        let wz_offset = WzOffset::from(test2);
        assert_eq!(wz_offset, WzOffset::from(test2));
        let wz_offset = WzOffset::from(test3);
        assert_eq!(wz_offset, WzOffset::from(test3));
        let wz_offset = WzOffset::from(test4); // truncated
        assert_eq!(wz_offset, WzOffset::from(u32::MAX));

        // Test Ord
        let wz_offset = WzOffset::from(17u32);
        assert!(wz_offset > WzOffset::from(test1));
        assert!(wz_offset > WzOffset::from(test2));
        assert!(wz_offset < WzOffset::from(test3));
        assert!(wz_offset < WzOffset::from(test4));
    }
}
