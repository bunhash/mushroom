//! Map Errors

use std::fmt;

/// Possible map errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Error pasting when the clipboard is empty
    ClipboardEmpty,

    /// Error when creating a new child node with a name that already exists at that position
    Duplicate(String),

    /// Invalid path. Typically only occurs when an empty path vector is provided.
    InvalidPath,

    /// No children to move to
    NoChildren,

    /// Error trying to move to the root's parent node
    NoParent,

    /// No sibling to move to. Could already be at the beginning or end of the linked list
    NoSibling,

    /// Error when the child node at the current position does not exist
    NotFound(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ClipboardEmpty => write!(f, "Clipboard is empty"),
            Error::Duplicate(name) => write!(f, "A node named {} already exists", name),
            Error::InvalidPath => write!(f, "Invalid path"),
            Error::NoChildren => write!(f, "No children to move to"),
            Error::NoParent => write!(f, "No parent to move to"),
            Error::NoSibling => write!(f, "No sibling to move to"),
            Error::NotFound(name) => write!(f, "Could not find {}", name),
        }
    }
}
