//! Parsed Canvas type

use crate::{
    error::{CanvasError, Result},
    io::xml::writer::ToXml,
    types::WzInt,
};
use image::{ImageFormat, RgbaImage};
use inflate::inflate_bytes_zlib;
use std::{fmt, path::Path};

mod conversions;
mod dxt;

pub(crate) use conversions::*;
pub(crate) use dxt::*;

#[derive(Clone, PartialEq, Eq)]
pub struct Canvas {
    width: WzInt,
    height: WzInt,
    format: WzInt,
    format2: u8,
    data: Vec<u8>,
}

impl Canvas {
    pub fn new(width: WzInt, height: WzInt, format: WzInt, format2: u8, data: Vec<u8>) -> Self {
        Self {
            width,
            height,
            format,
            format2,
            data,
        }
    }

    pub fn width(&self) -> WzInt {
        self.width
    }

    pub fn height(&self) -> WzInt {
        self.height
    }

    pub fn format(&self) -> WzInt {
        self.format + self.format2.into()
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn decompressed_data(&self) -> Result<Vec<u8>> {
        match inflate_bytes_zlib(&self.data) {
            Ok(d) => Ok(d),
            Err(e) => Err(CanvasError::Inflate(e).into()),
        }
    }

    pub fn image_buffer(&self) -> Result<RgbaImage> {
        encode_image(self)
    }

    pub fn save_to_disk<S>(&self, path: &S, format: ImageFormat) -> Result<()>
    where
        S: AsRef<Path>,
    {
        Ok(self.image_buffer()?.save_with_format(&path, format)?)
    }
}

impl fmt::Debug for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Canvas {{ width: {:?}, height: {:?}, format: {:?}, data: [..] }}",
            self.width, self.height, self.format,
        )
    }
}

impl ToXml for Canvas {
    fn tag(&self) -> &'static str {
        "canvas"
    }

    fn attributes(&self, name: &str) -> Vec<(String, String)> {
        vec![
            (String::from("name"), name.to_string()),
            (String::from("width"), self.width.to_string()),
            (String::from("height"), self.height.to_string()),
        ]
    }
}

fn encode_image(canvas: &Canvas) -> Result<RgbaImage> {
    let width = *canvas.width() as u32;
    let height = *canvas.height() as u32;
    let data = canvas.decompressed_data()?;
    let data_len = data.len();
    match *canvas.format() {
        1 => from_bgra4444(width, height, data),
        2 => match RgbaImage::from_raw(width, height, data) {
            Some(img) => Ok(img),
            None => Err(CanvasError::SizeMismatch(width, height, data_len).into()),
        },
        513 => from_rgb565(width, height, data),
        517 => expand_rgb565(width, height, data),
        1026 => from_dxt3(width, height, data),
        t => Err(CanvasError::EncodingFormat(WzInt::from(t)).into()),
    }
}
