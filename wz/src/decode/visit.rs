//! Visit Trait

use crate::decode::{Decode, Decoder};

/// Trait for visiting decoded objects
pub trait Visitor {
    type Value;

    /// Decode i8
    fn visit_i8<D: Decoder>(decoder: &mut D) -> Result<Self::Value, Error> {}
}

impl Visitor for i8 {}
impl Visitor for i16 {}
impl Visitor for i32 {}
impl Visitor for i64 {}
