//! Canvas Error Types

use crate::types::{CanvasFormat, WzInt};
use image::error::ImageError;
use std::fmt;

/// Possible canvas errors
#[derive(Debug)]
pub enum CanvasError {
    /// Unknown Canvas type
    EncodingFormat(WzInt, u8),

    /// Image Errors
    Image(image::error::ImageError),

    /// Inflate
    Inflate(String),

    /// Size mismatch
    SizeMismatch(CanvasFormat, u32, u32, usize),

    /// Too big
    TooBig(u32, u32),
}

impl fmt::Display for CanvasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EncodingFormat(t, t2) => {
                write!(f, "Unknown encoding format: `({}, {})`", **t, *t2)
            }
            Self::Image(e) => write!(f, "Image: {}", e),
            Self::Inflate(s) => write!(f, "Inflate: {}", s),
            Self::SizeMismatch(c, w, h, l) => write!(
                f,
                "Data length does not match Canvas Size {{ {:?}, Width({}), Height({}), PixelBytes({}) }}",
                c, w, h, l
            ),
            Self::TooBig(w, h) => write!(f, "canvas is too big: {{ Width({}), Height({}) }}", w, h),
        }
    }
}

impl From<ImageError> for CanvasError {
    fn from(other: ImageError) -> Self {
        Self::Image(other)
    }
}
