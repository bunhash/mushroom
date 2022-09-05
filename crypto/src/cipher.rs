//! Mushroom Cipher
//!
//! Defines the input parameters of the AES-OFB encryption used throughout Mushroom. The key stays
//! constant while the IV may change often.
//!
//! This structure uses a lazy allocation method of only growing the cipherstream to match the
//!

use std::fmt;

use aes::cipher::{
    generic_array::{typenum::U32, GenericArray},
    BlockCipher, BlockEncrypt, KeyInit,
};
use aes::Aes256;

#[derive(Copy, Clone, Debug)]
pub struct CipherError;

impl fmt::Display for CipherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Invalid cipher stream")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cipher {
    key: GenericArray<u8, U32>,
    iv: [u8; 4],
}

impl Cipher {
    pub fn new(key: [u8; 32], iv: [u8; 4]) -> Self {
        Cipher {
            key: GenericArray::from(key),
            iv: iv.clone(),
        }
    }

    pub fn to_stream(&self, min_len: usize) -> Result<Vec<u8>, CipherError> {
        let mut ret = Vec::new();
        self.grow(&mut ret, min_len)?;
        Ok(ret)
    }

    pub fn grow(&self, stream: &mut Vec<u8>, new_len: usize) -> Result<(), CipherError> {
        let current_len = stream.len();
        if new_len < current_len {
            return Ok(());
        }
        if current_len == 0 {}
        while current_len < new_len {}
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::cipher::Cipher;

    #[test]
    fn creation() {
        let cipher = Cipher::new([0x00; 32], [0x00; 4]);
    }

    #[test]
    fn copy() {
        let cipher1 = Cipher::new([0x00; 32], [0x00; 4]);
        let cipher2 = cipher1.clone();
        assert_eq!(cipher1, cipher2);
    }
}
