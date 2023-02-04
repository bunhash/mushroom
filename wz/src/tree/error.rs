use std::fmt;

/// Result used within the tree crate
pub type NodeResult<T> = Result<T, NodeError>;

/// Possible node failures
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeError {
    InsertBeforeSelf,
    InsertAfterSelf,
    Removed,
}

impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            NodeError::InsertBeforeSelf => "Attempted to insert a node as its own ancestor",
            NodeError::InsertAfterSelf => "Attemped to insert a node as its own descendent",
            NodeError::Removed => "Attempted to access a freed node",
        })
    }
}
