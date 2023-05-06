//! WZ Int and Long Formats

use crate::io::{decode, encode, Decode, Encode, WzReader, WzWriter};
use crypto::{Decryptor, Encryptor};
use std::{
    io::{Read, Seek, Write},
    ops::{Add, Deref, DerefMut, Sub},
};

/// Defines a WZ-INT structure and how to encode/decode it
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct WzInt(i32);

impl_primitive!(WzInt, i32);
impl_conversions!(WzInt, i32, i8);
impl_conversions!(WzInt, i32, i16);
impl_conversions!(WzInt, i32, i32);
impl_conversions!(WzInt, i32, i64);
impl_conversions!(WzInt, i32, u8);
impl_conversions!(WzInt, i32, u16);
impl_conversions!(WzInt, i32, u32);
impl_conversions!(WzInt, i32, u64);

impl Decode for WzInt {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let check = i8::decode(reader)?;
        Ok(Self(match check {
            i8::MIN => i32::decode(reader)?,
            _ => check as i32,
        }))
    }
}

impl Encode for WzInt {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        if self.0 > (i8::MAX as i32) || self.0 <= (i8::MIN as i32) {
            writer.write_byte(i8::MIN as u8)?;
            self.0.encode(writer)
        } else {
            writer.write_byte(self.0 as u8)
        }
    }
}

impl encode::SizeHint for WzInt {
    #[inline]
    fn size_hint(&self) -> i32 {
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

impl_primitive!(WzLong, i64);
impl_conversions!(WzLong, i64, i8);
impl_conversions!(WzLong, i64, i16);
impl_conversions!(WzLong, i64, i32);
impl_conversions!(WzLong, i64, i64);
impl_conversions!(WzLong, i64, u8);
impl_conversions!(WzLong, i64, u16);
impl_conversions!(WzLong, i64, u32);
impl_conversions!(WzLong, i64, u64);

impl Decode for WzLong {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let check = i8::decode(reader)?;
        Ok(Self(match check {
            i8::MIN => i64::decode(reader)?,
            _ => check as i64,
        }))
    }
}

impl Encode for WzLong {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        if self.0 > (i8::MAX as i64) || self.0 <= (i8::MIN as i64) {
            writer.write_byte(i8::MIN as u8)?;
            self.0.encode(writer)
        } else {
            writer.write_byte(self.0 as u8)
        }
    }
}

impl encode::SizeHint for WzLong {
    #[inline]
    fn size_hint(&self) -> i32 {
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
        assert_eq!(wz_int, test1);
        let wz_int = WzInt::from(test2);
        assert_eq!(wz_int, test2);
        let wz_int = WzInt::from(test3);
        assert_eq!(wz_int, test3);
        let wz_int = WzInt::from(test4); // truncated to be -1
        assert_eq!(wz_int, -1);

        // Test Ord
        let wz_int = WzInt::from(17);
        assert!(wz_int > test1);
        assert!(wz_int > test2);
        assert!(wz_int < test3);
        assert!(wz_int < test4);
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
        assert_eq!(wz_long, test1);
        let wz_long = WzLong::from(test2);
        assert_eq!(wz_long, test2);
        let wz_long = WzLong::from(test3);
        assert_eq!(wz_long, test3);
        let wz_long = WzLong::from(test4);
        assert_eq!(wz_long, test4);

        // Test Ord
        let wz_long = WzLong::from(17);
        assert!(wz_long > test1);
        assert!(wz_long > test2);
        assert!(wz_long < test3);
        assert!(wz_long < test4);
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
        assert_eq!(wz_long, 0x72);

        let long_notation = vec![(i8::MIN as u8), 1, 1, 0, 0, 0, 0, 0, 0];
        let mut reader = WzReader::unencrypted(0, 0, Cursor::new(long_notation));
        let wz_long = WzLong::decode(&mut reader).expect("error reading from cursor");
        assert_eq!(wz_long, 257);

        let failure = vec![(i8::MIN as u8), 1, 1, 1, 1];
        let mut reader = WzReader::unencrypted(0, 0, Cursor::new(failure));
        match WzLong::decode(&mut reader) {
            Ok(val) => panic!("WzLong got {}", *val),
            Err(_) => {}
        }
    }
}
