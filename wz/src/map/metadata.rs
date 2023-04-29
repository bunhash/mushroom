//! Trait for map metadata

use crate::map::SizeHint;

pub trait Metadata {
    fn update<S>(&mut self, children: &[&S])
    where
        S: SizeHint;
}
