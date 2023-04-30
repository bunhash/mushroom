//! WZ File ContentRef

use crate::{
    decode::Error,
    file::raw::RawContentRef,
    map::{Metadata, SizeHint},
    types::{WzInt, WzOffset, WzString},
};
use std::convert::TryFrom;

/// Used to reference a `Package` within the WZ file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageRef {
    pub(crate) name_size: WzInt,
    pub(crate) size: WzInt,
    pub(crate) offset: WzOffset,
    pub(crate) num_content: WzInt,
}

impl PackageRef {
    pub fn new(name: &str) -> Self {
        Self {
            name_size: WzString::from(name).size_hint(),
            size: WzInt::from(0),
            offset: WzOffset::from(0),
            num_content: WzInt::from(0),
        }
    }
}

/// Used to reference an `Image` within the WZ file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageRef {
    pub(crate) name_size: WzInt,
    pub(crate) size: WzInt,
    pub(crate) checksum: WzInt,
    pub(crate) offset: WzOffset,
}

impl ImageRef {
    pub fn new(name: &str, size: WzInt, checksum: WzInt) -> Self {
        Self {
            name_size: WzString::from(name).size_hint(),
            size,
            checksum,
            offset: WzOffset::from(0),
        }
    }
}

/// `ContentRef` found in WZ files
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentRef {
    /// Package Content Type
    Package(PackageRef),

    /// Image Content Type. Treated as a binary blob.
    Image(ImageRef),
}

impl ContentRef {
    pub(crate) fn into_raw(&self, name: WzString) -> RawContentRef {
        match self {
            ContentRef::Package(package) => RawContentRef {
                tag: 3u8,
                name,
                size: package.size,
                checksum: WzInt::from(0),
                offset: package.offset,
            },
            ContentRef::Image(image) => RawContentRef {
                tag: 4u8,
                name,
                size: image.size,
                checksum: image.checksum,
                offset: image.offset,
            },
        }
    }
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

impl TryFrom<&RawContentRef> for ContentRef {
    type Error = Error;

    fn try_from(raw_content: &RawContentRef) -> Result<Self, Self::Error> {
        match raw_content.tag {
            3 => Ok(ContentRef::Package(PackageRef {
                name_size: raw_content.name.size_hint(),
                size: WzInt::from(0),
                offset: raw_content.offset,
                num_content: WzInt::from(0),
            })),
            4 => Ok(ContentRef::Image(ImageRef {
                name_size: raw_content.name.size_hint(),
                size: raw_content.size,
                checksum: raw_content.checksum,
                offset: raw_content.offset,
            })),
            t => Err(Error::InvalidContentType(t).into()),
        }
    }
}
