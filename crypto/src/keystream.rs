//! Self-growing key stream

use aes::{
    cipher::{
        generic_array::{typenum::U16, GenericArray},
        BlockEncrypt, KeyInit,
    },
    Aes256,
};
use std::slice::Iter;

pub type Block = GenericArray<u8, U16>;

/// Represents a self-growing key stream
#[derive(Debug, Clone)]
pub struct KeyStream {
    cipher: Aes256,
    stream: Vec<u8>,
    block: Block,
}

impl KeyStream {
    /// Creates a new [`KeyStream`]
    pub fn new(key: &[u8; 32], iv: &[u8; 4]) -> Self {
        KeyStream {
            cipher: Aes256::new(&GenericArray::from_slice(key)),
            stream: Vec::new(),
            block: Block::clone_from_slice(iv.repeat(4).as_slice()),
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
            self.cipher.encrypt_block(&mut self.block);
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

#[cfg(test)]
mod tests {

    use crate::{KeyStream, GMS_IV, TRIMMED_KEY};

    #[test]
    fn stream_16() {
        let mut stream = KeyStream::new(&[0x00; 32], &[0x00; 4]);
        stream.grow(16);
        assert_eq!(
            stream.as_slice(),
            &[
                0xdc, 0x95, 0xc0, 0x78, 0xa2, 0x40, 0x89, 0x89, 0xad, 0x48, 0xa2, 0x14, 0x92, 0x84,
                0x20, 0x87
            ]
        );
    }

    #[test]
    fn stream_grow_to_32() {
        let mut stream = KeyStream::new(&[0x00; 32], &[0x00; 4]);
        stream.grow(24);
        assert_eq!(stream.len(), 32);
        assert_eq!(
            stream.as_slice(),
            &[
                0xdc, 0x95, 0xc0, 0x78, 0xa2, 0x40, 0x89, 0x89, 0xad, 0x48, 0xa2, 0x14, 0x92, 0x84,
                0x20, 0x87, 0x08, 0xc3, 0x74, 0x84, 0x8c, 0x22, 0x82, 0x33, 0xc2, 0xb3, 0x4f, 0x33,
                0x2b, 0xd2, 0xe9, 0xd3
            ]
        );
    }

    #[test]
    fn stream_32_no_grow() {
        let mut stream = KeyStream::new(&[0x00; 32], &[0x00; 4]);
        stream.grow(24);
        assert_eq!(stream.len(), 32);
        assert_eq!(
            stream.as_slice(),
            &[
                0xdc, 0x95, 0xc0, 0x78, 0xa2, 0x40, 0x89, 0x89, 0xad, 0x48, 0xa2, 0x14, 0x92, 0x84,
                0x20, 0x87, 0x08, 0xc3, 0x74, 0x84, 0x8c, 0x22, 0x82, 0x33, 0xc2, 0xb3, 0x4f, 0x33,
                0x2b, 0xd2, 0xe9, 0xd3
            ]
        );
        stream.grow(32);
        assert_eq!(stream.len(), 32);
        assert_eq!(
            stream.as_slice(),
            &[
                0xdc, 0x95, 0xc0, 0x78, 0xa2, 0x40, 0x89, 0x89, 0xad, 0x48, 0xa2, 0x14, 0x92, 0x84,
                0x20, 0x87, 0x08, 0xc3, 0x74, 0x84, 0x8c, 0x22, 0x82, 0x33, 0xc2, 0xb3, 0x4f, 0x33,
                0x2b, 0xd2, 0xe9, 0xd3
            ]
        );
    }

    #[test]
    fn stream_iter() {
        let mut stream = KeyStream::new(&[0x00; 32], &[0x00; 4]);
        stream.grow(24);
        assert_eq!(stream.len(), 32);
        let mut it1 = [
            0xdc, 0x95, 0xc0, 0x78, 0xa2, 0x40, 0x89, 0x89, 0xad, 0x48, 0xa2, 0x14, 0x92, 0x84,
            0x20, 0x87, 0x08, 0xc3, 0x74, 0x84, 0x8c, 0x22, 0x82, 0x33, 0xc2, 0xb3, 0x4f, 0x33,
            0x2b, 0xd2, 0xe9, 0xd3,
        ]
        .iter();
        let mut it2 = stream.iter();
        loop {
            let left = it1.next();
            let right = it2.next();
            if left == None || right == None {
                break;
            }
            assert_eq!(left, right);
        }
    }

    #[test]
    fn stream_xor() {
        let mut stream = KeyStream::new(&[0x00; 32], &[0x00; 4]);
        let mut data: Vec<u8> = Vec::from("success".as_bytes());
        stream.xor(&mut data);
        assert_eq!(stream.len(), 16);
        assert_eq!(data.as_slice(), &[0xaf, 0xe0, 0xa3, 0x1b, 0xc7, 0x33, 0xfa]);
    }

    #[test]
    fn stream_xor_grow() {
        let mut stream = KeyStream::new(&[0x00; 32], &[0x00; 4]);
        let mut data1: Vec<u8> = Vec::from("success".as_bytes());
        stream.xor(&mut data1);
        assert_eq!(
            data1.as_slice(),
            &[0xaf, 0xe0, 0xa3, 0x1b, 0xc7, 0x33, 0xfa]
        );
        let mut data2: Vec<u8> = Vec::from("bigger than sixteen".as_bytes());
        stream.xor(&mut data2);
        assert_eq!(stream.len(), 32);
        assert_eq!(
            data2.as_slice(),
            &[
                0xbe, 0xfc, 0xa7, 0x1f, 0xc7, 0x32, 0xa9, 0xfd, 0xc5, 0x29, 0xcc, 0x34, 0xe1, 0xed,
                0x58, 0xf3, 0x6d, 0xa6, 0x1a
            ]
        );
    }

    #[test]
    fn gms_encrypt() {
        let mut stream = KeyStream::new(&TRIMMED_KEY, &GMS_IV);
        let mut input: Vec<u8> = Vec::from("smap.img".as_bytes());
        stream.encrypt(&mut input);
        assert_eq!(input.as_slice(), &[229, 195, 94, 212, 102, 147, 176, 247]);
    }

    #[test]
    fn gms_decrypt() {
        let mut stream = KeyStream::new(&TRIMMED_KEY, &GMS_IV);
        let mut input = Vec::from([229, 195, 94, 212, 102, 147, 176, 247]);
        stream.decrypt(&mut input);
        let test = String::from_utf8(input).unwrap();
        assert_eq!(test, "smap.img");
    }

    #[test]
    fn sanity() {
        let mut stream = KeyStream::new(&TRIMMED_KEY, &GMS_IV);
        let mut input: Vec<u8> = Vec::from("Hello, World!".as_bytes());
        let control = input.clone();
        stream.encrypt(&mut input);
        stream.decrypt(&mut input);
        assert_eq!(input, control);
    }
}
