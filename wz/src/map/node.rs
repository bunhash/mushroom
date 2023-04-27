//! Node in the map. Holds a name.

use crate::{
    map::{Metadata, SizeHint},
    types::{WzInt, WzString},
};

/// Internal node structure
#[derive(Debug)]
pub struct MapNode<M, T>
where
    M: Metadata<T>,
    T: SizeHint,
{
    /// Name of the node
    pub(crate) name: WzString,

    /// Metadata of the [`MapNode`]'s contents
    pub(crate) metadata: Vec<M>,

    /// Data of the node
    pub(crate) data: T,
}

impl<M, T> MapNode<M, T>
where
    M: Metadata<T>,
    T: SizeHint,
{
    /// Creates a new node with the provided name and data
    pub(crate) fn new(name: WzString, data: T) -> Self {
        Self {
            name,
            metadata: Vec::new(),
            data,
        }
    }

    pub(crate) fn size_hint(&self) -> WzInt {
        self.data.size_hint(self.metadata.len())
    }
}
