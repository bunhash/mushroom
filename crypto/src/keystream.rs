//! Mushroom AES-OFB key stream

use aes::cipher::generic_array::{
    typenum::{U112, U128, U16, U32, U48, U64, U80, U96},
    GenericArray,
};
use std::slice::{Iter, IterMut};

#[derive(Debug, Clone, PartialEq)]
pub struct KeyStream {
    data: Vec<u8>,
}

impl KeyStream {
    pub fn new() -> Self {
        KeyStream { data: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> Iter<'_, u8> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, u8> {
        self.data.iter_mut()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn append(&mut self, block: &GenericArray<u8, U16>) {
        self.data.extend_from_slice(block.as_slice());
    }
}

impl IntoIterator for KeyStream {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

macro_rules! from_generic_array {
    ( $block_size:ty ) => {
        impl From<GenericArray<u8, $block_size>> for KeyStream {
            fn from(block: GenericArray<u8, $block_size>) -> Self {
                KeyStream {
                    data: Vec::from(block.as_slice()),
                }
            }
        }
    };
}

from_generic_array!(U16);
from_generic_array!(U32);
from_generic_array!(U48);
from_generic_array!(U64);
from_generic_array!(U80);
from_generic_array!(U96);
from_generic_array!(U112);
from_generic_array!(U128);
