//! Map Content Trait

pub trait Content<T>: SizeHint
where
    T: SizeHint,
{
    fn new(name: &str, data: T) -> Self;

    fn name(&self) -> &str;

    fn rename(&mut self, name: &str);

    fn update(&self, cursor: Cursor<Self, T>) -> Self;
}
