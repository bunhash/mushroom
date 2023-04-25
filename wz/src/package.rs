//! WZ Package Structure

use crate::{error::Result, types::WzInt, Decode, Encode, Reader, Writer};

mod content;
//mod map;
mod metadata;

pub use content::Content;
//pub use map::Map;
pub use metadata::{Metadata, Params};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    /// Total size of package
    pub size: u64,

    /// Number of contents in the package
    pub num_contents: WzInt,

    /// Vector of content information
    pub contents: Vec<Metadata>,
}

impl Decode for Package {
    fn decode<R>(_reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        todo!("decode package")
    }
}

impl Encode for Package {
    #[inline]
    fn encode_size(&self) -> u64 {
        self.size
    }

    fn encode<W>(&self, _writer: &mut W) -> Result<()>
    where
        W: Writer,
        Self: Sized,
    {
        todo!("encode packages")
    }
}

/*
use crate::{
    error::{Result, WzError},
    types::{WzInt, WzOffset, WzString},
    Decode, Encode, Reader, Writer,
};
use std::io::SeekFrom;

/// WZ Package Structure -- acts as a directory
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    /// Number of contents
    pub num_contents: WzInt,

    /// Package's contents
    pub contents: Vec<Content>,
}

impl Decode for Package {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let num_contents = WzInt::decode(reader)?;
        if num_contents.is_negative() {
            return Err(WzError::InvalidPackage.into());
        }
        let len = *num_entries as usize;
        let mut contents = Vec::with_capacity(len);
        for _ in 0..len {
            contents.push(Content::decode(reader)?);
        }
        Ok(Self { contents })
    }
}

impl Encode for Package {
    #[inline]
    fn encode_size(&self) -> u64 {
        WzInt::from(self.contents.len() as i32).encode_size()
            + self
                .contents
                .iter()
                .map(|content| content.encode_size())
                .sum::<u64>()
    }

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

    use crate::{
        package::{Content, Package},
        Decode, EncryptedReader, UnencryptedReader,
    };
    use crypto::{MushroomSystem, GMS_IV, TRIMMED_KEY};
    use std::fs::File;

    #[test]
    fn v83_package() {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = EncryptedReader::from_reader(
            "Base.wz",
            File::open("testdata/v83-base.wz").expect("error opening file"),
            &system,
        )
        .expect("error making reader");
        let package = Package::decode(&mut reader).expect("error parsing package");
        let contents = package
            .contents
            .iter()
            .map(|c| match c {
                Content::Package(name, _) => String::from(name.as_ref()),
                Content::Image(name, _) => String::from(name.as_ref()),
                e => panic!("{:?}", e),
            })
            .collect::<Vec<String>>();
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
        let mut reader = UnencryptedReader::from_reader(
            "Base.wz",
            File::open("testdata/v172-base.wz").expect("error opening file"),
        )
        .expect("error making reader file");
        let package = Package::decode(&mut reader).expect("error parsing package");
        let contents = package
            .contents
            .iter()
            .map(|c| match c {
                Content::Package(name, _) => String::from(name.as_ref()),
                Content::Image(name, _) => String::from(name.as_ref()),
                e => panic!("{:?}", e),
            })
            .collect::<Vec<String>>();
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
*/
