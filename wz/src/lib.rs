//! Wz handling library

mod error;
mod file;
mod reader;

pub use error::{WzError, WzResult};
pub use file::WzFile;
pub use reader::WzReader;

#[cfg(test)]
mod tests {

    use crate::{WzFile, WzReader};

    #[test]
    fn open_wzfile_old() {
        let mut reader = WzReader::open("testdata/old-base.wz").unwrap();
        let wz = WzFile::load(&mut reader).unwrap();
        assert_eq!(wz.identifier(), "PKG1");
        assert_eq!(wz.size(), 6480);
        assert_eq!(wz.start(), 60);
        assert_eq!(
            wz.description(),
            "Package file v1.0 Copyright 2002 Wizet, ZMS"
        );
        assert_eq!(wz.version(), 83);
    }

    #[test]
    fn open_wzfile_new() {
        let mut reader = WzReader::open("testdata/new-base.wz").unwrap();
        let wz = WzFile::load(&mut reader).unwrap();
        assert_eq!(wz.identifier(), "PKG1");
        assert_eq!(wz.size(), 6705);
        assert_eq!(wz.start(), 60);
        assert_eq!(
            wz.description(),
            "Package file v1.0 Copyright 2002 Wizet, ZMS"
        );
        assert_eq!(wz.version(), 176);
    }
}
