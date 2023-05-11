//! Compressed RGBA Pixel Type

const MAX_BGRA4444_SIZE: usize = 15;
const MAX_RGB565_RB: usize = 31;
const MAX_RGB565_G: usize = 63;

pub(crate) fn from_bgra4444(data: Vec<u8>) -> Vec<u8> {
    data.iter()
        .map(|b| [b.wrapping_shl(4) | b & 0xF, b & 0xF0 | b.wrapping_shr(4)])
        .flatten()
        .collect::<Vec<u8>>()
}

pub(crate) fn to_bgra4444(data: Vec<u8>) -> Vec<u8> {
    data.chunks(4)
        .map(|bytes| {
            let a = ((bytes[0] as usize * MAX_BGRA4444_SIZE) / u8::MAX as usize) as u8;
            let b = ((bytes[1] as usize * MAX_BGRA4444_SIZE) / u8::MAX as usize) as u8;
            let c = ((bytes[2] as usize * MAX_BGRA4444_SIZE) / u8::MAX as usize) as u8;
            let d = ((bytes[3] as usize * MAX_BGRA4444_SIZE) / u8::MAX as usize) as u8;
            [b.wrapping_shl(4) | a & 0xf, d.wrapping_shl(4) | c & 0xf]
        })
        .flatten()
        .collect::<Vec<u8>>()
}

pub(crate) fn from_rgb565(data: Vec<u8>) -> Vec<u8> {
    data.chunks(2)
        .map(|bytes| {
            let byte = u16::from_le_bytes([bytes[0], bytes[1]]);

            let r = byte.wrapping_shr(11) as usize;
            let g = (byte.wrapping_shr(5) & 0x3F) as usize;
            let b = (byte & 0x1F) as usize;

            let r = ((r * u8::MAX as usize) / MAX_RGB565_RB) as u8;
            let g = ((g * u8::MAX as usize) / MAX_RGB565_G) as u8;
            let b = ((b * u8::MAX as usize) / MAX_RGB565_RB) as u8;

            [r, g, b, 255]
        })
        .flatten()
        .collect::<Vec<u8>>()
}

pub(crate) fn to_rgb565(data: Vec<u8>) -> Vec<u8> {
    data.chunks(4)
        .map(|bytes| {
            let r = bytes[0] as usize;
            let g = bytes[1] as usize;
            let b = bytes[2] as usize;

            let r = ((r * MAX_RGB565_RB) / u8::MAX as usize) as u16 & 0x1F;
            let g = ((g * MAX_RGB565_G) / u8::MAX as usize) as u16 & 0x3F;
            let b = ((b * MAX_RGB565_RB) / u8::MAX as usize) as u16 & 0x1F;

            (r.wrapping_shl(11) | g.wrapping_shl(5) | b).to_le_bytes()
        })
        .flatten()
        .collect::<Vec<u8>>()
}
