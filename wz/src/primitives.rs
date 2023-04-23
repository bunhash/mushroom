//! Primitive WZ Formats

use crate::{
    error::Result,
    {Decode, Reader},
};
use core::{
    cmp::{Ordering, PartialOrd},
    ops::{Deref, DerefMut},
};

macro_rules! impl_primitive {
    ($lhs:ty, $rhs:ty ) => {
        impl PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                PartialEq::eq(&self.0, other)
            }
            #[inline]
            fn ne(&self, other: &$rhs) -> bool {
                PartialEq::ne(&self.0, other)
            }
        }

        impl PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                PartialEq::eq(self, &other.0)
            }
            #[inline]
            fn ne(&self, other: &$lhs) -> bool {
                PartialEq::ne(self, &other.0)
            }
        }

        impl PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
                PartialOrd::partial_cmp(&self.0, other)
            }
        }

        impl PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<Ordering> {
                PartialOrd::partial_cmp(self, &other.0)
            }
        }
    };
}

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
        reader.read_into(&mut buf)?;
        Ok(i16::from_le_bytes(buf))
    }
}

impl Decode for i32 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 4];
        reader.read_into(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }
}

impl Decode for i64 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 8];
        reader.read_into(&mut buf)?;
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
        reader.read_into(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }
}

impl Decode for u32 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 4];
        reader.read_into(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
}

impl Decode for u64 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 8];
        reader.read_into(&mut buf)?;
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
                reader.read_into(&mut buf)?;
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
        reader.read_into(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }
}

/// Defines a WZ-INT structure and how to encode/decode it
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct WzInt(i32);

impl From<i32> for WzInt {
    fn from(other: i32) -> Self {
        Self(other)
    }
}

impl Deref for WzInt {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WzInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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

impl From<i64> for WzLong {
    fn from(other: i64) -> Self {
        Self(other)
    }
}

impl Deref for WzLong {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WzLong {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
