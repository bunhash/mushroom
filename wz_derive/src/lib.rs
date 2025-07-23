#![crate_type = "proc-macro"]
#![forbid(unsafe_code)]
#![warn(
    clippy::unwrap_used,
    rust_2018_idioms,
    trivial_casts,
    unused_qualifications,
    missing_docs
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

fn hello_world() {
    println!("hello, world!");
}
