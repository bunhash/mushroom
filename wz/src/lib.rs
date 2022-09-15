//! Wz handling library

mod error;
mod file;
mod header;
mod node;
//mod object;
mod reader;

pub use error::{WzError, WzErrorType, WzResult};
pub use file::WzFile;
pub use header::WzHeader;
pub use node::{WzNode, WzNodeRef, WzNodeType};
//pub use object::{WzObject, WzUnparsed};
pub use reader::{WzEncryptedReader, WzRead, WzReader};

#[cfg(test)]
mod tests {

    use crate::{WzEncryptedReader, WzFile, WzNodeType, WzReader};
    use crypto::{MushroomSystem, GMS_IV, TRIMMED_KEY};

    fn open_v83(filename: &str, name: &str) -> WzFile {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = WzEncryptedReader::open(filename, &system).unwrap();
        WzFile::from_reader(name, &mut reader).unwrap()
    }

    fn open_v172(filename: &str, name: &str) -> WzFile {
        let mut reader = WzReader::open(filename).unwrap();
        WzFile::from_reader(name, &mut reader).unwrap()
    }

    #[test]
    fn open_v83_wz_base() {
        let wz = open_v83("testdata/v83-base.wz", "Base");
        let header = wz.header();
        assert_eq!(header.identifier(), "PKG1");
        assert_eq!(header.size(), 6480);
        assert_eq!(header.content_start(), 60);
        assert_eq!(
            header.description(),
            "Package file v1.0 Copyright 2002 Wizet, ZMS"
        );
        assert_eq!(wz.version(), 83);
    }

    #[test]
    fn read_v83_wz_base() {
        let wz = open_v83("testdata/v83-base.wz", "Base");

        // Check the WzFile properties
        match wz.root() {
            Some(WzNodeType::Directory(root)) => {
                assert_eq!(root.name(), "Base");
                assert_eq!(root.len(), 18);
            }
            _ => assert!(false),
        }

        // Cycle through all of the contents
        let objects: Vec<&str> = wz
            .iter()
            .map(|o| match o {
                WzNodeType::Directory(dir) => dir.name(),
                WzNodeType::Image(img) => img.name(),
            })
            .collect();

        // Assert the contents
        assert_eq!(
            objects,
            [
                "Base",
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
    fn search_v83_wz_base() {
        let wz = open_v83("testdata/v83-base.wz", "Base");
        let object = wz.get_from_path("Base/zmap.img").unwrap();
        match object {
            WzNodeType::Image(img) => assert_eq!(img.name(), "zmap.img"),
            _ => assert!(false),
        }
        assert_eq!(wz.to_path(object.into()).unwrap(), "Base/zmap.img");
    }

    #[test]
    fn open_v83_wz_string() {
        let wz = open_v83("testdata/v83-string.wz", "String");
        let header = wz.header();
        assert_eq!(header.identifier(), "PKG1");
        assert_eq!(header.size(), 3561413);
        assert_eq!(header.content_start(), 60);
        assert_eq!(
            header.description(),
            "Package file v1.0 Copyright 2002 Wizet, ZMS"
        );
        assert_eq!(wz.version(), 83);
    }

    #[test]
    fn read_v83_wz_string() {
        let wz = open_v83("testdata/v83-string.wz", "String");

        // Check the WzFile properties
        match wz.root() {
            Some(WzNodeType::Directory(root)) => {
                assert_eq!(root.name(), "String");
                assert_eq!(root.len(), 20);
            }
            _ => assert!(false),
        }

        // Cycle through all of the contents
        let objects: Vec<&str> = wz
            .iter()
            .map(|o| match o {
                WzNodeType::Directory(dir) => dir.name(),
                WzNodeType::Image(img) => img.name(),
            })
            .collect();

        // Assert the contents
        assert_eq!(
            objects,
            [
                "String",
                "GuestEULA.img",
                "NameChange.img",
                "Skill.img",
                "GLcloneC.img",
                "Mob.img",
                "TestEULA.img",
                "Map.img",
                "TransferWorld.img",
                "ToolTipHelp.img",
                "EULA.img",
                "PetDialog.img",
                "TrialEULA.img",
                "Npc.img",
                "MonsterBook.img",
                "Cash.img",
                "Ins.img",
                "Pet.img",
                "Eqp.img",
                "Consume.img",
                "Etc.img"
            ]
        );
    }

    #[test]
    fn search_v83_wz_string() {
        let wz = open_v83("testdata/v83-string.wz", "String");
        let object = wz.get_from_path("String/Cash.img").unwrap();
        match object {
            WzNodeType::Image(img) => assert_eq!(img.name(), "Cash.img"),
            _ => assert!(false),
        }
    }

    #[test]
    fn search_v83_wz_string_fail() {
        let wz = open_v83("testdata/v83-string.wz", "String");
        assert_eq!(wz.get_from_path("Fail/TestEULA.img"), None);
        assert_eq!(wz.get_from_path("String/Fail.img"), None);
    }

    #[test]
    fn open_v172_wz_base() {
        let wz = open_v172("testdata/v172-base.wz", "Base");
        let header = wz.header();
        assert_eq!(header.identifier(), "PKG1");
        assert_eq!(header.size(), 6705);
        assert_eq!(header.content_start(), 60);
        assert_eq!(
            header.description(),
            "Package file v1.0 Copyright 2002 Wizet, ZMS"
        );
        assert_eq!(wz.version(), 176);
    }

    #[test]
    fn read_v172_wz_base() {
        let wz = open_v172("testdata/v172-base.wz", "Base");

        // Check the WzFile properties
        match wz.root() {
            Some(WzNodeType::Directory(root)) => {
                assert_eq!(root.name(), "Base");
                assert_eq!(root.len(), 20);
            }
            _ => assert!(false),
        }

        // Cycle through all of the contents
        let objects: Vec<&str> = wz
            .iter()
            .map(|o| match o {
                WzNodeType::Directory(dir) => dir.name(),
                WzNodeType::Image(img) => img.name(),
            })
            .collect();

        // Assert the contents
        assert_eq!(
            objects,
            [
                "Base",
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

    #[test]
    fn search_v172_wz_base() {
        let wz = open_v172("testdata/v172-base.wz", "Base");
        let object = wz.get_from_path("Base/StandardPDD.img").unwrap();
        match object {
            WzNodeType::Image(img) => assert_eq!(img.name(), "StandardPDD.img"),
            _ => assert!(false),
        }
    }

    #[test]
    fn open_v172_wz_string() {
        let wz = open_v172("testdata/v172-string.wz", "String");
        let header = wz.header();
        assert_eq!(header.identifier(), "PKG1");
        assert_eq!(header.size(), 10331199);
        assert_eq!(header.content_start(), 60);
        assert_eq!(
            header.description(),
            "Package file v1.0 Copyright 2002 Wizet, ZMS"
        );
        assert_eq!(wz.version(), 176);
    }

    #[test]
    fn read_v172_wz_string() {
        let wz = open_v172("testdata/v172-string.wz", "String");

        // Check the WzFile properties
        match wz.root() {
            Some(WzNodeType::Directory(root)) => {
                assert_eq!(root.name(), "String");
                assert_eq!(root.len(), 32);
            }
            _ => assert!(false),
        }

        // Cycle through all of the contents
        let objects: Vec<&str> = wz
            .iter()
            .map(|o| match o {
                WzNodeType::Directory(dir) => dir.name(),
                WzNodeType::Image(img) => img.name(),
            })
            .collect();

        // Assert the contents
        assert_eq!(
            objects,
            [
                "String",
                "Ins.img",
                "MonsterBook.img",
                "Npc.img",
                "EULA.img",
                "Mob.img",
                "WorldMap.img",
                "Map.img",
                "StarplanetGuide.img",
                "TrialEULA.img",
                "CommandGuide.img",
                "AdventureLogbook.img",
                "GLcloneC.img",
                "GL_Award.img",
                "Familiar.img",
                "PetDialog.img",
                "UI.img",
                "Cash.img",
                "Etc.img",
                "FreudEvent.img",
                "NameChange.img",
                "Skill.img",
                "TransferWorld.img",
                "Pet.img",
                "CashItemSearch.img",
                "mirrorDungeon.img",
                "TestEULA.img",
                "MonString.img",
                "GuestEULA.img",
                "11thEvent.img",
                "ToolTipHelp.img",
                "Consume.img",
                "Eqp.img"
            ]
        );
    }

    #[test]
    fn search_v172_wz_string() {
        let wz = open_v172("testdata/v172-string.wz", "String");
        let object = wz.get_from_path("String/TestEULA.img").unwrap();
        match object {
            WzNodeType::Image(img) => assert_eq!(img.name(), "TestEULA.img"),
            _ => assert!(false),
        }
    }

    #[test]
    fn search_v172_wz_string_fail() {
        let wz = open_v172("testdata/v172-string.wz", "String");
        assert_eq!(wz.get_from_path("Fail/TestEULA.img"), None);
        assert_eq!(wz.get_from_path("String/Fail.img"), None);
    }

    #[test]
    fn get_node_v172_wz_string() {
        let wz = open_v172("testdata/v172-string.wz", "String");
        match wz.root() {
            Some(root) => assert_eq!(wz.get(root.into()).unwrap().name(), "String"),
            None => assert!(false),
        }
    }
}
