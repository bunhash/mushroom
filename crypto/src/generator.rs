use aes::cipher::{
    generic_array::{
        typenum::{U16, U32},
        ArrayLength, GenericArray,
    },
    BlockEncrypt, KeyInit,
};
use aes::Aes256;

/// Generator trait for generating cipher blocks
///
/// `Generator` defines a method of turning a block cipher into a stream cipher by continuously
/// generating cryptographically random chunks of ciphertext.
///
/// # Example
///
/// ```
/// use crypto::{
///     generic_array::{
///         typenum::{U16, U32},
///         ArrayLength, GenericArray,
///     },
///     Generator,
/// };
///
/// fn encrypt(key: [u8; 32], block: &mut GenericArray<u8, U16>) {
///     // do stuff to block
/// }
///
/// struct BlockCipher {
///     key: [u8; 32],
///     block: GenericArray<u8, U16>
/// }
///
/// impl Generator for BlockCipher {
///     type BlockSize = U16;
///
///     fn next_block(&mut self) -> &GenericArray<u8, Self::BlockSize> {
///         encrypt(self.key, &mut self.block);
///         &self.block
///     }
/// }
/// ```
pub trait Generator {
    /// The block size
    type BlockSize: ArrayLength<u8>;

    /// Acts similarly to `Iterator` except there should be no end and can generate endlessly.
    fn next_block(&mut self) -> &GenericArray<u8, Self::BlockSize>;
}

/// `Generator` implementation as used in Mushroom
#[derive(Clone, Debug)]
pub struct MushroomGenerator {
    cipher: Aes256,
    block: GenericArray<u8, U16>,
}

impl MushroomGenerator {

    /// Creates a new `MushroomGenerator` given the AES-256 key and 4-byte initialization vector
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
