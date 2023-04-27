//! Node in the map. Holds a name.

use crate::{
    map::SizeHint,
    types::{WzInt, WzString},
};

/// Internal node structure
#[derive(Debug)]
pub struct MapNode<T>
where
    T: SizeHint,
{
    /// Name of the node
    pub(crate) name: WzString,

    /// Data of the node
    pub(crate) data: T,
}

impl<T> MapNode<T>
where
    T: SizeHint,
{
    /// Creates a new node with the provided name and data
    pub(crate) fn new(name: WzString, data: T) -> Self {
        Self { name, data }
    }

    pub(crate) fn data_size(&self) -> WzInt {
        self.data.data_size()
    }

    pub(crate) fn header_size(&self, num_children: usize) -> WzInt {
        self.data.header_size(num_children)
    }

    pub(crate) fn metadata_size(&self) -> WzInt {
        self.data.metadata_size(self.name.as_ref())
    }
}
