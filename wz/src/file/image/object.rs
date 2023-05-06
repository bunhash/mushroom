//! Object in a WZ image

#[derive(Debug)]
pub enum Object {
    Property,
    Canvas,
    Convex2D,
    Vector2D,
    Uol,
    Sound,
}
