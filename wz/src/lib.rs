#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

mod decode;
mod macros;
mod map;
mod metadata;
mod reader;
mod writer;

pub mod error;
pub mod package;
pub mod types;

pub use decode::Decode;
pub use map::WzMap;
pub use metadata::Metadata;
pub use reader::{EncryptedWzReader, Reader, WzReader};
