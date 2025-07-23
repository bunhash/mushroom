//! Package Error Types

use std::fmt;

/// Possible package errors
#[derive(Debug)]
pub enum PackageError {
    /// Brute forcing the checksum failed
    BruteForceChecksum,

    ///  checksum
    Checksum,

    /// Content Type is unknown
    ContentType(u8),

    ///  header
    Header,

    ///  Path
    Path(String),

    /// Multiple Roots
    MultipleRoots,
}

impl fmt::Display for PackageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BruteForceChecksum => write!(f, "Brute force of the checksum failed"),
            Self::ContentType(t) => write!(f, "Unknown content type: `{}`", t),
            Self::Checksum => write!(f, "Invalid version checksum"),
            Self::Header => write!(f, "Invalid WZ archive header"),
            Self::Path(p) => write!(f, "Invalid path name: `{}`", p),
            Self::MultipleRoots => write!(f, "A WZ archive can only have 1 root"),
        }
    }
}
