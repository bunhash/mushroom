//! Tree Node

use crate::{Directory, Empty, Object};

pub enum Node {
    Directory(Directory),
    Object(Object),
    Empty(Empty),
}
