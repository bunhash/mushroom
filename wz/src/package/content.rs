//! WZ Package Contents

use crate::package::Params;

#[derive(Debug)]
pub enum Content {
    Package(Params),
    Image(Params),
}

impl Content {
    #[inline]
    pub fn is_package(&self) -> bool {
        match self {
            Content::Package(_) => true,
            Content::Image(_) => false,
        }
    }

    #[inline]
    pub fn is_image(&self) -> bool {
        !self.is_package()
    }
}
