//! Mushroom cipher error definitions

use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct KeyStreamError;

impl fmt::Display for KeyStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Invalid key stream")
    }
}
