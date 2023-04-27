//! WZ Package Contents

use crate::{
    map::SizeHint,
    types::{WzInt, WzOffset, WzString},
};

#[derive(Debug)]
pub enum Content {
    Package(WzOffset),
    Image(WzOffset, WzInt),
}

impl SizeHint for Content {
    fn size_hint(&self, num_children: usize) -> WzInt {
        match self {
            Content::Package(_) => WzInt::from(num_children as i32).size_hint(0),
            Content::Image(_, size) => *size,
        }
    }
}
