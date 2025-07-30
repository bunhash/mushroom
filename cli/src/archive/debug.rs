//! Parsing of WZ archives

use crate::{utils, Key};
use crypto::{KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use std::path::PathBuf;
use wz::archive::{Archive, Error};

pub fn do_debug(
    path: &PathBuf,
    directory: &Option<String>,
    key: Key,
    version: Option<u16>,
) -> Result<(), Error> {
    let name = utils::file_name(path)?;
    match key {
        Key::Gms => match version {
            Some(v) => debug(
                name,
                Archive::parse_as_version(path, v, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
                directory,
            ),
            None => debug(
                name,
                Archive::parse(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
                directory,
            ),
        },
        Key::Kms => match version {
            Some(v) => debug(
                name,
                Archive::parse_as_version(path, v, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
                directory,
            ),
            None => debug(
                name,
                Archive::parse(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
                directory,
            ),
        },
        Key::None => match version {
            Some(v) => debug(
                name,
                Archive::parse_unencrypted_as_version(path, v)?,
                directory,
            ),
            None => debug(name, Archive::parse_unencrypted(path)?, directory),
        },
    }
}

fn debug(name: &str, mut archive: Archive, directory: &Option<String>) -> Result<(), Error> {
    match directory {
        Some(path) => match archive.clone_subtree(path) {
            Some(tree) => println!("{}", tree),
            None => println!("{} not found", path),
        },
        None => println!("{}", archive.tree),
    }
    Ok(())
}
