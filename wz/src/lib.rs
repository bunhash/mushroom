#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

mod decode;
mod encode;
mod file;
mod metadata;
mod writer;

pub mod error;
pub mod map;
pub mod reader;
//pub mod package;
pub mod types;

pub use indextree;

pub use decode::Decode;
pub use encode::Encode;
pub use file::WzFile;
//pub use map::Map;
pub use metadata::Metadata;
pub use reader::WzReader;
pub use writer::WzWriter;
