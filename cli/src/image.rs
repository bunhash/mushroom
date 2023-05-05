//! ImageRef for WZ archiver

use crypto::Encryptor;
use std::{
    convert::AsRef,
    ffi::OsStr,
    fs::File,
    io::{BufReader, Read, Seek, Write},
    num::Wrapping,
    path::PathBuf,
};
use wz::{builder::ImageRef, error::Result, types::WzInt, WzWriter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImagePath {
    path: PathBuf,
    size: WzInt,
    checksum: WzInt,
}

impl ImagePath {
    pub fn new<S>(path: &S) -> Result<Self>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        let path = PathBuf::from(path);
        let size = WzInt::from(path.metadata()?.len());
        let reader = BufReader::new(File::open(&path)?);
        let checksum = WzInt::from(
            reader
                .bytes()
                .flatten()
                .map(|b| Wrapping(b as i32))
                .sum::<Wrapping<i32>>()
                .0,
        );
        Ok(Self {
            path,
            size,
            checksum,
        })
    }
}

impl ImageRef for ImagePath {
    fn size(&self) -> Result<WzInt> {
        Ok(self.size)
    }

    fn checksum(&self) -> Result<WzInt> {
        Ok(self.checksum)
    }

    fn write<W, E>(&self, writer: &mut WzWriter<W, E>) -> Result<()>
    where
        W: Write + Seek,
        E: Encryptor,
    {
        Ok(())
    }
}
