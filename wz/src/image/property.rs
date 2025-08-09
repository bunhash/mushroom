//! WZ Property structure

use crate::{
    decode::{self, Decode, Decoder},
    encode::{Encode, Encoder, SizeHint},
    image::{Error, UolString, Value},
    Int32,
};

/// Image property. This is just a name/value pair
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Property {
    /// Name of the property
    pub name: UolString,

    /// Value of the property
    pub value: Value,
}

impl Decode for Property {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        Ok(Property {
            name: UolString::decode(decoder)?,
            value: Value::decode(decoder)?,
        })
    }
}

impl Encode for Property {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        self.name.encode(encoder)?;
        self.value.encode(encoder)?;
        Ok(())
    }
}

impl SizeHint for Property {
    fn size_hint(&self) -> u64 {
        self.name.size_hint() + self.value.size_hint()
    }
}

/// Length prefixed list of properties
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PropertyList {
    properties: Vec<Property>,
}

impl Decode for PropertyList {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        u16::decode(decoder)?;
        let num_properties = Int32::decode(decoder)?;
        if num_properties.is_negative() {
            return Err(decode::Error::length("negative length decoding PropertyList").into());
        }
        let num_properties = *num_properties as usize;
        let mut properties = Vec::with_capacity(num_properties);
        for _ in 0..num_properties {
            properties.push(Property::decode(decoder)?);
        }
        Ok(Self { properties })
    }
}
