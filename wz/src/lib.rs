#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::checked_conversions,
    clippy::implicit_saturating_sub,
    clippy::mod_module_files,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

//mod encode;
//mod writer;
mod decode;
mod error;
mod header;
mod int;
mod offset;
mod primitives;
mod reader;
mod string;
//mod uol;

#[macro_use]
mod macros;

//pub use encode::Encode;
//pub use writer::Writer;
pub use decode::Decode;
pub use error::{DecodeError, Error};
pub use header::Header;
pub use int::{Integer, Long};
pub use offset::Offset;
pub use reader::Reader;
pub use string::WzString;
//pub use uol::{UolObject, UolString};
