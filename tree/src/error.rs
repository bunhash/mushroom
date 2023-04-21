use std::fmt;

/// Result used within the tree crate
pub type Result<T> = core::result::Result<T, Error>;

/// Possible node failures
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InUse,
    Removed,
    NotNamed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::InUse => "Attempted to insert a node already in use",
            Error::Removed => "Attempted to access a freed node",
            Error::NotNamed => "Attempted to insert a node with no name",
        })
    }
}
