//! WZ Image content

use crate::{
    decode::{self, Decode, Decoder},
    encode::{Encode, Encoder, SizeHint},
    image::{Error, UolString, Value},
    Int32,
};

/// Image property. This is just a name/value pair
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Content {
    /// Name of the property
    pub name: UolString,

    /// Value of the property
    pub value: Value,
}

impl Decode for Content {
    type Error = Error;

    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Self::Error> {
        Ok(Content {
            name: UolString::decode(decoder)?,
            value: Value::decode(decoder)?,
        })
    }
}

impl Encode for Content {
    type Error = Error;

    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Self::Error> {
        self.name.encode(encoder)?;
        self.value.encode(encoder)?;
        Ok(())
    }
}

impl SizeHint for Content {
    fn size_hint(&self) -> u64 {
        self.name.size_hint() + self.value.size_hint()
    }
}
