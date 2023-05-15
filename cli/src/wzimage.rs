//! Parsing of WZ images

use crate::{file_name, Key};
use crypto::{Decryptor, KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use std::{
    fs,
    io::{BufReader, Read, Seek},
    path::PathBuf,
};
use wz::{
    error::Result,
    image::map_image,
    io::{DummyDecryptor, WzReader},
};

mod extract;

use extract::extract_image_from_map;

pub(crate) fn do_extract(filename: &PathBuf, verbose: bool, key: Key) -> Result<()> {
    let name = file_name(filename)?;
    let file = BufReader::new(fs::File::open(filename)?);
    let result = match key {
        Key::Gms => extract(
            name,
            WzReader::new(0, 0, file, KeyStream::new(&TRIMMED_KEY, &GMS_IV)),
            verbose,
        ),
        Key::Kms => extract(
            name,
            WzReader::new(0, 0, file, KeyStream::new(&TRIMMED_KEY, &KMS_IV)),
            verbose,
        ),
        Key::None => extract(name, WzReader::new(0, 0, file, DummyDecryptor), verbose),
    };
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{} failed: {:?}", filename.display(), e);
            Ok(())
        }
    }
}

fn extract<R, D>(name: &str, mut reader: WzReader<R, D>, verbose: bool) -> Result<()>
where
    R: Read + Seek,
    D: Decryptor,
{
    let map = map_image(name, &mut reader)?;
    extract_image_from_map(&map, verbose)
}

pub(crate) fn do_debug(filename: &PathBuf, directory: &Option<String>, key: Key) -> Result<()> {
    let name = file_name(filename)?;
    let file = BufReader::new(fs::File::open(filename)?);
    let result = match key {
        Key::Gms => debug(
            name,
            WzReader::new(0, 0, file, KeyStream::new(&TRIMMED_KEY, &GMS_IV)),
            directory,
        ),
        Key::Kms => debug(
            name,
            WzReader::new(0, 0, file, KeyStream::new(&TRIMMED_KEY, &KMS_IV)),
            directory,
        ),
        Key::None => debug(name, WzReader::new(0, 0, file, DummyDecryptor), directory),
    };
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{} failed: {:?}", filename.display(), e);
            Ok(())
        }
    }
}

fn debug<R, D>(name: &str, mut reader: WzReader<R, D>, directory: &Option<String>) -> Result<()>
where
    R: Read + Seek,
    D: Decryptor,
{
    let map = map_image(name, &mut reader)?;
    let mut cursor = match directory {
        // Find the optional directory
        Some(ref path) => {
            let path = path.split('/').collect::<Vec<&str>>();
            map.cursor_at(&path)?
        }
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
