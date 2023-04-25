//! Map Errors

use crate::types::WzString;
use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error<'a> {
    DuplicateError(WzString),
    NotFound(&'a str),
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DuplicateError(name) => {
                write!(f, "A node named {} already exists", name.as_ref())
            }
            Error::NotFound(name) => write!(f, "Could not find {}", name),
        }
    }
}
