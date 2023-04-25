//! WZ Package Contents

use crate::types::{WzInt, WzOffset, WzString};

#[derive(Debug)]
pub enum Content {
    Directory(WzString),
    Image(WzString, WzInt, WzOffset),
}

impl Content {
    #[inline]
    pub fn is_image(&self) -> bool {
        match self {
            Content::Directory(_) => false,
            Content::Image(_, _, _) => true,
        }
    }

    #[inline]
    pub fn is_directory(&self) -> bool {
        !self.is_image()
    }
}
