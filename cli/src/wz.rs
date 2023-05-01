//! WZ CLI Tool

use clap::{Args, Parser};
use crypto::{Decryptor, Encryptor, KeyStream, GMS_IV, TRIMMED_KEY};
use std::{
    fs,
    io::{copy, BufReader, Read},
    path::Path,
};
use wz::{
    error::{Error, Result},
    file::{ContentRef, ImageRef, PackageRef},
    map::{CursorMut, Map},
    reader::DummyDecryptor,
    types::{WzInt, WzString},
    writer::DummyEncryptor,
    WzFile,
};

#[derive(Parser)]
struct Cli {
    /// File for input/output
    #[arg(short, long, required = true)]
    file: String,

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
    #[arg(short, long, default_value_t = false)]
    legacy: bool,

    /// The version of WZ package to make
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
}

fn recursive_do_create(
    directory: &str,
    cursor: &mut CursorMut<ContentRef>,
    verbose: bool,
) -> Result<()> {
    for child in fs::read_dir(directory)? {
        match child {
            Ok(child) => {
                let name = WzString::from(child.file_name().to_str().unwrap());
                let path = child.path();
                if verbose {
                    println!("{}", path.to_str().unwrap());
                }
                if path.is_dir() {
                    let package = ContentRef::Package(PackageRef::new(name.as_ref()));
                    cursor.create(name, package)?;
                } else if path.is_file() {
                    let metadata = path.metadata()?;
                    let mut checksum = WzInt::from(0);
                    let reader = BufReader::new(fs::File::open(path)?);
                    for byte in reader.bytes() {
                        checksum = WzInt::from(checksum.wrapping_add(byte? as i32));
                    }
                    let image = ContentRef::Image(ImageRef::new(
                        name.as_ref(),
                        WzInt::from(metadata.len()),
                        checksum,
                    ));
                    cursor.create(name, image)?;
                }
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}

fn do_create<E>(file: WzFile, directory: &str, _encryptor: E, verbose: bool) -> Result<()>
where
    E: Encryptor,
{
    let name = file.file_name()?;
    let mut map = Map::new(
        WzString::from(name),
        ContentRef::Package(PackageRef::new(name)),
    );
    recursive_do_create(directory, &mut map.cursor_mut(), verbose)?;
    file.calculate_offsets(&mut map)?;
    println!("{:?}", map.debug_pretty_print());
    Ok(())
}

fn do_list<D>(file: WzFile, decryptor: D) -> Result<()>
where
    D: Decryptor,
{
    let name = file.file_name()?;
    let name_without_extension = &name.replace(".wz", "");
    let map = file.map(decryptor)?;
    map.walk::<Error>(|cursor| {
        let path = cursor.pwd().join("/").replace(name, name_without_extension);
        println!("{}", &path);
        Ok(())
    })
}

fn do_extract<D>(file: WzFile, decryptor: D, verbose: bool) -> Result<()>
where
    D: Decryptor,
{
    let name = file.file_name()?;
    let name_without_extension = &name.replace(".wz", "");
    let map = file.map(decryptor)?;
    map.walk::<Error>(|cursor| {
        let path = cursor.pwd().join("/").replace(name, name_without_extension);
        let data = cursor.get();
        if verbose {
            println!("{}", path);
        }
        match data {
            ContentRef::Package(_) => {
                if !Path::new(&path).exists() {
                    fs::create_dir(&path)?;
                }
            }
            ContentRef::Image(image) => {
                if Path::new(&path).exists() {
                    fs::remove_file(&path)?;
                }
                let mut reader = file.image_reader(&image)?;
                let mut writer = fs::File::create(&path)?;
                copy(&mut reader, &mut writer)?;
            }
        }
        Ok(())
    })
}

fn do_debug<D>(file: WzFile, decryptor: D) -> Result<()>
where
    D: Decryptor,
{
    let map = file.map(decryptor)?;
    //println!("{:?}", map.debug_pretty_print());
    map.walk::<Error>(|cursor| {
        println!(
            "Path: {} -- Data: {:?}",
            cursor.pwd().join("/"),
            cursor.get()
        );
        Ok(())
    })
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // Assume encrypted
    let keystream = KeyStream::new(&TRIMMED_KEY, &GMS_IV);

    let action = &args.action;
    if action.create {
        if args.legacy {
            do_create(
                WzFile::create(args.file.as_str(), args.version.unwrap())?,
                &args.directory.unwrap(),
                keystream,
                args.verbose,
            )?
        } else {
            do_create(
                WzFile::create(args.file.as_str(), args.version.unwrap())?,
                &args.directory.unwrap(),
                DummyEncryptor,
                args.verbose,
            )?
        }
    } else if action.list {
        if args.legacy {
            do_list(WzFile::open(args.file.as_str())?, keystream)?
        } else {
            do_list(WzFile::open(args.file.as_str())?, DummyDecryptor)?
        }
    } else if action.extract {
        if args.legacy {
            do_extract(WzFile::open(args.file.as_str())?, keystream, args.verbose)?
        } else {
            do_extract(
                WzFile::open(args.file.as_str())?,
                DummyDecryptor,
                args.verbose,
            )?
        }
    } else if action.debug {
        if args.legacy {
            do_debug(WzFile::open(args.file.as_str())?, keystream)?
        } else {
            do_debug(WzFile::open(args.file.as_str())?, DummyDecryptor)?
        }
    }
    Ok(())
}
