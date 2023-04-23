//! WZ Offset Structure

use crate::{error::Result, impl_primitive, Decode, Reader};
use core::num::Wrapping;

/// Defines a WZ-OFFSET structure and how to encode/decode it
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct WzOffset(u32);

impl_primitive!(WzOffset, u32);

impl WzOffset {
    /// Creates a WZ-OFFSET given the relavent information
    pub fn new(position: u64, value: u32, abs_pos: u32, version_checksum: u32) -> Self {
        let cur = Wrapping(position as u32);
        let abs_pos = Wrapping(abs_pos);
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
        Self(offset.0)
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
            position,
            encoded,
            reader.metadata().absolute_position,
            reader.metadata().version_checksum,
        ))
    }
}
