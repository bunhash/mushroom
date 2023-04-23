//! Primitive WZ Formats

use crate::{
    error::Result,
    impl_primitive, {Decode, Reader},
};

impl Decode for i8 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        Ok(reader.read_byte()? as i8)
    }
}

impl Decode for i16 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(i16::from_le_bytes(buf))
    }
}

impl Decode for i32 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }
}

impl Decode for i64 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }
}

impl Decode for u8 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        reader.read_byte()
    }
}

impl Decode for u16 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }
}

impl Decode for u32 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
}

impl Decode for u64 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
}

impl Decode for f32 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        Ok(match reader.read_byte()? {
            0x80 => {
                let mut buf = [0u8; 4];
                reader.read_exact(&mut buf)?;
                f32::from_le_bytes(buf)
            }
            _ => 0f32,
        })
    }
}

impl Decode for f64 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }
}

/// Defines a WZ-INT structure and how to encode/decode it
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct WzInt(i32);

impl_primitive!(WzInt, i32);

impl Decode for WzInt {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let check = i8::decode(reader)?;
        Ok(Self(match check {
            i8::MIN => i32::decode(reader)?,
            _ => check as i32,
        }))
    }
}

/// Defines a WZ-LONG structure and how to encode/decode it
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct WzLong(i64);

impl_primitive!(WzLong, i64);

impl Decode for WzLong {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let check = i8::decode(reader)?;
        Ok(Self(match check {
            i8::MIN => i64::decode(reader)?,
            _ => check as i64,
        }))
    }
}
