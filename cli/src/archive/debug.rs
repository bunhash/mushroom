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
    match key {
        Key::Gms => match version {
            Some(v) => debug(
                Archive::parse_as_version(path, v, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
                directory,
            ),
            None => debug(
                Archive::parse(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
                directory,
            ),
        },
        Key::Kms => match version {
            Some(v) => debug(
                Archive::parse_as_version(path, v, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
                directory,
            ),
            None => debug(
                Archive::parse(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
                directory,
            ),
        },
        Key::None => match version {
            Some(v) => debug(Archive::parse_unencrypted_as_version(path, v)?, directory),
            None => debug(Archive::parse_unencrypted(path)?, directory),
        },
    }
}

fn debug(archive: Archive, directory: &Option<String>) -> Result<(), Error> {
    match directory {
        Some(path) => match archive.clone_subtree(path) {
            Some(tree) => println!("{}", tree),
            None => println!("{} not found", path),
        },
        None => println!("{}", archive.tree),
    }
    Ok(())
}
