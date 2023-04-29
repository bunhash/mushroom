//! WZ File ContentRef

use crate::{
    map::{Metadata, SizeHint},
    types::{WzInt, WzOffset, WzString},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageRef {
    pub(crate) name_size: WzInt,
    pub(crate) size: WzInt,
    pub(crate) offset: WzOffset,
    pub(crate) num_content: WzInt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageRef {
    pub(crate) name_size: WzInt,
    pub(crate) size: WzInt,
    pub(crate) checksum: WzInt,
    pub(crate) offset: WzOffset,
}

/// `ContentRef` found in WZ files
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentRef {
    /// Package holding more content
    Package(PackageRef),

    /// Image--treated as a binary blob
    Image(ImageRef),
}

impl Metadata for ContentRef {
    fn update(&mut self, name: &WzString, children_sizes: &[WzInt]) {
        let size = WzInt::from(
            children_sizes
                .iter()
                .map(|size| i32::from(*size))
                .sum::<i32>(),
        );
        match self {
            ContentRef::Package(ref mut package) => {
                package.name_size = name.size_hint();
                package.size = size;
                package.num_content = WzInt::from(children_sizes.len() as i32);
            }
            ContentRef::Image(ref mut image) => image.name_size = name.size_hint(),
        }
    }
}

impl SizeHint for ContentRef {
    fn size_hint(&self) -> WzInt {
        match &self {
            ContentRef::Package(package) => {
                1 + package.name_size
                    + package.size.size_hint()
                    + WzInt::from(0).size_hint()
                    + package.offset.size_hint()
                    + package.num_content.size_hint()
                    + package.size
            }
            ContentRef::Image(image) => {
                1 + image.name_size
                    + image.size.size_hint()
                    + image.checksum.size_hint()
                    + image.offset.size_hint()
                    + image.size
            }
        }
    }
}
