//! Parsed Canvas type

use crate::error::{CanvasError, Result};
use crate::io::{xml::writer::ToXml, Decode, Encode, WzRead, WzWrite};
use crate::types::{VerboseDebug, WzInt};
use deflate::deflate_bytes_zlib;
use image::{ImageFormat, RgbaImage};
use inflate::inflate_bytes_zlib;
use std::{fmt, io, path::Path};

mod conversions;
mod squish;

pub(crate) use self::squish::*;
pub(crate) use conversions::*;

/// Canvas Image format types.
///
/// This is non-exhaustive. I stopped at v172 and later versions have more formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasFormat {
    Bgra4444,
    Bgra8888,
    Rgb565,
    CompressedRgb565,
    Bc3,
}

impl CanvasFormat {
    /// Returns the [`WzInt`] representation
    pub fn to_int(&self) -> WzInt {
        match self {
            CanvasFormat::Bgra4444 => WzInt::from(1),
            CanvasFormat::Bgra8888 => WzInt::from(2),
            CanvasFormat::Rgb565 => WzInt::from(513),
            CanvasFormat::CompressedRgb565 => WzInt::from(517),
            CanvasFormat::Bc3 => WzInt::from(1026),
        }
    }

    /// Tries to make a CanvasFormat from a WzInt
    pub fn from_int(val: WzInt) -> Result<Self> {
        match *val {
            1 => Ok(CanvasFormat::Bgra4444),
            2 => Ok(CanvasFormat::Bgra8888),
            513 => Ok(CanvasFormat::Rgb565),
            517 => Ok(CanvasFormat::CompressedRgb565),
            1026 => Ok(CanvasFormat::Bc3),
            _ => Err(CanvasError::EncodingFormat(val, 0).into()),
        }
    }
}

impl Decode for CanvasFormat {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: WzRead + ?Sized,
    {
        let format = WzInt::decode(reader)?;
        let format2 = u8::decode(reader)?;
        match (*format, format2) {
            (1, 0) => Ok(Self::Bgra4444),
            (2, 0) => Ok(Self::Bgra8888),
            (513, 0) => Ok(Self::Rgb565),
            (513, 4) => Ok(Self::CompressedRgb565),
            (1026, 0) => Ok(Self::Bc3),
            _ => Err(CanvasError::EncodingFormat(format, format2).into()),
        }
    }
}

impl Encode for CanvasFormat {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite + ?Sized,
    {
        match self {
            CanvasFormat::Bgra4444 => {
                WzInt::from(1).encode(writer)?;
                0u8.encode(writer)?;
            }
            CanvasFormat::Bgra8888 => {
                WzInt::from(2).encode(writer)?;
                0u8.encode(writer)?;
            }
            CanvasFormat::Rgb565 => {
                WzInt::from(513).encode(writer)?;
                0u8.encode(writer)?;
            }
            CanvasFormat::CompressedRgb565 => {
                WzInt::from(513).encode(writer)?;
                4u8.encode(writer)?;
            }
            CanvasFormat::Bc3 => {
                WzInt::from(1026).encode(writer)?;
                0u8.encode(writer)?;
            }
        }
        Ok(())
    }
}

/// Canvas objects that hold graphics data.
///
/// Later version of MS will have empty canvases that point to other canvas objects as reference.
/// This is useful to cut down on unnecessary duplicated data. So if a plain white PNG is saved, it
/// is likely, the source is elsewhere in the WZ image.
#[derive(Clone, PartialEq, Eq)]
pub struct Canvas {
    width: WzInt,
    height: WzInt,
    format: CanvasFormat,
    data: Vec<u8>,
}

impl Canvas {
    /// Creates a new [`Canvas`]
    pub fn new(width: WzInt, height: WzInt, format: CanvasFormat, data: Vec<u8>) -> Self {
        Self {
            width,
            height,
            format,
            data,
        }
    }

    /// Creates a new [`Canvas`] from a provided image and encoding format
    pub fn from_image<S>(path: S, format: CanvasFormat) -> Result<Self>
    where
        S: AsRef<Path>,
    {
        let img = image::io::Reader::open(path)?.decode()?;
        let (width, height, data) = encode_image(format, img.into_rgba8())?;
        Ok(Self::new(
            width.into(),
            height.into(),
            format,
            deflate_bytes_zlib(&data),
        ))
    }

    /// Returns the width of the image
    pub fn width(&self) -> WzInt {
        self.width
    }

    /// Returns the height of the image
    pub fn height(&self) -> WzInt {
        self.height
    }

    /// Returns the format of the image
    pub fn format(&self) -> CanvasFormat {
        self.format
    }

    /// Returns a reference to the raw data
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns a vector of the decompressed raw data
    pub fn decompressed_data(&self) -> Result<Vec<u8>> {
        match inflate_bytes_zlib(&self.data) {
            Ok(d) => Ok(d),
            Err(e) => Err(CanvasError::Inflate(e).into()),
        }
    }

    /// Returns the decoded image data
    pub fn image_buffer(&self) -> Result<RgbaImage> {
        decode_image(self)
    }

    /// Saves the image to file
    pub fn save_to_file<S>(&self, path: &S, format: ImageFormat) -> Result<()>
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

impl VerboseDebug for Canvas {
    fn debug(&self, f: &mut dyn io::Write) -> io::Result<()> {
        f.write_fmt(format_args!(
            "Canvas {{ width: {:?}, height: {:?}, format: {:?}, data: {:x?} }}",
            self.width, self.height, self.format, self.data
        ))
    }
}

impl Encode for Canvas {
    fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: WzWrite + ?Sized,
    {
        self.width.encode(writer)?;
        self.height.encode(writer)?;
        self.format.encode(writer)?;
        0i32.encode(writer)?;
        (self.data.len() as i32 + 1).encode(writer)?;
        0u8.encode(writer)?;
        writer.write_all(&self.data)
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

fn encode_image(format: CanvasFormat, img: RgbaImage) -> Result<(u32, u32, Vec<u8>)> {
    match format {
        CanvasFormat::Bgra4444 => Ok(to_bgra4444(img)),
        CanvasFormat::Bgra8888 => Ok(to_bgra8888(img)),
        CanvasFormat::Rgb565 => Ok(to_rgb565(img)),
        CanvasFormat::CompressedRgb565 => compress_rgb565(img),
        CanvasFormat::Bc3 => to_bc3(img),
    }
}

fn decode_image(canvas: &Canvas) -> Result<RgbaImage> {
    let width = *canvas.width() as u32;
    let height = *canvas.height() as u32;
    let data = canvas.decompressed_data()?;
    match canvas.format() {
        CanvasFormat::Bgra4444 => from_bgra4444(width, height, data),
        CanvasFormat::Bgra8888 => from_bgra8888(width, height, data),
        CanvasFormat::Rgb565 => from_rgb565(width, height, data),
        CanvasFormat::CompressedRgb565 => expand_rgb565(width, height, data),
        CanvasFormat::Bc3 => from_bc3(width, height, data),
    }
}
