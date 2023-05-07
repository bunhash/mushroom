//! WZ File Archive

use crate::{
    error::{Result, WzError},
    file::{package::ContentRef, Header, Package},
    io::{Decode, WzReader},
    map::{CursorMut, Map},
    types::{WzInt, WzOffset, WzString},
};
use crypto::{checksum, Decryptor};
use std::{fs::File, io::BufReader};

/// Map node pointing to WZ archive contents
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Node {
    Package,
    Image { offset: WzOffset, size: WzInt },
}

/// Represents a WZ archive. Basically used as an intermediary to map the contents and make the
/// reader.
///
/// Example:
///
/// ```no_run
/// use crypto::{KeyStream, GMS_IV, TRIMMED_KEY};
/// use std::fs::File;
/// use wz::Archive;
///
/// let file = File::open("Character.wz").unwrap();
/// let mut archive = Archive::open(file, KeyStream::new(&TRIMMED_KEY, &GMS_IV)).unwrap();
/// let map = archive.map("Character.wz").unwrap();
/// let reader = archive.into_inner();
/// println!("{:?}", map.debug_pretty_print());
/// // Do stuf with reader
/// ```
#[derive(Debug)]
pub struct Archive<D>
where
    D: Decryptor,
{
    header: Header,
    inner: WzReader<BufReader<File>, D>,
}

impl<D> Archive<D>
where
    D: Decryptor,
{
    /// Opens a WZ archive and reads the header data. Attemps to brute force the version
    pub fn open(file: File, decryptor: D) -> Result<Self> {
        let mut buf = BufReader::new(file);
        let header = Header::from_reader(&mut buf)?;
        let inner = Archive::bruteforce_version(&header, buf, decryptor)?;
        Ok(Self { header, inner })
    }

    /// Opens a WZ archive and reads the header data.
    pub fn open_as_version(file: File, version: u16, decryptor: D) -> Result<Self> {
        let mut buf = BufReader::new(file);
        let header = Header::from_reader(&mut buf)?;
        let absolute_position = header.absolute_position;
        let (version_hash, version_checksum) = checksum(&version.to_string());
        if version_hash != header.version_hash {
            Err(WzError::InvalidChecksum.into())
        } else {
            Ok(Self {
                header,
                inner: WzReader::new(absolute_position, version_checksum, buf, decryptor),
            })
        }
    }

    /// Returns a reference to the header
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Maps the archive contents. The root will be named `name`
    pub fn map(&mut self, name: &str) -> Result<Map<Node>> {
        let name = WzString::from(name);
        let mut map = Map::new(name, Node::Package);
        self.inner.seek_to_start()?;
        Archive::map_package_to(&mut self.inner, &mut map.cursor_mut())?;
        Ok(map)
    }

    /// Consumes the archive and returns the inner reader
    pub fn into_inner(self) -> WzReader<BufReader<File>, D> {
        self.inner
    }

    // *** PRIVATES *** //

    fn bruteforce_version(
        header: &Header,
        buf: BufReader<File>,
        decryptor: D,
    ) -> Result<WzReader<BufReader<File>, D>> {
        let lower_bound = WzOffset::from(header.absolute_position as u32);
        let upper_bound = WzOffset::from(header.absolute_position as u32 + header.size as u32);
        let mut inner = WzReader::new(header.absolute_position, 0u32, buf, decryptor);
        for (_, version_checksum) in Header::possible_versions(header.version_hash) {
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
        Err(WzError::BruteForceChecksum.into())
    }

    fn map_package_to(
        reader: &mut WzReader<BufReader<File>, D>,
        cursor: &mut CursorMut<Node>,
    ) -> Result<()> {
        let package = Package::decode(reader)?;
        for content in package.contents() {
            match &content {
                ContentRef::Package(ref data) => {
                    cursor.create(WzString::from(data.name()), Node::Package)?;
                    cursor.move_to(data.name.as_ref())?;
                    reader.seek(data.offset())?;
                    Archive::map_package_to(reader, cursor)?;
                    cursor.parent()?;
                }
                ContentRef::Image(ref data) => {
                    cursor.create(
                        WzString::from(data.name()),
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
}
