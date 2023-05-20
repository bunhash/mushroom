//! Parsing of WZ images

use crate::{utils, Key};
use crypto::{KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use std::path::PathBuf;
use wz::{
    error::{Error, Result},
    image::Reader,
    io::DummyDecryptor,
};

pub(crate) fn do_list(path: &PathBuf, key: Key) -> Result<()> {
    let name = utils::file_name(path)?;
    let map = match key {
        Key::Gms => Reader::open(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?.map(name)?,
        Key::Kms => Reader::open(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?.map(name)?,
        Key::None => Reader::open(path, DummyDecryptor)?.map(name)?,
    };
    map.walk::<Error>(|cursor| Ok(println!("{}", &cursor.pwd())))
}
