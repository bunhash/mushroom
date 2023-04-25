//! WZ CLI Tool

use clap::{Args, Parser};
use crypto::{MushroomSystem, GMS_IV, TRIMMED_KEY};
use std::fs::File;
use wz::{
    error::Result, package::Content, Decode, Encode, EncryptedWzReader, Metadata, Reader, WzMap,
    WzReader,
};

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

fn do_list(reader: impl Reader) -> Result<()> {
    let mut reader = reader;
    reader.seek_to_start()?;
    let map = WzMap::decode(&mut reader)?;
    for (name, _) in map.traverse() {
        println!("{}", name)
    }
    Ok(())
}

fn do_extract(reader: impl Reader) -> Result<()> {
    let mut reader = reader;
    reader.seek_to_start()?;
    unimplemented!()
}

fn do_debug(reader: impl Reader) -> Result<()> {
    let mut reader = reader;
    reader.seek_to_start()?;
    let map = WzMap::decode(&mut reader)?;
    let mut sum = 0;
    for (uri, id) in map.traverse() {
        let content = map.get(id)?;
        match content {
            Content::Unknown(bytes) => panic!("Unknown bytes: {:?}", bytes),
            Content::Package(name, info) => {
                println!(
                    "type: {} name: {} size: {} checksum: {} offset: {}",
                    1,
                    name.encode_size(),
                    info.size.encode_size(),
                    info.checksum.encode_size(),
                    info.offset.encode_size()
                );
                let info_size = content.encode_size();
                let content_size = *info.size as u64;
                sum = sum + info_size + content_size;
                println!(
                    "{}: Info Size: {} Content Size: {} Offset: {}",
                    uri,
                    content.encode_size(),
                    *info.size as u64,
                    *info.offset as u64,
                )
            }
            Content::Image(name, info) => {
                println!(
                    "type: {} name: {} size: {} checksum: {} offset: {}",
                    1,
                    name.encode_size(),
                    info.size.encode_size(),
                    info.checksum.encode_size(),
                    info.offset.encode_size()
                );
                let info_size = content.encode_size();
                let content_size = *info.size as u64;
                sum = sum + info_size + content_size;
                println!(
                    "{}: Info Size: {} Content Size: {} Offset: {}",
                    uri,
                    content.encode_size(),
                    *info.size as u64,
                    *info.offset as u64,
                )
            }
        }
    }
    println!("Total: {} -- Expected: {}", sum, reader.metadata().size);
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // Assume encrypted
    let system = MushroomSystem::new(&TRIMMED_KEY, &GMS_IV);

    let action = &args.action;
    if action.create {
        unimplemented!();
    } else if action.list {
        let metadata = Metadata::from_file(&args.file)?;
        let file = File::open(&args.file)?;
        if args.legacy {
            do_list(EncryptedWzReader::new(file, metadata, &system))?;
        } else {
            do_list(WzReader::new(file, metadata))?;
        }
    } else if action.extract {
        let metadata = Metadata::from_file(&args.file)?;
        let file = File::open(&args.file)?;
        if args.legacy {
            do_extract(EncryptedWzReader::new(file, metadata, &system))?;
        } else {
            do_extract(WzReader::new(file, metadata))?;
        }
    } else if action.debug {
        let metadata = Metadata::from_file(&args.file)?;
        let file = File::open(&args.file)?;
        if args.legacy {
            do_debug(EncryptedWzReader::new(file, metadata, &system))?;
        } else {
            do_debug(WzReader::new(file, metadata))?;
        }
    }
    Ok(())
}
