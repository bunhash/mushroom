//! Wz handling library

mod node;
mod error;
mod file;
mod header;
//mod object;
mod reader;

pub use node::{WzNode, WzNodeType};
pub use error::{WzError, WzErrorType, WzResult};
pub use file::WzFile;
pub use header::WzHeader;
//pub use object::{WzObject, WzUnparsed};
pub use reader::{WzEncryptedReader, WzRead, WzReader};

#[cfg(test)]
mod tests {

    use crate::{WzEncryptedReader, WzFile, WzNodeType, WzReader};
    use crypto::{MushroomSystem, GMS_IV, TRIMMED_KEY};

    #[test]
    fn open_v83_base() {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = WzEncryptedReader::open("testdata/v83-base.wz", &system).unwrap();
        let wz = WzFile::from_reader("Base", &mut reader).unwrap();
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
    fn read_v83_base() {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = WzEncryptedReader::open("testdata/v83-base.wz", &system).unwrap();
        let wz = WzFile::from_reader("Base", &mut reader).unwrap();

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
    fn search_v83_base() {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = WzEncryptedReader::open("testdata/v83-base.wz", &system).unwrap();
        let wz = WzFile::from_reader("Base", &mut reader).unwrap();
        let object = wz.get_path("Base/zmap.img").unwrap();
        match object {
            WzNodeType::Image(img) => assert_eq!(img.name(), "zmap.img"),
            _ => assert!(false),
        }
    }

    #[test]
    fn open_v83_string() {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = WzEncryptedReader::open("testdata/v83-string.wz", &system).unwrap();
        let wz = WzFile::from_reader("String", &mut reader).unwrap();
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
    fn read_v83_string() {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = WzEncryptedReader::open("testdata/v83-string.wz", &system).unwrap();
        let wz = WzFile::from_reader("String", &mut reader).unwrap();

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
    fn search_v83_string() {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = WzEncryptedReader::open("testdata/v83-string.wz", &system).unwrap();
        let wz = WzFile::from_reader("String", &mut reader).unwrap();
        let object = wz.get_path("String/Cash.img").unwrap();
        match object {
            WzNodeType::Image(img) => assert_eq!(img.name(), "Cash.img"),
            _ => assert!(false),
        }
    }

    #[test]
    fn search_v83_string_fail() {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = WzEncryptedReader::open("testdata/v83-string.wz", &system).unwrap();
        let wz = WzFile::from_reader("String", &mut reader).unwrap();
        assert_eq!(wz.get_path("Fail/TestEULA.img"), None);
        assert_eq!(wz.get_path("String/Fail.img"), None);
    }

    #[test]
    fn open_v172_base() {
        let mut reader = WzReader::open("testdata/v172-base.wz").unwrap();
        let wz = WzFile::from_reader("Base", &mut reader).unwrap();
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
    fn read_v172_base() {
        let mut reader = WzReader::open("testdata/v172-base.wz").unwrap();
        let wz = WzFile::from_reader("Base", &mut reader).unwrap();

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
    fn search_v172_base() {
        let mut reader = WzReader::open("testdata/v172-base.wz").unwrap();
        let wz = WzFile::from_reader("Base", &mut reader).unwrap();
        let object = wz.get_path("Base/StandardPDD.img").unwrap();
        match object {
            WzNodeType::Image(img) => assert_eq!(img.name(), "StandardPDD.img"),
            _ => assert!(false),
        }
    }

    #[test]
    fn open_v172_string() {
        let mut reader = WzReader::open("testdata/v172-string.wz").unwrap();
        let wz = WzFile::from_reader("String", &mut reader).unwrap();
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
    fn read_v172_string() {
        let mut reader = WzReader::open("testdata/v172-string.wz").unwrap();
        let wz = WzFile::from_reader("String", &mut reader).unwrap();

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
    fn search_v172_string() {
        let mut reader = WzReader::open("testdata/v172-string.wz").unwrap();
        let wz = WzFile::from_reader("String", &mut reader).unwrap();
        let object = wz.get_path("String/TestEULA.img").unwrap();
        match object {
            WzNodeType::Image(img) => assert_eq!(img.name(), "TestEULA.img"),
            _ => assert!(false),
        }
    }

    #[test]
    fn search_v172_string_fail() {
        let mut reader = WzReader::open("testdata/v172-string.wz").unwrap();
        let wz = WzFile::from_reader("String", &mut reader).unwrap();
        assert_eq!(wz.get_path("Fail/TestEULA.img"), None);
        assert_eq!(wz.get_path("String/Fail.img"), None);
    }
}
