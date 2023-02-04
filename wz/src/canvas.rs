use crate::{WzError, WzErrorType, WzRead, WzResult};

#[derive(Clone, Debug, PartialEq)]
pub struct WzCanvas {
    width: i32,
    height: i32,
    format: i32,
    mag_level: u8,
    data: Vec<u8>,
    // children
}

impl WzCanvas {
    pub(crate) fn from_reader(offset: u64, reader: &mut dyn WzRead) -> WzResult<Self> {
        let width = reader.read_wz_int()?;
        let height = reader.read_wz_int()?;
        let format = reader.read_wz_int()?;
        let mag_level = reader.read_byte()?;
        if i32::from_le_bytes(reader.read_word()?) != 0 {
            return Err(WzError::from(WzErrorType::InvalidProp));
        }
        let len = i32::from_le_bytes(reader.read_word()?);
        if len <= 0 {
            return Err(WzError::from(WzErrorType::InvalidProp));
        }
        let len = (len - 1) as usize;
        let mut data = Vec::with_capacity(len);
        reader.read_nbytes(len, &mut data)?;
        Ok(WzCanvas {
            width,
            height,
            format,
            mag_level,
            data,
        })
    }
}
