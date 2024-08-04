#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

use clap::{Args, Parser, ValueEnum};
use std::path::PathBuf;
use wz::error::Result;

pub(crate) mod image;
pub(crate) mod utils;

#[derive(Parser)]
struct Cli {
    /// File for input/output
    #[arg(short, long, required = true)]
    file: PathBuf,

    /// XML file to build the WZ image from
    #[arg()]
    path: Option<String>,

    /// Command to do
    #[command(flatten)]
    action: Action,

    /// Verbose
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Expect encrypted strings
    #[arg(short, long, value_enum, default_value_t = Key::None)]
    key: Key,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Action {
    /// Create a new WZ image
    #[arg(short = 'c', requires = "path")]
    create: bool,

    /// List the WZ image contents
    #[arg(short = 't')]
    list: bool,

    /// Extract the WZ image
    #[arg(short = 'x')]
    extract: bool,

    /// Debug the WZ image
    #[arg(short = 'd')]
    debug: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Key {
    Gms,
    Kms,
    None,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let action = &args.action;
    if action.create {
        image::do_create(&args.file, &args.path.unwrap(), args.verbose, args.key)?;
    } else if action.list {
        image::do_list(&args.file, args.key)?;
    } else if action.extract {
        image::do_extract(&args.file, args.verbose, args.key)?;
    } else if action.debug {
        image::do_debug(&args.file, &args.path, args.verbose, args.key)?;
    }
    Ok(())
}
