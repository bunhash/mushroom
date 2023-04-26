//! Node in the map. Holds a name.

use crate::types::WzString;

/// Internal node structure
#[derive(Debug)]
pub struct MapNode<T> {
    /// Name of the node
    pub(crate) name: WzString,

    /// Data of the node
    pub(crate) data: T,
}

impl<T> MapNode<T> {
    /// Creates a new node with the provided name and data
    pub(crate) fn new(name: WzString, data: T) -> Self {
        Self { name, data }
    }
}
