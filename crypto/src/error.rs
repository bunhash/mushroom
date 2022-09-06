//! Mushroom cipher error definitions

use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CipherError;

impl fmt::Display for CipherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Invalid cipher stream")
    }
}
