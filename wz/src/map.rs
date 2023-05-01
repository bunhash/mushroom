//! Generic map of WZ Package and Image structures

use crate::types::WzString;

use indextree::{Arena, NodeId};

mod children;
mod cursor;
mod cursor_mut;
mod error;
mod metadata;
mod node;
mod size_hint;

pub use children::{ChildNames, Children};
pub use cursor::Cursor;
pub use cursor_mut::CursorMut;
pub use error::Error;
pub use indextree::DebugPrettyPrint;
pub use metadata::Metadata;
pub use node::MapNode;
pub use size_hint::SizeHint;

use std::fmt::Debug;

/// A named tree structure. Each node in the tree is given a name. The full path name is guaranteed
/// to be unique.
#[derive(Debug)]
pub struct Map<T>
where
    T: Metadata + SizeHint,
{
    arena: Arena<MapNode<T>>,
    root: NodeId,
}

impl<T> Map<T>
where
    T: Metadata + SizeHint,
{
    /// Creates a new map with the provided root data
    pub fn new(name: WzString, data: T) -> Self {
        let mut arena = Arena::new();
        let root = arena.new_node(MapNode::new(name, data));
        Self { arena, root }
    }

    /// Creates a cursor inside the root that has read-only access to the map data
    pub fn cursor<'a>(&'a self) -> Cursor<'a, T> {
        Cursor::new(self.root, &self.arena)
    }

    /// Creates a cursor inside the root that has mutable access to the map data
    pub fn cursor_mut<'a>(&'a mut self) -> CursorMut<'a, T> {
        CursorMut::new(self.root, &mut self.arena)
    }

    /// Creates a cursor inside the root that has read-only access to the map data
    pub fn cursor_at<'a>(&'a self, uri: &[&str]) -> Result<Cursor<'a, T>, Error> {
        Ok(Cursor::new(self.get_id(uri)?, &self.arena))
    }

    /// Creates a cursor inside the root that has mutable access to the map data
    pub fn cursor_mut_at<'a>(&'a mut self, uri: &[&str]) -> Result<CursorMut<'a, T>, Error> {
        Ok(CursorMut::new(self.get_id(uri)?, &mut self.arena))
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
    pub fn rename(&mut self, name: WzString) -> Result<(), Error> {
        self.cursor_mut().rename(name)?;
        Ok(())
    }

    /// Gets the data at the uri path. Errors when the node does not exist.
    pub fn get(&self, uri: &[&str]) -> Result<&T, Error> {
        Ok(&self
            .arena
            .get(self.get_id(uri)?)
            .expect("get() node should exist")
            .get()
            .data)
    }

    /// Walks the map depth-first
    pub fn walk<E>(&self, closure: impl Fn(Cursor<T>) -> Result<(), E>) -> Result<(), E>
    where
        E: Debug,
    {
        for id in self.root.descendants(&self.arena) {
            closure(Cursor::new(id, &self.arena))?;
        }
        Ok(())
    }

    /// Creates a printable string of the tree structure. To be used in `{:?}` formatting.
    pub fn debug_pretty_print<'a>(&'a self) -> DebugPrettyPrint<'a, MapNode<T>> {
        self.root.debug_pretty_print(&self.arena)
    }

    // *** PRIVATES *** //

    fn get_id(&self, uri: &[&str]) -> Result<NodeId, Error> {
        if uri.len() == 0 {
            Err(Error::InvalidPath)
        } else {
            if uri[0] == self.name() {
                let mut cursor = self.cursor();
                for name in &uri[1..] {
                    cursor.move_to(name)?;
                }
                Ok(cursor.position)
            } else {
                Err(Error::NotFound(String::from(uri[0])))
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        map::{Map, Metadata},
        types::{WzInt, WzString},
    };

    impl Metadata for i32 {
        fn update(&mut self, _: &WzString, _: &[WzInt]) {}
    }

    #[test]
    fn make_map() {
        let mut map = Map::new(WzString::from("root"), 100);
        let mut cursor = map.cursor_mut();
        assert_eq!(cursor.name(), "root");
        cursor
            .rename(WzString::from("n1"))
            .expect("rename should work");
        assert_eq!(cursor.name(), "n1");
        let children: &[&str] = &[];
        assert_eq!(&cursor.list().collect::<Vec<&str>>(), children);
        // let cursor die
        assert_eq!(map.name(), "n1");
    }

    #[test]
    fn get_uri() {
        let mut map = Map::new(WzString::from("n1"), 100);
        // arena ownership moves to cursor in the next line.
        let mut cursor = map.cursor_mut();
        cursor
            .create(WzString::from("n1_1"), 150)
            .expect("error creating n1_1")
            .move_to("n1_1")
            .expect("error moving into n1_1")
            .create(WzString::from("n1_1_1"), 155)
            .expect("error creating n1_1_1")
            .create(WzString::from("n1_1_2"), 175)
            .expect("error creating n1_1_1")
            .move_to("n1_1_1")
            .expect("error moving into n1_1_1")
            .create(WzString::from("n1_1_1_1"), 255)
            .expect("error creating n1_1_1_1")
            .move_to("n1_1_1_1")
            .expect("error moving into n1_1_1_1");
        assert_eq!(&cursor.pwd(), &["n1", "n1_1", "n1_1_1", "n1_1_1_1"]);
        // `cursor` dies here--arena ownership moves back to map in the next line.
        assert_eq!(
            *map.get(&["n1", "n1_1", "n1_1_1", "n1_1_1_1"])
                .expect("error getting uri"),
            255
        );
        assert!(map.get(&["n1", "n1_1", "fail"]).is_err());
    }
}
