//! Parsing of WZ images

use crate::{file_name, Key};
use crypto::{KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use std::{fs, path::PathBuf};
use wz::{
    error::Result,
    image::{Reader, Writer},
    io::{DummyDecryptor, DummyEncryptor, WzRead},
};

mod create;
mod extract;

use create::map_image_from_xml;
use extract::extract_image_from_map;

pub(crate) fn do_create(path: &PathBuf, directory: &str, verbose: bool, key: Key) -> Result<()> {
    // Remove the WZ archive if it exists
    if path.is_file() {
        fs::remove_file(path)?;
    }
    let target = file_name(path)?;
    if verbose {
        println!("{}", target);
    }
    let mut writer = Writer::from_map(map_image_from_xml(target, directory, verbose)?);
    match key {
        Key::Gms => writer.save(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV)),
        Key::Kms => writer.save(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV)),
        Key::None => writer.save(path, DummyEncryptor),
    }
}

pub(crate) fn do_extract(path: &PathBuf, verbose: bool, key: Key) -> Result<()> {
    let name = file_name(path)?;
    let result = match key {
        Key::Gms => extract(
            name,
            Reader::open(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
            verbose,
        ),
        Key::Kms => extract(
            name,
            Reader::open(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
            verbose,
        ),
        Key::None => extract(name, Reader::open(path, DummyDecryptor)?, verbose),
    };
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{} failed: {:?}", path.display(), e);
            Ok(())
        }
    }
}

fn extract<R>(name: &str, mut reader: Reader<R>, verbose: bool) -> Result<()>
where
    R: WzRead,
{
    let map = reader.map(name)?;
    extract_image_from_map(&map, verbose)
}

pub(crate) fn do_debug(path: &PathBuf, directory: &Option<String>, key: Key) -> Result<()> {
    let name = file_name(path)?;
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
