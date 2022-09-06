//! Mushroom crypto implementation
//!
//! Handles the AES-OFB encryption used throughout Mushroom.

mod error;
mod system;
mod keystream;

pub use error::KeyStreamError;
pub use system::MushroomSystem;
pub use keystream::KeyStream;

pub fn xor<T>(data: &mut T)
where
    T: IntoIterator<Item = u8>,
{
}

#[cfg(test)]
mod tests {

    use crate::MushroomSystem;

    #[test]
    fn stream_16() {
        let gen = MushroomSystem::new([0x00; 32], [0x00; 4]);
        let stream = gen.generate(16).unwrap();
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
        let gen = MushroomSystem::new([0x00; 32], [0x00; 4]);
        let mut stream = gen.generate(16).unwrap();
        gen.grow(&mut stream, 32).unwrap();
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
        let gen = MushroomSystem::new([0x00; 32], [0x00; 4]);
        let mut stream = gen.generate(32).unwrap();
        assert_eq!(
            stream.as_slice(),
            &[
                0xdc, 0x95, 0xc0, 0x78, 0xa2, 0x40, 0x89, 0x89, 0xad, 0x48, 0xa2, 0x14, 0x92, 0x84,
                0x20, 0x87, 0x08, 0xc3, 0x74, 0x84, 0x8c, 0x22, 0x82, 0x33, 0xc2, 0xb3, 0x4f, 0x33,
                0x2b, 0xd2, 0xe9, 0xd3
            ]
        );
        gen.grow(&mut stream, 32).unwrap(); // should not change
        assert_eq!(
            stream.as_slice(),
            &[
                0xdc, 0x95, 0xc0, 0x78, 0xa2, 0x40, 0x89, 0x89, 0xad, 0x48, 0xa2, 0x14, 0x92, 0x84,
                0x20, 0x87, 0x08, 0xc3, 0x74, 0x84, 0x8c, 0x22, 0x82, 0x33, 0xc2, 0xb3, 0x4f, 0x33,
                0x2b, 0xd2, 0xe9, 0xd3
            ]
        );
    }
}
