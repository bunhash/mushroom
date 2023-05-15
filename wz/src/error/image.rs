//! Image Error Types

use std::fmt;

/// Possible image errors
#[derive(Debug)]
pub enum ImageError {
    /// The Image root must be a [`Property`](crate::types::image::Node::Property)
    ImageRoot,

    /// Unknown Object Type
    ObjectType(String),

    /// Unknown Property Type
    PropertyType(u8),

    /// Unknown UOL type
    UolType(u8),
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImageRoot => write!(f, "The root of the image is not a property"),
            Self::ObjectType(t) => write!(f, "Unknown Object type: `{}`", t),
            Self::PropertyType(t) => write!(f, "Unknown Property type: `{}`", t),
            Self::UolType(t) => write!(f, "Unknown UOL type: `{}`", t),
        }
    }
}
