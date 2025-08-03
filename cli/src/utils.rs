//! Random utilities I got tired of rewriting

use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

macro_rules! verbose {
    ($verbose:expr, $($args:tt)*) => {
        if $verbose {
            println!($($args)*)
        }
    }
}
pub(crate) use verbose;

//pub fn parent<S>(path: &S) -> Result<&Path, Error>
//where
//    S: AsRef<Path>,
//{
//    Ok(path.as_ref().parent().ok_or(ErrorKind::NotFound)?)
//}

pub fn create_dir<S>(path: S) -> Result<(), Error>
where
    S: AsRef<Path>,
{
    if !path.as_ref().is_dir() {
        fs::create_dir(path)?;
    }
    Ok(())
}

pub fn remove_file<S>(path: S) -> Result<(), Error>
where
    S: AsRef<Path>,
{
    if path.as_ref().is_file() {
        fs::remove_file(path)?;
    }
    Ok(())
}
