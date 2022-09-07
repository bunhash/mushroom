//! Mushroom crypto implementation
//!
//! Handles the AES-OFB encryption used throughout Mushroom.

mod generator;
mod keystream;
mod system;

pub use generator::{Generator, MushroomGenerator};
pub use keystream::KeyStream;
pub use system::{MushroomSystem, System};

#[macro_use]
extern crate lazy_static;

pub static GMS_KEY: [u8; 32] = [
    0x13, 0x52, 0x2a, 0x5b, 0x08, 0x02, 0x10, 0x60, 0x06, 0x02, 0x43, 0x0f, 0xb4, 0x4b, 0x35, 0x05,
    0x1b, 0x0a, 0x5f, 0x09, 0x0f, 0x50, 0x0c, 0x1b, 0x33, 0x55, 0x01, 0x09, 0x52, 0xde, 0xc7, 0x1e,
];
pub static GMS_IV: [u8; 4] = [0x4d, 0x23, 0xc7, 0x2b];

lazy_static! {
    static ref GMS_CRYPTO_SYSTEM: MushroomSystem = MushroomSystem::new(GMS_KEY, GMS_IV);
    static ref GMS_KEY_STREAM: KeyStream<MushroomSystem> = KeyStream::new(&GMS_CRYPTO_SYSTEM);
}

#[cfg(test)]
mod tests {

    use crate::{KeyStream, MushroomSystem, System};

    #[test]
    fn test_system() {
        let system = MushroomSystem::new([0x00; 32], [0x00; 4]);
        let _ = system.into_generator();
    }

    #[test]
    fn stream_16() {
        let system = MushroomSystem::new([0x00; 32], [0x00; 4]);
        let mut stream = KeyStream::new(&system);
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
        let system = MushroomSystem::new([0x00; 32], [0x00; 4]);
        let mut stream = KeyStream::new(&system);
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
        let system = MushroomSystem::new([0x00; 32], [0x00; 4]);
        let mut stream = KeyStream::new(&system);
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
        let system = MushroomSystem::new([0x00; 32], [0x00; 4]);
        let mut stream = KeyStream::new(&system);
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
        let system = MushroomSystem::new([0x00; 32], [0x00; 4]);
        let mut stream = KeyStream::new(&system);
        let mut data: Vec<u8> = Vec::from("success".as_bytes());
        stream.xor(&mut data);
        assert_eq!(stream.len(), 16);
        assert_eq!(data.as_slice(), &[0xaf, 0xe0, 0xa3, 0x1b, 0xc7, 0x33, 0xfa]);
    }

    #[test]
    fn stream_xor_grow() {
        let system = MushroomSystem::new([0x00; 32], [0x00; 4]);
        let mut stream = KeyStream::new(&system);
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
}
