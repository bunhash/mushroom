//! Node in the map. Holds a name.

use crate::{
    map::{Metadata, SizeHint},
    types::{WzInt, WzString},
};

/// Internal node structure
#[derive(Debug)]
pub struct MapNode<T>
where
    T: Metadata + SizeHint,
{
    /// Name of the node
    pub(crate) name: WzString,

    /// Data of the node
    pub(crate) data: T,
}

impl<T> MapNode<T>
where
    T: Metadata + SizeHint,
{
    /// Creates a new node with the provided name and data
    pub(crate) fn new(name: WzString, data: T) -> Self {
        Self { name, data }
    }
}

impl<T> Metadata for MapNode<T>
where
    T: Metadata + SizeHint,
{
    fn update<S>(&mut self, children: &[&S])
    where
        S: SizeHint,
    {
        self.data.update(children);
    }
}

impl<T> SizeHint for MapNode<T>
where
    T: Metadata + SizeHint,
{
    fn size_hint(&self) -> WzInt {
        self.data.size_hint()
    }
}
