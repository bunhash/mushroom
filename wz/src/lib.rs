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
//! let root = wz.root().unwrap().get();
//! assert!(root.is_directory());
//! assert_eq!(root.name(), "Base");
//! assert_eq!(wz.arena().count(), 19);
//!
//! // Cycle through all of the contents
//! let objects: Vec<&str> = wz.iter().map(|o| o.get().name()).collect();
//! assert_eq!(
//!     objects,
//!     [
//!         "Base",
//!         "smap.img",
//!         "zmap.img",
//!         "StandardPDD.img",
//!         "UI",
//!         "Effect",
//!         "Sound",
//!         "Map",
//!         "Character",
//!         "Item",
//!         "TamingMob",
//!         "Etc",
//!         "Npc",
//!         "Reactor",
//!         "Skill",
//!         "Morph",
//!         "String",
//!         "Mob",
//!         "Quest"
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
pub mod tree;
mod vector;

pub use canvas::WzCanvas;
pub use error::{WzError, WzErrorType, WzResult};
pub use file::WzFile;
pub use header::WzHeader;
pub use image::WzImage;
pub use node::{WzNode, WzNodeType};
pub use object::{WzObject, WzObjectValue};
pub use property::WzProperty;
pub use reader::{WzEncryptedReader, WzRead, WzReader};
pub use sound::WzSound;
pub use vector::WzVector;

#[cfg(test)]
mod tests {

    use crate::{WzEncryptedReader, WzFile, WzImage, WzObjectValue, WzProperty, WzReader};
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
        let root = wz.root().unwrap().get();
        assert!(root.is_directory());
        assert_eq!(root.name(), "Base");
        assert_eq!(wz.arena().count(), 19);

        // Cycle through all of the contents
        let objects: Vec<&str> = wz.iter().map(|o| o.get().name()).collect();

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
        let node = wz.get_from_path("Base/zmap.img").unwrap();
        let object = node.get();
        assert!(!object.is_directory());
        assert_eq!(object.name(), "zmap.img");
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
        let root = wz.root().unwrap().get();
        assert!(root.is_directory());
        assert_eq!(root.name(), "String");
        assert_eq!(wz.arena().count(), 21);

        // Cycle through all of the contents
        let objects: Vec<&str> = wz.iter().map(|o| o.get().name()).collect();

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
        let node = wz.get_from_path("String/Cash.img").unwrap();
        let object = node.get();
        assert!(!object.is_directory());
        assert_eq!(object.name(), "Cash.img");
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
        let root = wz.root().unwrap().get();
        assert!(root.is_directory());
        assert_eq!(root.name(), "Base");
        assert_eq!(wz.arena().count(), 21);

        // Cycle through all of the contents
        let objects: Vec<&str> = wz.iter().map(|o| o.get().name()).collect();

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
        let node = wz.get_from_path("Base/StandardPDD.img").unwrap();
        let object = node.get();
        assert!(!object.is_directory());
        assert_eq!(object.name(), "StandardPDD.img");
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
        let root = wz.root().unwrap().get();
        assert!(root.is_directory());
        assert_eq!(root.name(), "String");
        assert_eq!(wz.arena().count(), 33);

        // Cycle through all of the contents
        let objects: Vec<&str> = wz.iter().map(|o| o.get().name()).collect();

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
        let node = wz.get_from_path("String/TestEULA.img").unwrap();
        let object = node.get();
        assert!(!object.is_directory());
        assert_eq!(object.name(), "TestEULA.img");
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
        let root = img.root().unwrap().get();
        assert_eq!(root.value(), &WzObjectValue::Property(WzProperty::Variant));
        assert_eq!(root.name(), "Weapon");
        assert_eq!(img.arena().count(), 177);

        // Cycle through all of the contents
        let objects: Vec<&str> = img.iter().map(|o| o.get().name()).collect();

        // Assert the contents
        assert_eq!(
            objects[0..10],
            [
                "Weapon",
                "info",
                "icon",
                "iconRaw",
                "islot",
                "vslot",
                "walk",
                "stand",
                "attack",
                "afterImage"
            ]
        );
    }

    #[test]
    fn open_v83_img_tamingmob() {
        let img = open_v83_img("testdata/v83-tamingmob.img", "TamingMob");
        let root = img.root().unwrap().get();
        assert_eq!(root.value(), &WzObjectValue::Property(WzProperty::Variant));
        assert_eq!(root.name(), "TamingMob");
        assert_eq!(img.arena().count(), 110);

        // Cycle through all of the contents
        let objects: Vec<&str> = img.iter().map(|o| o.get().name()).collect();

        // Assert the contents
        assert_eq!(
            objects[0..10],
            [
                "TamingMob",
                "info",
                "icon",
                "iconRaw",
                "islot",
                "vslot",
                "reqJob",
                "reqLevel",
                "tuc",
                "tamingMob"
            ]
        );
    }

    #[test]
    fn open_v83_wz_string_cash() {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = WzEncryptedReader::open("testdata/v83-string.wz", &system).unwrap();
        let wz = WzFile::from_reader("String", &mut reader).unwrap();
        let node = wz.get_from_path("String/Cash.img").unwrap();
        let mut img = WzImage::try_from(node.get()).unwrap();
        img.load(&mut reader).unwrap();
        let root = img.root().unwrap().get();
        assert_eq!(root.value(), &WzObjectValue::Property(WzProperty::Variant));
        assert_eq!(root.name(), "Cash.img");
        assert_eq!(img.arena().count(), 1505);

        // Cycle through all of the contents
        let objects: Vec<&str> = img.iter().map(|o| o.get().name()).collect();

        // Assert the contents
        assert_eq!(
            objects[0..10],
            [
                "Cash.img", "5010000", "name", "desc", "5010001", "name", "desc", "5010002",
                "name", "desc"
            ]
        );

        img.unload().unwrap();
    }

    #[test]
    fn open_v172_img_weapon() {
        let img = open_v172_img("testdata/v172-weapon.img", "Weapon");
        let root = img.root().unwrap().get();
        assert_eq!(root.value(), &WzObjectValue::Property(WzProperty::Variant));
        assert_eq!(root.name(), "Weapon");
        assert_eq!(img.arena().count(), 265);

        // Cycle through all of the contents
        let objects: Vec<&str> = img.iter().map(|o| o.get().name()).collect();

        // Assert the contents
        assert_eq!(
            objects[0..10],
            [
                "Weapon",
                "info",
                "icon",
                "iconRaw",
                "islot",
                "vslot",
                "walk",
                "stand",
                "attack",
                "afterImage"
            ]
        );
    }

    #[test]
    fn open_v172_img_tamingmob() {
        let img = open_v172_img("testdata/v172-tamingmob.img", "TamingMob");
        let root = img.root().unwrap().get();
        assert_eq!(root.value(), &WzObjectValue::Property(WzProperty::Variant));
        assert_eq!(root.name(), "TamingMob");
        assert_eq!(img.arena().count(), 110);

        // Cycle through all of the contents
        let objects: Vec<&str> = img.iter().map(|o| o.get().name()).collect();

        // Assert the contents
        assert_eq!(
            objects[0..10],
            [
                "TamingMob",
                "info",
                "icon",
                "iconRaw",
                "islot",
                "vslot",
                "reqJob",
                "reqLevel",
                "tuc",
                "tamingMob"
            ]
        );
    }

    #[test]
    fn open_v172_wz_string_cash() {
        let mut reader = WzReader::open("testdata/v172-string.wz").unwrap();
        let wz = WzFile::from_reader("String", &mut reader).unwrap();
        let node = wz.get_from_path("String/Cash.img").unwrap();
        let mut img = WzImage::try_from(node.get()).unwrap();
        img.load(&mut reader).unwrap();
        let root = img.root().unwrap().get();
        assert_eq!(root.value(), &WzObjectValue::Property(WzProperty::Variant));
        assert_eq!(root.name(), "Cash.img");
        assert_eq!(img.arena().count(), 6998);

        // Cycle through all of the contents
        let objects: Vec<&str> = img.iter().map(|o| o.get().name()).collect();

        // Assert the contents
        assert_eq!(
            objects[0..10],
            [
                "Cash.img", "5010000", "name", "desc", "5010001", "name", "desc", "5010002",
                "name", "desc"
            ]
        );

        img.unload().unwrap();
    }
}
