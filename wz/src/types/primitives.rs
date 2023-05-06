//! Primitive WZ Formats

use crate::io::{decode, encode, Decode, Encode, WzReader, WzWriter};
use crypto::{Decryptor, Encryptor};
use std::io::{Read, Seek, Write};

impl Decode for i8 {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        Ok(reader.read_byte()? as i8)
    }
}

impl Encode for i8 {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        writer.write_byte(*self as u8)
    }
}

impl encode::SizeHint for i8 {
    #[inline]
    fn size_hint(&self) -> i32 {
        1
    }
}

impl Decode for i16 {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(i16::from_le_bytes(buf))
    }
}

impl Encode for i16 {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        writer.write_all(&self.to_le_bytes())
    }
}

impl encode::SizeHint for i16 {
    #[inline]
    fn size_hint(&self) -> i32 {
        2
    }
}

impl Decode for i32 {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }
}

impl Encode for i32 {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        writer.write_all(&self.to_le_bytes())
    }
}

impl encode::SizeHint for i32 {
    #[inline]
    fn size_hint(&self) -> i32 {
        4
    }
}

impl Decode for i64 {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }
}

impl Encode for i64 {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        writer.write_all(&self.to_le_bytes())
    }
}

impl encode::SizeHint for i64 {
    #[inline]
    fn size_hint(&self) -> i32 {
        8
    }
}

impl Decode for u8 {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        reader.read_byte()
    }
}

impl Encode for u8 {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        writer.write_byte(*self)
    }
}

impl encode::SizeHint for u8 {
    #[inline]
    fn size_hint(&self) -> i32 {
        1
    }
}

impl Decode for u16 {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }
}

impl Encode for u16 {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        writer.write_all(&self.to_le_bytes())
    }
}

impl encode::SizeHint for u16 {
    #[inline]
    fn size_hint(&self) -> i32 {
        2
    }
}

impl Decode for u32 {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
}

impl Encode for u32 {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        writer.write_all(&self.to_le_bytes())
    }
}

impl encode::SizeHint for u32 {
    #[inline]
    fn size_hint(&self) -> i32 {
        4
    }
}

impl Decode for u64 {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
}

impl Encode for u64 {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        writer.write_all(&self.to_le_bytes())
    }
}

impl encode::SizeHint for u64 {
    #[inline]
    fn size_hint(&self) -> i32 {
        8
    }
}

impl Decode for f32 {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
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

impl Encode for f32 {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        if *self as u32 == 0 {
            writer.write_byte(0)
        } else {
            writer.write_byte(0x80)?;
            writer.write_all(&self.to_le_bytes())
        }
    }
}

impl encode::SizeHint for f32 {
    #[inline]
    fn size_hint(&self) -> i32 {
        if *self as u32 == 0 {
            1
        } else {
            5
        }
    }
}

impl Decode for f64 {
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self, decode::Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }
}

impl Encode for f64 {
    fn encode<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<(), encode::Error>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        writer.write_all(&self.to_le_bytes())
    }
}

impl encode::SizeHint for f64 {
    #[inline]
    fn size_hint(&self) -> i32 {
        8
    }
}
