//! Parsing of WZ archives

use crate::{archive::ImagePath, utils, Key};
use crypto::{KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use std::{
    fs,
    path::{Path, PathBuf},
};
use wz::{
    archive,
    error::{PackageError, Result},
    io::DummyEncryptor,
    types::WzHeader,
};

pub(crate) fn do_create(
    path: &PathBuf,
    directory: &str,
    verbose: bool,
    key: Key,
    version: u16,
) -> Result<()> {
    // Remove the WZ archive if it exists
    utils::remove_file(path)?;

    // Get the target directory and ensure it is actually a directory
    let directory = PathBuf::from(&directory);
    if !directory.is_dir() {
        return Err(PackageError::Path(directory.to_string_lossy().into()).into());
    }
    let target = utils::file_name(&directory)?;
    utils::verbose!(verbose, "{}", target);

    // Get the parent path of the directory (used to strip it from the WZ contents)
    let parent = utils::parent(&directory)?;

    // Create new WZ archive map
    let mut writer = archive::Writer::new(target);
    recursive_do_create(&directory, parent, &mut writer, verbose)?;

    // Create a new header
    let header = WzHeader::new(version);

    // Save the WZ archive with the proper encryption
    match key {
        Key::Gms => writer.save(path, version, header, KeyStream::new(&TRIMMED_KEY, &GMS_IV)),
        Key::Kms => writer.save(path, version, header, KeyStream::new(&TRIMMED_KEY, &KMS_IV)),
        Key::None => writer.save(path, version, header, DummyEncryptor),
    }
}

fn recursive_do_create(
    current: &Path,
    parent: &Path,
    writer: &mut archive::Writer<ImagePath>,
    verbose: bool,
) -> Result<()> {
    for file in fs::read_dir(current)? {
        let path = file?.path();
        let stripped_path = path.strip_prefix(parent).expect("prefix should exist");
        utils::verbose!(verbose, "{}", stripped_path.display());
        if path.is_dir() {
            writer.add_package(stripped_path)?;
            recursive_do_create(&path, parent, writer, verbose)?;
        } else if path.is_file() {
            writer.add_image(stripped_path, ImagePath::new(&path)?)?;
        }
    }
    Ok(())
}
