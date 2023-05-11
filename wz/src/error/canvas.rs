//! Canvas Error Types

use crate::types::WzInt;
use image::error::ImageError;
use std::fmt;

/// Possible canvas errors
#[derive(Debug)]
pub enum CanvasError {
    /// Unknown Canvas type
    EncodingFormat(WzInt),

    /// Image Errors
    Image(image::error::ImageError),

    /// Inflate
    Inflate(String),

    /// Size mismatch
    SizeMismatch(u32, u32, usize),

    /// Too big
    TooBig(u32, u32),
}

impl fmt::Display for CanvasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EncodingFormat(t) => write!(f, "Unknown encoding format: `{}`", **t),
            Self::Image(e) => write!(f, "Image: {}", e),
            Self::Inflate(s) => write!(f, "Inflate: {}", s),
            Self::SizeMismatch(w, h, l) => write!(
                f,
                "ImageBuffer size mismatch {{ Width({}), Height({}), PixelBytes({}) }}",
                w, h, l
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
