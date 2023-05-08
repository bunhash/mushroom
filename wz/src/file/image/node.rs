//! Image Node

use crate::{
    file::image::{Canvas, Sound, UolObject, UolString, Vector2D},
    types::{WzInt, WzLong},
};

#[derive(Debug)]
pub enum Node {
    Null,
    Short(i16),
    Int(WzInt),
    Long(WzLong),
    Float(f32),
    Double(f64),
    String(UolString),
    Property,
    Canvas(Canvas),
    Convex2D,
    Vector2D(Vector2D),
    Uol(UolObject),
    Sound(Sound),
}
