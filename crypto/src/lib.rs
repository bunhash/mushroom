#![warn(missing_docs)]
//! Mushroom crypto implementation
//!
//! Handles the AES-OFB encryption used to encrypt/decrypt strings in WZ packages.
//!
//! # Example
//!
//! ```
//! use crypto::{Decryptor, Encryptor, KeyStream, GMS_IV, TRIMMED_KEY};
//!
//! let mut stream = KeyStream::new(&TRIMMED_KEY, &GMS_IV);
//!
//! let mut input: Vec<u8> = Vec::from("smap.img".as_bytes());
//! stream.encrypt(&mut input);
//! assert_eq!(input.as_slice(), &[229, 195, 94, 212, 102, 147, 176, 247]);
//!
//! stream.decrypt(&mut input);
//! assert_eq!(
//!     String::from_utf8(input).unwrap(),
//!     "smap.img"
//! );
//! ```

mod keystream;
mod utils;

pub use keystream::KeyStream;
pub use utils::checksum;

/// Default key used in Mushroom
pub const USER_KEY: [u8; 128] = [
    0x13, 0x00, 0x00, 0x00, 0x52, 0x00, 0x00, 0x00, 0x2a, 0x00, 0x00, 0x00, 0x5b, 0x00, 0x00, 0x00,
    0x08, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00,
    0x06, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x43, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00,
    0xb4, 0x00, 0x00, 0x00, 0x4b, 0x00, 0x00, 0x00, 0x35, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00,
    0x1b, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x5f, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00,
    0x0f, 0x00, 0x00, 0x00, 0x50, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x1b, 0x00, 0x00, 0x00,
    0x33, 0x00, 0x00, 0x00, 0x55, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00,
    0x52, 0x00, 0x00, 0x00, 0xde, 0x00, 0x00, 0x00, 0xc7, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00,
];

/// Trimmed key
pub const TRIMMED_KEY: [u8; 32] = [
    0x13, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0xb4, 0x00, 0x00, 0x00,
    0x1b, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x33, 0x00, 0x00, 0x00, 0x52, 0x00, 0x00, 0x00,
];

/// The AES-256 IV used in GMS
pub const GMS_IV: [u8; 4] = [0x4d, 0x23, 0xc7, 0x2b];

/// The AES-256 IV used in KMS
pub const KMS_IV: [u8; 4] = [0xb9, 0x7d, 0x63, 0xe9];

/// Trait representing Encryptors
pub trait Encryptor {
    /// Encrypts an array of bytes
    fn encrypt(&mut self, bytes: &mut Vec<u8>);
}

/// Trait representing Decryptors
pub trait Decryptor {
    /// Decrypts an array of bytes
    fn decrypt(&mut self, bytes: &mut Vec<u8>);
}
