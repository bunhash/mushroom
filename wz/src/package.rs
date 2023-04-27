//! WZ Package Structure

use crate::{
    error::Result,
    map::{CursorMut, Map},
    types::{WzInt, WzOffset, WzString},
    Decode, Encode, Reader, Writer,
};

mod content;
mod error;
mod metadata;

pub use content::Content;
pub use error::Error;
pub use metadata::{Metadata, Params};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    /// Vector of content metadata
    pub contents: Vec<Metadata>,
}

impl Package {
    pub fn map_at<R>(name: &str, params: Params, reader: &mut R) -> Result<Map<Content>>
    where
        R: Reader,
    {
        reader.seek(params.offset)?;
        let package = Package::decode(reader)?;
        let mut map = Map::new(WzString::from(name), Content::Package(params));
        let mut cursor = map.cursor_mut();
        for content in package.contents {
            Self::map_content_to(content, &mut cursor, reader)?;
        }
        Ok(map)
    }

    pub fn map<R>(name: &str, reader: &mut R) -> Result<Map<Content>>
    where
        R: Reader,
    {
        let size = WzInt::from(reader.metadata().size as i32);
        let checksum = WzInt::from(0);
        let offset = WzOffset::from(reader.metadata().absolute_position as u32 + 2);
        Self::map_at(
            name,
            Params {
                size,
                checksum,
                offset,
            },
            reader,
        )
    }

    // *** PRIVATES *** //

    fn map_content_to<'a, R>(
        content: Metadata,
        cursor: &mut CursorMut<'a, Content>,
        reader: &mut R,
    ) -> Result<()>
    where
        R: Reader,
    {
        match content {
            Metadata::Package(name, params) => {
                reader.seek(params.offset)?;
                cursor
                    .create(name.clone(), Content::Package(params))?
                    .move_to(name.as_ref())?;
                let package = Package::decode(reader)?;
                for content in package.contents {
                    Self::map_content_to(content, cursor, reader)?;
                }
                cursor.parent()?;
                Ok(())
            }
            Metadata::Image(name, params) => {
                cursor.create(name, Content::Image(params))?;
                Ok(())
            }
        }
    }
}

impl Decode for Package {
    fn decode<R>(reader: &mut R) -> Result<Self>
    where
        R: Reader,
    {
        let num_contents = WzInt::decode(reader)?;
        if num_contents.is_negative() {
            return Err(Error::InvalidPackage.into());
        }
        let num_contents = *num_contents as usize;
        let mut contents = Vec::with_capacity(num_contents);
        for _ in 0..num_contents {
            contents.push(Metadata::decode(reader)?);
        }
        Ok(Self { contents })
    }
}

impl Encode for Package {
    fn encode<W>(&self, _writer: &mut W) -> Result<()>
    where
        W: Writer,
    {
        todo!("implement encode")
    }
}

#[cfg(test)]
mod tests {

    use crate::{package::Package, EncryptedReader, UnencryptedReader};
    use crypto::{MushroomSystem, GMS_IV, TRIMMED_KEY};
    use std::fs::File;

    #[test]
    fn map_v83_base() {
        let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);
        let mut reader = EncryptedReader::from_reader(
            File::open("testdata/v83-base.wz").expect("error opening file"),
            &system,
        )
        .expect("error making reader");
        let map = Package::map("Base.wz", &mut reader).expect("error mapping v83-base.wz");
        let cursor = map.cursor();
        assert_eq!(
            &cursor.list(),
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
        );
    }

    #[test]
    fn map_v172_base() {
        let mut reader = UnencryptedReader::from_reader(
            File::open("testdata/v172-base.wz").expect("error opening file"),
        )
        .expect("error making reader");
        let map = Package::map("Base.wz", &mut reader).expect("error mapping v172-base.wz");
        let cursor = map.cursor();
        assert_eq!(
            &cursor.list(),
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
        );
    }
}
