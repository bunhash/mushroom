//! Mushroom utils module
//!
//! Handles the checksum calculations
//!
//! # Example
//!
//! ```
//! use crypto::checksum;
//!
//! fn brute_force() -> (u16, u16) {
//!     let encrypted_version = 0xac;
//!     for real_version in 1..1000 {
//!         let (calc_version, csum) = checksum(&real_version.to_string());
//!         if calc_version == encrypted_version {
//!             return (real_version as u16, csum as u16);
//!         }
//!     }
//!     panic!("failed");
//! }
//! ```

/// Calculates the version checksum (or, encrypted version)
pub fn checksum(version: &str) -> (u16, u32) {
    let mut y = 0;
    for c in version.as_bytes() {
        y = (y << 5) + (*c as u32) + 1;
    }
    let x = 0xFF;
    let x = x ^ (((y >> 24) & 0xFF) as u16);
    let x = x ^ (((y >> 16) & 0xFF) as u16);
    let x = x ^ (((y >> 8) & 0xFF) as u16);
    let x = x ^ ((y & 0xFF) as u16);
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
