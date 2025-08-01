//! Parsing of WZ archives

use crate::Key;
use crypto::{KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use std::path::PathBuf;
use wz::archive::{Archive, Error};

pub fn do_debug(
    path: &PathBuf,
    directory: &Option<String>,
    key: Key,
    version: Option<u16>,
) -> Result<(), Error> {
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
    match directory {
        Some(path) => match archive.map.get_subtree(path) {
            Some(tree) => println!("{}", tree),
            None => println!("{} not found", path),
        },
        None => println!("{}", archive.map),
    }
    Ok(())
}
