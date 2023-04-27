//! WZ Package Contents

use crate::{
    map::SizeHint,
    package::Params,
    types::{WzInt, WzString},
};

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

impl SizeHint for Content {
    fn data_size(&self) -> WzInt {
        match self {
            Content::Package(_) => WzInt::from(0),
            Content::Image(params) => params.size,
        }
    }

    fn header_size(&self, num_children: usize) -> WzInt {
        match self {
            Content::Package(_) => WzInt::from(num_children as i32).data_size(),
            Content::Image(_) => WzInt::from(0),
        }
    }

    fn metadata_size(&self, name: &str) -> WzInt {
        match self {
            Content::Package(params) => 1 + WzString::from(name).data_size() + params.data_size(),
            Content::Image(params) => 1 + WzString::from(name).data_size() + params.data_size(),
        }
    }
}
