//! Trait for map metadata

use crate::types::WzInt;

/// Metadata for [`Map`](crate::map::Map) structures. The idea of this metadata is allow a
/// mechanism for updating internal metadata of each node when the structure changes. Thereforce,
/// once the data has been added to the [`Map`](crate::map::Map), it can only be changed by
/// utilizing [`Map`](crate::map::Map) functions.
pub trait Metadata {
    /// Allows the node to update its own metadata--specifically the
    /// [`SizeHint`](crate::map::SizeHint) trait.
    fn update(&mut self, name: &str, children_sizes: &[WzInt]);
}
