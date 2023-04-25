//! Errors

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::ErrorKind),
    Map(MapError),
    //    Tree(tree::error::Error),
    Utf8(std::string::FromUtf8Error),
    Unicode(std::string::FromUtf16Error),
    Package(PackageError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(kind) => write!(f, "IO: {}", kind),
            Error::Map(e) => write!(f, "Map: {}", e),
            //Error::Tree(e) => write!(f, "Tree: {}", e),
            Error::Utf8(e) => write!(f, "UTF-8: {}", e),
            Error::Unicode(e) => write!(f, "Unicode: {}", e),
            Error::Package(e) => write!(f, "WZ: {}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Error::Io(other.kind())
    }
}

impl From<MapError> for Error {
    fn from(other: MapError) -> Self {
        Error::Map(other)
    }
}

/*
impl From<tree::error::Error> for Error {
    fn from(other: tree::error::Error) -> Self {
        Error::Tree(other)
    }
}
*/

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

impl From<PackageError> for Error {
    fn from(other: PackageError) -> Self {
        Error::Package(other)
    }
}

#[derive(Debug)]
pub enum PackageError {
    BruteForceChecksum,
    CreateError,
    DirectoryName(String),
    InvalidPackage,
    InvalidContentType(u8),
    InvalidLength(i32),
}

impl std::fmt::Display for PackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageError::BruteForceChecksum => write!(f, "Brute force of the checksum failed"),
            PackageError::CreateError => write!(f, "Failed to create image or directory"),
            PackageError::DirectoryName(name) => {
                write!(f, "Invalid directory name: `{}`", name)
            }
            PackageError::InvalidPackage => write!(f, "Invalid Package"),
            PackageError::InvalidContentType(t) => {
                write!(f, "Package ContentType is invalid `{}`", t)
            }
            PackageError::InvalidLength(l) => write!(f, "Invalid length: `{}`", l),
        }
    }
}

#[derive(Debug)]
pub enum MapError {
    InvalidNodeId,
    InvalidPath,
}

impl std::fmt::Display for MapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapError::InvalidNodeId => write!(f, "Invalid Node ID"),
            MapError::InvalidPath => write!(f, "Invalid Path"),
        }
    }
}
