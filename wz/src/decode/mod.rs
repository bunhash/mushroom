//! Decode Trait

mod decoder;
mod error;

pub use decoder::{Decoder, SliceDecoder};
pub use error::{Error, ErrorCode};

/// Trait for decoding objects
pub trait Decode: Sized {
    /// Type of error thrown. Must implement `From<decode::Error>`
    type Error;

    /// Decodes objects
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error>
    where
        Self::Error: From<Error>;
}

impl Decode for i8 {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        let mut buf = [0u8];
        decoder.decode_bytes(&mut buf)?;
        Ok(buf[0] as i8)
    }
}

impl Decode for i16 {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 2];
        decoder.decode_bytes(&mut buf)?;
        Ok(i16::from_le_bytes(buf))
    }
}

impl Decode for i32 {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 4];
        decoder.decode_bytes(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }
}

impl Decode for i64 {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 8];
        decoder.decode_bytes(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }
}

impl Decode for u8 {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        let mut buf = [0u8];
        decoder.decode_bytes(&mut buf)?;
        Ok(buf[0])
    }
}

impl Decode for u16 {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 2];
        decoder.decode_bytes(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }
}

impl Decode for u32 {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 4];
        decoder.decode_bytes(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
}

impl Decode for u64 {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 8];
        decoder.decode_bytes(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
}

impl Decode for f32 {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        Ok(match u8::decode(decoder)? {
            0x80 => {
                let mut buf = [0u8; 4];
                decoder.decode_bytes(&mut buf)?;
                f32::from_le_bytes(buf)
            }
            _ => 0f32,
        })
    }
}

impl Decode for f64 {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 8];
        decoder.decode_bytes(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }
}

impl Decode for String {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        let check = i8::decode(decoder)?;
        let length = match check {
            i8::MIN | i8::MAX => i32::decode(decoder)?,
            0 => return Ok(String::from("")),
            _ => (check as i32).wrapping_abs(),
        };
        // Sanity check
        if length <= 0 {
            return Err(Error::length(&format!(
                "negative length while decoding string"
            )));
        }
        Ok(if check < 0 {
            // UTF-8
            String::from_utf8(decode_utf8(decoder, length as usize)?)?
        } else {
            // Unicode
            String::from_utf16(&decode_unicode(decoder, length as usize)?)?
        })
    }
}

/// Decode UTF-8 bytes
fn decode_utf8<D: Decoder>(decoder: &mut D, len: usize) -> Result<Vec<u8>, Error> {
    let mut buf = vec![0u8; len];
    decoder.decode_bytes(&mut buf)?;
    decoder.decrypt_bytes(&mut buf);
    let mut mask = 0xaa;
    buf.iter_mut().for_each(|b| {
        *b = *b ^ mask;
        mask = mask.checked_add(1).unwrap_or(0);
    });
    Ok(buf)
}

/// Decode unicode bytes
fn decode_unicode<D: Decoder>(decoder: &mut D, len: usize) -> Result<Vec<u16>, Error> {
    let mut buf = vec![0u8; len * 2];
    decoder.decode_bytes(&mut buf)?;
    decoder.decrypt_bytes(&mut buf);
    let mut mask = 0xaaaa;
    Ok(buf
        .chunks(2)
        .map(|c| {
            let wchar = u16::from_le_bytes([c[0], c[1]]);
            let wchar = wchar ^ mask;
            mask = mask.checked_add(1).unwrap_or(0);
            wchar
        })
        .collect())
}
