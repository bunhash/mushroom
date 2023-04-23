use aes::{
    cipher::{
        generic_array::{typenum::U16, ArrayLength, GenericArray},
        BlockEncrypt, KeyInit,
    },
    Aes256,
};

/// Used for input to and output from the `System`
pub type Block<B> = GenericArray<u8, B>;

/// System trait for defining cryptographic systems
///
/// `System` is designed to be an immutable representation of the cryptographic system. It defines
/// how to process a block in order to generate a cryptographically random key stream
/// (theoretically).
///
/// # Example
///
/// ```
/// use crypto::{generic_array::{typenum::U16}, Block, System};
///
/// fn encrypt(key: &[u8; 32], block: &mut Block<U16>) {
///     // Do stuff to block
/// }
///
/// struct SystemImpl {
///     pub key: [u8; 32],
///     pub iv: [u8; 16],
/// }
///
/// impl System<U16> for SystemImpl {
///     fn prepare(&self) -> Block<U16> {
///         Block::from(self.iv)
///     }
///
///     fn process(&self, block: &mut Block<U16>) {
///         encrypt(&self.key, block);
///     }
/// }
///
/// let system = SystemImpl { key: [0x00; 32], iv: [0x00; 16] };
/// let mut block = system.prepare();
/// system.process(&mut block);
/// ```
pub trait System<B>
where
    B: ArrayLength<u8>,
{
    /// Prepares the initial input into the `System`
    fn prepare(&self) -> Block<B>;

    /// Mangles the block
    fn process(&self, block: &mut Block<B>);
}

/// `System` implementation as used in Mushroom
#[derive(Clone, Debug)]
pub struct MushroomSystem {
    cipher: Aes256,
    iv: [u8; 4],
}

impl MushroomSystem {
    /// Creates a new `MushroomSystem` given the key and initialization vector
    pub fn new(key: &[u8; 32], iv: &[u8; 4]) -> Self {
        MushroomSystem {
            cipher: Aes256::new(&GenericArray::from_slice(key)),
            iv: *iv,
        }
    }
}

impl System<U16> for MushroomSystem {
    fn prepare(&self) -> Block<U16> {
        Block::clone_from_slice(self.iv.repeat(4).as_slice())
    }

    fn process(&self, block: &mut Block<U16>) {
        self.cipher.encrypt_block(block);
    }
}
