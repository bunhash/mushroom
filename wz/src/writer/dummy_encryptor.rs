///! Dummy Encryptor

use crypto::Encryptor;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DummyEncryptor;

impl Encryptor for DummyEncryptor {
    fn encrypt(&mut self, _: &mut Vec<u8>) {}
}
