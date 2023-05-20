//! WZ Archive Reader

use crate::{
    error::{PackageError, Result},
    io::{Decode, DummyDecryptor, WzRead, WzReader},
    map::{CursorMut, Map},
    types::{
        raw::{package::ContentRef, Package},
        WzHeader, WzInt, WzOffset,
    },
};
use crypto::{checksum, Decryptor};
use std::{fs::File, io::BufReader, path::Path};

/// Map node pointing to WZ archive contents
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Node {
    Package,
    Image { offset: WzOffset, size: WzInt },
}

/// Reads a WZ archive
///
/// Example:
///
/// ```no_run
/// use crypto::{KeyStream, GMS_IV, TRIMMED_KEY};
/// use std::fs::File;
/// use wz::archive::Reader;
///
/// let mut archive = Reader::open("Character.wz", KeyStream::new(&TRIMMED_KEY, &GMS_IV)).unwrap();
/// let map = archive.map("Character.wz").unwrap();
/// let reader = archive.into_inner();
/// println!("{:?}", map.debug_pretty_print());
/// // Do stuf with reader
/// ```
#[derive(Debug)]
pub struct Reader<R>
where
    R: WzRead,
{
    header: WzHeader,
    inner: R,
}

impl Reader<WzReader<BufReader<File>, DummyDecryptor>> {
    pub fn unencrypted<S>(path: S) -> Result<Self>
    where
        S: AsRef<Path>,
    {
        Reader::open(path, DummyDecryptor)
    }
}

impl<D> Reader<WzReader<BufReader<File>, D>>
where
    D: Decryptor,
{
    /// Opens a WZ archive and reads the header data. Attemps to brute force the version
    pub fn open<S>(path: S, decryptor: D) -> Result<Reader<WzReader<BufReader<File>, D>>>
    where
        S: AsRef<Path>,
    {
        let mut buf = BufReader::new(File::open(path)?);
        let header = WzHeader::from_reader(&mut buf)?;
        let inner = bruteforce_version(&header, buf, decryptor)?;
        Ok(Reader::new(header, inner))
    }

    /// Opens a WZ archive and reads the header data.
    pub fn open_as_version<S>(
        path: S,
        version: u16,
        decryptor: D,
    ) -> Result<Reader<WzReader<BufReader<File>, D>>>
    where
        S: AsRef<Path>,
    {
        let mut buf = BufReader::new(File::open(path)?);
        let header = WzHeader::from_reader(&mut buf)?;
        let absolute_position = header.absolute_position;
        let (version_hash, version_checksum) = checksum(&version.to_string());
        if version_hash != header.version_hash {
            Err(PackageError::Checksum.into())
        } else {
            Ok(Reader::new(
                header,
                WzReader::new(absolute_position, version_checksum, buf, decryptor),
            ))
        }
    }
}

impl<R> Reader<R>
where
    R: WzRead,
{
    /// Creates a new archive reader from a WzRead
    pub fn new(header: WzHeader, inner: R) -> Self {
        Self { header, inner }
    }

    /// Returns a reference to the header
    pub fn header(&self) -> &WzHeader {
        &self.header
    }

    /// Maps the archive contents. The root will be named `name`
    pub fn map(&mut self, name: &str) -> Result<Map<Node>> {
        let name = String::from(name);
        let mut map = Map::new(name, Node::Package);
        self.inner.seek_to_start()?;
        map_package_to(&mut self.inner, &mut map.cursor_mut())?;
        Ok(map)
    }

    /// Consumes the archive and returns the inner reader
    pub fn into_inner(self) -> R {
        self.inner
    }
}

fn bruteforce_version<D>(
    header: &WzHeader,
    buf: BufReader<File>,
    decryptor: D,
) -> Result<WzReader<BufReader<File>, D>>
where
    D: Decryptor,
{
    let lower_bound = WzOffset::from(header.absolute_position as u32);
    let upper_bound = WzOffset::from(header.absolute_position as u32 + header.size as u32);
    let mut inner = WzReader::new(header.absolute_position, 0u32, buf, decryptor);
    for (_, version_checksum) in WzHeader::possible_versions(header.version_hash) {
        inner.set_version_checksum(version_checksum);
        inner.seek_to_start()?;

        // Decodes the top-level directory contents. If all contents lie within the lower and
        // upper bounds, we can assume the version checksum is good.
        let package = Package::decode(&mut inner)?;
        let filtered_len = package
            .contents()
            .map(|content| content.offset())
            .filter(|off| *off >= lower_bound && *off < upper_bound)
            .count();
        if package.num_contents() == filtered_len {
            return Ok(inner);
        }
    }
    Err(PackageError::BruteForceChecksum.into())
}

fn map_package_to<R>(reader: &mut R, cursor: &mut CursorMut<Node>) -> Result<()>
where
    R: WzRead,
{
    let package = Package::decode(reader)?;
    for content in package.contents() {
        match &content {
            ContentRef::Package(ref data) => {
                cursor.create(String::from(data.name()), Node::Package)?;
                cursor.move_to(data.name.as_ref())?;
                reader.seek(data.offset())?;
                map_package_to(reader, cursor)?;
                cursor.parent()?;
            }
            ContentRef::Image(ref data) => {
                cursor.create(
                    String::from(data.name()),
                    Node::Image {
                        offset: data.offset(),
                        size: data.size(),
                    },
                )?;
            }
        }
    }
    Ok(())
}
