//! Mushroom hash module
//!
//! Handles the checksum calculations
//!
//! # Example
//!
//! ```
//! use crypto::hash::checksum;
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
    let mut x = 0xFF;
    x = x ^ (((y >> 24) & 0xFF) as u16);
    x = x ^ (((y >> 16) & 0xFF) as u16);
    x = x ^ (((y >> 8) & 0xFF) as u16);
    x = x ^ ((y & 0xFF) as u16);
    (x, y)
}
