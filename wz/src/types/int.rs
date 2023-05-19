//! WZ Int and Long Formats

use crate::{
    error::Result,
    io::{Decode, Encode, SizeHint, WzRead, WzWrite},
};
use std::ops::{Add, Deref, DerefMut, Div, Mul, Rem, Sub};

/// Defines a WZ-INT structure and how to encode/decode it
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct WzInt(i32);

impl_num!(WzInt, i32);
impl_from!(WzInt, i8, i32);
impl_from!(WzInt, i16, i32);
impl_from!(WzInt, i32, i32);
impl_from!(WzInt, i64, i32);
impl_from!(WzInt, u8, i32);
impl_from!(WzInt, u16, i32);
impl_from!(WzInt, u32, i32);
impl_from!(WzInt, u64, i32);
impl_from!(WzInt, usize, i32);

impl Decode for WzInt {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead + ?Sized,
    {
        let check = i8::decode(reader)?;
        Ok(Self(match check {
            i8::MIN => i32::decode(reader)?,
            v => v as i32,
        }))
    }
}

impl Encode for WzInt {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite + ?Sized,
    {
        if self.0 > (i8::MAX as i32) || self.0 <= (i8::MIN as i32) {
            writer.write_byte(i8::MIN as u8)?;
            self.0.encode(writer)
        } else {
            writer.write_byte(self.0 as u8)
        }
    }
}

impl SizeHint for WzInt {
    #[inline]
    fn size_hint(&self) -> u32 {
        if self.0 > (i8::MAX as i32) || self.0 <= (i8::MIN as i32) {
            5
        } else {
            1
        }
    }
}

/// Defines a WZ-LONG structure and how to encode/decode it
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct WzLong(i64);

impl_num!(WzLong, i64);
impl_from!(WzLong, i8, i64);
impl_from!(WzLong, i16, i64);
impl_from!(WzLong, i32, i64);
impl_from!(WzLong, i64, i64);
impl_from!(WzLong, u8, i64);
impl_from!(WzLong, u16, i64);
impl_from!(WzLong, u32, i64);
impl_from!(WzLong, u64, i64);
impl_from!(WzLong, usize, i64);

impl Decode for WzLong {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead + ?Sized,
    {
        let check = i8::decode(reader)?;
        Ok(Self(match check {
            i8::MIN => i64::decode(reader)?,
            v => v as i64,
        }))
    }
}

impl Encode for WzLong {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite + ?Sized,
    {
        if self.0 > (i8::MAX as i64) || self.0 <= (i8::MIN as i64) {
            writer.write_byte(i8::MIN as u8)?;
            self.0.encode(writer)
        } else {
            writer.write_byte(self.0 as u8)
        }
    }
}

impl SizeHint for WzLong {
    #[inline]
    fn size_hint(&self) -> u32 {
        if self.0 > (i8::MAX as i64) || self.0 <= (i8::MIN as i64) {
            9
        } else {
            1
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        io::{Decode, WzReader},
        types::{WzInt, WzLong},
    };
    use std::io::Cursor;

    #[test]
    fn wz_int() {
        let test1: i8 = 5;
        let test2: i16 = 15;
        let test3: i32 = 25;
        let test4: i64 = i64::MAX;

        // Test conversions
        let wz_int = WzInt::from(test1);
        assert_eq!(wz_int, WzInt::from(test1));
        let wz_int = WzInt::from(test2);
        assert_eq!(wz_int, WzInt::from(test2));
        let wz_int = WzInt::from(test3);
        assert_eq!(wz_int, WzInt::from(test3));
        let wz_int = WzInt::from(test4); // truncated to be -1
        assert_eq!(wz_int, WzInt::from(-1));

        // Test Ord
        let wz_int = WzInt::from(17);
        assert!(wz_int > WzInt::from(test1));
        assert!(wz_int > WzInt::from(test2));
        assert!(wz_int < WzInt::from(test3));
        assert!(wz_int > WzInt::from(test4));
    }

    #[test]
    fn calculate_with_wz_int() {
        let i1 = 5;
        let i2 = 7;

        let wz_int1 = WzInt::from(i1);
        let wz_int2 = WzInt::from(i2);
        assert_eq!(wz_int1 + wz_int2, WzInt::from(i1 + i2));
        assert_eq!(wz_int2 - wz_int1, WzInt::from(i2 - i1));
    }

    #[test]
    fn decode_wz_int() {
        let short_notation = vec![0x72];
        let mut reader = WzReader::unencrypted(0, 0, Cursor::new(short_notation));
        let wz_int = WzInt::decode(&mut reader).expect("error reading from cursor");
        assert_eq!(wz_int, 0x72);

        let long_notation = vec![(i8::MIN as u8), 1, 1, 0, 0];
        let mut reader = WzReader::unencrypted(0, 0, Cursor::new(long_notation));
        let wz_int = WzInt::decode(&mut reader).expect("error reading from cursor");
        assert_eq!(wz_int, 257);

        let failure = vec![(i8::MIN as u8), 1, 1];
        let mut reader = WzReader::unencrypted(0, 0, Cursor::new(failure));
        match WzInt::decode(&mut reader) {
            Ok(val) => panic!("WzInt got {}", *val),
            Err(_) => {}
        }
    }

    #[test]
    fn wz_long() {
        let test1: i8 = 5;
        let test2: i16 = 15;
        let test3: i32 = 25;
        let test4: i64 = i64::MAX;

        // Test conversions
        let wz_long = WzLong::from(test1);
        assert_eq!(wz_long, WzLong::from(test1));
        let wz_long = WzLong::from(test2);
        assert_eq!(wz_long, WzLong::from(test2));
        let wz_long = WzLong::from(test3);
        assert_eq!(wz_long, WzLong::from(test3));
        let wz_long = WzLong::from(test4);
        assert_eq!(wz_long, WzLong::from(test4));

        // Test Ord
        let wz_long = WzLong::from(17);
        assert!(wz_long > WzLong::from(test1));
        assert!(wz_long > WzLong::from(test2));
        assert!(wz_long < WzLong::from(test3));
        assert!(wz_long < WzLong::from(test4));
    }

    #[test]
    fn calculate_with_wz_long() {
        let i1 = 5;
        let i2 = 7;

        let wz_long1 = WzLong::from(i1);
        let wz_long2 = WzLong::from(i2);
        assert_eq!(wz_long1 + wz_long2, WzLong::from(i1 + i2));
        assert_eq!(wz_long2 - wz_long1, WzLong::from(i2 - i1));
    }

    #[test]
    fn decode_wz_long() {
        let short_notation = vec![0x72];
        let mut reader = WzReader::unencrypted(0, 0, Cursor::new(short_notation));
        let wz_long = WzLong::decode(&mut reader).expect("error reading from cursor");
        assert_eq!(wz_long, WzLong::from(0x72));

        let long_notation = vec![(i8::MIN as u8), 1, 1, 0, 0, 0, 0, 0, 0];
        let mut reader = WzReader::unencrypted(0, 0, Cursor::new(long_notation));
        let wz_long = WzLong::decode(&mut reader).expect("error reading from cursor");
        assert_eq!(wz_long, WzLong::from(257));

        let failure = vec![(i8::MIN as u8), 1, 1, 1, 1];
        let mut reader = WzReader::unencrypted(0, 0, Cursor::new(failure));
        match WzLong::decode(&mut reader) {
            Ok(val) => panic!("WzLong got {}", *val),
            Err(_) => {}
        }
    }
}
