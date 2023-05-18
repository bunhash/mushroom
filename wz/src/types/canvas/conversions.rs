//! Compressed RGBA Pixel Type

use crate::{
    error::{CanvasError, Result},
    types::CanvasFormat,
};
use image::{Pixel, Rgb, RgbaImage};

#[inline]
pub(crate) fn split4444(pixel: u16) -> [u8; 4] {
    let b = (pixel & 0xf) as u8;
    let g = (pixel.wrapping_shr(4) & 0xf) as u8;
    let r = (pixel.wrapping_shr(8) & 0xf) as u8;
    let a = pixel.wrapping_shr(12) as u8;
    [
        r.wrapping_shl(4) | r,
        g.wrapping_shl(4) | g,
        b.wrapping_shl(4) | b,
        a.wrapping_shl(4) | a,
    ]
}

#[inline]
pub(crate) fn join4444(r: u8, g: u8, b: u8, a: u8) -> u16 {
    let r = r.wrapping_shr(4) as u16;
    let g = g.wrapping_shr(4) as u16;
    let b = b.wrapping_shr(4) as u16;
    let a = a.wrapping_shr(4) as u16;
    (b & 0x000f)
        | (g.wrapping_shl(4) & 0x00f0)
        | (r.wrapping_shl(8) & 0x0f00)
        | (a.wrapping_shl(12) & 0xf000)
}

#[inline]
pub(crate) fn split565(pixel: u16) -> [u8; 3] {
    let r = pixel.wrapping_shr(11) as u8;
    let g = (pixel.wrapping_shr(5) & 0x3f) as u8;
    let b = (pixel & 0x1f) as u8;
    [
        r.wrapping_shl(3) | r.wrapping_shr(2),
        g.wrapping_shl(2) | g.wrapping_shr(4),
        b.wrapping_shl(3) | b.wrapping_shr(2),
    ]
}

#[inline]
pub(crate) fn join565(r: u8, g: u8, b: u8) -> u16 {
    let r = ((r as usize) * 249 + 1014).wrapping_shr(11) as u16;
    let g = ((g as usize) * 253 + 505).wrapping_shr(10) as u16;
    let b = ((b as usize) * 249 + 1014).wrapping_shr(11) as u16;
    r.wrapping_shl(11) | g.wrapping_shl(5) | b
}

/// DirectX DXGI_FORMAT_B8G8R8A8
pub(crate) fn from_bgra8888(width: u32, height: u32, data: Vec<u8>) -> Result<RgbaImage> {
    let data_len = (width * height * 4) as usize;
    if data.len() < data_len {
        return Err(
            CanvasError::SizeMismatch(CanvasFormat::Bgra8888, width, height, data.len()).into(),
        );
    }
    Ok(RgbaImage::from_raw(
        width,
        height,
        data.chunks(4)
            .take(data_len / 4)
            .flat_map(|b| [b[2], b[1], b[0], b[3]])
            .collect::<Vec<u8>>(),
    )
    .expect("RGBA8888 size should be good"))
}

/// DirectX DXGI_FORMAT_B8G8R8A8
pub(crate) fn to_bgra8888(img: RgbaImage) -> (u32, u32, Vec<u8>) {
    (
        img.width(),
        img.height(),
        img.pixels()
            .flat_map(|p| {
                let rgba = p.channels();
                [rgba[2], rgba[1], rgba[0], rgba[3]]
            })
            .collect::<Vec<u8>>(),
    )
}

/// DirectX DXGI_FORMAT_B4G4R4A4
pub(crate) fn from_bgra4444(width: u32, height: u32, data: Vec<u8>) -> Result<RgbaImage> {
    let data_len = (width * height * 2) as usize;
    if data.len() < data_len {
        return Err(
            CanvasError::SizeMismatch(CanvasFormat::Bgra4444, width, height, data.len()).into(),
        );
    }
    Ok(RgbaImage::from_raw(
        width,
        height,
        data.chunks(2)
            .take(data_len / 2)
            .flat_map(|b| split4444(u16::from_le_bytes([b[0], b[1]])))
            .collect::<Vec<u8>>(),
    )
    .expect("Rgba4444 size should be good"))
}

/// DirectX DXGI_FORMAT_B4G4R4A4
pub(crate) fn to_bgra4444(img: RgbaImage) -> (u32, u32, Vec<u8>) {
    (
        img.width(),
        img.height(),
        img.pixels()
            .flat_map(|p| {
                let rgba = p.channels();
                join4444(rgba[0], rgba[1], rgba[2], rgba[3]).to_le_bytes()
            })
            .collect::<Vec<u8>>(),
    )
}

/// DirectX DXGI_FORMAT_B5G6R5
pub(crate) fn from_rgb565(width: u32, height: u32, data: Vec<u8>) -> Result<RgbaImage> {
    let data_len = (width * height * 2) as usize;
    if data.len() < data_len {
        return Err(
            CanvasError::SizeMismatch(CanvasFormat::Rgb565, width, height, data.len()).into(),
        );
    }
    Ok(RgbaImage::from_raw(
        width,
        height,
        data.chunks(2)
            .take(data_len / 2)
            .flat_map(|b| {
                let rgb = split565(u16::from_le_bytes([b[0], b[1]]));
                [rgb[0], rgb[1], rgb[2], u8::MAX]
            })
            .collect::<Vec<u8>>(),
    )
    .expect("Rgba565 size should be good"))
}

/// DirectX DXGI_FORMAT_B5G6R5
pub(crate) fn to_rgb565(img: RgbaImage) -> (u32, u32, Vec<u8>) {
    (
        img.width(),
        img.height(),
        img.pixels()
            .flat_map(|p| {
                let rgba = p.channels();
                join565(rgba[0], rgba[1], rgba[2]).to_le_bytes()
            })
            .collect::<Vec<u8>>(),
    )
}

/// This format just blows up an RGB565 image 16x. I assume repeating the pixel is faster than the
/// standard resize algorithms.
pub(crate) fn expand_rgb565(width: u32, height: u32, data: Vec<u8>) -> Result<RgbaImage> {
    if width % 16 != 0 || height % 16 != 0 {
        return Err(CanvasError::SizeMismatch(
            CanvasFormat::CompressedRgb565,
            width,
            height,
            data.len(),
        )
        .into());
    }

    let mut img = RgbaImage::new(width, height);

    let width = width / 16;
    let height = height / 16;
    let data_len = (width * height * 2) as usize;
    if data.len() < data_len {
        return Err(CanvasError::SizeMismatch(
            CanvasFormat::CompressedRgb565,
            width,
            height,
            data.len(),
        )
        .into());
    }

    for y in 0..height {
        for x in 0..width {
            let ind = (((y * width) + x) * 2) as usize;
            let pixel = Rgb(split565(u16::from_le_bytes([data[ind], data[ind + 1]]))).to_rgba();
            for j in 0..16 {
                for i in 0..16 {
                    img.put_pixel((x * 16) + i, (y * 16) + j, pixel);
                }
            }
        }
    }

    Ok(img)
}

/// This grabs a single pixel from every 16x16 block
pub(crate) fn compress_rgb565(img: RgbaImage) -> Result<(u32, u32, Vec<u8>)> {
    let (width, height) = img.dimensions();
    if width % 16 != 0 || height % 16 != 0 {
        return Err(CanvasError::SizeMismatch(
            CanvasFormat::CompressedRgb565,
            width,
            height,
            img.pixels().count() * 4,
        )
        .into());
    }
    let mut data = Vec::with_capacity((width * height / 256) as usize);
    for y in (0..height).step_by(16) {
        for x in (0..width).step_by(16) {
            let rgba = img.get_pixel(x, y).channels();
            data.extend_from_slice(&join565(rgba[0], rgba[1], rgba[2]).to_le_bytes());
        }
    }
    Ok((width, height, data))
}

#[cfg(test)]
mod tests {

    use crate::types::canvas::{join4444, join565, split4444, split565};

    #[test]
    fn rgba4444() {
        let pixel = 0xF820u16;
        let b = split4444(pixel);
        assert_eq!(pixel, join4444(b[0], b[1], b[2], b[3]));
    }

    #[test]
    fn rgb565() {
        let pixel = 0xF820u16;
        let b = split565(pixel);
        assert_eq!(pixel, join565(b[0], b[1], b[2]));
    }
}
