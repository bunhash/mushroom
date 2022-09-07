//! Wz handling library

mod reader;

pub use reader::WzReader;

#[cfg(test)]
mod tests {

    use crate::WzReader;

    #[test]
    fn open_file() {
        WzReader::new("testdata/old-base.wz").unwrap();
    }

    #[test]
    fn read_header() {
        let mut reader = WzReader::new("testdata/old-base.wz").unwrap();
        let (identifier, filesize, position, description) = reader.read_header().unwrap();
        assert_eq!(identifier, "PKG1");
        assert_eq!(filesize, 6480);
        assert_eq!(position, 60);
        assert_eq!(description, "testing");
    }
}
