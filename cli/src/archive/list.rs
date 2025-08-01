//! Parsing of WZ archives

use crate::Key;
use crypto::{KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use std::path::PathBuf;
use wz::archive::{Archive, ContentType, Error};

pub fn do_list(path: &PathBuf, key: Key, version: Option<u16>) -> Result<(), Error> {
    let archive = match key {
        Key::Gms => match version {
            Some(v) => Archive::parse_as_version(path, v, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
            None => Archive::parse(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
        },
        Key::Kms => match version {
            Some(v) => Archive::parse_as_version(path, v, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
            None => Archive::parse(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
        },
        Key::None => match version {
            Some(v) => Archive::parse_unencrypted_as_version(path, v)?,
            None => Archive::parse_unencrypted(path)?,
        },
    };
    for (path, content) in archive.map.iter() {
        match &content.content_type {
            ContentType::Image(name) => println!("{}/{}", path, name),
            _ => {}
        }
    }
    Ok(())
}

/*
pub fn do_list_file(path: &PathBuf, key: Key) -> Result<()> {
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
*/
