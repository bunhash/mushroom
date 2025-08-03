//! Parsing of WZ archives

use crypto::KeyStream;
use std::path::PathBuf;
use wz::archive::{Error, Reader};

pub fn do_debug(
    path: &PathBuf,
    key: Option<KeyStream>,
    version: Option<u16>,
    directory: &Option<String>,
) -> Result<(), Error> {
    let archive = match key {
        Some(k) => match version {
            Some(v) => Reader::as_version(path, v, k)?.parse()?,
            None => Reader::new(path, k)?.parse()?,
        },
        None => match version {
            Some(v) => Reader::unencrypted_as_version(path, v)?.parse()?,
            None => Reader::unencrypted(path)?.parse()?,
        },
    };
    match directory {
        Some(path) => match archive.get_subtree(path) {
            Some(tree) => println!("{}", tree),
            None => println!("{} not found", path),
        },
        None => println!("{}", archive),
    }
    Ok(())
}
