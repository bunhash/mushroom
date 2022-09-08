use std::{io, string::FromUtf8Error};
use thiserror::Error;

pub type WzResult<T> = Result<T, WzError>;

#[derive(Debug, Error)]
pub enum WzError {
    #[error("Invalid WZ file")]
    InvalidWz,

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] FromUtf8Error),
}
