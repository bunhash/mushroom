//! Map Cursor
//!
//! Used to navigate the map. This is to abstract the internals so no undefined behavior can occur.

use crate::{
    map::{error::Error, Arena, MapNode, NodeEdge, NodeId},
    types::WzString,
};

pub struct Cursor<'a, T> {
    position: NodeId,
    arena: &'a mut Arena<MapNode<T>>,
    clipboard: Option<NodeId>,
}

impl<'a, T> Cursor<'a, T> {
    pub(crate) fn new(position: NodeId, arena: &'a mut Arena<MapNode<T>>) -> Self {
        Self {
            position,
            arena,
            clipboard: None,
        }
    }

    pub fn list(&'a self) -> Vec<&'a str> {
        self.position
            .children(self.arena)
            .map(|id| self.arena.get(id).unwrap().get().name.as_ref())
            .collect::<Vec<&'a str>>()
    }

    pub fn new_node(&mut self, name: WzString, data: T) -> Result<(), Error> {
        if self.list().contains(&name.as_ref()) {
            Err(Error::DuplicateError(name))
        } else {
            let node = self.arena.new_node(MapNode::new(name, data));
            self.position.append(node, &mut self.arena);
            Ok(())
        }
    }

    fn get_id(&self, name: &str) -> Option<NodeId> {
        self.position
            .children(self.arena)
            .filter(|id| self.arena.get(*id).unwrap().get().name.as_ref() == name)
            .next()
    }

    fn purge_node(&mut self, id: NodeId) {
        let nodes = id
            .reverse_traverse(&self.arena)
            .filter_map(|edge| match edge {
                NodeEdge::Start(n) => Some(n),
                NodeEdge::End(_) => None,
            })
            .collect::<Vec<NodeId>>();
        for node in nodes {
            node.remove(&mut self.arena);
        }
    }

    pub fn delete_node<'n>(&mut self, name: &'n str) -> Result<(), Error<'n>> {
        let id = self.get_id(name);
        match id {
            Some(id) => Ok(self.purge_node(id)),
            None => Err(Error::NotFound(name)),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{map::Map, types::WzString};

    #[test]
    fn add_nodes() {
        let mut map = Map::new(WzString::from("root"), 100);
        let mut cursor = map.cursor();
        cursor
            .new_node(WzString::from("child1"), 150)
            .expect("error creating child1");
        cursor
            .new_node(WzString::from("child2"), 3500)
            .expect("error creating child2");
        assert!(
            cursor.new_node(WzString::from("child2"), 0).is_err(),
            "should have errored here"
        );
        assert_eq!(&cursor.list(), &["child1", "child2"]);
    }

    #[test]
    fn remove_node() {
        let mut map = Map::new(WzString::from("root"), 100);
        let mut cursor = map.cursor();
        cursor
            .new_node(WzString::from("child1"), 150)
            .expect("error creating child1");
        cursor
            .new_node(WzString::from("child2"), 3500)
            .expect("error creating child2");
        cursor
            .delete_node("child1")
            .expect("should have deleted child1");
        assert!(cursor.delete_node("child1").is_err(), "should have errored");
        assert_eq!(&cursor.list(), &["child2"]);
    }
}
