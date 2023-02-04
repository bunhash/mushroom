use crate::{WzError, WzErrorType, WzRead, WzResult};

#[derive(Clone, Debug, PartialEq)]
pub enum WzProperty {
    Null,
    Short(i16),
    Int(i32),
    Float(f32),
    Double(f64),
    String(String),
    Variant,
    Long(i64),
}

impl WzProperty {
    pub(crate) fn from_reader(
        prop_type: u8,
        offset: u64,
        reader: &mut dyn WzRead,
    ) -> WzResult<Self> {
        match prop_type {
            0 => Ok(WzProperty::Null),
            2 | 11 => {
                let val = i16::from_le_bytes(reader.read_short()?);
                Ok(WzProperty::Short(val))
            }
            3 | 19 => {
                let val = reader.read_wz_int()?;
                Ok(WzProperty::Int(val))
            }
            20 => {
                let val = reader.read_wz_long()?;
                Ok(WzProperty::Long(val))
            }
            4 => {
                let t = reader.read_byte()?;
                Ok(WzProperty::Float(match t {
                    0x80 => f32::from_le_bytes(reader.read_word()?),
                    _ => 0.0,
                }))
            }
            5 => {
                let val = f64::from_le_bytes(reader.read_long()?);
                Ok(WzProperty::Double(val))
            }
            8 => {
                let val = reader.read_wz_uol(offset)?;
                Ok(WzProperty::String(val))
            }
            9 => Ok(WzProperty::Variant),
            _ => return Err(WzError::from(WzErrorType::InvalidProp)),
        }
    }
}
