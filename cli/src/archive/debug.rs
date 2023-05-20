//! Parsing of WZ archives

use crate::{utils, Key};
use crypto::{KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use std::path::PathBuf;
use wz::{
    archive,
    error::Result,
    io::{DummyDecryptor, WzRead},
};

pub(crate) fn do_debug(
    path: &PathBuf,
    directory: &Option<String>,
    key: Key,
    version: Option<u16>,
) -> Result<()> {
    let name = utils::file_name(path)?;
    match key {
        Key::Gms => match version {
            Some(v) => debug(
                name,
                archive::Reader::open_as_version(path, v, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
                directory,
            ),
            None => debug(
                name,
                archive::Reader::open(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
                directory,
            ),
        },
        Key::Kms => match version {
            Some(v) => debug(
                name,
                archive::Reader::open_as_version(path, v, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
                directory,
            ),
            None => debug(
                name,
                archive::Reader::open(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
                directory,
            ),
        },
        Key::None => match version {
            Some(v) => debug(
                name,
                archive::Reader::open_as_version(path, v, DummyDecryptor)?,
                directory,
            ),
            None => debug(
                name,
                archive::Reader::open(path, DummyDecryptor)?,
                directory,
            ),
        },
    }
}

fn debug<R>(name: &str, mut archive: archive::Reader<R>, directory: &Option<String>) -> Result<()>
where
    R: WzRead,
{
    // Print the archive header
    println!("{:?}", archive.header());
    let map = archive.map(name)?;
    let mut cursor = match directory {
        // Find the optional directory
        Some(ref path) => map.cursor_at(path)?,
        // Get the root
        None => map.cursor(),
    };

    // Print the directory and its immediate children
    println!("{:?} : {:?}", cursor.name(), cursor.get());
    let mut num_children = cursor.children().count();
    if num_children > 0 {
        cursor.first_child()?;
        loop {
            if num_children <= 1 {
                println!("`-- {:?} : {:?}", cursor.name(), cursor.get());
                break;
            } else {
                println!("|-- {:?} : {:?}", cursor.name(), cursor.get());
            }
            num_children -= 1;
            cursor.next_sibling()?;
        }
    }
    Ok(())
}
