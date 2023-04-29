//! WZ CLI Tool

use clap::{Args, Parser};
use crypto::{Decryptor, KeyStream, GMS_IV, TRIMMED_KEY};
use wz::{error::Result, reader::DummyDecryptor, WzFile};

#[derive(Parser)]
struct Cli {
    /// File for input/output
    #[arg(short, long, required = true)]
    file: String,

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

fn do_list<D>(_file: WzFile, _decryptor: D) -> Result<()>
where
    D: Decryptor,
{
    unimplemented!()
}

fn do_extract<D>(_file: WzFile, _decryptor: D) -> Result<()>
where
    D: Decryptor,
{
    unimplemented!()
}

fn do_debug<D>(file: WzFile, decryptor: D) -> Result<()>
where
    D: Decryptor,
{
    let map = file.map(decryptor)?;
    println!("{:?}", map.debug_pretty_print());
    /*
    let size: WzInt = match map.cursor().get() {
        Content::Package(_, params, num_content) => 2 + num_content.size_hint() + params.size(),
        _ => panic!("something went wrong"),
    };
    println!("Total Size: {:?}", size);
    */
    println!("Metadata Size: {:?}", file.metadata().size);
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // Assume encrypted
    let keystream = KeyStream::new(&TRIMMED_KEY, &GMS_IV);

    let action = &args.action;
    if action.create {
        unimplemented!();
    } else if action.list {
        if args.legacy {
            do_list(WzFile::open(args.file.as_str())?, keystream)?
        } else {
            do_list(WzFile::open(args.file.as_str())?, DummyDecryptor)?
        }
    } else if action.extract {
        if args.legacy {
            do_extract(WzFile::open(args.file.as_str())?, keystream)?
        } else {
            do_extract(WzFile::open(args.file.as_str())?, DummyDecryptor)?
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
