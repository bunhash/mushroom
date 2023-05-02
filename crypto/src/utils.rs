//! Mushroom utils module
//!
//! Handles the checksum calculations
//!
//! # Example
//!
//! ```
//! use crypto::checksum;
//!
//! fn brute_force() -> (u16, u32) {
//!     let encrypted_version = 0xac;
//!     for real_version in 1..1000 {
//!         let (calc_version, csum) = checksum(&real_version.to_string());
//!         if calc_version == encrypted_version {
//!             return (real_version as u16, csum);
//!         }
//!     }
//!     panic!("failed");
//! }
//! ```

/// Calculates the version checksum (or, encrypted version)
pub fn checksum(version: &str) -> (u16, u32) {
    let mut y = 0u32;
    for c in version.as_bytes() {
        y = (y.rotate_left(5) & 0xFFE0)
            .wrapping_add(*c as u32)
            .wrapping_add(1);
    }
    let x = (y.rotate_right(24) & 0xFF) as u16;
    let x = x ^ ((y.rotate_right(16) & 0xFF) as u16);
    let x = x ^ ((y.rotate_right(8) & 0xFF) as u16);
    let x = x ^ ((y & 0xFF) as u16);
    let x = x ^ 0xFF; // Flip all bits
    (x, y)
}

#[cfg(test)]
mod tests {

    use crate::checksum;

    #[test]
    fn calc_83_checksum() {
        let (calc_version, csum) = checksum("83");
        assert_eq!(calc_version, 0xac);
        assert_eq!(csum, 1876);
    }

    #[test]
    fn calc_176_checksum() {
        let (calc_version, csum) = checksum("176");
        assert_eq!(calc_version, 0x07);
        assert_eq!(csum, 53047);
    }
}
