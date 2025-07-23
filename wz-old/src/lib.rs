#![forbid(unsafe_code)]
#![warn(
    clippy::unwrap_used,
    rust_2018_idioms,
    trivial_casts,
    unused_qualifications
    missing_docs
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub mod archive;
pub mod error;
pub mod image;
pub mod io;
pub mod list;
pub mod map;
pub mod types;
