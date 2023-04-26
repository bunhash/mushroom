//! Map Errors

use std::fmt;

/// Possible map errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Error trying to move to the root's parent node
    AlreadyRoot,

    /// Error pasting when the clipboard is empty
    ClipboardEmpty,

    /// Error when creating a new child node with a name that already exists at that position
    DuplicateError(String),

    /// Invalid path. Typically only occurs when an empty path vector is provided.
    InvalidPath,

    /// Error when the child node at the current position does not exist
    NotFound(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AlreadyRoot => write!(f, "Cursor is already at root node"),
            Error::ClipboardEmpty => write!(f, "Clipboard is empty"),
            Error::DuplicateError(name) => write!(f, "A node named {} already exists", name),
            Error::InvalidPath => write!(f, "Invalid path"),
            Error::NotFound(name) => write!(f, "Could not find {}", name),
        }
    }
}
