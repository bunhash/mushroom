//! DXT* Compressed Images

use crate::{
    error::{CanvasError, Result},
    file::image::canvas::{join565, ratio, split565},
};
use image::{Pixel, Rgb, Rgba, RgbaImage};

const MAX_44_SIZE: u8 = 15;

#[inline]
pub(crate) fn split44(byte: u8) -> [u8; 2] {
    [
        ratio(byte & 0xf, MAX_44_SIZE, u8::MAX),
        ratio(byte.wrapping_shr(4), MAX_44_SIZE, u8::MAX),
    ]
}

#[inline]
pub(crate) fn join44(a: u8, b: u8) -> u8 {
    ratio(b, u8::MAX, MAX_44_SIZE).wrapping_shl(4) | ratio(a, u8::MAX, MAX_44_SIZE)
}

#[inline]
pub(crate) fn split2222(byte: u8) -> [u8; 4] {
    [
        byte & 0x3,
        byte.wrapping_shr(2) & 0x3,
        byte.wrapping_shr(4) & 0x3,
        byte.wrapping_shr(6) & 0x3,
    ]
}

#[inline]
pub(crate) fn join2222(a: u8, b: u8, c: u8, d: u8) -> u8 {
    (d.wrapping_shl(6) & 0xc0) | (c.wrapping_shl(4) & 0x30) | (b.wrapping_shl(2) & 0xc) | (a & 0x3)
}

#[inline]
pub(crate) fn expand_colors(u0: u16, u1: u16) -> [Rgb<u8>; 4] {
    let c0 = Rgb(split565(u0));
    let c1 = Rgb(split565(u1));
    let (c2, c3) = if u0 > u1 {
        (
            c0.map2(&c1, |b0, b1| (((2 * b0 as usize) + b1 as usize) / 3) as u8),
            c0.map2(&c1, |b0, b1| (((2 * b1 as usize) + b0 as usize) / 3) as u8),
        )
    } else {
        (
            c0.map2(&c1, |b0, b1| ((b0 as usize + b1 as usize) / 2) as u8),
            Rgb([0u8, 0u8, 0u8]),
        )
    };
    [c0, c1, c2, c3]
}

/// DirectX DXGI_FORMAT_BC3
pub(crate) fn from_dxt3(width: u32, height: u32, data: Vec<u8>) -> Result<RgbaImage> {
    let mut img = RgbaImage::new(width, height);

    assert!(width % 4 == 0, "is this image DXT3?");
    assert!(height % 4 == 0, "is this image DXT3?");
    assert!(data.len() % 16 == 0, "is this image DXT3?");

    for y in (0..height).step_by(4) {
        for x in (0..width).step_by(4) {
            let offset = ((x * 4) + (y * width)) as usize;
            let slice = &data[offset..offset + 16];

            // Get the pixel palette from the 16 bytes
            let alphas = slice[0..8]
                .iter()
                .flat_map(|b| split44(*b))
                .collect::<Vec<u8>>();
            let colors = expand_colors(
                u16::from_le_bytes([slice[8], slice[9]]),
                u16::from_le_bytes([slice[10], slice[11]]),
            );
            let color_index = slice[12..16]
                .iter()
                .flat_map(|b| split2222(*b))
                .collect::<Vec<u8>>();

            // Set the pixels
            for j in 0..4 {
                for i in 0..4 {
                    let idx = ((j * 4) + i) as usize;
                    let mut color = colors[color_index[idx] as usize].to_rgba();
                    color.channels_mut()[3] = alphas[idx];
                    img.put_pixel(x + i, y + j, color);
                }
            }
        }
    }

    Ok(img)
}

/// DirectX DXGI_FORMAT_BC3
pub(crate) fn to_dxt3(img: &RgbaImage) -> (u32, u32, Vec<u8>) {
    unimplemented!()
}
