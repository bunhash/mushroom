//! Image representation

use std::io::{Error, Read};

/// Representation of a WZ Image
pub trait Image {
    /// Return the name of the image
    fn name(&self) -> &str;

    /// Return the size of the image
    fn size(&self) -> u64;

    /// Return the checksum of the image
    fn checksum(&self) -> i32;

    /// Transforms the image into a Read
    fn to_reader(&self) -> Result<Box<dyn Read>, Error>;
}
