//! Wz handling library

mod error;
mod file;
mod reader;

pub use error::{WzError, WzResult};
pub use file::WzFile;
pub use reader::WzReader;

#[cfg(test)]
mod tests {

    use crate::WzFile;

    #[test]
    fn open_wzfile_old() {
        let wz = WzFile::open("testdata/old-base.wz").unwrap();
        assert_eq!(wz.identifier, "PKG1");
        assert_eq!(wz.size, 6480);
        assert_eq!(wz.start, 60);
        assert_eq!(wz.description, "Package file v1.0 Copyright 2002 Wizet, ZMS");
    }

    #[test]
    fn open_wzfile_new() {
        let wz = WzFile::open("testdata/new-base.wz").unwrap();
        assert_eq!(wz.identifier, "PKG1");
        assert_eq!(wz.size, 6705);
        assert_eq!(wz.start, 60);
        assert_eq!(wz.description, "Package file v1.0 Copyright 2002 Wizet, ZMS");
    }
}
