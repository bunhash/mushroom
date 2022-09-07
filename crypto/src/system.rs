use crate::{Generator, MushroomGenerator};

use aes::cipher::generic_array::{typenum::U32, GenericArray};

/// System trait for defining cryptographic systems
///
/// `System` is designed to be an immutable representation of the cryptographic system. The
/// `System` builds a mutable `Generator` specific to itself that infinitely generates blocks for
/// input into the `KeyStream`.
///
/// # Example
///
/// ```
/// use crypto::{
///     generic_array::{
///         typenum::{U16, U32},
///         ArrayLength, GenericArray,
///     },
///     Generator, System,
/// };
///
/// struct CustomGenerator {
///     block: GenericArray<u8, U16>
/// }
/// ```
pub trait System {
    type Gen: Generator;
    fn into_generator(&self) -> Self::Gen;
}

/// `System` implementation as used in Mushroom
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MushroomSystem {
    key: GenericArray<u8, U32>,
    iv: [u8; 4],
}

impl MushroomSystem {

    /// Creates a new `MushroomSystem` given the key and initialization vector
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
