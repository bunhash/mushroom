/// Package Errors

#[derive(Debug)]
pub enum Error {
    InvalidPackage,
    InvalidContentType(u8),
    DirectoryNameError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidPackage => write!(f, "Invalid Package"),
            Error::InvalidContentType(t) => write!(f, "Package ContentType is invalid `{}`", t),
            Error::DirectoryNameError(name) => write!(f, "Invalid directory name: `{}`", name),
        }
    }
}
