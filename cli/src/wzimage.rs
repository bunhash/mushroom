//! Parsing of WZ images

use crate::{file_name, Key};
use crypto::{checksum, Decryptor, KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use std::{
    fs,
    io::{BufReader, ErrorKind, Read, Seek},
    path::{Path, PathBuf},
};
use wz::{
    archive,
    error::{Error, Result, WzError},
    file::{image::Node, Image},
    io::{DummyDecryptor, WzReader},
};

pub(crate) fn do_debug(
    filename: &PathBuf,
    directory: &Option<String>,
    key: Key,
    position: i32,
    version: u16,
) -> Result<()> {
    let name = file_name(&filename)?;
    let file = BufReader::new(fs::File::open(&filename)?);
    let (_, version_checksum) = checksum(&version.to_string());
    let result = match key {
        Key::Gms => debug(
            name,
            WzReader::new(
                position,
                version_checksum,
                file,
                KeyStream::new(&TRIMMED_KEY, &GMS_IV),
            ),
            directory,
        ),
        Key::Kms => debug(
            name,
            WzReader::new(
                position,
                version_checksum,
                file,
                KeyStream::new(&TRIMMED_KEY, &KMS_IV),
            ),
            directory,
        ),
        Key::None => debug(
            name,
            WzReader::new(position, version_checksum, file, DummyDecryptor),
            directory,
        ),
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
    let image = Image::parse(name, &mut reader)?;
    /*
    let map = image.map();
    let mut cursor = match directory {
        // Find the optional directory
        Some(ref path) => {
            let path = path.split("/").collect::<Vec<&str>>();
            map.cursor_at(&path)?
        }
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
            num_children = num_children - 1;
            cursor.next_sibling()?;
        }
    }
    */
    Ok(())
}
