//! WZ Int32 and Int64 Formats

use crate::{
    decode::{Decode, Error, Reader},
    macros,
};
use crypto::Decryptor;
use std::io::{Read, Seek};

/// Defines a WZ int structure and how to encode/decode it.
///
/// This is a compressed `i32`. WZ archives use both `i32` and `Int32` so a separate structure was
/// created to differentiate them.
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Int32(i32);

macros::impl_num!(Int32, i32);
macros::impl_from!(Int32, i8, i32);
macros::impl_from!(Int32, i16, i32);
macros::impl_from!(Int32, i32, i32);
macros::impl_from!(Int32, i64, i32);
macros::impl_from!(Int32, u8, i32);
macros::impl_from!(Int32, u16, i32);
macros::impl_from!(Int32, u32, i32);
macros::impl_from!(Int32, u64, i32);
macros::impl_from!(Int32, usize, i32);

impl Decode for Int32 {
    type Error = Error;

    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let check = i8::decode(reader)?;
        Ok(Self(match check {
            i8::MIN => i32::decode(reader)?,
            v => v as i32,
        }))
    }
}

/// Defines a WZ long structure and how to encode/decode it.
///
/// This is a compressed `i64`. WZ archives use both `i64` and `Int64` so a separate structure was
/// created to differentiate them.
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Int64(i64);

macros::impl_num!(Int64, i64);
macros::impl_from!(Int64, i8, i64);
macros::impl_from!(Int64, i16, i64);
macros::impl_from!(Int64, i32, i64);
macros::impl_from!(Int64, i64, i64);
macros::impl_from!(Int64, u8, i64);
macros::impl_from!(Int64, u16, i64);
macros::impl_from!(Int64, u32, i64);
macros::impl_from!(Int64, u64, i64);
macros::impl_from!(Int64, usize, i64);

impl Decode for Int64 {
    type Error = Error;

    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let check = i8::decode(reader)?;
        Ok(Self(match check {
            i8::MIN => i64::decode(reader)?,
            v => v as i64,
        }))
    }
}

#[cfg(test)]
mod tests {

    use crate::{Decode, Int32, Int64, Reader};
    use std::io::Cursor;

    #[test]
    fn wz_int() {
        let test1: i8 = 5;
        let test2: i16 = 15;
        let test3: i32 = 25;
        let test4: i64 = i64::MAX;

        // Test conversions
        let wz_int = Int32::from(test1);
        assert_eq!(wz_int, Int32::from(test1));
        let wz_int = Int32::from(test2);
        assert_eq!(wz_int, Int32::from(test2));
        let wz_int = Int32::from(test3);
        assert_eq!(wz_int, Int32::from(test3));
        let wz_int = Int32::from(test4); // truncated to be -1
        assert_eq!(wz_int, Int32::from(-1));

        // Test Ord
        let wz_int = Int32::from(17);
        assert!(wz_int > Int32::from(test1));
        assert!(wz_int > Int32::from(test2));
        assert!(wz_int < Int32::from(test3));
        assert!(wz_int > Int32::from(test4));
    }

    #[test]
    fn calculate_with_wz_int() {
        let i1 = 5;
        let i2 = 7;

        let wz_int1 = Int32::from(i1);
        let wz_int2 = Int32::from(i2);
        assert_eq!(wz_int1 + wz_int2, Int32::from(i1 + i2));
        assert_eq!(wz_int2 - wz_int1, Int32::from(i2 - i1));
    }

    #[test]
    fn decode_wz_int() {
        let short_notation = vec![0x72];
        let mut reader = Reader::unencrypted(0, 0, Cursor::new(short_notation));
        let wz_int = Int32::decode(&mut reader).expect("error reading from cursor");
        assert_eq!(wz_int, 0x72);

        let long_notation = vec![(i8::MIN as u8), 1, 1, 0, 0];
        let mut reader = Reader::unencrypted(0, 0, Cursor::new(long_notation));
        let wz_int = Int32::decode(&mut reader).expect("error reading from cursor");
        assert_eq!(wz_int, 257);

        let failure = vec![(i8::MIN as u8), 1, 1];
        let mut reader = Reader::unencrypted(0, 0, Cursor::new(failure));
        match Int32::decode(&mut reader) {
            Ok(val) => panic!("Int32 got {}", *val),
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
        let wz_long = Int64::from(test1);
        assert_eq!(wz_long, Int64::from(test1));
        let wz_long = Int64::from(test2);
        assert_eq!(wz_long, Int64::from(test2));
        let wz_long = Int64::from(test3);
        assert_eq!(wz_long, Int64::from(test3));
        let wz_long = Int64::from(test4);
        assert_eq!(wz_long, Int64::from(test4));

        // Test Ord
        let wz_long = Int64::from(17);
        assert!(wz_long > Int64::from(test1));
        assert!(wz_long > Int64::from(test2));
        assert!(wz_long < Int64::from(test3));
        assert!(wz_long < Int64::from(test4));
    }

    #[test]
    fn calculate_with_wz_long() {
        let i1 = 5;
        let i2 = 7;

        let wz_long1 = Int64::from(i1);
        let wz_long2 = Int64::from(i2);
        assert_eq!(wz_long1 + wz_long2, Int64::from(i1 + i2));
        assert_eq!(wz_long2 - wz_long1, Int64::from(i2 - i1));
    }

    #[test]
    fn decode_wz_long() {
        let short_notation = vec![0x72];
        let mut reader = Reader::unencrypted(0, 0, Cursor::new(short_notation));
        let wz_long = Int64::decode(&mut reader).expect("error reading from cursor");
        assert_eq!(wz_long, Int64::from(0x72));

        let long_notation = vec![(i8::MIN as u8), 1, 1, 0, 0, 0, 0, 0, 0];
        let mut reader = Reader::unencrypted(0, 0, Cursor::new(long_notation));
        let wz_long = Int64::decode(&mut reader).expect("error reading from cursor");
        assert_eq!(wz_long, Int64::from(257));

        let failure = vec![(i8::MIN as u8), 1, 1, 1, 1];
        let mut reader = Reader::unencrypted(0, 0, Cursor::new(failure));
        match Int64::decode(&mut reader) {
            Ok(val) => panic!("Int64 got {}", *val),
            Err(_) => {}
        }
    }
}
