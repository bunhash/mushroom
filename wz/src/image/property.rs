//! WZ Property structure

use crate::{image::Value, macros, string::UolString, Decode, Error, Reader};
use crypto::Decryptor;
use std::io::{Read, Seek};

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Property {
    pub name: UolString,
    pub value: Value,
}

impl Decode for PropertyType {
    fn decode<R, D>(reader: &mut Reader<R, D>) -> Result<Self, Error>
    where
        R: Read + Seek,
        D: Decryptor,
    {
        Ok(Property {
            name: UolString::decode(reader)?,
            value: Value::decode(reader)?,
        })
    }
}
