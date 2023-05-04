#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub mod archive;
pub mod decode;
pub mod encode;
pub mod error;
pub mod file;
pub mod map;
pub mod reader;
pub mod types;
pub mod writer;

pub use archive::Archive;
pub use decode::Decode;
pub use encode::Encode;
pub use reader::WzReader;
pub use writer::WzWriter;
