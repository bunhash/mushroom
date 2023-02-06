//! Wz handling library
//!
//! Example
//! ```
//! use wz::{WzEncryptedReader, WzFile, WzNodeType, WzReader};
//! use crypto::{MushroomSystem, GMS_IV, TRIMMED_KEY};
//!
//! let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
//! let mut reader = WzEncryptedReader::open("testdata/v83-base.wz", &system).unwrap();
//! let wz = WzFile::from_reader("Base", &mut reader).unwrap();
//!
//! // Check the WzFile properties
//! let root = wz.root().unwrap().get().unwrap();
//! assert!(root.is_directory());
//! assert_eq!(wz.root_name(), Some("Base"));
//! assert_eq!(wz.arena().count(), 19);
//!
//! // Cycle through all of the contents
//! let mut objects: Vec<&str> = wz
//!     .root_id()
//!     .children(wz.arena())
//!     .map(|s| s.as_str())
//!     .collect();
//! objects.sort();
//! assert_eq!(
//!     objects,
//!     [
//!         "Character",
//!         "Effect",
//!         "Etc",
//!         "Item",
//!         "Map",
//!         "Mob",
//!         "Morph",
//!         "Npc",
//!         "Quest",
//!         "Reactor",
//!         "Skill",
//!         "Sound",
//!         "StandardPDD.img",
//!         "String",
//!         "TamingMob",
//!         "UI",
//!         "smap.img",
//!         "zmap.img"
//!     ]
//! );
//! ```

mod canvas;
mod error;
mod file;
mod header;
mod image;
mod node;
mod object;
mod property;
mod reader;
mod sound;
mod vector;

pub use canvas::WzCanvas;
pub use error::{WzError, WzErrorType, WzResult};
pub use file::WzFile;
pub use header::WzHeader;
pub use image::WzImage;
pub use node::{WzNode, WzNodeType};
pub use object::WzObject;
pub use property::WzProperty;
pub use reader::{WzEncryptedReader, WzRead, WzReader};
pub use sound::WzSound;
pub use vector::WzVector;

#[cfg(test)]
mod tests {

    use crate::{WzEncryptedReader, WzFile, WzImage, WzObject, WzProperty, WzReader};
    use crypto::{MushroomSystem, GMS_IV, TRIMMED_KEY};

    fn open_v83(filename: &str, name: &str) -> WzFile {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = WzEncryptedReader::open(filename, &system).unwrap();
        WzFile::from_reader(name, &mut reader).unwrap()
    }

    fn open_v83_img(filename: &str, name: &str) -> WzImage {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = WzEncryptedReader::open(filename, &system).unwrap();
        WzImage::from_reader(name, &mut reader).unwrap()
    }

    fn open_v172(filename: &str, name: &str) -> WzFile {
        let mut reader = WzReader::open(filename).unwrap();
        WzFile::from_reader(name, &mut reader).unwrap()
    }

    fn open_v172_img(filename: &str, name: &str) -> WzImage {
        let mut reader = WzReader::open(filename).unwrap();
        WzImage::from_reader(name, &mut reader).unwrap()
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
        let root = wz.root().unwrap().get().unwrap();
        assert!(root.is_directory());
        assert_eq!(wz.root_name(), Some("Base"));
        assert_eq!(wz.arena().count(), 19);

        // Cycle through all of the contents
        let mut objects: Vec<&str> = wz
            .root_id()
            .children(wz.arena())
            .map(|s| s.as_str())
            .collect();
        objects.sort();

        // Assert the contents
        assert_eq!(
            objects,
            [
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
    fn search_v83_wz_base() {
        let wz = open_v83("testdata/v83-base.wz", "Base");
        let node = wz.get_from_path("Base/zmap.img").unwrap();
        let object = node.get().unwrap();
        assert!(!object.is_directory());
        let node_id = wz.get_node_id(node).unwrap();
        assert_eq!(wz.to_path(node_id).unwrap(), "Base/zmap.img");
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
        let root = wz.root().unwrap().get().unwrap();
        assert!(root.is_directory());
        assert_eq!(wz.root_name(), Some("String"));
        assert_eq!(wz.arena().count(), 21);

        // Cycle through all of the contents
        let mut objects: Vec<&str> = wz
            .root_id()
            .children(wz.arena())
            .map(|s| s.as_str())
            .collect();
        objects.sort();

        // Assert the contents
        assert_eq!(
            objects,
            [
                "Cash.img",
                "Consume.img",
                "EULA.img",
                "Eqp.img",
                "Etc.img",
                "GLcloneC.img",
                "GuestEULA.img",
                "Ins.img",
                "Map.img",
                "Mob.img",
                "MonsterBook.img",
                "NameChange.img",
                "Npc.img",
                "Pet.img",
                "PetDialog.img",
                "Skill.img",
                "TestEULA.img",
                "ToolTipHelp.img",
                "TransferWorld.img",
                "TrialEULA.img"
            ]
        );
    }

    #[test]
    fn search_v83_wz_string() {
        let wz = open_v83("testdata/v83-string.wz", "String");
        let node = wz.get_from_path("String/Cash.img").unwrap();
        let object = node.get().unwrap();
        assert!(!object.is_directory());
        let node_id = wz.get_node_id(node).unwrap();
        assert_eq!(wz.to_path(node_id).unwrap(), "String/Cash.img");
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
        let root = wz.root().unwrap().get().unwrap();
        assert!(root.is_directory());
        assert_eq!(wz.root_name(), Some("Base"));
        assert_eq!(wz.arena().count(), 21);

        // Cycle through all of the contents
        let mut objects: Vec<&str> = wz
            .root_id()
            .children(wz.arena())
            .map(|s| s.as_str())
            .collect();
        objects.sort();

        // Assert the contents
        assert_eq!(
            objects,
            [
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

    #[test]
    fn search_v172_wz_base() {
        let wz = open_v172("testdata/v172-base.wz", "Base");
        let node = wz.get_from_path("Base/StandardPDD.img").unwrap();
        let object = node.get().unwrap();
        assert!(!object.is_directory());
        let node_id = wz.get_node_id(node).unwrap();
        assert_eq!(wz.to_path(node_id).unwrap(), "Base/StandardPDD.img");
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
        let root = wz.root().unwrap().get().unwrap();
        assert!(root.is_directory());
        assert_eq!(wz.root_name(), Some("String"));
        assert_eq!(wz.arena().count(), 33);

        // Cycle through all of the contents
        let mut objects: Vec<&str> = wz
            .root_id()
            .children(wz.arena())
            .map(|s| s.as_str())
            .collect();
        objects.sort();

        // Assert the contents
        assert_eq!(
            objects,
            [
                "11thEvent.img",
                "AdventureLogbook.img",
                "Cash.img",
                "CashItemSearch.img",
                "CommandGuide.img",
                "Consume.img",
                "EULA.img",
                "Eqp.img",
                "Etc.img",
                "Familiar.img",
                "FreudEvent.img",
                "GL_Award.img",
                "GLcloneC.img",
                "GuestEULA.img",
                "Ins.img",
                "Map.img",
                "Mob.img",
                "MonString.img",
                "MonsterBook.img",
                "NameChange.img",
                "Npc.img",
                "Pet.img",
                "PetDialog.img",
                "Skill.img",
                "StarplanetGuide.img",
                "TestEULA.img",
                "ToolTipHelp.img",
                "TransferWorld.img",
                "TrialEULA.img",
                "UI.img",
                "WorldMap.img",
                "mirrorDungeon.img"
            ]
        );
    }

    #[test]
    fn search_v172_wz_string() {
        let wz = open_v172("testdata/v172-string.wz", "String");
        let node = wz.get_from_path("String/TestEULA.img").unwrap();
        let object = node.get().unwrap();
        assert!(!object.is_directory());
        let node_id = wz.get_node_id(node).unwrap();
        assert_eq!(wz.to_path(node_id).unwrap(), "String/TestEULA.img");
    }

    #[test]
    fn search_v172_wz_string_fail() {
        let wz = open_v172("testdata/v172-string.wz", "String");
        assert_eq!(wz.get_from_path("Fail/TestEULA.img"), None);
        assert_eq!(wz.get_from_path("String/Fail.img"), None);
    }

    #[test]
    fn open_v83_img_weapon() {
        let img = open_v83_img("testdata/v83-weapon.img", "Weapon");
        let root = img.root().unwrap().get().unwrap();
        assert_eq!(img.root_name(), Some("Weapon"));
        assert_eq!(root, &WzObject::Property(WzProperty::Variant));
        assert_eq!(img.arena().count(), 177);

        // Cycle through all of the contents
        let mut objects: Vec<&str> = img
            .root_id()
            .children(img.arena())
            .map(|s| s.as_str())
            .collect();
        objects.sort();

        // Assert the contents
        assert_eq!(
            objects[0..10],
            [
                "alert",
                "fly",
                "heal",
                "info",
                "jump",
                "prone",
                "proneStab",
                "stabO1",
                "stabO2",
                "stabOF"
            ]
        );
    }

    #[test]
    fn open_v83_img_tamingmob() {
        let img = open_v83_img("testdata/v83-tamingmob.img", "TamingMob");
        let root = img.root().unwrap().get().unwrap();
        assert_eq!(img.root_name(), Some("TamingMob"));
        assert_eq!(root, &WzObject::Property(WzProperty::Variant));
        assert_eq!(img.arena().count(), 110);

        // Cycle through all of the contents
        let mut objects: Vec<&str> = img
            .root_id()
            .children(img.arena())
            .map(|s| s.as_str())
            .collect();
        objects.sort();

        // Assert the contents
        assert_eq!(
            objects[0..10],
            [
                "fly", "info", "jump", "ladder", "prone", "rope", "stand1", "stand2", "tired",
                "walk1"
            ]
        );
    }

    #[test]
    fn open_v83_wz_string_cash() {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = WzEncryptedReader::open("testdata/v83-string.wz", &system).unwrap();
        let wz = WzFile::from_reader("String", &mut reader).unwrap();
        let node = wz.get_from_path("String/Cash.img").unwrap();
        let mut img = WzImage::from_node("Cash.img", node.get().unwrap()).unwrap();
        img.load(&mut reader).unwrap();
        let root = img.root().unwrap().get().unwrap();
        assert_eq!(img.root_name(), Some("Cash.img"));
        assert_eq!(root, &WzObject::Property(WzProperty::Variant));
        assert_eq!(img.arena().count(), 1505);

        // Cycle through all of the contents
        let mut objects: Vec<&str> = img
            .root_id()
            .children(img.arena())
            .map(|s| s.as_str())
            .collect();
        objects.sort();

        // Assert the contents
        assert_eq!(
            objects[0..10],
            [
                "5010000", "5010001", "5010002", "5010003", "5010004", "5010005", "5010006",
                "5010007", "5010008", "5010009"
            ]
        );

        img.unload().unwrap();
    }

    #[test]
    fn open_v172_img_weapon() {
        let img = open_v172_img("testdata/v172-weapon.img", "Weapon");
        let root = img.root().unwrap().get().unwrap();
        assert_eq!(img.root_name(), Some("Weapon"));
        assert_eq!(root, &WzObject::Property(WzProperty::Variant));
        assert_eq!(img.arena().count(), 265);

        // Cycle through all of the contents
        let mut objects: Vec<&str> = img
            .root_id()
            .children(img.arena())
            .map(|s| s.as_str())
            .collect();
        objects.sort();

        // Assert the contents
        assert_eq!(
            objects[0..10],
            [
                "alert",
                "fly",
                "heal",
                "info",
                "jump",
                "prone",
                "proneStab",
                "shoot1",
                "shootF",
                "stabO1"
            ]
        );
    }

    #[test]
    fn open_v172_img_tamingmob() {
        let img = open_v172_img("testdata/v172-tamingmob.img", "TamingMob");
        let root = img.root().unwrap().get().unwrap();
        assert_eq!(img.root_name(), Some("TamingMob"));
        assert_eq!(root, &WzObject::Property(WzProperty::Variant));
        assert_eq!(img.arena().count(), 110);

        // Cycle through all of the contents
        let mut objects: Vec<&str> = img
            .root_id()
            .children(img.arena())
            .map(|s| s.as_str())
            .collect();
        objects.sort();

        // Assert the contents
        assert_eq!(
            objects[0..10],
            [
                "fly", "info", "jump", "ladder", "prone", "rope", "stand1", "stand2", "tired",
                "walk1"
            ]
        );
    }

    #[test]
    fn open_v172_wz_string_cash() {
        let mut reader = WzReader::open("testdata/v172-string.wz").unwrap();
        let wz = WzFile::from_reader("String", &mut reader).unwrap();
        let node = wz.get_from_path("String/Cash.img").unwrap();
        let mut img = WzImage::from_node("Cash.img", node.get().unwrap()).unwrap();
        img.load(&mut reader).unwrap();
        let root = img.root().unwrap().get().unwrap();
        assert_eq!(img.root_name(), Some("Cash.img"));
        assert_eq!(root, &WzObject::Property(WzProperty::Variant));
        assert_eq!(img.arena().count(), 6998);

        // Cycle through all of the contents
        let mut objects: Vec<&str> = img
            .root_id()
            .children(img.arena())
            .map(|s| s.as_str())
            .collect();
        objects.sort();

        // Assert the contents
        assert_eq!(
            objects[0..10],
            [
                "5010000", "5010001", "5010002", "5010003", "5010004", "5010005", "5010006",
                "5010007", "5010008", "5010009"
            ]
        );

        img.unload().unwrap();
    }
}
