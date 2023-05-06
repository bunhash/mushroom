//! WZ Offset Structure

use crate::{
    impl_conversions,
    io::{decode, encode, Decode, Encode, WzReader, WzWriter},
};
use core::ops::{Add, Deref, DerefMut, Sub};
use crypto::{Decryptor, Encryptor};
use std::io::{Read, Seek, Write};

/// Defines a WZ-OFFSET structure and how to encode/decode it
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct WzOffset(u32);

impl_primitive!(WzOffset, u32);
impl_conversions!(WzOffset, u32, u8);
impl_conversions!(WzOffset, u32, u16);
impl_conversions!(WzOffset, u32, u32);
impl_conversions!(WzOffset, u32, u64);
impl_conversions!(WzOffset, u32, i8);
impl_conversions!(WzOffset, u32, i16);
impl_conversions!(WzOffset, u32, i32);
impl_conversions!(WzOffset, u32, i64);

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
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
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
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
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

impl encode::SizeHint for WzOffset {
    #[inline]
    fn size_hint(&self) -> i32 {
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
        assert_eq!(wz_offset, test1);
        let wz_offset = WzOffset::from(test2);
        assert_eq!(wz_offset, test2);
        let wz_offset = WzOffset::from(test3);
        assert_eq!(wz_offset, test3);
        let wz_offset = WzOffset::from(test4); // truncated
        assert_eq!(wz_offset, u32::MAX);

        // Test Ord
        let wz_offset = WzOffset::from(17u32);
        assert!(wz_offset > test1);
        assert!(wz_offset > test2);
        assert!(wz_offset < test3);
        assert!(wz_offset < test4);
    }
}
