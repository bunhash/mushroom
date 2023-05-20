//! Parsing of WZ archives

use crate::{utils, Key};
use crypto::{KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use std::path::PathBuf;
use wz::{
    archive,
    error::{Error, Result},
    io::DummyDecryptor,
    list,
};

pub(crate) fn do_list(path: &PathBuf, key: Key, version: Option<u16>) -> Result<()> {
    let name = utils::file_name(path)?;

    // Map the WZ archive
    let map = match key {
        Key::Gms => match version {
            Some(v) => {
                archive::Reader::open_as_version(path, v, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?
                    .map(name)?
            }
            None => {
                archive::Reader::open(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?.map(name)?
            }
        },
        Key::Kms => match version {
            Some(v) => {
                archive::Reader::open_as_version(path, v, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?
                    .map(name)?
            }
            None => {
                archive::Reader::open(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?.map(name)?
            }
        },
        Key::None => match version {
            Some(v) => archive::Reader::open_as_version(path, v, DummyDecryptor)?.map(name)?,
            None => archive::Reader::open(path, DummyDecryptor)?.map(name)?,
        },
    };

    // Walk the map
    map.walk::<Error>(|cursor| Ok(println!("{}", &cursor.pwd())))
}

pub(crate) fn do_list_file(path: &PathBuf, key: Key) -> Result<()> {
    let reader = match key {
        Key::Gms => list::Reader::parse(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
        Key::Kms => list::Reader::parse(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
        Key::None => list::Reader::parse(path, DummyDecryptor)?,
    };
    for string in reader.strings() {
        println!("{}", string);
    }
    Ok(())
}
