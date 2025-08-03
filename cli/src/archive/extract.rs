//! Parsing of WZ archives

use crate::utils;
use crypto::{Decryptor, KeyStream};
use std::{fs, io, path::PathBuf};
use wz::archive::{ContentType, Error, Reader};

pub(crate) fn do_extract(
    path: &PathBuf,
    key: Option<KeyStream>,
    version: Option<u16>,
    verbose: bool,
) -> Result<(), Error> {
    // Get the filename
    let basename = path
        .file_name()
        .ok_or(io::Error::from(io::ErrorKind::InvalidFilename))?
        .to_str()
        .ok_or(io::Error::from(io::ErrorKind::InvalidFilename))?;

    // Map the archive
    match key {
        Some(k) => match version {
            Some(v) => extract(basename, Reader::as_version(path, v, k)?, verbose),
            None => extract(basename, Reader::new(path, k)?, verbose),
        },
        None => match version {
            Some(v) => extract(basename, Reader::unencrypted_as_version(path, v)?, verbose),
            None => extract(basename, Reader::unencrypted(path)?, verbose),
        },
    }
}

fn extract<D>(name: &str, mut reader: Reader<D>, verbose: bool) -> Result<(), Error>
where
    D: Decryptor,
{
    let archive = reader.parse()?;
    let base = name.trim_end_matches(".wz");
    for (path, content) in archive.iter() {
        let path = format!("{}{}/{}", base, path, content.name());
        match &content.content_type {
            ContentType::Package(_) => {
                utils::create_dir(&path)?;
            }
            ContentType::Image(_) => {
                utils::remove_file(&path)?;
                let mut output = fs::File::create(&path)?;
                reader.copy_to(&mut output, content.offset, content.size)?;
            }
        }
        utils::verbose!(verbose, "{}", path);
    }
    Ok(())
}
