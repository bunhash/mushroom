#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

use clap::{Args, Parser, ValueEnum};
use std::path::PathBuf;
use wz::decode::Error;

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

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    Ok(())
}
