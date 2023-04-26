//! Map Cursor
//!
//! Used to navigate the map. This is to abstract the internals so no undefined behavior can occur.

use crate::{
    map::{error::Error, Arena, MapNode, NodeId},
    types::WzString,
};
use std::collections::VecDeque;

pub struct Cursor<'a, T> {
    position: NodeId,
    arena: &'a Arena<MapNode<T>>,
}

impl<'a, T> Cursor<'a, T> {
    pub(crate) fn new(position: NodeId, arena: &'a Arena<MapNode<T>>) -> Self {
        Self { position, arena }
    }

    pub(crate) fn position(&self) -> NodeId {
        self.position
    }

    pub fn pwd(&'a self) -> Vec<&'a str> {
        let mut path = VecDeque::new();
        for id in self.position.ancestors(&self.arena) {
            path.push_front(
                self.arena
                    .get(id)
                    .expect("pwd() node should exist")
                    .get()
                    .name
                    .as_ref(),
            );
        }
        path.into()
    }

    pub fn list(&'a self) -> Vec<&'a str> {
        self.position
            .children(self.arena)
            .map(|id| {
                self.arena
                    .get(id)
                    .expect("list() node should exist")
                    .get()
                    .name
                    .as_ref()
            })
            .collect::<Vec<&'a str>>()
    }

    pub fn get(&self) -> &T {
        &self
            .arena
            .get(self.position)
            .expect("get() node should exist")
            .get()
            .data
    }

    pub fn move_to<'n>(&mut self, name: &'n str) -> Result<&mut Self, Error<'n>> {
        let id = self.get_id(name)?;
        self.position = id;
        Ok(self)
    }

    pub fn parent(&mut self) -> Result<&mut Self, Error> {
        match self
            .arena
            .get(self.position)
            .expect("current position should exist")
            .parent()
        {
            Some(id) => {
                self.position = id;
                Ok(self)
            }
            None => Err(Error::AlreadyRoot),
        }
    }

    // *** PRIVATES *** //

    fn get_id<'n>(&self, name: &'n str) -> Result<NodeId, Error<'n>> {
        match self
            .position
            .children(self.arena)
            .filter(|id| {
                self.arena
                    .get(*id)
                    .expect("get_id() node should exist")
                    .get()
                    .name
                    .as_ref()
                    == name
            })
            .next()
        {
            Some(id) => Ok(id),
            None => Err(Error::NotFound(name)),
        }
    }
}

pub struct CursorMut<'a, T> {
    position: NodeId,
    arena: &'a mut Arena<MapNode<T>>,
    clipboard: Option<NodeId>,
}

impl<'a, T> CursorMut<'a, T> {
    pub(crate) fn new(position: NodeId, arena: &'a mut Arena<MapNode<T>>) -> Self {
        Self {
            position,
            arena,
            clipboard: None,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn position(&self) -> NodeId {
        self.position
    }

    pub fn pwd(&'a self) -> Vec<&'a str> {
        let mut path = VecDeque::new();
        for id in self.position.ancestors(&self.arena) {
            path.push_front(
                self.arena
                    .get(id)
                    .expect("pwd() node should exist")
                    .get()
                    .name
                    .as_ref(),
            );
        }
        path.into()
    }

    pub fn list(&'a self) -> Vec<&'a str> {
        self.position
            .children(self.arena)
            .map(|id| {
                self.arena
                    .get(id)
                    .expect("list() node should exist")
                    .get()
                    .name
                    .as_ref()
            })
            .collect::<Vec<&'a str>>()
    }

    pub fn get(&self) -> &T {
        &self
            .arena
            .get(self.position)
            .expect("get() node should exist")
            .get()
            .data
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self
            .arena
            .get_mut(self.position)
            .expect("get_mut() node should exist")
            .get_mut()
            .data
    }

    pub fn move_to<'n>(&mut self, name: &'n str) -> Result<&mut Self, Error<'n>> {
        let id = self.get_id(name)?;
        self.position = id;
        Ok(self)
    }

    pub fn parent(&mut self) -> Result<&mut Self, Error> {
        match self
            .arena
            .get(self.position)
            .expect("current position should exist")
            .parent()
        {
            Some(id) => {
                self.position = id;
                Ok(self)
            }
            None => Err(Error::AlreadyRoot),
        }
    }

    pub fn create(&mut self, name: WzString, data: T) -> Result<&mut Self, Error> {
        if self.list().contains(&name.as_ref()) {
            Err(Error::DuplicateError(name))
        } else {
            let node = self.arena.new_node(MapNode::new(name, data));
            self.position.append(node, &mut self.arena);
            Ok(self)
        }
    }

    pub fn cut<'n>(&mut self, name: &'n str) -> Result<&mut Self, Error<'n>> {
        let id = self.get_id(name)?;
        id.detach(&mut self.arena);
        if let Some(to_delete) = self.clipboard {
            to_delete.remove_subtree(&mut self.arena);
        }
        self.clipboard = Some(id);
        Ok(self)
    }

    pub fn paste(&mut self) -> Result<&mut Self, Error> {
        match self.clipboard {
            Some(id) => {
                self.position.append(id, &mut self.arena);
                Ok(self)
            }
            None => Err(Error::ClipboardEmpty),
        }
    }

    pub fn delete<'n>(&mut self, name: &'n str) -> Result<&mut Self, Error<'n>> {
        let id = self.get_id(name)?;
        id.remove_subtree(&mut self.arena);
        Ok(self)
    }

    // *** PRIVATES *** //

    fn get_id<'n>(&self, name: &'n str) -> Result<NodeId, Error<'n>> {
        match self
            .position
            .children(self.arena)
            .filter(|id| {
                self.arena
                    .get(*id)
                    .expect("get_id() node should exist")
                    .get()
                    .name
                    .as_ref()
                    == name
            })
            .next()
        {
            Some(id) => Ok(id),
            None => Err(Error::NotFound(name)),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{map::Map, types::WzString};

    #[test]
    fn add_nodes() {
        let mut map = Map::new(WzString::from("n1"), 100);
        let mut cursor = map.cursor_mut();
        cursor
            .create(WzString::from("n1_1"), 150)
            .expect("error creating n1_1")
            .create(WzString::from("n1_2"), 3500)
            .expect("error creating n1_2");
        assert!(
            cursor.create(WzString::from("n1_2"), 0).is_err(),
            "should have errored here"
        );
        assert_eq!(&cursor.list(), &["n1_1", "n1_2"]);
    }

    #[test]
    fn remove_node() {
        let mut map = Map::new(WzString::from("n1"), 100);
        let mut cursor = map.cursor_mut();
        cursor
            .create(WzString::from("n1_1"), 150)
            .expect("error creating n1_1")
            .create(WzString::from("n1_2"), 3500)
            .expect("error creating n1_2")
            .delete("n1_1")
            .expect("should have deleted n1_1");
        assert!(cursor.delete("n1_1").is_err(), "should have errored");
        assert_eq!(&cursor.list(), &["n1_2"]);
    }

    #[test]
    fn remove_subtree() {
        let mut map = Map::new(WzString::from("n1"), 100);
        let mut cursor = map.cursor_mut();
        cursor
            .create(WzString::from("n1_1"), 150)
            .expect("error creating n1_1")
            .create(WzString::from("n1_2"), 3500)
            .expect("error creating n1_2")
            .move_to("n1_1")
            .expect("error moving into n1_1")
            .create(WzString::from("n1_2_1"), 50)
            .expect("error creating n1_2_1")
            .parent()
            .expect("error moving back to n1")
            .delete("n1_1")
            .expect("should have deleted n1_1");
        assert_eq!(&cursor.list(), &["n1_2"]);
    }

    #[test]
    fn move_node() {
        let mut map = Map::new(WzString::from("n1"), 100);
        let mut cursor = map.cursor_mut();
        cursor
            .create(WzString::from("n1_1"), 150)
            .expect("error creating n1_1")
            .create(WzString::from("n1_2"), 3500)
            .expect("error creating n1_2")
            .move_to("n1_1")
            .expect("error moving into n1_1")
            .create(WzString::from("n1_2_1"), 50)
            .expect("error creating n1_2_1")
            .parent()
            .expect("error moving back to n1")
            .cut("n1_1")
            .expect("should have cut n1_1")
            .move_to("n1_2")
            .expect("error moving to n1_2")
            .paste()
            .expect("should paste n1_1");
        assert_eq!(&cursor.list(), &["n1_1"]);
    }

    #[test]
    fn pwd() {
        let mut map = Map::new(WzString::from("n1"), 100);
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
    }
}
