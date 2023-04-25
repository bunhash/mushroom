/// Package Errors

#[derive(Debug)]
pub enum PackageError {
    BruteForceChecksum,
    InvalidPackage,
    InvalidContentType(u8),
    DirectoryNameError(String),
}

impl std::fmt::Display for PackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageError::BruteForceChecksum => write!(f, "Brute force of the checksum failed"),
            PackageError::InvalidPackage => write!(f, "Invalid Package"),
            PackageError::InvalidContentType(t) => {
                write!(f, "Package ContentType is invalid `{}`", t)
            }
            PackageError::DirectoryNameError(name) => {
                write!(f, "Invalid directory name: `{}`", name)
            }
        }
    }
}
