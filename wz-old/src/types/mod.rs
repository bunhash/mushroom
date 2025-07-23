//! WZ types

use std::io;

mod canvas;
mod header;
mod int;
mod offset;
mod primitives;
mod property;
mod sound;
mod string;
mod uol;
mod vector;

pub(crate) mod macros;
pub(crate) mod raw;

pub use canvas::{Canvas, CanvasFormat};
pub use header::WzHeader;
pub use int::{WzInt, WzLong};
pub use offset::WzOffset;
pub use property::Property;
pub use sound::{Sound, SoundHeader, WavHeader};
pub use uol::{UolObject, UolString};
pub use vector::Vector;

pub trait VerboseDebug {
    fn debug(&self, f: &mut dyn io::Write) -> io::Result<()>;
}
