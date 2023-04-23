//! Errors

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BruteForceChecksum,
    InvalidPackage,
    OffsetMissingParameters,
    InvalidContentType,
    InvalidLength(i32),
    Io(std::io::ErrorKind),
    Tree(tree::error::Error),
    Utf8(std::string::FromUtf8Error),
    Unicode(std::string::FromUtf16Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BruteForceChecksum => write!(f, "Brute force of the checksum failed"),
            Error::InvalidPackage => write!(f, "Invalid Package"),
            Error::OffsetMissingParameters => write!(
                f,
                "WZ-OFFSET needs to know the absolute_position and version_checksum"
            ),
            Error::InvalidContentType => write!(f, "Package ContentType is invalid"),
            Error::InvalidLength(l) => write!(f, "Invalid length: `{}`", l),
            Error::Io(kind) => write!(f, "IO: {}", kind),
            Error::Tree(e) => write!(f, "Tree: {}", e),
            Error::Utf8(e) => write!(f, "UTF-8: {}", e),
            Error::Unicode(e) => write!(f, "Unicode: {}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Error::Io(other.kind())
    }
}

impl From<tree::error::Error> for Error {
    fn from(other: tree::error::Error) -> Self {
        Error::Tree(other)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(other: std::string::FromUtf8Error) -> Self {
        Error::Utf8(other)
    }
}

impl From<std::string::FromUtf16Error> for Error {
    fn from(other: std::string::FromUtf16Error) -> Self {
        Error::Unicode(other)
    }
}
