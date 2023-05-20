//! Random utilities I got tired of rewriting

use std::{fs, io::ErrorKind, path::Path};
use wz::error::Result;

macro_rules! verbose {
    ($verbose:expr, $($args:tt)*) => {
        if $verbose {
            println!($($args)*)
        }
    }
}
pub(crate) use verbose;

pub(crate) fn file_name<S>(path: &S) -> Result<&str>
where
    S: AsRef<Path>,
{
    Ok(path
        .as_ref()
        .file_name()
        .ok_or(ErrorKind::NotFound)?
        .to_str()
        .ok_or(ErrorKind::NotFound)?)
}

pub(crate) fn parent<S>(path: &S) -> Result<&Path>
where
    S: AsRef<Path>,
{
    Ok(path.as_ref().parent().ok_or(ErrorKind::NotFound)?)
}

pub(crate) fn create_dir<S>(path: S) -> Result<()>
where
    S: AsRef<Path>,
{
    if !path.as_ref().is_dir() {
        fs::create_dir(path)?;
    }
    Ok(())
}

pub(crate) fn remove_file<S>(path: S) -> Result<()>
where
    S: AsRef<Path>,
{
    if path.as_ref().is_file() {
        fs::remove_file(path)?;
    }
    Ok(())
}
