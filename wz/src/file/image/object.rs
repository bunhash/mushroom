//! WZ Property Object
//!

/// These are just complex structures compared to the primitive values contained in WZ properties
pub enum Object {
    /// Contains an embedded list of properties
    Property,

    /// Canvas type
    Canvas,

    /// Shape2D#Convex2D
    Convex2D,

    /// Shape2D#Vector2D
    Vector2D,

    /// UOL object
    Uol,

    /// Sound_DX8
    Sound,
}
