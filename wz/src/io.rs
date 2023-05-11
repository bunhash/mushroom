//! WZ IO

mod decode;
mod encode;
mod reader;
mod writer;

pub mod xml;

pub(crate) use encode::SizeHint;

pub use decode::Decode;
pub use encode::Encode;
pub use reader::{DummyDecryptor, WzReader};
pub use writer::{DummyEncryptor, WzWriter};
