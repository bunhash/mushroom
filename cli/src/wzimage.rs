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
    file::Image,
    io::{DummyDecryptor, WzReader},
};

pub(crate) fn do_debug(
    file: &PathBuf,
    directory: &Option<String>,
    key: Key,
    position: i32,
    version: u16,
) -> Result<()> {
    let name = file_name(&file)?;
    let file = BufReader::new(fs::File::open(&file)?);
    let (_, version_checksum) = checksum(&version.to_string());
    match key {
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
    }
}

fn debug<R, D>(name: &str, mut reader: WzReader<R, D>, directory: &Option<String>) -> Result<()>
where
    R: Read + Seek,
    D: Decryptor,
{
    let image = Image::parse(name, &mut reader)?;
    println!("{:?}", image.map().debug_pretty_print());
    Ok(())
}
