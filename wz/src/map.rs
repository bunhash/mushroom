//! Maps the WZ file

use crate::{
    error::Result,
    package::{Content, ContentInfo, Package},
    types::{WzInt, WzOffset, WzString},
    Decode, Reader,
};
use std::io::SeekFrom;
use tree::{self, Arena, NodeId, Traverse};

#[derive(Debug)]
pub struct WzMap {
    pub description: String,
    pub version: u16,
    arena: Arena<Content>,
    root: NodeId,
}

impl WzMap {
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
            Content::Uol(loc) => {
                let child = arena.new_node(format!("UOL#{}", loc), content.clone());
                parent.insert(child, arena)?;
            }
            Content::Package(info) => {
                // Add the directory under parent
                let child = arena.new_node(String::from(info.name.as_ref()), content.clone());
                parent.insert(child, arena)?;

                // Seek to the offset and parse it
                let offset = *info.offset as u64;
                reader.seek(SeekFrom::Start(offset))?;
                let package = Package::decode(reader)?;

                // Continue decoding it
                for content in package.contents {
                    WzMap::recursive_decode(arena, child, &content, reader)?;
                }
            }
            Content::Image(info) => {
                // Add the image under parent
                let child = arena.new_node(String::from(info.name.as_ref()), content.clone());
                parent.insert(child, arena)?;
            }
        }
        Ok(())
    }

    /// Traverses the WZ map of Packages
    pub fn traverse(&self) -> Traverse<'_, Content> {
        self.root.breadth_first(&self.arena)
    }
}

impl Decode for WzMap {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        // Create a new arena
        let mut arena = Arena::new();
        let root = arena.new_node(
            String::from(""),
            Content::Package(ContentInfo {
                name: WzString::from(""),
                size: WzInt::from((reader.metadata().size - 2) as i32),
                checksum: WzInt::from(0),
                offset: WzOffset::from(reader.metadata().absolute_position + 2),
            }),
        );

        // Decode root directory
        let package = Package::decode(reader)?;
        for content in package.contents {
            WzMap::recursive_decode(&mut arena, root, &content, reader)?;
        }

        Ok(Self {
            description: String::from(reader.metadata().description.as_ref()),
            version: reader.metadata().version,
            arena,
            root,
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::{Decode, EncryptedWzReader, WzMap, WzReader};
    use crypto::{MushroomSystem, GMS_IV, TRIMMED_KEY};
    use std::fs::File;

    #[test]
    fn v83_contents() {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = EncryptedWzReader::new(
            File::open("testdata/v83-base.wz").expect("error opening file"),
            &system,
        )
        .expect("error making reader");

        let map = WzMap::decode(&mut reader).expect("error decoding");
        let mut names = map
            .traverse()
            .map(|item| item.0)
            .flatten()
            .collect::<Vec<&str>>();
        names.sort();
        assert_eq!(
            &names,
            &[
                "",
                "Character",
                "Effect",
                "Etc",
                "Item",
                "Map",
                "Mob",
                "Morph",
                "Npc",
                "Quest",
                "Reactor",
                "Skill",
                "Sound",
                "StandardPDD.img",
                "String",
                "TamingMob",
                "UI",
                "smap.img",
                "zmap.img"
            ]
        );
    }

    #[test]
    fn v172_contents() {
        let mut reader =
            WzReader::new(File::open("testdata/v172-base.wz").expect("error opening file"))
                .expect("error making reader");

        let map = WzMap::decode(&mut reader).expect("error decoding");
        let mut names = map
            .traverse()
            .map(|item| item.0)
            .flatten()
            .collect::<Vec<&str>>();
        names.sort();
        assert_eq!(
            &names,
            &[
                "",
                "Character",
                "Effect",
                "Etc",
                "Item",
                "Map",
                "Map2",
                "Mob",
                "Mob2",
                "Morph",
                "Npc",
                "Quest",
                "Reactor",
                "Skill",
                "Sound",
                "StandardPDD.img",
                "String",
                "TamingMob",
                "UI",
                "smap.img",
                "zmap.img"
            ]
        );
    }
}
