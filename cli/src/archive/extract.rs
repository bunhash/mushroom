//! Parsing of WZ archives

use crate::{utils, Key};
use crypto::{KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use std::{fs, path::PathBuf};
use wz::{
    archive::{self, reader},
    error::{Error, Result},
    io::{DummyDecryptor, WzRead},
};

pub(crate) fn do_extract(
    path: &PathBuf,
    verbose: bool,
    key: Key,
    version: Option<u16>,
) -> Result<()> {
    let filename = utils::file_name(path)?;
    match key {
        Key::Gms => extract(
            filename,
            match version {
                Some(v) => archive::Reader::open_as_version(
                    path,
                    v,
                    KeyStream::new(&TRIMMED_KEY, &GMS_IV),
                )?,
                None => archive::Reader::open(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
            },
            verbose,
        ),
        Key::Kms => extract(
            filename,
            match version {
                Some(v) => archive::Reader::open_as_version(
                    path,
                    v,
                    KeyStream::new(&TRIMMED_KEY, &KMS_IV),
                )?,
                None => archive::Reader::open(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
            },
            verbose,
        ),
        Key::None => extract(
            filename,
            match version {
                Some(v) => archive::Reader::open_as_version(path, v, DummyDecryptor)?,
                None => archive::Reader::open(path, DummyDecryptor)?,
            },
            verbose,
        ),
    }
}

fn extract<R>(name: &str, mut archive: archive::Reader<R>, verbose: bool) -> Result<()>
where
    R: WzRead,
{
    let map = archive.map(&name.replace(".wz", ""))?;
    let mut reader = archive.into_inner();
    map.walk::<Error>(|cursor| {
        let path = cursor.pwd();
        match cursor.get() {
            reader::Node::Package => {
                utils::create_dir(&path)?;
            }
            reader::Node::Image { offset, size } => {
                utils::remove_file(&path)?;
                let mut output = fs::File::create(&path)?;
                reader.copy_to(&mut output, *offset, *size)?;
            }
        }
        utils::verbose!(verbose, "{}", path);
        Ok(())
    })
}
