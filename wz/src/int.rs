//! WZ Int and Long Formats

use crate::{macros, Decode, Error, Reader};
use std::io::{Read, Seek};

/// Defines a WZ int structure and how to encode/decode it.
///
/// This is a compressed `i32`. WZ archives use both `i32` and `Integer` so a separate structure was
/// created to differentiate them.
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
#[repr(transparent)]
pub struct Integer(i32);

macros::impl_num!(Integer, i32);
macros::impl_from!(Integer, i8, i32);
macros::impl_from!(Integer, i16, i32);
macros::impl_from!(Integer, i32, i32);
macros::impl_from!(Integer, i64, i32);
macros::impl_from!(Integer, u8, i32);
macros::impl_from!(Integer, u16, i32);
macros::impl_from!(Integer, u32, i32);
macros::impl_from!(Integer, u64, i32);
macros::impl_from!(Integer, usize, i32);

impl Decode for Integer {
    fn decode<R>(reader: &mut Reader<R>) -> Result<Self, Error>
    where
        R: Read + Seek,
    {
        let check = i8::decode(reader)?;
        Ok(Self(match check {
            i8::MIN => i32::decode(reader)?,
            v => v as i32,
        }))
    }
}

//impl Encode for Integer {
//    fn encode<W>(&self, writer: &mut W) -> Result<()>
//    where
//        W: WzWrite + ?Sized,
//    {
//        if self.0 > (i8::MAX as i32) || self.0 <= (i8::MIN as i32) {
//            writer.write_byte(i8::MIN as u8)?;
//            self.0.encode(writer)
//        } else {
//            writer.write_byte(self.0 as u8)
//        }
//    }
//}
//
//impl SizeHint for Integer {
//    #[inline]
//    fn size_hint(&self) -> u32 {
//        if self.0 > (i8::MAX as i32) || self.0 <= (i8::MIN as i32) {
//            5
//        } else {
//            1
//        }
//    }
//}

/// Defines a WZ long structure and how to encode/decode it.
///
/// This is a compressed `i64`. WZ archives use both `i64` and `Long` so a separate structure was
/// created to differentiate them.
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
#[repr(transparent)]
pub struct Long(i64);

macros::impl_num!(Long, i64);
macros::impl_from!(Long, i8, i64);
macros::impl_from!(Long, i16, i64);
macros::impl_from!(Long, i32, i64);
macros::impl_from!(Long, i64, i64);
macros::impl_from!(Long, u8, i64);
macros::impl_from!(Long, u16, i64);
macros::impl_from!(Long, u32, i64);
macros::impl_from!(Long, u64, i64);
macros::impl_from!(Long, usize, i64);

impl Decode for Long {
    fn decode<R>(reader: &mut Reader<R>) -> Result<Self, Error>
    where
        R: Read + Seek,
    {
        let check = i8::decode(reader)?;
        Ok(Self(match check {
            i8::MIN => i64::decode(reader)?,
            v => v as i64,
        }))
    }
}

//impl Encode for Long {
//    fn encode<W>(&self, writer: &mut W) -> Result<()>
//    where
//        W: WzWrite + ?Sized,
//    {
//        if self.0 > (i8::MAX as i64) || self.0 <= (i8::MIN as i64) {
//            writer.write_byte(i8::MIN as u8)?;
//            self.0.encode(writer)
//        } else {
//            writer.write_byte(self.0 as u8)
//        }
//    }
//}
//
//impl SizeHint for Long {
//    #[inline]
//    fn size_hint(&self) -> u32 {
//        if self.0 > (i8::MAX as i64) || self.0 <= (i8::MIN as i64) {
//            9
//        } else {
//            1
//        }
//    }
//}

#[cfg(test)]
mod tests {

    use crate::{Decode, Integer, Long, Reader};
    use std::io::Cursor;

    #[test]
    fn wz_int() {
        let test1: i8 = 5;
        let test2: i16 = 15;
        let test3: i32 = 25;
        let test4: i64 = i64::MAX;

        // Test conversions
        let wz_int = Integer::from(test1);
        assert_eq!(wz_int, Integer::from(test1));
        let wz_int = Integer::from(test2);
        assert_eq!(wz_int, Integer::from(test2));
        let wz_int = Integer::from(test3);
        assert_eq!(wz_int, Integer::from(test3));
        let wz_int = Integer::from(test4); // truncated to be -1
        assert_eq!(wz_int, Integer::from(-1));

        // Test Ord
        let wz_int = Integer::from(17);
        assert!(wz_int > Integer::from(test1));
        assert!(wz_int > Integer::from(test2));
        assert!(wz_int < Integer::from(test3));
        assert!(wz_int > Integer::from(test4));
    }

    #[test]
    fn calculate_with_wz_int() {
        let i1 = 5;
        let i2 = 7;

        let wz_int1 = Integer::from(i1);
        let wz_int2 = Integer::from(i2);
        assert_eq!(wz_int1 + wz_int2, Integer::from(i1 + i2));
        assert_eq!(wz_int2 - wz_int1, Integer::from(i2 - i1));
    }

    #[test]
    fn decode_wz_int() {
        let short_notation = vec![0x72];
        let mut reader = Reader::new(0, 0, Cursor::new(short_notation));
        let wz_int = Integer::decode(&mut reader).expect("error reading from cursor");
        assert_eq!(wz_int, 0x72);

        let long_notation = vec![(i8::MIN as u8), 1, 1, 0, 0];
        let mut reader = Reader::new(0, 0, Cursor::new(long_notation));
        let wz_int = Integer::decode(&mut reader).expect("error reading from cursor");
        assert_eq!(wz_int, 257);

        let failure = vec![(i8::MIN as u8), 1, 1];
        let mut reader = Reader::new(0, 0, Cursor::new(failure));
        match Integer::decode(&mut reader) {
            Ok(val) => panic!("Integer got {}", *val),
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
        let wz_long = Long::from(test1);
        assert_eq!(wz_long, Long::from(test1));
        let wz_long = Long::from(test2);
        assert_eq!(wz_long, Long::from(test2));
        let wz_long = Long::from(test3);
        assert_eq!(wz_long, Long::from(test3));
        let wz_long = Long::from(test4);
        assert_eq!(wz_long, Long::from(test4));

        // Test Ord
        let wz_long = Long::from(17);
        assert!(wz_long > Long::from(test1));
        assert!(wz_long > Long::from(test2));
        assert!(wz_long < Long::from(test3));
        assert!(wz_long < Long::from(test4));
    }

    #[test]
    fn calculate_with_wz_long() {
        let i1 = 5;
        let i2 = 7;

        let wz_long1 = Long::from(i1);
        let wz_long2 = Long::from(i2);
        assert_eq!(wz_long1 + wz_long2, Long::from(i1 + i2));
        assert_eq!(wz_long2 - wz_long1, Long::from(i2 - i1));
    }

    #[test]
    fn decode_wz_long() {
        let short_notation = vec![0x72];
        let mut reader = Reader::new(0, 0, Cursor::new(short_notation));
        let wz_long = Long::decode(&mut reader).expect("error reading from cursor");
        assert_eq!(wz_long, Long::from(0x72));

        let long_notation = vec![(i8::MIN as u8), 1, 1, 0, 0, 0, 0, 0, 0];
        let mut reader = Reader::new(0, 0, Cursor::new(long_notation));
        let wz_long = Long::decode(&mut reader).expect("error reading from cursor");
        assert_eq!(wz_long, Long::from(257));

        let failure = vec![(i8::MIN as u8), 1, 1, 1, 1];
        let mut reader = Reader::new(0, 0, Cursor::new(failure));
        match Long::decode(&mut reader) {
            Ok(val) => panic!("Long got {}", *val),
            Err(_) => {}
        }
    }
}
