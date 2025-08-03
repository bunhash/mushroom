//! WZ Offset Structure

use crate::{
    archive::Error,
    decode::{Decode, Decoder},
    encode::{Encode, Encoder, SizeHint},
    macros,
};
use std::fmt;

/// Defines a WZ offset structure and how to encode/decode it.
///
/// WZ offsets are annoying encoded offsets used in WZ archives. WZ offsets are an obfuscated
/// position within the WZ archive. This position is calculated based on the version checksum and
/// content start position. This makes it impossible to drop older WZ archives into the latest MS
/// game data. This also means the version must be known when reading or writing WZ archives. The
/// `archive::Reader` structure offers a method to bruteforce the version but it should not be
/// relied on to work 100% of the time.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
pub struct Offset(u32);

macros::impl_num!(Offset, u32);
macros::impl_from!(Offset, i8, u32);
macros::impl_from!(Offset, i16, u32);
macros::impl_from!(Offset, i32, u32);
macros::impl_from!(Offset, i64, u32);
macros::impl_from!(Offset, u8, u32);
macros::impl_from!(Offset, u16, u32);
macros::impl_from!(Offset, u32, u32);
macros::impl_from!(Offset, u64, u32);
macros::impl_from!(Offset, usize, u32);

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:08X}", self.0)
    }
}

impl Offset {
    pub(crate) fn decode_with(
        value: u32,
        position: u32,
        content_start: u32,
        version_checksum: u32,
    ) -> Self {
        let magic = 0x581C3F6D;

        // Make decoding key (?)
        let enc_offset = position.wrapping_sub(content_start);
        let enc_offset = enc_offset ^ u32::MAX;
        let enc_offset = enc_offset.wrapping_mul(version_checksum);
        let enc_offset = enc_offset.wrapping_sub(magic);
        let enc_offset = enc_offset.rotate_left(enc_offset & 0x1F);

        // Decode offset
        let offset = value;
        let offset = offset ^ enc_offset;
        Offset::from(offset.wrapping_add(content_start.wrapping_mul(2)))
    }

    pub(crate) fn encode_with(
        self,
        position: u32,
        content_start: u32,
        version_checksum: u32,
    ) -> u32 {
        let magic = 0x581C3F6D;

        // Make decoding key (?)
        let enc_offset = position.wrapping_sub(content_start);
        let enc_offset = enc_offset ^ u32::MAX;
        let enc_offset = enc_offset.wrapping_mul(version_checksum);
        let enc_offset = enc_offset.wrapping_sub(magic);
        let enc_offset = enc_offset.rotate_left(enc_offset & 0x1F);

        // Encode offset
        let offset = self.0;
        let offset = offset.wrapping_sub(content_start.wrapping_mul(2));
        offset ^ enc_offset
    }
}

impl Decode for Offset {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        Ok(decoder.decode_offset()?)
    }
}

impl Encode for Offset {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        Ok(encoder.encode_offset(*self)?)
    }
}

impl SizeHint for Offset {
    fn size_hint(&self) -> u64 {
        4
    }
}

#[cfg(test)]
mod tests {

    use crate::archive::Offset;

    #[test]
    fn wz_offset() {
        let test1: u8 = 5;
        let test2: u16 = 15;
        let test3: u32 = 25;
        let test4: u64 = u64::MAX;

        // Test conversions
        let wz_offset = Offset::from(test1);
        assert_eq!(wz_offset, Offset::from(test1));
        let wz_offset = Offset::from(test2);
        assert_eq!(wz_offset, Offset::from(test2));
        let wz_offset = Offset::from(test3);
        assert_eq!(wz_offset, Offset::from(test3));
        let wz_offset = Offset::from(test4); // truncated
        assert_eq!(wz_offset, Offset::from(u32::MAX));

        // Test Ord
        let wz_offset = Offset::from(17u32);
        assert!(wz_offset > Offset::from(test1));
        assert!(wz_offset > Offset::from(test2));
        assert!(wz_offset < Offset::from(test3));
        assert!(wz_offset < Offset::from(test4));
    }
}
