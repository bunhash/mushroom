//! WZ Package Structure

use crate::{
    error::{Error, Result},
    Decode, Reader, WzInt, WzOffset, WzString,
};

/// Possible `Content` types found in a `Package`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Content {
    /// I don't know what this is...
    Unknown([u8; 10]),

    /// UOL pointing to a different location
    Uol(i32),

    /// Package or directory
    Package(ContentInfo),

    /// Object - treat as a binary blob
    Object(ContentInfo),
}

impl Decode for Content {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        match reader.read_byte()? {
            1 => {
                let mut data = [0u8; 10];
                reader.read_into(&mut data)?;
                Ok(Content::Unknown(data))
            }
            2 => Ok(Content::Uol(i32::decode(reader)?)),
            3 => Ok(Content::Package(ContentInfo::decode(reader)?)),
            4 => Ok(Content::Object(ContentInfo::decode(reader)?)),
            _ => Err(Error::InvalidContentType),
        }
    }
}

/// Info describing the known `Content`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentInfo {
    /// Name of the `Content`
    pub name: WzString,

    /// Size of the `Content`
    pub size: WzInt,

    /// Checksum of the `Content` -- basically ignored
    pub checksum: WzInt,

    /// Position of the `Content` in the WZ file
    pub offset: WzOffset,
}

impl Decode for ContentInfo {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let name = WzString::decode(reader)?;
        let size = WzInt::decode(reader)?;
        let checksum = WzInt::decode(reader)?;
        let offset = WzOffset::decode(reader)?;
        Ok(Self {
            name,
            size,
            checksum,
            offset,
        })
    }
}

/// WZ Package Structure -- acts as a directory
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    /// Package's contents
    pub contents: Vec<Content>,
}

impl Decode for Package {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let num_entries = WzInt::decode(reader)?;
        if num_entries < 0 {
            return Err(Error::InvalidPackage);
        }
        let num_entries = *num_entries as usize;
        let mut contents = Vec::with_capacity(num_entries);
        for _ in 0..num_entries {
            contents.push(Content::decode(reader)?);
        }
        Ok(Self { contents })
    }
}

#[cfg(test)]
mod tests {

    use crate::{Content, Decode, EncryptedWzReader, Package, WzReader};
    use crypto::{MushroomSystem, GMS_IV, TRIMMED_KEY};
    use std::fs::File;

    #[test]
    fn v83_package() {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = EncryptedWzReader::new(
            File::open("testdata/v83-base.wz").expect("error opening file"),
            &system,
        )
        .expect("error making reader");
        let package = Package::decode(&mut reader).expect("error parsing package");
        let contents = package
            .contents
            .iter()
            .map(|c| match c {
                Content::Package(o) => o.name.as_ref(),
                Content::Object(o) => o.name.as_ref(),
                e => panic!("{:?}", e),
            })
            .collect::<Vec<&str>>();
        assert_eq!(
            &contents,
            &[
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
        )
    }

    #[test]
    fn v172_package() {
        let mut reader =
            WzReader::new(File::open("testdata/v172-base.wz").expect("error opening file"))
                .expect("error making reader");
        let package = Package::decode(&mut reader).expect("error parsing package");
        let contents = package
            .contents
            .iter()
            .map(|c| match c {
                Content::Package(o) => o.name.as_ref(),
                Content::Object(o) => o.name.as_ref(),
                e => panic!("{:?}", e),
            })
            .collect::<Vec<&str>>();
        assert_eq!(
            &contents,
            &[
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
        )
    }
}
