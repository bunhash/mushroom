//! WZ File

use crate::{error::Result, reader::DummyDecryptor, Metadata, WzReader};
use crypto::{Decryptor, KeyStream};
use std::{fs::File, io::BufReader, path::PathBuf};

pub struct WzFile {
    path: PathBuf,
    metadata: Metadata,
}

impl WzFile {
    /// Creates a `WzFile` object by reading the file at `path`
    pub fn open(path: &str) -> Result<Self> {
        let path = PathBuf::from(path);
        let metadata = Metadata::from_reader(File::open(&path)?)?;
        Ok(Self { path, metadata })
    }

    /// Creates a new [`WzReader`]. This creates a new file descriptor as well. This method can be
    /// called multiple times. However, it is important to ensure the file is read-only while these
    /// reader(s) exist.
    pub fn to_reader<D>(&self, decryptor: D) -> Result<WzReader<BufReader<File>, D>>
    where
        D: Decryptor,
    {
        Ok(WzReader::new(
            &self.metadata,
            BufReader::new(File::open(&self.path)?),
            decryptor,
        ))
    }

    /// Creates a new unencrypted [`WzReader`]. See [`WzFile::to_reader`].
    pub fn unencrypted_reader(&self) -> Result<WzReader<BufReader<File>, DummyDecryptor>> {
        self.to_reader(DummyDecryptor)
    }

    /// Creates a new encrypted [`WzReader`]. See [`WzFile::to_reader`].
    pub fn encrypted_reader(
        &self,
        keystream: KeyStream,
    ) -> Result<WzReader<BufReader<File>, KeyStream>> {
        self.to_reader(keystream)
    }
}
