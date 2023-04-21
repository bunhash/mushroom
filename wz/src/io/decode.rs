//! WZ Decode Trait



pub trait Decode<'a>: Sized {
    fn decode<R: Reader<'a>>(decoder: &mut R);

    fn from_file(
}
