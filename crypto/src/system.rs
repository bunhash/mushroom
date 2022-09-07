//! Mushroom crypto system

use crate::{Generator, MushroomGenerator};

use aes::cipher::generic_array::{typenum::U32, GenericArray};

pub trait System {
    type Gen: Generator;
    fn into_generator(&self) -> Self::Gen;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MushroomSystem {
    key: GenericArray<u8, U32>,
    iv: [u8; 4],
}

impl MushroomSystem {
    pub fn new(key: [u8; 32], iv: [u8; 4]) -> Self {
        MushroomSystem {
            key: GenericArray::from(key),
            iv: iv,
        }
    }
}

impl System for MushroomSystem {
    type Gen = MushroomGenerator;

    fn into_generator(&self) -> MushroomGenerator {
        MushroomGenerator::new(&self.key, &self.iv)
    }
}
