//! Decoder Trait

use crate::{error::Result, io::WzReader};
use crypto::Decryptor;
use std::io::{Read, Seek};

/// Trait for decoding objects
pub trait Decode {
    /// Decodes objects
    fn decode<R, D>(reader: &mut WzReader<R, D>) -> Result<Self>
    where
        R: Read + Seek,
        D: Decryptor,
        Self: Sized;
}
