//! Xml Error Types

use std::fmt;

/// Possible XML errors
#[derive(Debug)]
pub enum XmlError {
    /// XML reading errors
    Read(xml::reader::Error),

    /// XML writing errors
    Write(xml::writer::Error),
}

impl fmt::Display for XmlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read(e) => write!(f, "Read: {}", e),
            Self::Write(e) => write!(f, "Write: {}", e),
        }
    }
}

impl From<xml::reader::Error> for XmlError {
    fn from(other: xml::reader::Error) -> Self {
        Self::Read(other)
    }
}

impl From<xml::writer::Error> for XmlError {
    fn from(other: xml::writer::Error) -> Self {
        Self::Write(other)
    }
}
