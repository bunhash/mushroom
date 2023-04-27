//! Primitive WZ Formats

use crate::{error::Result, map::SizeHint, types::WzInt, Decode, Encode, Reader, Writer};

impl Decode for i8 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        Ok(reader.read_byte()? as i8)
    }
}

impl Encode for i8 {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        writer.write_byte(*self as u8)
    }
}

impl SizeHint for i8 {
    fn data_size(&self) -> WzInt {
        WzInt::from(1)
    }
}

impl Decode for i16 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(i16::from_le_bytes(buf))
    }
}

impl Encode for i16 {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        writer.write_all(&self.to_le_bytes())
    }
}

impl SizeHint for i16 {
    fn data_size(&self) -> WzInt {
        WzInt::from(2)
    }
}

impl Decode for i32 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }
}

impl Encode for i32 {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        writer.write_all(&self.to_le_bytes())
    }
}

impl SizeHint for i32 {
    fn data_size(&self) -> WzInt {
        WzInt::from(4)
    }
}

impl Decode for i64 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }
}

impl Encode for i64 {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        writer.write_all(&self.to_le_bytes())
    }
}

impl SizeHint for i64 {
    fn data_size(&self) -> WzInt {
        WzInt::from(8)
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

impl Encode for u8 {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        writer.write_byte(*self)
    }
}

impl SizeHint for u8 {
    fn data_size(&self) -> WzInt {
        WzInt::from(1)
    }
}

impl Decode for u16 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }
}

impl Encode for u16 {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        writer.write_all(&self.to_le_bytes())
    }
}

impl SizeHint for u16 {
    fn data_size(&self) -> WzInt {
        WzInt::from(2)
    }
}

impl Decode for u32 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
}

impl Encode for u32 {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        writer.write_all(&self.to_le_bytes())
    }
}

impl SizeHint for u32 {
    fn data_size(&self) -> WzInt {
        WzInt::from(4)
    }
}

impl Decode for u64 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
}

impl Encode for u64 {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        writer.write_all(&self.to_le_bytes())
    }
}

impl SizeHint for u64 {
    fn data_size(&self) -> WzInt {
        WzInt::from(8)
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
                reader.read_exact(&mut buf)?;
                f32::from_le_bytes(buf)
            }
            _ => 0f32,
        })
    }
}

impl Encode for f32 {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        if *self as u32 == 0 {
            writer.write_byte(0)
        } else {
            writer.write_byte(0x80)?;
            writer.write_all(&self.to_le_bytes())
        }
    }
}

impl SizeHint for f32 {
    fn data_size(&self) -> WzInt {
        if *self as u32 == 0 {
            WzInt::from(1)
        } else {
            WzInt::from(5)
        }
    }
}

impl Decode for f64 {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }
}

impl Encode for f64 {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        writer.write_all(&self.to_le_bytes())
    }
}

impl SizeHint for f64 {
    fn data_size(&self) -> WzInt {
        WzInt::from(8)
    }
}
