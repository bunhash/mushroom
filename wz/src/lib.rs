//! Wz handling library

mod directory;
mod error;
mod file;
mod header;
mod node;
mod object;
mod reader;

pub use directory::WzDirectory;
pub use error::{WzError, WzErrorType, WzResult};
pub use file::WzFile;
pub use header::WzHeader;
pub use node::WzNode;
pub use object::{WzObject, WzObjectType, WzUnparsed};
pub use reader::{WzEncryptedReader, WzRead, WzReader};

#[cfg(test)]
mod tests {

    use crate::{WzEncryptedReader, WzFile, WzNode, WzObject, WzRead, WzReader};
    use crypto::{MushroomSystem, GMS_IV, TRIMMED_KEY};

    #[test]
    fn open_wzfile_old() {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = WzEncryptedReader::open("testdata/old-base.wz", &system).unwrap();
        let wz = WzFile::from_reader("base.wz", &mut reader).unwrap();
        let header = wz.header();
        assert_eq!(header.identifier(), "PKG1");
        assert_eq!(header.size(), 6480);
        assert_eq!(header.content_start(), 60);
        assert_eq!(
            header.description(),
            "Package file v1.0 Copyright 2002 Wizet, ZMS"
        );
        assert_eq!(wz.version(), 83);

        // Check contents
        match wz.root() {
            Some(WzObject::Directory(root)) => {
                assert_eq!(root.name(), "base.wz");
                assert_eq!(root.len(), 18);
            }
            _ => assert!(false),
        }
        let objects: Vec<&str> = wz
            .iter()
            .map(|o| match o {
                WzObject::Unparsed(obj) => obj.name(),
                WzObject::Directory(dir) => dir.name(),
                _ => {
                    assert!(false);
                    "unreachable"
                }
            })
            .collect();
        assert_eq!(
            objects,
            [
                "base.wz",
                "smap.img",
                "zmap.img",
                "StandardPDD.img",
                "UI",
                "Effect",
                "Sound",
                "Map",
                "Character",
                "Item",
                "TamingMob",
                "Etc",
                "Npc",
                "Reactor",
                "Skill",
                "Morph",
                "String",
                "Mob",
                "Quest"
            ]
        );
    }

    #[test]
    fn open_wzfile_new() {
        let mut reader = WzReader::open("testdata/new-base.wz").unwrap();
        let wz = WzFile::from_reader("base.wz", &mut reader).unwrap();
        let header = wz.header();
        assert_eq!(header.identifier(), "PKG1");
        assert_eq!(header.size(), 6705);
        assert_eq!(header.content_start(), 60);
        assert_eq!(
            header.description(),
            "Package file v1.0 Copyright 2002 Wizet, ZMS"
        );
        assert_eq!(wz.version(), 176);

        // Check contents
        match wz.root() {
            Some(WzObject::Directory(root)) => {
                assert_eq!(root.name(), "base.wz");
                assert_eq!(root.len(), 20);
            }
            _ => assert!(false),
        }
        let objects: Vec<&str> = wz
            .iter()
            .map(|o| match o {
                WzObject::Unparsed(obj) => obj.name(),
                WzObject::Directory(dir) => dir.name(),
                _ => {
                    assert!(false);
                    "unreachable"
                }
            })
            .collect();
        assert_eq!(
            objects,
            [
                "base.wz",
                "zmap.img",
                "StandardPDD.img",
                "smap.img",
                "String",
                "TamingMob",
                "Item",
                "Effect",
                "Quest",
                "Npc",
                "Map2",
                "UI",
                "Morph",
                "Map",
                "Sound",
                "Mob",
                "Skill",
                "Reactor",
                "Character",
                "Mob2",
                "Etc"
            ]
        );
    }
}
