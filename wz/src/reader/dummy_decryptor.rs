///! Dummy Decryptor

use crypto::Decryptor;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DummyDecryptor;

impl Decryptor for DummyDecryptor {
    fn decrypt(&mut self, _: &mut Vec<u8>) {}
}
