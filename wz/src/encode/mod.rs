//! Encode Trait

mod encoder;
mod error;
mod size;

pub use encoder::Encoder;
pub use error::Error;
pub use size::SizeHint;

/// Trait for encoding objects
pub trait Encode: SizeHint {
    /// Type of error thrown. Must implement `From<io::Error>`
    type Error;

    /// Encodes objects
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error>
    where
        Self::Error: From<Error>;
}

impl Encode for i8 {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.encode_bytes(&[*self as u8])
    }
}

impl Encode for i16 {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.encode_bytes(&self.to_le_bytes())
    }
}

impl Encode for i32 {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.encode_bytes(&self.to_le_bytes())
    }
}

impl Encode for i64 {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.encode_bytes(&self.to_le_bytes())
    }
}

impl Encode for u8 {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.encode_bytes(&[*self])
    }
}

impl Encode for u16 {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.encode_bytes(&self.to_le_bytes())
    }
}

impl Encode for u32 {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.encode_bytes(&self.to_le_bytes())
    }
}

impl Encode for u64 {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.encode_bytes(&self.to_le_bytes())
    }
}

impl Encode for f32 {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        match self {
            0f32 => 0u8.encode(encoder),
            _ => {
                0x80u8.encode(encoder)?;
                encoder.encode_bytes(&self.to_le_bytes())
            }
        }
    }
}

impl Encode for f64 {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.encode_bytes(&self.to_le_bytes())
    }
}

impl Encode for &str {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        // If length is 0 just write 0 and be done with it
        if self.len() == 0 {
            return 0u8.encode(encoder);
        }

        // If everything is ASCII, encode as UTF-8, else Unicode
        if self.is_ascii() {
            let length = self.len() as i32;
            // Write the length
            // length CAN equal i8::MAX here as the 2s compliment is not i8::MIN
            if length > (i8::MAX as i32) {
                i8::MIN.encode(encoder)?;
                length.encode(encoder)?;
            } else {
                ((-length) as i8).encode(encoder)?;
            }
            // Write the string
            encode_utf8(encoder, self.as_bytes())
        } else {
            let bytes = self.encode_utf16().collect::<Vec<u16>>();
            let length = bytes.len() as i32;
            // Write the length
            // If lenth is equal to i8::MAX it will be treated as a long-length marker
            if length >= (i8::MAX as i32) {
                i8::MAX.encode(encoder)?;
                length.encode(encoder)?;
            } else {
                (length as i8).encode(encoder)?;
            }
            // Write the string
            encode_unicode(encoder, &bytes)
        }
    }
}

impl Encode for String {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        Ok(self.as_str().encode(encoder)?)
    }
}

/// Encode UTF-8 bytes
fn encode_utf8<E: Encoder>(encoder: &mut E, bytes: &[u8]) -> Result<(), Error> {
    let mut mask = 0xaa;
    let mut buf = bytes
        .iter()
        .map(|b| {
            let c = b ^ mask;
            mask = mask.checked_add(1).unwrap_or(0);
            c
        })
        .collect::<Vec<u8>>();
    encoder.encrypt_bytes(&mut buf);
    encoder.encode_bytes(&buf)
}

/// Encode unicode bytes
fn encode_unicode<E: Encoder>(encoder: &mut E, bytes: &[u16]) -> Result<(), Error> {
    let mut mask: u16 = 0xaaaa;
    let mut buf = bytes
        .iter()
        .flat_map(|c| {
            let wchar = c ^ mask;
            mask = mask.checked_add(1).unwrap_or(0);
            wchar.to_le_bytes()
        })
        .collect::<Vec<u8>>();
    encoder.encrypt_bytes(&mut buf);
    encoder.encode_bytes(&buf)
}
