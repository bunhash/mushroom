//! Trait for map metadata

use crate::{
    error::Result,
    map::{Cursor, SizeHint},
    types::WzInt,
};

pub trait Metadata<T>: SizeHint + Sized
where
    T: SizeHint,
{
    fn new(name: &str) -> Self;

    fn name(&self) -> &str;

    fn size(&self) -> WzInt;

    fn rename(&mut self, name: &str);

    fn update(&self, cursor: Cursor<Self, T>) -> Self;
}
