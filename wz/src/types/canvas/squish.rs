//! BC* Compressed Images

use crate::{
    error::{CanvasError, Result},
    types::CanvasFormat,
};
use image::{Pixel, RgbaImage};
use squish::{Format, Params};

fn from_bc(format: Format, width: usize, height: usize, data: Vec<u8>) -> Result<RgbaImage> {
    let mut output = vec![0u8; width * height * 4];
    format.decompress(&data, width, height, &mut output);
    RgbaImage::from_raw(width as u32, height as u32, output).ok_or_else(|| {
        CanvasError::SizeMismatch(CanvasFormat::Bc3, width as u32, height as u32, data.len()).into()
    })
}

fn to_bc(format: Format, width: usize, height: usize, data: Vec<u8>) -> (u32, u32, Vec<u8>) {
    let output_size = format.compressed_size(width, height);
    let mut output = vec![0u8; output_size];
    format.compress(&data, width, height, Params::default(), &mut output);
    (width as u32, height as u32, output)
}

/// DirectX DXGI_FORMAT_BC3
pub(crate) fn from_bc3(width: u32, height: u32, data: Vec<u8>) -> Result<RgbaImage> {
    if width % 4 != 0 || height % 4 != 0 {
        return Err(CanvasError::SizeMismatch(CanvasFormat::Bc3, width, height, data.len()).into());
    }
    let data_len = (width * height) as usize;
    if data.len() < data_len {
        return Err(CanvasError::SizeMismatch(CanvasFormat::Bc3, width, height, data.len()).into());
    }
    from_bc(
        Format::Bc3,
        width as usize,
        height as usize,
        Vec::from(&data[0..data_len]),
    )
}

/// DirectX DXGI_FORMAT_BC3
pub(crate) fn to_bc3(img: RgbaImage) -> Result<(u32, u32, Vec<u8>)> {
    let (width, height) = img.dimensions();
    if width % 4 != 0 || height % 4 != 0 {
        return Err(CanvasError::SizeMismatch(
            CanvasFormat::Bc3,
            width,
            height,
            img.pixels().count() * 4,
        )
        .into());
    }
    Ok(to_bc(
        Format::Bc3,
        width as usize,
        height as usize,
        img.pixels()
            .flat_map(|p| {
                let rgba = p.channels();
                [rgba[0], rgba[1], rgba[2], rgba[3]]
            })
            .collect::<Vec<u8>>(),
    ))
}

/*
 * Not supported in stable squish crate yet
 *
 * /// DirectX DXGI_FORMAT_BC5
 * pub(crate) fn from_bc5(width: u32, height: u32, data: Vec<u8>) -> Result<RgbaImage> {
 *     assert!(width % 4 == 0, "is this image BC5?");
 *     assert!(height % 4 == 0, "is this image BC5?");
 *     assert!(data.len() % 16 == 0, "is this image BC5?");
 *     from_bc(Format::Bc5, width as usize, height as usize, data)
 * }
 *
 * /// DirectX DXGI_FORMAT_BC5
 * pub(crate) fn to_bc3(img: RgbaImage) -> (u32, u32, Vec<u8>) {
 *     let (width, height) = img.dimensions();
 *     assert!(width % 4 == 0, "is this image BC5?");
 *     assert!(height % 4 == 0, "is this image BC5?");
 *     to_bc(Format::Bc5, width as usize, height as usize, img.into_raw())
 * }
 */
