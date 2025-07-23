///! Dummy Encryptor
use crypto::Encryptor;

/// Encryptor that does nothing
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DummyEncryptor;

impl Encryptor for DummyEncryptor {
    /// Empty function that does nothing to the provided bytes
    fn encrypt(&mut self, _: &mut Vec<u8>) {}
}
