//! WZ CLI Tool

use clap::{Args, Parser, ValueEnum};
use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};
use wz::error::Result;

mod wzfile;
mod wzimage;

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

    /// The target file is an image
    #[arg(short, long, default_value_t = false)]
    image: bool,

    /// Expect encrypted GMS strings
    #[arg(short, long, value_enum, default_value_t = Key::None)]
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

fn file_name(path: &Path) -> Result<&str> {
    match path.file_name() {
        Some(name) => match name.to_str() {
            Some(name) => Ok(name),
            None => Err(ErrorKind::NotFound.into()),
        },
        None => Err(ErrorKind::NotFound.into()),
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let action = &args.action;
    if args.image {
        if action.create {
            unimplemented!()
        } else if action.list {
            unimplemented!()
        } else if action.extract {
            wzimage::do_extract(&args.file, args.verbose, args.key)?;
        } else if action.debug {
            wzimage::do_debug(&args.file, &args.directory, args.key)?;
        } else if action.list_file {
            unimplemented!()
        }
    } else if action.create {
        wzfile::do_create(
            &args.file,
            &args.directory.unwrap(),
            args.verbose,
            args.key,
            args.version.unwrap(),
        )?;
    } else if action.list {
        wzfile::do_list(&args.file, args.key, args.version)?;
    } else if action.extract {
        wzfile::do_extract(&args.file, args.verbose, args.key, args.version)?;
    } else if action.debug {
        wzfile::do_debug(&args.file, &args.directory, args.key, args.version)?;
    } else if action.list_file {
        wzfile::do_list_file(&args.file, args.key)?;
    }
    Ok(())
}
