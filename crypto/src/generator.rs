//! Mushroom key stream generator

use aes::cipher::{
    generic_array::{
        typenum::{U16, U32},
        ArrayLength, GenericArray,
    },
    BlockEncrypt, KeyInit,
};
use aes::Aes256;

pub trait Generator {
    type BlockSize: ArrayLength<u8>;
    fn next_block(&mut self) -> &GenericArray<u8, Self::BlockSize>;
}

#[derive(Clone, Debug)]
pub struct MushroomGenerator {
    cipher: Aes256,
    block: GenericArray<u8, U16>,
}

impl MushroomGenerator {
    pub fn new(key: &GenericArray<u8, U32>, iv: &[u8; 4]) -> Self {
        MushroomGenerator {
            cipher: Aes256::new(&key),
            block: GenericArray::clone_from_slice(iv.repeat(4).as_slice()),
        }
    }
}

impl Generator for MushroomGenerator {
    type BlockSize = U16;

    fn next_block(&mut self) -> &GenericArray<u8, Self::BlockSize> {
        self.cipher.encrypt_block(&mut self.block);
        &self.block
    }
}
