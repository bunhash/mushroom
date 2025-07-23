//! Generic map of WZ Archive and Image structures

use crate::error::MapError;
use indextree::{Arena, NodeId};
use std::path::Path;

mod children;
mod cursor;
mod cursor_mut;
mod node;

pub use children::{ChildNames, Children};
pub use cursor::Cursor;
pub use cursor_mut::CursorMut;
pub use indextree::DebugPrettyPrint;
pub use node::MapNode;

use std::fmt::Debug;

/// A named tree structure. Each node in the tree is given a name. The full path name is guaranteed
/// to be unique.
#[derive(Debug)]
pub struct Map<T> {
    arena: Arena<MapNode<T>>,
    root: NodeId,
}

impl<T> Map<T> {
    /// Creates a new map with the provided root data
    pub fn new(name: String, data: T) -> Self {
        let mut arena = Arena::new();
        let root = arena.new_node(MapNode::new(name, data));
        Self { arena, root }
    }

    /// Creates a cursor inside the root that has read-only access to the map data
    pub fn cursor(&self) -> Cursor<'_, T> {
        Cursor::new(self.root, &self.arena)
    }

    /// Creates a cursor inside the root that has mutable access to the map data
    pub fn cursor_mut(&mut self) -> CursorMut<'_, T> {
        CursorMut::new(self.root, &mut self.arena)
    }

    /// Creates a cursor inside the root that has read-only access to the map data
    pub fn cursor_at<S>(&self, path: S) -> Result<Cursor<'_, T>, MapError>
    where
        S: AsRef<Path>,
    {
        Ok(Cursor::new(self.get_id(path)?, &self.arena))
    }

    /// Creates a cursor inside the root that has mutable access to the map data
    pub fn cursor_mut_at<S>(&mut self, path: S) -> Result<CursorMut<T>, MapError>
    where
        S: AsRef<Path>,
    {
        Ok(CursorMut::new(self.get_id(path)?, &mut self.arena))
    }

    /// Returns the name of the root node
    pub fn name(&self) -> &str {
        self.arena
            .get(self.root)
            .expect("get() node should exist")
            .get()
            .name
            .as_ref()
    }

    /// Renames the root node
    pub fn rename(&mut self, name: String) -> Result<(), MapError> {
        self.cursor_mut().rename(name)?;
        Ok(())
    }

    /// Gets the data at the path. Errors when the node does not exist.
    pub fn get<S>(&self, path: S) -> Result<&T, MapError>
    where
        S: AsRef<Path>,
    {
        Ok(&self
            .arena
            .get(self.get_id(path)?)
            .expect("get() node should exist")
            .get()
            .data)
    }

    /// Walks the map depth-first
    pub fn walk<E>(&self, closure: impl FnMut(Cursor<T>) -> Result<(), E>) -> Result<(), E>
    where
        E: Debug,
    {
        self.cursor().walk(closure)
    }

    /// Creates a printable string of the tree structure. To be used in `{:?}` formatting.
    pub fn debug_pretty_print(&self) -> DebugPrettyPrint<'_, MapNode<T>> {
        self.root.debug_pretty_print(&self.arena)
    }

    // *** PRIVATES *** //

    fn get_id<S>(&self, path: S) -> Result<NodeId, MapError>
    where
        S: AsRef<Path>,
    {
        let mut it = path.as_ref().iter();
        let mut cursor = match it.next() {
            Some(root) => {
                if root.to_string_lossy() == self.name() {
                    self.cursor()
                } else {
                    return Err(MapError::Path(path.as_ref().to_string_lossy().into()));
                }
            }
            _ => return Err(MapError::Path(path.as_ref().to_string_lossy().into())),
        };
        for name in it {
            cursor.move_to(&name.to_string_lossy())?;
        }
        Ok(cursor.position)
    }
}

#[cfg(test)]
mod tests {

    use crate::map::Map;

    #[test]
    fn make_map() {
        let mut map = Map::new(String::from("root"), 100);
        let mut cursor = map.cursor_mut();
        assert_eq!(cursor.name(), "root");
        cursor
            .rename(String::from("n1"))
            .expect("rename should work");
        assert_eq!(cursor.name(), "n1");
        let children: &[&str] = &[];
        assert_eq!(&cursor.list().collect::<Vec<&str>>(), children);
        // let cursor die
        assert_eq!(map.name(), "n1");
    }

    #[test]
    fn get_uri() {
        let mut map = Map::new(String::from("n1"), 100);
        // arena ownership moves to cursor in the next line.
        let mut cursor = map.cursor_mut();
        cursor
            .create(String::from("n1_1"), 150)
            .expect("error creating n1_1")
            .move_to("n1_1")
            .expect("error moving into n1_1")
            .create(String::from("n1_1_1"), 155)
            .expect("error creating n1_1_1")
            .create(String::from("n1_1_2"), 175)
            .expect("error creating n1_1_1")
            .move_to("n1_1_1")
            .expect("error moving into n1_1_1")
            .create(String::from("n1_1_1_1"), 255)
            .expect("error creating n1_1_1_1")
            .move_to("n1_1_1_1")
            .expect("error moving into n1_1_1_1");
        assert_eq!(&cursor.pwd(), "n1/n1_1/n1_1_1/n1_1_1_1");
        // `cursor` dies here--arena ownership moves back to map in the next line.
        assert_eq!(
            *map.get("n1/n1_1/n1_1_1/n1_1_1_1")
                .expect("error getting uri"),
            255
        );
        assert!(map.get("n1/n1_1/fail").is_err());
    }
}
