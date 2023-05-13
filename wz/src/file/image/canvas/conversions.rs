//! Compressed RGBA Pixel Type

use crate::error::{CanvasError, Result};
use image::{imageops::FilterType, DynamicImage, Rgba, RgbaImage};

const MAX_BGRA4444_SIZE: u8 = 15;
const MAX_RGB565_RB: u8 = 31;
const MAX_RGB565_G: u8 = 63;

#[inline]
pub(crate) fn ratio(src_val: u8, src_max: u8, dst_max: u8) -> u8 {
    ((src_val as usize * dst_max as usize) / src_max as usize) as u8
}

#[inline]
pub(crate) fn split4444(pixel: u16) -> [u8; 4] {
    let b = pixel.wrapping_shr(12) as u8;
    let g = (pixel.wrapping_shr(8) & 0xf) as u8;
    let r = (pixel.wrapping_shr(4) & 0xf) as u8;
    let a = (pixel & 0xf) as u8;
    [
        ratio(a, MAX_BGRA4444_SIZE, u8::MAX),
        ratio(r, MAX_BGRA4444_SIZE, u8::MAX),
        ratio(g, MAX_BGRA4444_SIZE, u8::MAX),
        ratio(b, MAX_BGRA4444_SIZE, u8::MAX),
    ]
}

#[inline]
pub(crate) fn join4444(a: u8, r: u8, g: u8, b: u8) -> u16 {
    let a = ratio(a, u8::MAX, MAX_BGRA4444_SIZE);
    let r = ratio(r, u8::MAX, MAX_BGRA4444_SIZE);
    let g = ratio(g, u8::MAX, MAX_BGRA4444_SIZE);
    let b = ratio(b, u8::MAX, MAX_BGRA4444_SIZE);
    (b.wrapping_shl(12) as u16 & 0xf000)
        | (g.wrapping_shl(8) as u16 & 0x0f00)
        | (r.wrapping_shl(4) as u16 & 0x00f0)
        | (a as u16 & 0x000f)
}

#[inline]
pub(crate) fn split565(pixel: u16) -> [u8; 3] {
    let b = (pixel & 0x1f) as usize;
    let g = (pixel.wrapping_shr(5) & 0x3f) as usize;
    let r = pixel.wrapping_shr(11) as usize;
    [
        ratio(r as u8, MAX_RGB565_RB, u8::MAX),
        ratio(g as u8, MAX_RGB565_G, u8::MAX),
        ratio(b as u8, MAX_RGB565_RB, u8::MAX),
    ]
}

#[inline]
pub(crate) fn join565(r: u8, g: u8, b: u8) -> u16 {
    (r as u16).wrapping_shl(11) | (g as u16).wrapping_shl(5) | (b as u16)
}

/// DirectX DXGI_FORMAT_B4G4R4A4
pub(crate) fn from_bgra4444(width: u32, height: u32, data: Vec<u8>) -> Result<RgbaImage> {
    assert!(data.len() % 2 == 0, "invald bgra4444 size");
    match RgbaImage::from_raw(
        width,
        height,
        data.chunks(2)
            .flat_map(|b| split4444(u16::from_le_bytes([b[0], b[1]])))
            .collect::<Vec<u8>>(),
    ) {
        Some(img) => Ok(img),
        None => Err(CanvasError::SizeMismatch(width, height, data.len()).into()),
    }
}

/// DirectX DXGI_FORMAT_B4G4R4A4
pub(crate) fn to_bgra4444(img: &RgbaImage) -> (u32, u32, Vec<u8>) {
    (
        img.width(),
        img.height(),
        img.as_raw()
            .chunks(4)
            .flat_map(|b| join4444(b[0], b[1], b[2], b[3]).to_le_bytes())
            .collect::<Vec<u8>>(),
    )
}

/// DirectX DXGI_FORMAT_B5G6R5
pub(crate) fn from_rgb565(width: u32, height: u32, data: Vec<u8>) -> Result<RgbaImage> {
    assert!(data.len() % 2 == 0, "invald rgb565 size");
    match RgbaImage::from_raw(
        width,
        height,
        data.chunks(2)
            .flat_map(|b| {
                let rgb = split565(u16::from_le_bytes([b[0], b[1]]));
                [rgb[0], rgb[1], rgb[2], u8::MAX]
            })
            .collect::<Vec<u8>>(),
    ) {
        Some(img) => Ok(img),
        None => Err(CanvasError::SizeMismatch(width, height, data.len()).into()),
    }
}

/// DirectX DXGI_FORMAT_B5G6R5
pub(crate) fn to_rgb565(img: &RgbaImage) -> (u32, u32, Vec<u8>) {
    (
        img.width(),
        img.height(),
        img.as_raw()
            .chunks(4)
            .flat_map(|b| join565(b[0], b[1], b[2]).to_le_bytes())
            .collect::<Vec<u8>>(),
    )
}

/// This format just blows up an RGB565 image 16x
pub(crate) fn expand_rgb565(width: u32, height: u32, data: Vec<u8>) -> Result<RgbaImage> {
    let img = from_rgb565(width / 16, height / 16, data)?;
    println!("{:?}", img);
    Ok(DynamicImage::ImageRgba8(img)
        .resize(width, height, FilterType::Nearest)
        .into_rgba8())
}
