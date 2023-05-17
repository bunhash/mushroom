//! Image Error Types

use std::fmt;

/// Possible image errors
#[derive(Debug)]
pub enum ImageError {
    /// The Image root must be a [`ImgDir`](crate::types::Property::ImgDir)
    ImageRoot,

    /// Image name mismatch
    Name(String, String),

    /// Unknown Object Type
    ObjectType(String),

    /// Path
    Path(String),

    /// Cannot construct property
    Property(String),

    /// Unknown Property Type
    PropertyType(u8),

    /// Unknown UOL type
    UolType(u8),

    /// Error parsing value from string
    Value(String),
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImageRoot => write!(f, "The root of the image is not a property"),
            Self::Name(e, v) => write!(f, "Expected the image to be called {}, found {}", e, v),
            Self::ObjectType(t) => write!(f, "Unknown Object type: `{}`", t),
            Self::Path(p) => write!(f, "Invalid path: `{}`", p),
            Self::Property(s) => write!(f, "Cannot construct property: `{}`", s),
            Self::PropertyType(t) => write!(f, "Unknown Property type: `{}`", t),
            Self::UolType(t) => write!(f, "Unknown UOL type: `{}`", t),
            Self::Value(s) => write!(f, "Value cannot be parsed: `{}`", s),
        }
    }
}
