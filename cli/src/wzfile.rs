//! Parsing of WZ archives

use crate::{file_name, Key};
use crypto::{Decryptor, KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};
use wz::{
    archive,
    error::{Error, Result, WzError},
    file::Header,
    io::{DummyDecryptor, DummyEncryptor},
    Archive, Builder, List,
};

mod imagepath;

use imagepath::ImagePath;

pub(crate) fn do_create(
    file: &PathBuf,
    directory: &String,
    verbose: bool,
    key: Key,
    version: u16,
) -> Result<()> {
    // Remove the WZ archive if it exists
    if Path::new(&file).is_file() {
        fs::remove_file(&file)?;
    }
    let file = fs::File::create(&file)?;

    // Get the target directory and ensure it is actually a directory
    let path = PathBuf::from(&directory);
    if !path.is_dir() {
        return Err(WzError::InvalidPackage.into());
    }
    let target = file_name(&path)?;
    if verbose {
        println!("{}", target);
    }

    // Get the parent path of the directory (used to strip it from the WZ contents)
    let parent = match path.parent() {
        Some(p) => p,
        None => return Err(ErrorKind::NotFound.into()),
    };

    // Create new WZ archive map
    let mut builder = Builder::new(target);
    recursive_do_create(&path, parent, &mut builder, verbose)?;

    // Create a new header
    let header = Header::new(version);

    // Save the WZ file with the proper encryption
    match key {
        Key::Gms => builder.save(version, header, file, KeyStream::new(&TRIMMED_KEY, &GMS_IV)),
        Key::Kms => builder.save(version, header, file, KeyStream::new(&TRIMMED_KEY, &KMS_IV)),
        Key::None => builder.save(version, header, file, DummyEncryptor),
    }
}

fn recursive_do_create(
    current: &Path,
    parent: &Path,
    builder: &mut Builder<ImagePath>,
    verbose: bool,
) -> Result<()> {
    for file in fs::read_dir(&current)? {
        let path = file?.path();
        let stripped_path = path.strip_prefix(parent).expect("prefix should exist");
        if verbose {
            println!("{}", stripped_path.display())
        }
        if path.is_dir() {
            builder.add_package(&stripped_path)?;
            recursive_do_create(&path, parent, builder, verbose)?;
        } else if path.is_file() {
            builder.add_image(&stripped_path, ImagePath::new(&path)?)?;
        }
    }
    Ok(())
}

pub(crate) fn do_list(file: &PathBuf, key: Key, version: Option<u16>) -> Result<()> {
    let name = file_name(&file)?;
    let file = fs::File::open(&file)?;

    // Map the WZ archive
    let map = match key {
        Key::Gms => match version {
            Some(v) => Archive::open_as_version(file, v, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?
                .map(name)?,
            None => Archive::open(file, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?.map(name)?,
        },
        Key::Kms => match version {
            Some(v) => Archive::open_as_version(file, v, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?
                .map(name)?,
            None => Archive::open(file, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?.map(name)?,
        },
        Key::None => match version {
            Some(v) => Archive::open_as_version(file, v, DummyDecryptor)?.map(name)?,
            None => Archive::open(file, DummyDecryptor)?.map(name)?,
        },
    };

    // Walk the map
    map.walk::<Error>(|cursor| {
        let path = cursor.pwd().join("/");
        Ok(println!("{}", &path))
    })
}

pub(crate) fn do_extract(
    file: &PathBuf,
    verbose: bool,
    key: Key,
    version: Option<u16>,
) -> Result<()> {
    let filename = file_name(&file)?;
    let file = fs::File::open(&file)?;
    match key {
        Key::Gms => extract(
            filename,
            match version {
                Some(v) => {
                    Archive::open_as_version(file, v, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?
                }
                None => Archive::open(file, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
            },
            verbose,
        ),
        Key::Kms => extract(
            filename,
            match version {
                Some(v) => {
                    Archive::open_as_version(file, v, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?
                }
                None => Archive::open(file, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
            },
            verbose,
        ),
        Key::None => extract(
            filename,
            match version {
                Some(v) => Archive::open_as_version(file, v, DummyDecryptor)?,
                None => Archive::open(file, DummyDecryptor)?,
            },
            verbose,
        ),
    }
}

fn extract<D>(name: &str, mut archive: Archive<D>, verbose: bool) -> Result<()>
where
    D: Decryptor,
{
    let map = archive.map(&name.replace(".wz", ""))?;
    let mut reader = archive.into_inner();
    map.walk::<Error>(|cursor| {
        let path = cursor.pwd().join("/");
        match cursor.get() {
            archive::Node::Package => {
                if !Path::new(&path).is_dir() {
                    fs::create_dir(&path)?;
                }
            }
            archive::Node::Image { offset, size } => {
                if Path::new(&path).is_file() {
                    fs::remove_file(&path)?;
                }
                let mut output = fs::File::create(&path)?;
                reader.copy_to(&mut output, *offset, *size)?;
            }
        }
        if verbose {
            println!("{}", path);
        }
        Ok(())
    })
}

pub(crate) fn do_debug(
    file: &PathBuf,
    directory: &Option<String>,
    key: Key,
    version: Option<u16>,
) -> Result<()> {
    let name = file_name(&file)?;
    let file = fs::File::open(&file)?;
    match key {
        Key::Gms => match version {
            Some(v) => debug(
                name,
                Archive::open_as_version(file, v, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
                directory,
            ),
            None => debug(
                name,
                Archive::open(file, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
                directory,
            ),
        },
        Key::Kms => match version {
            Some(v) => debug(
                name,
                Archive::open_as_version(file, v, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
                directory,
            ),
            None => debug(
                name,
                Archive::open(file, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
                directory,
            ),
        },
        Key::None => match version {
            Some(v) => debug(
                name,
                Archive::open_as_version(file, v, DummyDecryptor)?,
                directory,
            ),
            None => debug(name, Archive::open(file, DummyDecryptor)?, directory),
        },
    }
}

fn debug<D>(name: &str, mut archive: Archive<D>, directory: &Option<String>) -> Result<()>
where
    D: Decryptor,
{
    // Print the archive header
    println!("{:?}", archive.header());
    let map = archive.map(name)?;
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
    Ok(())
}

pub(crate) fn do_list_file(file: &PathBuf, key: Key) -> Result<()> {
    let file = fs::File::open(&file)?;
    let list = match key {
        Key::Gms => List::parse(file, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
        Key::Kms => List::parse(file, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
        Key::None => List::parse(file, DummyDecryptor)?,
    };
    for s in list.strings() {
        println!("{}", s);
    }
    Ok(())
}
