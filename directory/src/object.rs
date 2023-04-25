//! Object structure

use alloc::string::String;

pub struct Object<T> {
    pub name: String,
    pub data: T,
}
