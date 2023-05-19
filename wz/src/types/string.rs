//! WZ String Format

use crate::{
    error::{DecodeError, Result},
    io::{Decode, Encode, SizeHint, WzRead, WzWrite},
};

impl Encode for &str {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite + ?Sized,
    {
        let length = self.len() as i32;

        // If length is 0 just write 0 and be done with it
        if length == 0 {
            return writer.write_byte(0);
        }

        // If everything is ASCII, encode as UTF-8, else Unicode
        if self.is_ascii() {
            // Write the length
            // length CAN equal i8::MAX here as the 2s compliment is not i8::MIN
            if length > (i8::MAX as i32) {
                writer.write_byte(i8::MIN as u8)?;
                length.encode(writer)?;
            } else {
                writer.write_byte((-length) as u8)?;
            }
            // Write the string
            writer.write_utf8_bytes(self.as_bytes())
        } else {
            // Write the length
            // If lenth is equal to i8::MAX it will be treated as a long-length marker
            if length >= (i8::MAX as i32) {
                writer.write_byte(i8::MAX as u8)?;
                length.encode(writer)?;
            } else {
                writer.write_byte(length as u8)?;
            }
            // Write the string
            let bytes = self.encode_utf16().collect::<Vec<u16>>();
            writer.write_unicode_bytes(&bytes)
        }
    }
}

impl SizeHint for &str {
    #[inline]
    fn size_hint(&self) -> u32 {
        let length = self.len() as u32;

        // If length is 0 just write 0 and be done with it
        if length == 0 {
            return 1;
        }

        // If everything is ASCII, encode as UTF-8, else Unicode
        if self.is_ascii() {
            // length CAN equal i8::MAX here as the 2s compliment is not i8::MIN
            if length > (i8::MAX as u32) {
                5 + length
            } else {
                1 + length
            }
        } else {
            // If lenth is equal to i8::MAX it will be treated as a long-length marker
            if length >= (i8::MAX as u32) {
                5 + (length * 2)
            } else {
                1 + (length * 2)
            }
        }
    }
}

impl Decode for String {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead + ?Sized,
    {
        let check = i8::decode(reader)?;
        let length = match check {
            i8::MIN | i8::MAX => i32::decode(reader)?,
            0 => return Ok(String::from("")),
            _ => (check as i32).wrapping_abs(),
        };
        // Sanity check
        if length <= 0 {
            return Err(DecodeError::Length(length).into());
        }
        Ok(if check < 0 {
            // UTF-8
            String::from_utf8_lossy(reader.read_utf8_bytes(length as usize)?.as_slice()).into()
        } else {
            // Unicode
            String::from_utf16_lossy(reader.read_unicode_bytes(length as usize)?.as_slice())
        })
    }
}

impl Encode for String {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite + ?Sized,
    {
        self.as_str().encode(writer)
    }
}

impl SizeHint for String {
    #[inline]
    fn size_hint(&self) -> u32 {
        self.as_str().size_hint()
    }
}
