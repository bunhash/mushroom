#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

mod decode;
mod encode;
mod writer;

pub mod error;
pub mod file;
pub mod map;
pub mod reader;
//pub mod package;
pub mod types;

pub use decode::Decode;
pub use encode::Encode;
pub use file::WzFile;
pub use reader::WzReader;
pub use writer::WzWriter;
