//! Parsing of WZ archives

use crypto::KeyStream;
use std::path::PathBuf;
use wz::archive::{ContentType, Error, Reader};

pub fn do_list(path: &PathBuf, key: Option<KeyStream>, version: Option<u16>) -> Result<(), Error> {
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
    for (path, content) in archive.iter() {
        match &content.content_type {
            ContentType::Image(name) => println!("{}/{}", path, name),
            _ => {}
        }
    }
    Ok(())
}

/*
pub fn do_list_file(path: &PathBuf, key: Key) -> Result<()> {
    let reader = match key {
        Key::Gms => list::Reader::open(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
        Key::Kms => list::Reader::open(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
        Key::None => list::Reader::open(path, DummyDecryptor)?,
    };
    for string in reader.strings() {
        println!("{}", string);
    }
    Ok(())
}
*/
