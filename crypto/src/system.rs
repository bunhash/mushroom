//! Mushroom AES-OFB key stream generator

use crate::{KeyStream, KeyStreamError};

use aes::cipher::{
    generic_array::{
        typenum::{U16, U32},
        GenericArray,
    },
    BlockEncrypt, KeyInit,
};
use aes::Aes256;

pub trait CryptoSystem<T> {
    fn grow(&self, input: &GenericArray<u8, T>) -> GenericArray<u8, T>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MushroomSystem {
    key: GenericArray<u8, U32>,
    iv: [u8; 4],
}

impl CryptoSystem<U16> for MushroomSystem {
    fn ini
    fn process(input: &GenericArray<u8, U16>) -> GenericArray<u8, U16> {
        let cipher = Aes256::new(&self.key);
        let mut block = input.clone();
        cipher.encrypt_block(&mut block);
        block
    }
}

impl MushroomSystem {
    pub fn new(key: [u8; 32], iv: [u8; 4]) -> Self {
        MushroomSystem {
            key: GenericArray::from(key),
            iv: iv,
        }
    }

    pub fn generate(&self, min_len: usize) -> Result<KeyStream, KeyStreamError> {
        let mut ret = KeyStream::new();
        self.grow(&mut ret, min_len)?;
        Ok(ret)
    }

    pub fn grow(&self, stream: &mut KeyStream, new_len: usize) -> Result<(), KeyStreamError> {
        let current_len = stream.len();

        if new_len <= current_len {
            Ok(())
        } else if current_len % 16 != 0 {
            Err(KeyStreamError)
        } else {
            let cipher = Aes256::new(&self.key);
            let mut block: GenericArray<u8, U16> = match current_len {
                // Take the IV as the first block if the stream's length is 0
                0 => GenericArray::clone_from_slice(self.iv.repeat(4).as_slice()),

                // Otherwise, grab the last 16 bytes. At this point there should be at least 16
                // values. But it'll error if there isn't anyways.
                _ => {
                    let mut last_block = [0u8; 16];
                    let mut vals = stream.iter().rev();
                    for i in (0..16).rev() {
                        if let Some(val) = vals.next() {
                            last_block[i] = *val;
                        } else {
                            return Err(KeyStreamError);
                        }
                    }
                    GenericArray::from(last_block)
                }
            };

            // Keep encrypting until the stream is big enough
            loop {
                cipher.encrypt_block(&mut block);
                stream.append_block(&block);
                if stream.len() >= new_len {
                    break;
                }
            }

            Ok(())
        }
    }
}
