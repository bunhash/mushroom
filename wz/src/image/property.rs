//! WZ Property structure

use crate::{
    decode::{Decode, Decoder},
    encode::{Encode, Encoder, SizeHint},
    image::{Error, UolString},
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Property {
    /// Name of the property
    pub name: UolString,

    /// Value of the property
    pub value: Value,
}

impl Decode for PropertyType {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        Ok(Property {
            name: UolString::decode(decoder)?,
            value: Value::decode(decoder)?,
        })
    }
}

impl Encode for PropertyType {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {}
}

impl SizeHint for PropertyType {
    fn size_hint(&self) -> u64 {}
}
