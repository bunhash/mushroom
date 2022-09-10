//! Wz handling library

mod directory;
mod error;
mod file;
mod header;
mod node;
mod object;
mod reader;

pub use directory::WzDirectory;
pub use error::{WzError, WzErrorType, WzResult};
pub use file::WzFile;
pub use header::WzHeader;
pub use node::WzNode;
pub use object::{WzObject, WzObjectType};
pub use reader::WzReader;

#[cfg(test)]
mod tests {

    use crate::{WzFile, WzNode, WzReader};

    #[test]
    fn open_wzfile_old() {
        let mut reader = WzReader::open("testdata/old-base.wz").unwrap();
        let wz = WzFile::from_reader("base.wz", &mut reader).unwrap();
        let header = wz.header();
        assert_eq!(header.identifier(), "PKG1");
        assert_eq!(header.size(), 6480);
        assert_eq!(header.content_start(), 60);
        assert_eq!(
            header.description(),
            "Package file v1.0 Copyright 2002 Wizet, ZMS"
        );
        assert_eq!(wz.version(), 83);

        // Check contents
        let root = wz.root();
        assert_eq!(root.name(), "base.wz");
        assert_eq!(root.len(), 3);
    }

    #[test]
    fn open_wzfile_new() {
        let mut reader = WzReader::open("testdata/new-base.wz").unwrap();
        let wz = WzFile::from_reader("base.wz", &mut reader).unwrap();
        let header = wz.header();
        assert_eq!(header.identifier(), "PKG1");
        assert_eq!(header.size(), 6705);
        assert_eq!(header.content_start(), 60);
        assert_eq!(
            header.description(),
            "Package file v1.0 Copyright 2002 Wizet, ZMS"
        );
        assert_eq!(wz.version(), 176);

        // Check contents
        let root = wz.root();
        assert_eq!(root.name(), "base.wz");
        assert_eq!(root.len(), 3);
    }
}
