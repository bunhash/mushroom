//! Generic map of WZ Package and Image structures

use crate::types::WzString;

pub use indextree::*;

mod cursor;
mod node;

pub mod error;

pub use cursor::{Cursor, CursorMut};
pub use node::MapNode;

use error::Error;

pub struct Map<T> {
    arena: Arena<MapNode<T>>,
    root: NodeId,
}

impl<T> Map<T> {
    /// Creates a new map.
    pub fn new(name: WzString, data: T) -> Self {
        let mut arena = Arena::new();
        let root = arena.new_node(MapNode::new(name, data));
        Self { arena, root }
    }

    /// Creates a cursor inside the root that cannot modify the map
    pub fn cursor<'a>(&'a self) -> Cursor<'a, T> {
        Cursor::new(self.root, &self.arena)
    }

    /// Creates a mutable cursor inside the root that can modify the map
    pub fn cursor_mut<'a>(&'a mut self) -> CursorMut<'a, T> {
        CursorMut::new(self.root, &mut self.arena)
    }

    /// Gets the data at the uri path
    pub fn get<'n>(&self, uri: &'n [&str]) -> Result<&T, Error<'n>> {
        Ok(&self
            .arena
            .get(self.get_id(uri)?)
            .expect("get() node should exist")
            .get()
            .data)
    }

    /// Gets the mutable data at the uri path
    pub fn get_mut<'n>(&self, uri: &'n [&str]) -> Result<&T, Error<'n>> {
        Ok(&self
            .arena
            .get(self.get_id(uri)?)
            .expect("get_mut() node should exist")
            .get()
            .data)
    }

    // *** PRIVATES *** //

    fn get_id<'n>(&self, uri: &'n [&str]) -> Result<NodeId, Error<'n>> {
        let mut cursor = self.cursor();
        for name in uri {
            cursor.move_to(name)?;
        }
        Ok(cursor.position())
    }
}

#[cfg(test)]
mod tests {

    use crate::{map::Map, types::WzString};

    #[test]
    fn make_map() {
        let mut map = Map::new(WzString::from("root"), 100);
        let cursor = map.cursor_mut();
        let children: &[&str] = &[];
        assert_eq!(&cursor.list(), children);
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
            *map.get(&["n1_1", "n1_1_1", "n1_1_1_1"])
                .expect("error getting uri"),
            255
        );
        assert!(map.get(&["n1_1", "fail"]).is_err());
    }
}

/*
use crate::{
    error::{MapError, Result},
    package::{Content, ContentInfo, Package},
    types::{WzInt, WzOffset, WzString},
    Decode, Encode, Reader, Writer,
};
use std::{
    collections::{hash_map::Iter, VecDeque},
    io::SeekFrom,
};
//use tree::{self, Arena, Node, NodeId, TraverseType, TraverseUri};
use indextree::{Arena, NodeId};

#[derive(Debug)]
pub struct Map {
    pub name: String,
    pub description: String,
    pub version: u16,
    arena: Arena<Content>,
    root: NodeId,
}

impl Map {
    /// Get root
    pub fn root(&self) -> NodeId {
        self.root
    }

    /// Gets the [`Node`]
    fn get_node(&self, id: NodeId) -> Result<&Node<Content>> {
        match self.arena.get(id) {
            Some(node) => Ok(node),
            None => Err(MapError::InvalidNodeId.into()),
        }
    }

    /// Get [`Content`]
    pub fn get(&self, id: NodeId) -> Result<&Content> {
        Ok(self.get_node(id)?.get()?)
    }

    fn recursive_by_path(&self, path: &[&str], current: NodeId) -> Option<NodeId> {
        if path.is_empty() {
            Some(current)
        } else {
            match self
                .get_node(current)
                .expect("invalid node in tree")
                .get_child(path[0])
            {
                Some(next) => self.recursive_by_path(&path[1..], next),
                None => None,
            }
        }
    }

    /// Get [`NodeId`] by path
    pub fn get_id_by_uri(&self, path: &str) -> Option<NodeId> {
        let path = path.split("/").collect::<Vec<&str>>();
        if path[0] != self.name {
            None
        } else {
            self.recursive_by_path(&path[1..], self.root)
        }
    }

    /// Get [`Content`] by path
    pub fn get_by_uri(&self, path: &str) -> Option<&Content> {
        Some(
            self.get(self.get_id_by_uri(path)?)
                .expect("invalid node in tree"),
        )
    }

    /// Get [`Content`]'s URI
    pub fn get_uri(&self, id: NodeId) -> Result<String> {
        self.get(id)?; // sanity
        if id == self.root {
            return Ok(String::from(""));
        }

        let mut path = VecDeque::new();
        for node in id.ancestors(&self.arena) {
            path.push_front(node.0);
        }
        Ok(path.make_contiguous().join("/"))
    }

    /// Get Children of node
    pub fn children(&self, id: NodeId) -> Result<Iter<'_, String, NodeId>> {
        Ok(self.get_node(id)?.iter())
    }

    /// Traverses the WZ map of Packages
    pub fn breadth_first(&self) -> TraverseUri<'_, Content> {
        TraverseUri::new(
            &self.arena,
            self.name.as_str(),
            self.root,
            TraverseType::BreadthFirst,
        )
    }

    /// Traverses the WZ map of Packages
    pub fn depth_first(&self) -> TraverseUri<'_, Content> {
        TraverseUri::new(
            &self.arena,
            self.name.as_str(),
            self.root,
            TraverseType::DepthFirst,
        )
    }

    /// Traverses the WZ map of Packages
    pub fn traverse(&self) -> TraverseUri<'_, Content> {
        self.depth_first()
    }

    /// Recursively decodes
    fn recursive_decode<R>(
        arena: &mut Arena<Content>,
        parent: NodeId,
        content: &Content,
        reader: &mut R,
    ) -> Result<()>
    where
        R: Reader,
    {
        match content {
            Content::Unknown(bytes) => {
                // Add the unknown content under parent
                let child = arena.new_node(format!("{:x?}", bytes), content.clone());
                parent.insert(child, arena)?;
            }
            Content::Package(name, info) => {
                // Add the directory under parent
                let child = arena.new_node(String::from(name.as_ref()), content.clone());
                parent.insert(child, arena)?;

                // Seek to the offset and parse it
                let offset = *info.offset as u64;
                reader.seek(SeekFrom::Start(offset))?;
                let package = Package::decode(reader)?;

                // Continue decoding it
                for content in package.contents {
                    Map::recursive_decode(arena, child, &content, reader)?;
                }
            }
            Content::Image(name, _) => {
                // Add the image under parent
                let child = arena.new_node(String::from(name.as_ref()), content.clone());
                parent.insert(child, arena)?;
            }
        }
        Ok(())
    }
}

impl Decode for Map {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        // Create a new arena
        let mut arena = Arena::new();
        let root = arena.new_node(
            String::from(""),
            Content::Package(
                WzString::from(reader.metadata().name.as_str()),
                ContentInfo {
                    size: WzInt::from((reader.metadata().size - 2) as i32),
                    checksum: WzInt::from(0),
                    offset: WzOffset::from((reader.metadata().absolute_position + 2) as u32),
                },
            ),
        );

        // Decode root directory
        let package = Package::decode(reader)?;
        for content in package.contents {
            Map::recursive_decode(&mut arena, root, &content, reader)?;
        }

        Ok(Self {
            name: String::from(reader.metadata().name.as_str()),
            description: String::from(reader.metadata().description.as_ref()),
            version: reader.metadata().version,
            arena,
            root,
        })
    }
}

impl Encode for Map {
    /// Get the size
    fn encode_size(&self) -> u64 {
        unimplemented!()
    }

    /// Encodes objects
    fn encode<W>(&self, _writer: &mut W) -> Result<()>
    where
        W: Writer,
        Self: Sized,
    {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {

    use crate::{package::Content, Decode, Encode, EncryptedReader, Reader, Map, UnencryptedReader};
    use crypto::{MushroomSystem, GMS_IV, TRIMMED_KEY};
    use std::fs::File;
    use tree::NodeId;

    #[test]
    fn v83_contents() {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = EncryptedReader::from_reader(
            "Base.wz",
            File::open("testdata/v83-base.wz").expect("error opening file"),
            &system,
        )
        .expect("error making reader");
        let map = Map::decode(&mut reader).expect("error decoding");
        let mut uris = map.traverse().map(|item| item.0).collect::<Vec<String>>();
        uris.sort();
        assert_eq!(
            &uris,
            &[
                "Base.wz/Character",
                "Base.wz/Effect",
                "Base.wz/Etc",
                "Base.wz/Item",
                "Base.wz/Map",
                "Base.wz/Mob",
                "Base.wz/Morph",
                "Base.wz/Npc",
                "Base.wz/Quest",
                "Base.wz/Reactor",
                "Base.wz/Skill",
                "Base.wz/Sound",
                "Base.wz/StandardPDD.img",
                "Base.wz/String",
                "Base.wz/TamingMob",
                "Base.wz/UI",
                "Base.wz/smap.img",
                "Base.wz/zmap.img"
            ]
        );
    }

    #[test]
    fn v83_get_uri() {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = EncryptedReader::from_reader(
            "Base.wz",
            File::open("testdata/v83-base.wz").expect("error opening file"),
            &system,
        )
        .expect("error making reader");
        let map = Map::decode(&mut reader).expect("error decoding");
        let uris = [
            "Base.wz/Character",
            "Base.wz/Effect",
            "Base.wz/Etc",
            "Base.wz/Item",
            "Base.wz/Map",
            "Base.wz/Mob",
            "Base.wz/Morph",
            "Base.wz/Npc",
            "Base.wz/Quest",
            "Base.wz/Reactor",
            "Base.wz/Skill",
            "Base.wz/Sound",
            "Base.wz/StandardPDD.img",
            "Base.wz/String",
            "Base.wz/TamingMob",
            "Base.wz/UI",
            "Base.wz/smap.img",
            "Base.wz/zmap.img",
        ];
        assert!(
            map.get_by_uri("Base.wz").is_some(),
            "could not find root by path"
        );
        for uri in uris {
            let content = map.get_by_uri(&uri);
            assert!(content.is_some(), "could not find {}", uri);
        }
        assert!(map.get_by_uri("failure").is_none(), "found failure");
        assert!(
            map.get_by_uri("Base.wz/failure").is_none(),
            "found /failure"
        );
    }

    #[test]
    fn v83_content_sum() {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = EncryptedReader::from_reader(
            "Base.wz",
            File::open("testdata/v83-base.wz").expect("error opening file"),
            &system,
        )
        .expect("error making reader");
        let map = Map::decode(&mut reader).expect("error decoding");
        let sum = 2 + map
            .children(map.root())
            .expect("could not get children")
            .map(|(_, id)| {
                let content = map.get(*id).expect("invalid id?");
                match content {
                    Content::Package(name, info) => *info.size as u64 + content.encode_size(),
                    Content::Image(name, info) => *info.size as u64 + content.encode_size(),
                    _ => panic!("should only contain packages and images"),
                }
            })
            .sum::<u64>();
        assert_eq!(sum, reader.metadata().size);
    }

    #[test]
    fn v172_contents() {
        let mut reader = UnencryptedReader::from_reader(
            "Base.wz",
            File::open("testdata/v172-base.wz").expect("error opening file"),
        )
        .expect("error making reader file");
        let map = Map::decode(&mut reader).expect("error decoding");
        let mut uris = map.traverse().map(|item| item.0).collect::<Vec<String>>();
        uris.sort();
        assert_eq!(
            &uris,
            &[
                "Base.wz/Character",
                "Base.wz/Effect",
                "Base.wz/Etc",
                "Base.wz/Item",
                "Base.wz/Map",
                "Base.wz/Map2",
                "Base.wz/Mob",
                "Base.wz/Mob2",
                "Base.wz/Morph",
                "Base.wz/Npc",
                "Base.wz/Quest",
                "Base.wz/Reactor",
                "Base.wz/Skill",
                "Base.wz/Sound",
                "Base.wz/StandardPDD.img",
                "Base.wz/String",
                "Base.wz/TamingMob",
                "Base.wz/UI",
                "Base.wz/smap.img",
                "Base.wz/zmap.img"
            ]
        );
    }

    #[test]
    fn v172_get_uri() {
        let mut reader = UnencryptedReader::from_reader(
            "Base.wz",
            File::open("testdata/v172-base.wz").expect("error opening file"),
        )
        .expect("error making reader file");
        let map = Map::decode(&mut reader).expect("error decoding");
        let uris = [
            "Base.wz/Character",
            "Base.wz/Effect",
            "Base.wz/Etc",
            "Base.wz/Item",
            "Base.wz/Map",
            "Base.wz/Map2",
            "Base.wz/Mob",
            "Base.wz/Mob2",
            "Base.wz/Morph",
            "Base.wz/Npc",
            "Base.wz/Quest",
            "Base.wz/Reactor",
            "Base.wz/Skill",
            "Base.wz/Sound",
            "Base.wz/StandardPDD.img",
            "Base.wz/String",
            "Base.wz/TamingMob",
            "Base.wz/UI",
            "Base.wz/smap.img",
            "Base.wz/zmap.img",
        ];
        assert!(
            map.get_by_uri("Base.wz").is_some(),
            "could not find root by path"
        );
        for uri in uris {
            let content = map.get_by_uri(&uri);
            assert!(content.is_some(), "could not find {}", uri);
        }
        assert!(map.get_by_uri("failure").is_none(), "found failure");
        assert!(
            map.get_by_uri("Base.wz/failure").is_none(),
            "found /failure"
        );
    }
}
*/
