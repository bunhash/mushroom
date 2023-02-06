use std::fmt;

/// Result used within the tree crate
pub type NodeResult<T> = Result<T, NodeError>;

/// Possible node failures
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeError {
    InUse,
    Removed,
    NotNamed,
}

impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            NodeError::InUse => "Attempted to insert a node already in use",
            NodeError::Removed => "Attempted to access a freed node",
            NodeError::NotNamed => "Attempted to insert a node with no name",
        })
    }
}
