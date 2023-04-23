#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

mod decode;
mod metadata;
mod offset;
mod package;
mod primitives;
mod reader;
mod string;

pub mod error;

pub use decode::Decode;
pub use metadata::Metadata;
pub use offset::WzOffset;
pub use package::{Content, ContentInfo, Package};
pub use primitives::{WzInt, WzLong};
pub use reader::{EncryptedWzReader, Reader, WzReader};
pub use string::{CString, WzString};
