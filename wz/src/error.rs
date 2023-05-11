//! Errors

use std::{fmt, io, string};

mod canvas;
mod decode;
mod image;
mod map;
mod package;
mod xml;

pub use self::image::ImageError;
pub use self::xml::XmlError;
pub use canvas::CanvasError;
pub use decode::DecodeError;
pub use map::MapError;
pub use package::PackageError;

pub type Result<T> = std::result::Result<T, Error>;

/// Overall Error catcher
#[derive(Debug)]
pub enum Error {
    /// Canvas errors
    Canvas(CanvasError),

    /// Decoding errors
    Decode(DecodeError),

    /// Image errors
    Image(ImageError),

    /// IO errors
    Io(io::ErrorKind),

    /// Map errors
    Map(MapError),

    /// Package errors
    Package(PackageError),

    /// XML errors
    Xml(XmlError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Canvas(e) => write!(f, "Canvas: {}", e),
            Self::Decode(e) => write!(f, "Decode: {}", e),
            Self::Image(e) => write!(f, "Image: {}", e),
            Self::Io(kind) => write!(f, "IO: {}", kind),
            Self::Map(e) => write!(f, "Map: {}", e),
            Self::Package(e) => write!(f, "Package: {}", e),
            Self::Xml(e) => write!(f, "XML: {}", e),
        }
    }
}

impl From<CanvasError> for Error {
    fn from(other: CanvasError) -> Self {
        Error::Canvas(other)
    }
}

impl From<::image::error::ImageError> for Error {
    fn from(other: ::image::error::ImageError) -> Self {
        Self::Canvas(other.into())
    }
}

impl From<DecodeError> for Error {
    fn from(other: DecodeError) -> Self {
        Error::Decode(other)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(other: string::FromUtf8Error) -> Self {
        Self::Decode(other.into())
    }
}

impl From<string::FromUtf16Error> for Error {
    fn from(other: string::FromUtf16Error) -> Self {
        Self::Decode(other.into())
    }
}

impl From<ImageError> for Error {
    fn from(other: ImageError) -> Self {
        Error::Image(other)
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other.kind())
    }
}

impl From<io::ErrorKind> for Error {
    fn from(other: io::ErrorKind) -> Self {
        Error::Io(other)
    }
}

impl From<MapError> for Error {
    fn from(other: MapError) -> Self {
        Error::Map(other)
    }
}

impl From<PackageError> for Error {
    fn from(other: PackageError) -> Self {
        Error::Package(other)
    }
}

impl From<XmlError> for Error {
    fn from(other: XmlError) -> Self {
        Error::Xml(other)
    }
}

impl From<::xml::reader::Error> for Error {
    fn from(other: ::xml::reader::Error) -> Self {
        Self::Xml(other.into())
    }
}

impl From<::xml::writer::Error> for Error {
    fn from(other: ::xml::writer::Error) -> Self {
        Self::Xml(other.into())
    }
}
