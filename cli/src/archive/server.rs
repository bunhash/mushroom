//! Parsing of WZ archives

use crate::{utils, Key};
use crypto::{KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use std::{fs, path::PathBuf};
use wz::{
    archive::{self, reader},
    error::{Error, Result},
    image,
    io::{xml::writer::XmlWriter, DummyDecryptor, WzImageReader, WzRead},
};

pub(crate) fn do_server(
    path: &PathBuf,
    verbose: bool,
    key: Key,
    version: Option<u16>,
) -> Result<()> {
    let filename = utils::file_name(path)?;
    match key {
        Key::Gms => server(
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
        Key::Kms => server(
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
        Key::None => server(
            filename,
            match version {
                Some(v) => archive::Reader::open_as_version(path, v, DummyDecryptor)?,
                None => archive::Reader::open(path, DummyDecryptor)?,
            },
            verbose,
        ),
    }
}

fn server<R>(name: &str, mut archive: archive::Reader<R>, verbose: bool) -> Result<()>
where
    R: WzRead,
{
    let map = archive.map(name)?;
    let mut reader = archive.into_inner();
    map.walk::<Error>(|cursor| {
        match cursor.get() {
            reader::Node::Package => {
                let path = cursor.pwd();
                utils::create_dir(path)?;
            }
            reader::Node::Image { offset, .. } => {
                let path = format!("{}.xml", cursor.pwd());
                utils::remove_file(&path)?;
                let mut image_reader = WzImageReader::with_offset(&mut reader, *offset);
                image_reader.seek_to_start()?;
                let mut image = image::Reader::new(image_reader);
                let map = image.map(cursor.name())?;
                utils::verbose!(verbose, "{}", path);
                let mut writer = XmlWriter::new(fs::File::create(&path)?);
                writer.write(&mut map.cursor())?;
            }
        }
        Ok(())
    })
}
