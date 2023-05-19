///! Dummy Decryptor
use crypto::Decryptor;

/// Decryptor that does nothing
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DummyDecryptor;

impl Decryptor for DummyDecryptor {
    /// Empty function that does nothing to the provided bytes
    fn decrypt(&mut self, _: &mut Vec<u8>) {}
}
