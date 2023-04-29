//! Trait for map metadata

use crate::types::{WzInt, WzString};

/// Metadata for [`Map`](crate::map::Map) structures. The idea of this metadata is allow a
/// mechanism for updating internal metadata of each node when the structure changes. It is heavily
/// advised to not modify the name outside of [`CursorMut`](crate::map::CursorMut). If any other
/// fields are modified outside of [`CursorMut`](crate::map::CursorMut), a
/// [`send_update`](crate::map::CursorMut::send_update) call is needed to accurately update the
/// metadata of each ancestor node.
pub trait Metadata {
    /// Returns the name of the content
    fn name(&self) -> &str;

    /// Renames the content. The validity checking occurs in [`CursorMut`](crate::map::CursorMut).
    /// It is advised not to change the name outside of [`CursorMut`](crate::map::CursorMut) since
    /// it will bypass the internal checks and can result in multiple nodes with the same name.
    fn rename(&mut self, new_name: WzString);

    /// Allows the node to update its own metadata--specifically the
    /// [`SizeHint`](crate::map::SizeHint) trait.
    fn update(&mut self, children_sizes: &[WzInt]);
}
