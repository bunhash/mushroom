//! Parsing of WZ images

use crate::{utils, Key};
use crypto::{KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use std::path::PathBuf;
use wz::{
    error::Result,
    image::Reader,
    io::{DummyDecryptor, WzRead},
};

pub(crate) fn do_debug(path: &PathBuf, directory: &Option<String>, key: Key) -> Result<()> {
    let name = utils::file_name(path)?;
    let result = match key {
        Key::Gms => debug(
            name,
            Reader::open(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
            directory,
        ),
        Key::Kms => debug(
            name,
            Reader::open(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
            directory,
        ),
        Key::None => debug(name, Reader::open(path, DummyDecryptor)?, directory),
    };
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{} failed: {:?}", path.display(), e);
            Ok(())
        }
    }
}

fn debug<R>(name: &str, mut reader: Reader<R>, directory: &Option<String>) -> Result<()>
where
    R: WzRead,
{
    let map = reader.map(name)?;
    let mut cursor = match directory {
        // Find the optional directory
        Some(ref path) => map.cursor_at(path)?,
        // Get the root
        None => {
            println!("{:?}", map.debug_pretty_print());
            return Ok(());
        }
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
