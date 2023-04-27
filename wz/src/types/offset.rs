//! WZ Offset Structure

use crate::{
    error::Result, impl_conversions, map::SizeHint, types::WzInt, Decode, Encode, Reader, Writer,
};
use core::{
    num::Wrapping,
    ops::{Add, Deref, DerefMut, Sub},
};

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
        let cur = Wrapping(*position);
        let abs_pos = Wrapping(abs_pos as u32);
        let version_checksum = Wrapping(version_checksum as u32);
        let magic = Wrapping(0x581C3F6D);

        // Make decoding key (?)
        let encoding = cur - abs_pos;
        let encoding = encoding ^ Wrapping(u32::MAX);
        let encoding = encoding * version_checksum;
        let encoding = encoding - magic;
        let encoding = encoding.0.rotate_left(encoding.0 & 0x1F);
        // unwrapped

        // Decode offset
        let encoded_offset = value;
        let offset = Wrapping(encoded_offset ^ encoding);
        let offset = offset + (abs_pos * Wrapping(2));
        offset.0
    }

    fn encode_with(&self, position: u64, abs_pos: i32, version_checksum: u32) -> u32 {
        let cur = Wrapping(position as u32);
        let abs_pos = Wrapping(abs_pos as u32);
        let version_checksum = Wrapping(version_checksum as u32);
        let magic = Wrapping(0x581C3F6D);

        // Make decoding key (?)
        let encoding = cur - abs_pos;
        let encoding = encoding ^ Wrapping(u32::MAX);
        let encoding = encoding * version_checksum;
        let encoding = encoding - magic;
        let encoding = encoding.0.rotate_left(encoding.0 & 0x1F);
        // unwrapped

        // Encode offset
        let offset = Wrapping(self.0);
        let offset = offset - (abs_pos * Wrapping(2));
        let offset = offset ^ Wrapping(encoding);
        offset.0
    }
}

impl Decode for WzOffset {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let position = reader.position()?;
        let encoded = u32::decode(reader)?;
        Ok(WzOffset::new(
            encoded,
            position,
            reader.metadata().absolute_position,
            reader.metadata().version_checksum,
        ))
    }
}

impl Encode for WzOffset {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        let position = writer.position()?;
        let encoded = self.encode_with(
            position,
            writer.metadata().absolute_position,
            writer.metadata().version_checksum,
        );
        encoded.encode(writer)
    }
}

impl SizeHint for WzOffset {
    fn data_size(&self) -> WzInt {
        WzInt::from(4)
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
