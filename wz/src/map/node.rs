//! Node in the map. Holds a name.

use crate::types::WzString;

pub struct MapNode<T> {
    pub name: WzString,
    pub data: T,
}

impl<T> MapNode<T> {
    pub fn new(name: WzString, data: T) -> Self {
        Self { name, data }
    }
}
