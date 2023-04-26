//! WZ CLI Tool

use clap::{Args, Parser};
use crypto::{MushroomSystem, GMS_IV, TRIMMED_KEY};
use std::{fs::File, io::ErrorKind, path::PathBuf};
use wz::{error::Result, package::Package, EncryptedReader, Reader, UnencryptedReader};

#[derive(Parser)]
struct Cli {
    /// File for input/output
    #[arg(short, long, required = true)]
    file: PathBuf,

    /// Action
    #[command(flatten)]
    action: Action,

    /// Verbose
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// When legacy == true, use encrypted GMS
    #[arg(long, default_value_t = false)]
    legacy: bool,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Action {
    /// Create package
    #[arg(short = 'c')]
    create: bool,

    /// List package contents
    #[arg(short = 't')]
    list: bool,

    /// Extract package
    #[arg(short = 'x')]
    extract: bool,

    /// Debug package
    #[arg(short = 'd')]
    debug: bool,
}

fn do_list(_reader: impl Reader) -> Result<()> {
    unimplemented!()
}

fn do_extract(_reader: impl Reader) -> Result<()> {
    unimplemented!()
}

fn do_debug(name: &str, reader: impl Reader) -> Result<()> {
    let mut reader = reader;
    let map = Package::map(name, &mut reader)?;
    println!("{:?}", map.debug_pretty_print());
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // Assume encrypted
    let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);

    // Get filename
    let name = match args.file.file_name() {
        Some(name) => match name.to_str() {
            Some(name) => name,
            None => return Err(ErrorKind::NotFound.into()),
        },
        None => return Err(ErrorKind::NotFound.into()),
    };

    let action = &args.action;
    if action.create {
        unimplemented!();
    } else if action.list {
        let file = File::open(&args.file)?;
        if args.legacy {
            do_list(EncryptedReader::from_reader(file, &system)?)?;
        } else {
            do_list(UnencryptedReader::from_reader(file)?)?;
        }
    } else if action.extract {
        let file = File::open(&args.file)?;
        if args.legacy {
            do_extract(EncryptedReader::from_reader(file, &system)?)?;
        } else {
            do_extract(UnencryptedReader::from_reader(file)?)?;
        }
    } else if action.debug {
        let file = File::open(&args.file)?;
        if args.legacy {
            do_debug(name, EncryptedReader::from_reader(file, &system)?)?;
        } else {
            do_debug(name, UnencryptedReader::from_reader(file)?)?;
        }
    }
    Ok(())
}
