//! WZ CLI Tool

use clap::{Args, Parser, ValueEnum};
use crypto::{Decryptor, Encryptor, KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
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

mod image;

use image::ImagePath;

#[derive(Parser)]
struct Cli {
    /// File for input/output
    #[arg(short, long, required = true)]
    file: PathBuf,

    /// Directory to create the WZ package from
    #[arg(value_name = "DIR")]
    directory: Option<String>,

    /// Command to do
    #[command(flatten)]
    action: Action,

    /// Verbose
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Expect encrypted GMS strings
    #[arg(short, long, value_enum)]
    key: Key,

    /// The version of WZ package. Required if create. Overrides the WZ version otherwise.
    #[arg(short = 'm', long)]
    version: Option<u16>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Action {
    /// Create a new WZ package
    #[arg(short = 'c', requires = "version", requires = "directory")]
    create: bool,

    /// List the WZ package contents
    #[arg(short = 't')]
    list: bool,

    /// Extract the WZ package
    #[arg(short = 'x')]
    extract: bool,

    /// Debug the WZ package
    #[arg(short = 'd')]
    debug: bool,

    /// Decode List.wz file
    #[arg(short = 'L')]
    list_file: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Key {
    Gms,
    Kms,
    None,
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

fn do_create<E>(
    version: u16,
    file: fs::File,
    encryptor: E,
    directory: &str,
    verbose: bool,
) -> Result<()>
where
    E: Encryptor,
{
    let path = PathBuf::from(&directory);
    if !path.is_dir() {
        return Err(WzError::InvalidPackage.into());
    }
    let target = file_name(&path)?;
    if verbose {
        println!("{}", target);
    }
    let parent = match path.parent() {
        Some(p) => p,
        None => return Err(ErrorKind::NotFound.into()),
    };
    let mut builder = Builder::new(target);
    recursive_do_create(&path, parent, &mut builder, verbose)?;
    builder.save(version, Header::new(version), file, encryptor)?;
    Ok(())
}

fn do_list<D>(name: &str, mut archive: Archive<D>) -> Result<()>
where
    D: Decryptor,
{
    let map = archive.map(name)?;
    map.walk::<Error>(|cursor| {
        let path = cursor.pwd().join("/");
        Ok(println!("{}", &path))
    })
}

fn do_extract<D>(name: &str, mut archive: Archive<D>, verbose: bool) -> Result<()>
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

fn do_debug<D>(name: &str, mut archive: Archive<D>, directory: &Option<String>) -> Result<()>
where
    D: Decryptor,
{
    println!("{:?}", archive.header());
    let map = archive.map(name)?;
    let mut cursor = match directory {
        Some(ref path) => {
            let path = path.split("/").collect::<Vec<&str>>();
            map.cursor_at(&path)?
        }
        None => map.cursor(),
    };
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

fn do_list_file<D>(file: fs::File, decryptor: D) -> Result<()>
where
    D: Decryptor,
{
    let list = List::parse(file, decryptor)?;
    for s in list.strings() {
        println!("{}", s);
    }
    Ok(())
}

fn file_name(path: &PathBuf) -> Result<&str> {
    match path.file_name() {
        Some(name) => match name.to_str() {
            Some(name) => Ok(name),
            None => return Err(ErrorKind::NotFound.into()),
        },
        None => return Err(ErrorKind::NotFound.into()),
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let action = &args.action;
    if action.create {
        if Path::new(&args.file).is_file() {
            fs::remove_file(&args.file)?;
        }
        let file = fs::File::create(&args.file)?;
        match args.key {
            Key::Gms => do_create(
                args.version.unwrap(),
                file,
                KeyStream::new(&TRIMMED_KEY, &GMS_IV),
                args.directory.unwrap().as_str(),
                args.verbose,
            )?,
            Key::Kms => do_create(
                args.version.unwrap(),
                file,
                KeyStream::new(&TRIMMED_KEY, &KMS_IV),
                args.directory.unwrap().as_str(),
                args.verbose,
            )?,
            Key::None => do_create(
                args.version.unwrap(),
                file,
                DummyEncryptor,
                args.directory.unwrap().as_str(),
                args.verbose,
            )?,
        }
    } else {
        let filename = file_name(&args.file)?;
        let file = fs::File::open(&args.file)?;
        if action.list {
            match args.key {
                Key::Gms => do_list(
                    filename,
                    match args.version {
                        Some(v) => Archive::open_as_version(
                            file,
                            v,
                            KeyStream::new(&TRIMMED_KEY, &GMS_IV),
                        )?,
                        None => Archive::open(file, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
                    },
                )?,
                Key::Kms => do_list(
                    filename,
                    match args.version {
                        Some(v) => Archive::open_as_version(
                            file,
                            v,
                            KeyStream::new(&TRIMMED_KEY, &KMS_IV),
                        )?,
                        None => Archive::open(file, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
                    },
                )?,
                Key::None => do_list(
                    filename,
                    match args.version {
                        Some(v) => Archive::open_as_version(file, v, DummyDecryptor)?,
                        None => Archive::open(file, DummyDecryptor)?,
                    },
                )?,
            }
        } else if action.extract {
            match args.key {
                Key::Gms => do_extract(
                    filename,
                    match args.version {
                        Some(v) => Archive::open_as_version(
                            file,
                            v,
                            KeyStream::new(&TRIMMED_KEY, &GMS_IV),
                        )?,
                        None => Archive::open(file, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
                    },
                    args.verbose,
                )?,
                Key::Kms => do_extract(
                    filename,
                    match args.version {
                        Some(v) => Archive::open_as_version(
                            file,
                            v,
                            KeyStream::new(&TRIMMED_KEY, &KMS_IV),
                        )?,
                        None => Archive::open(file, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
                    },
                    args.verbose,
                )?,
                Key::None => do_extract(
                    filename,
                    match args.version {
                        Some(v) => Archive::open_as_version(file, v, DummyDecryptor)?,
                        None => Archive::open(file, DummyDecryptor)?,
                    },
                    args.verbose,
                )?,
            }
        } else if action.debug {
            match args.key {
                Key::Gms => do_debug(
                    filename,
                    match args.version {
                        Some(v) => Archive::open_as_version(
                            file,
                            v,
                            KeyStream::new(&TRIMMED_KEY, &GMS_IV),
                        )?,
                        None => Archive::open(file, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
                    },
                    &args.directory,
                )?,
                Key::Kms => do_debug(
                    filename,
                    match args.version {
                        Some(v) => Archive::open_as_version(
                            file,
                            v,
                            KeyStream::new(&TRIMMED_KEY, &KMS_IV),
                        )?,
                        None => Archive::open(file, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
                    },
                    &args.directory,
                )?,
                Key::None => do_debug(
                    filename,
                    match args.version {
                        Some(v) => Archive::open_as_version(file, v, DummyDecryptor)?,
                        None => Archive::open(file, DummyDecryptor)?,
                    },
                    &args.directory,
                )?,
            }
        } else if action.list_file {
            match args.key {
                Key::Gms => do_list_file(file, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
                Key::Kms => do_list_file(file, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
                Key::None => do_list_file(file, DummyDecryptor)?,
            }
        }
    }
    Ok(())
}
