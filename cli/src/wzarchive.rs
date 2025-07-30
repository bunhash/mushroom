#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

use clap::{Args, Parser, ValueEnum};
use std::path::PathBuf;
use wz::archive::Error;

mod archive;
mod utils;

#[derive(Parser)]
struct Cli {
    /// File for input/output
    #[arg(short, long, required = true)]
    file: PathBuf,

    /// Directory to create the WZ archive from
    #[arg(value_name = "DIR")]
    directory: Option<String>,

    /// Command to do
    #[command(flatten)]
    action: Action,

    /// Verbose
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Expect encrypted strings
    #[arg(short, long, value_enum, default_value_t = Key::None)]
    key: Key,

    /// The version of WZ archive. Required if create. Overrides the WZ version otherwise.
    #[arg(short = 'm', long)]
    version: Option<u16>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Action {
    /// Create a new WZ archive
    #[arg(short = 'c', requires = "version", requires = "directory")]
    create: bool,

    /// List the WZ archive contents
    #[arg(short = 't')]
    list: bool,

    /// Extract the WZ archive
    #[arg(short = 'x')]
    extract: bool,

    /// Debug the WZ archive
    #[arg(short = 'd')]
    debug: bool,

    /// Decode List.wz file
    #[arg(short = 'L')]
    list_file: bool,

    /// Generate server XML files based on the wz archive
    #[arg(short = 'S')]
    server: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Key {
    Gms,
    Kms,
    None,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    archive::do_debug(&args.file, &args.directory, args.key, args.version)?;
    Ok(())
}
