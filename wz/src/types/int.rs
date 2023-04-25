//! WZ Int and Long Formats

use crate::{error::Result, impl_primitive, Decode, Encode, Reader, Writer};
use core::ops::{Deref, DerefMut};

/// Defines a WZ-INT structure and how to encode/decode it
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct WzInt(i32);

impl_deref!(WzInt, i32);
impl_primitive!(WzInt, i32, i8);
impl_primitive!(WzInt, i32, i16);
impl_primitive!(WzInt, i32, i32);
impl_primitive!(WzInt, i32, i64);

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

impl Encode for WzInt {
    #[inline]
    fn encode_size(&self) -> u64 {
        if self.0 > (i8::MAX as i32) || self.0 <= (i8::MIN as i32) {
            5
        } else {
            1
        }
    }

    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        if self.0 > (i8::MAX as i32) || self.0 <= (i8::MIN as i32) {
            writer.write_byte(i8::MIN as u8)?;
            self.0.encode(writer)
        } else {
            writer.write_byte(self.0 as u8)
        }
    }
}

/// Defines a WZ-LONG structure and how to encode/decode it
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct WzLong(i64);

impl_deref!(WzLong, i64);
impl_primitive!(WzLong, i64, i8);
impl_primitive!(WzLong, i64, i16);
impl_primitive!(WzLong, i64, i32);
impl_primitive!(WzLong, i64, i64);

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

impl Encode for WzLong {
    #[inline]
    fn encode_size(&self) -> u64 {
        if self.0 > (i8::MAX as i64) || self.0 <= (i8::MIN as i64) {
            9
        } else {
            1
        }
    }

    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        if self.0 > (i8::MAX as i64) || self.0 <= (i8::MIN as i64) {
            writer.write_byte(i8::MIN as u8)?;
            self.0.encode(writer)
        } else {
            writer.write_byte(self.0 as u8)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        types::{WzInt, WzLong},
        Decode, Encode, Metadata, UnencryptedReader,
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
    fn decode_wz_int() {
        let short_notation = vec![0x72];
        let size = short_notation.len() as u64;
        let mut reader = UnencryptedReader::new(Cursor::new(short_notation), Metadata::new("", 0));
        let wz_int = WzInt::decode(&mut reader).expect("error reading from cursor");
        assert_eq!(wz_int, 0x72);
        assert_eq!(wz_int.encode_size(), size);

        let long_notation = vec![(i8::MIN as u8), 1, 1, 0, 0];
        let size = long_notation.len() as u64;
        let mut reader = UnencryptedReader::new(Cursor::new(long_notation), Metadata::new("", 0));
        let wz_int = WzInt::decode(&mut reader).expect("error reading from cursor");
        assert_eq!(wz_int, 257);
        assert_eq!(wz_int.encode_size(), size);

        let failure = vec![(i8::MIN as u8), 1, 1];
        let mut reader = UnencryptedReader::new(Cursor::new(failure), Metadata::new("", 0));
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
    fn decode_wz_long() {
        let short_notation = vec![0x72];
        let size = short_notation.len() as u64;
        let mut reader = UnencryptedReader::new(Cursor::new(short_notation), Metadata::new("", 0));
        let wz_long = WzLong::decode(&mut reader).expect("error reading from cursor");
        assert_eq!(wz_long, 0x72);
        assert_eq!(wz_long.encode_size(), size);

        let long_notation = vec![(i8::MIN as u8), 1, 1, 0, 0, 0, 0, 0, 0];
        let size = long_notation.len() as u64;
        let mut reader = UnencryptedReader::new(Cursor::new(long_notation), Metadata::new("", 0));
        let wz_long = WzLong::decode(&mut reader).expect("error reading from cursor");
        assert_eq!(wz_long, 257);
        assert_eq!(wz_long.encode_size(), size);

        let failure = vec![(i8::MIN as u8), 1, 1, 1, 1];
        let mut reader = UnencryptedReader::new(Cursor::new(failure), Metadata::new("", 0));
        match WzLong::decode(&mut reader) {
            Ok(val) => panic!("WzLong got {}", *val),
            Err(_) => {}
        }
    }
}
