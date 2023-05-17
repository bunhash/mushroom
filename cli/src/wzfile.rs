//! Parsing of WZ archives

use crate::{file_name, Key};
use crypto::{KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};
use wz::{
    archive::{self, reader},
    error::{Error, PackageError, Result},
    image,
    io::{xml::writer::XmlWriter, DummyDecryptor, DummyEncryptor, WzImageReader, WzRead},
    list,
    types::WzHeader,
};

mod imagepath;

use imagepath::ImagePath;

pub(crate) fn do_create(
    path: &PathBuf,
    directory: &str,
    verbose: bool,
    key: Key,
    version: u16,
) -> Result<()> {
    // Remove the WZ archive if it exists
    if Path::new(path).is_file() {
        fs::remove_file(path)?;
    }

    // Get the target directory and ensure it is actually a directory
    let path = PathBuf::from(&directory);
    if !path.is_dir() {
        return Err(PackageError::Path(path.to_string_lossy().into()).into());
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
    let mut writer = archive::Writer::new(target);
    recursive_do_create(&path, parent, &mut writer, verbose)?;

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
        if verbose {
            println!("{}", stripped_path.display())
        }
        if path.is_dir() {
            writer.add_package(stripped_path)?;
            recursive_do_create(&path, parent, writer, verbose)?;
        } else if path.is_file() {
            writer.add_image(stripped_path, ImagePath::new(&path)?)?;
        }
    }
    Ok(())
}

pub(crate) fn do_list(path: &PathBuf, key: Key, version: Option<u16>) -> Result<()> {
    let name = file_name(path)?;

    // Map the WZ archive
    let map = match key {
        Key::Gms => match version {
            Some(v) => {
                archive::Reader::open_as_version(path, v, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?
                    .map(name)?
            }
            None => {
                archive::Reader::open(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?.map(name)?
            }
        },
        Key::Kms => match version {
            Some(v) => {
                archive::Reader::open_as_version(path, v, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?
                    .map(name)?
            }
            None => {
                archive::Reader::open(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?.map(name)?
            }
        },
        Key::None => match version {
            Some(v) => archive::Reader::open_as_version(path, v, DummyDecryptor)?.map(name)?,
            None => archive::Reader::open(path, DummyDecryptor)?.map(name)?,
        },
    };

    // Walk the map
    map.walk::<Error>(|cursor| {
        let path = cursor.pwd().join("/");
        Ok(println!("{}", &path))
    })
}

pub(crate) fn do_extract(
    path: &PathBuf,
    verbose: bool,
    key: Key,
    version: Option<u16>,
) -> Result<()> {
    let filename = file_name(path)?;
    match key {
        Key::Gms => extract(
            filename,
            match version {
                Some(v) => archive::Reader::open_as_version(
                    path,
                    v,
                    KeyStream::new(&TRIMMED_KEY, &GMS_IV),
                )?,
                None => archive::Reader::open(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
            },
            verbose,
        ),
        Key::Kms => extract(
            filename,
            match version {
                Some(v) => archive::Reader::open_as_version(
                    path,
                    v,
                    KeyStream::new(&TRIMMED_KEY, &KMS_IV),
                )?,
                None => archive::Reader::open(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
            },
            verbose,
        ),
        Key::None => extract(
            filename,
            match version {
                Some(v) => archive::Reader::open_as_version(path, v, DummyDecryptor)?,
                None => archive::Reader::open(path, DummyDecryptor)?,
            },
            verbose,
        ),
    }
}

fn extract<R>(name: &str, mut archive: archive::Reader<R>, verbose: bool) -> Result<()>
where
    R: WzRead,
{
    let map = archive.map(&name.replace(".wz", ""))?;
    let mut reader = archive.into_inner();
    map.walk::<Error>(|cursor| {
        let path = cursor.pwd().join("/");
        match cursor.get() {
            reader::Node::Package => {
                if !Path::new(&path).is_dir() {
                    fs::create_dir(&path)?;
                }
            }
            reader::Node::Image { offset, size } => {
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
    path: &PathBuf,
    directory: &Option<String>,
    key: Key,
    version: Option<u16>,
) -> Result<()> {
    let name = file_name(path)?;
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

pub(crate) fn do_list_file(path: &PathBuf, key: Key) -> Result<()> {
    let list = match key {
        Key::Gms => list::Reader::parse(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
        Key::Kms => list::Reader::parse(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
        Key::None => list::Reader::parse(path, DummyDecryptor)?,
    };
    for s in list.strings() {
        println!("{}", s);
    }
    Ok(())
}

pub(crate) fn do_server(
    path: &PathBuf,
    verbose: bool,
    key: Key,
    version: Option<u16>,
) -> Result<()> {
    let filename = file_name(path)?;
    match key {
        Key::Gms => server(
            filename,
            match version {
                Some(v) => archive::Reader::open_as_version(
                    path,
                    v,
                    KeyStream::new(&TRIMMED_KEY, &GMS_IV),
                )?,
                None => archive::Reader::open(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
            },
            verbose,
        ),
        Key::Kms => server(
            filename,
            match version {
                Some(v) => archive::Reader::open_as_version(
                    path,
                    v,
                    KeyStream::new(&TRIMMED_KEY, &KMS_IV),
                )?,
                None => archive::Reader::open(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
            },
            verbose,
        ),
        Key::None => server(
            filename,
            match version {
                Some(v) => archive::Reader::open_as_version(path, v, DummyDecryptor)?,
                None => archive::Reader::open(path, DummyDecryptor)?,
            },
            verbose,
        ),
    }
}

fn server<R>(name: &str, mut archive: archive::Reader<R>, verbose: bool) -> Result<()>
where
    R: WzRead,
{
    let map = archive.map(name)?;
    let mut reader = archive.into_inner();
    map.walk::<Error>(|cursor| {
        match cursor.get() {
            reader::Node::Package => {
                let path = cursor.pwd().join("/");
                if !Path::new(&path).is_dir() {
                    fs::create_dir(&path)?;
                }
            }
            reader::Node::Image { offset, .. } => {
                let path = format!("{}.xml", cursor.pwd().join("/"));
                if Path::new(&path).is_file() {
                    fs::remove_file(&path)?;
                }
                let mut image_reader = WzImageReader::new(&mut reader, *offset);
                image_reader.seek_to_start()?;
                let mut image = image::Reader::new(image_reader);
                let map = image.map(cursor.name())?;
                if verbose {
                    println!("{}", path);
                }
                let mut writer = XmlWriter::new(fs::File::create(&path)?);
                writer.write(&mut map.cursor())?;
            }
        }
        Ok(())
    })
}
