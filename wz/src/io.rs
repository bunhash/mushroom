//! WZ IO

mod decode;
mod encode;
mod read;
mod write;

pub mod xml;

pub(crate) use encode::SizeHint;

pub use decode::Decode;
pub use encode::Encode;
pub use read::{DummyDecryptor, WzImageReader, WzRead, WzReader};
pub use write::{DummyEncryptor, WzImageWriter, WzWrite, WzWriter};
