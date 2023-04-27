#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

mod decode;
mod encode;
mod metadata;
mod reader;
mod writer;

pub mod error;
pub mod map;
//pub mod package;
pub mod types;

pub use indextree;

pub use decode::Decode;
pub use encode::Encode;
//pub use map::Map;
pub use metadata::Metadata;
pub use reader::{EncryptedReader, Reader, UnencryptedReader};
pub use writer::Writer;
