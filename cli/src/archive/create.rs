//! Parsing of WZ archives

use crate::utils;
use crypto::{Encryptor, KeyStream};
use std::{
    fs::{read_dir, File},
    io::{self, BufReader, Read},
    num::Wrapping,
    path::{Path, PathBuf},
};
use wz::archive::{
    builder::{Image, Package},
    Builder, Error, Writer,
};

struct ImageRef {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub checksum: i32,
}

impl ImageRef {
    fn new(name: &str, path: PathBuf) -> Result<Self, Error> {
        let size = path.metadata()?.len();
        let file = BufReader::new(File::open(&path)?);
        let checksum = file
            .bytes()
            .flatten()
            .map(|b| Wrapping(b as i32))
            .sum::<Wrapping<i32>>()
            .0;
        Ok(Self {
            name: name.to_string(),
            path,
            size,
            checksum,
        })
    }
}

impl Image for ImageRef {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn size(&self) -> u64 {
        self.size
    }

    fn checksum(&self) -> i32 {
        self.checksum
    }

    fn to_reader(&self) -> Result<Box<dyn Read>, io::Error> {
        Ok(Box::new(BufReader::new(File::open(&self.path)?)))
    }
}

pub fn do_create(
    path: &PathBuf,
    key: Option<KeyStream>,
    version: u16,
    directory: &str,
    verbose: bool,
) -> Result<(), Error> {
    match key {
        Some(k) => create(&mut Writer::new(path, k, version)?, directory, verbose),
        None => create(&mut Writer::unencrypted(path, version)?, directory, verbose),
    }
}

fn create<E>(writer: &mut Writer<E>, directory: &str, verbose: bool) -> Result<(), Error>
where
    E: Encryptor,
{
    let mut builder = Builder::new();
    recursive_do_create(
        &Path::new(directory),
        builder.root(),
        &Path::new(directory),
        verbose,
    )?;
    let archive = builder.write(writer)?;
    Ok(())
}

fn recursive_do_create<'a>(
    current: &Path,
    mut package: Package<'a, ImageRef>,
    root: &Path,
    verbose: bool,
) -> Result<(), Error> {
    // Images first
    let mut images = read_dir(current)?
        .filter_map(|path| match path {
            Ok(p) => {
                if !p.path().is_dir() {
                    Some(p.path())
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .collect::<Vec<_>>();
    images.sort();
    for path in images {
        let name = path
            .file_name()
            .ok_or(Error::other("invalid filename"))?
            .to_str()
            .ok_or(Error::other("invalid filename"))?;
        utils::verbose!(
            verbose,
            "{}",
            path.strip_prefix(root)
                .expect("panic! prefix should exist")
                .display()
        );
        let image = ImageRef::new(name, path.clone())?;
        package.add_image(image)?;
    }

    // Packages second
    let mut packages = read_dir(current)?
        .filter_map(|path| match path {
            Ok(p) => {
                if p.path().is_dir() {
                    Some(p.path())
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .collect::<Vec<_>>();
    packages.sort();
    for path in packages {
        let name = path
            .file_name()
            .ok_or(Error::other("invalid filename"))?
            .to_str()
            .ok_or(Error::other("invalid filename"))?;
        utils::verbose!(
            verbose,
            "{}",
            path.strip_prefix(root)
                .expect("panic! prefix should exist")
                .display()
        );
        recursive_do_create(&path, package.add_package(name)?, root, verbose)?;
    }
    Ok(())
}
