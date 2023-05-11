//! Map Error Types

use std::fmt;

/// Possible map errors
#[derive(Debug)]
pub enum MapError {
    /// Error pasting when the clipboard is empty
    ClipboardEmpty,

    /// Error when creating a new child node with a name that already exists at that position
    Duplicate(String),

    /// No children to move to
    NoChildren,

    /// Error trying to move to the root's parent node
    NoParent,

    /// No sibling to move to. Could already be at the beginning or end of the linked list
    NoSibling,

    /// Error when the child node at the current position does not exist
    NotFound(String),

    /// Invalid path. Typically only occurs when an empty path vector is provided.
    Path(String),
}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClipboardEmpty => write!(f, "Clipboard is empty"),
            Self::Duplicate(name) => write!(f, "A node named {} already exists", name),
            Self::NoChildren => write!(f, "No children to move to"),
            Self::NoParent => write!(f, "No parent to move to"),
            Self::NoSibling => write!(f, "No sibling to move to"),
            Self::NotFound(name) => write!(f, "Could not find {}", name),
            Self::Path(p) => write!(f, "Invalid path: `{}`", p),
        }
    }
}
