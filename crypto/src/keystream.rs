use crate::{Generator, System};

use std::slice::Iter;

/// Represents a self-growing key stream
#[derive(Clone, Debug, PartialEq)]
pub struct KeyStream<S: System> {
    generator: S::Gen,
    stream: Vec<u8>,
}

impl<S: System> KeyStream<S> {

    /// Creates a new `KeyStream` given a reference to a `System`
    pub fn new(system: &S) -> Self {
        KeyStream {
            generator: system.into_generator(),
            stream: Vec::new(),
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
        loop {
            if self.stream.len() >= new_len {
                break;
            }
            let block = self.generator.next_block();
            self.stream.extend_from_slice(block.as_slice());
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

    /// Same functionality as `xor`
    pub fn encrypt(&mut self, input: &mut Vec<u8>) {
        self.xor(input);
    }

    /// Same functionality as `xor`
    pub fn decrypt(&mut self, input: &mut Vec<u8>) {
        self.xor(input);
    }
}
