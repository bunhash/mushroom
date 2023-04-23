use crate::{Block, System};
use aes::cipher::generic_array::ArrayLength;
use std::slice::Iter;

/// Represents a self-growing key stream
#[derive(Clone, Debug, PartialEq)]
pub struct KeyStream<'a, B, S>
where
    B: ArrayLength<u8>,
    S: System<B>,
{
    system: &'a S,
    stream: Vec<u8>,
    block: Block<B>,
}

impl<'a, B, S> KeyStream<'a, B, S>
where
    B: ArrayLength<u8>,
    S: System<B>,
{
    /// Creates a new `KeyStream` given a reference to a `System`
    pub fn new(system: &'a S) -> Self {
        KeyStream {
            system: system,
            stream: Vec::new(),
            block: system.prepare(),
        }
    }

    /// Returns the current length of the key stream
    pub fn len(&self) -> usize {
        self.stream.len()
    }

    /// Returns an immutable iterator over the bytes in the key stream
    pub fn iter(&self) -> Iter<'_, u8> {
        self.stream.iter()
    }

    /// Returns a clone of the internal `Vec` holding the key stream
    pub fn to_vec(&self) -> Vec<u8> {
        self.stream.clone()
    }

    /// Returns an immutable slice of the key stream
    pub fn as_slice(&self) -> &[u8] {
        self.stream.as_slice()
    }

    /// Grows the key stream to be big enough to cover `new_len`
    pub fn grow(&mut self, new_len: usize) {
        let system = self.system;
        loop {
            if self.stream.len() >= new_len {
                break;
            }
            system.process(&mut self.block);
            self.stream.extend_from_slice(self.block.as_slice());
        }
    }

    /// Computes a bitwise XOR on the input
    pub fn xor(&mut self, input: &mut Vec<u8>) {
        let input_len = input.len();
        self.grow(input_len);
        for i in 0..input_len {
            let val = input[i];
            input[i] = val ^ self.stream[i];
        }
    }

    /// Encrypts the vector with the key stream
    pub fn encrypt(&mut self, input: &mut Vec<u8>) {
        self.xor(input);
    }

    /// Decrypts the vector with the key stream
    pub fn decrypt(&mut self, input: &mut Vec<u8>) {
        self.xor(input);
    }
}
