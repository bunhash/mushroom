//! Mushroom key stream

use crate::{Generator, System};

use std::slice::{Iter, IterMut};

#[derive(Clone, Debug, PartialEq)]
pub struct KeyStream<S: System> {
    generator: S::Gen,
    stream: Vec<u8>,
}

impl<S: System> KeyStream<S> {
    pub fn new(system: &S) -> Self {
        KeyStream {
            generator: system.into_generator(),
            stream: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.stream.len()
    }

    pub fn iter(&self) -> Iter<'_, u8> {
        self.stream.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, u8> {
        self.stream.iter_mut()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.stream.clone()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.stream.as_slice()
    }

    pub fn grow(&mut self, new_len: usize) {
        loop {
            if self.stream.len() >= new_len {
                break;
            }
            let block = self.generator.next_block();
            self.stream.extend_from_slice(block.as_slice());
        }
    }

    pub fn xor(&mut self, input: &mut Vec<u8>) {
        let input_len = input.len();
        self.grow(input_len);
        for i in 0..input_len {
            let val = input[i];
            input[i] = val ^ self.stream[i];
        }
    }
}
