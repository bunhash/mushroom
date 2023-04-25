#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod directory;
mod object;
mod id;

pub use directory::Directory;
pub use object::Object;
pub use id::ContentId;
