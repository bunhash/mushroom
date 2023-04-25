//! Directory structure

use crate::ContentId;
use alloc::{string::String, vec::Vec};

pub struct Directory {
    pub name: String,
    pub contents: Vec<ContentId>,
}
