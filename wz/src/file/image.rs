//! WZ Image

use crate::types::{Uol, WzInt, WzLong};

mod content;
mod object;
mod property;

pub use content::ContentRef;
pub use object::Object;
pub use property::Property;

pub enum Node {
    Null,
    Short(i16),
    Int(WzInt),
    Long(WzLong),
    Float(f32),
    Double(f64),
    String(Uol),
    Object { size: i32 },
}

pub struct Image {}
