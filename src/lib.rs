pub mod cursor;

pub mod prelude {
    pub use super::{
        Down, DownMut, OpaqueVerticalCursor, VerticalCursor, VerticalCursorMut,
    };
}

#[cfg(test)]
mod tests;

pub trait Down {
    fn down(&self, idx: usize) -> Option<&Self>;
}

pub trait DownMut {
    fn down_mut(&mut self, idx: usize) -> Option<&mut Self>;
}

pub trait OpaqueVerticalCursor {
    fn zero(&mut self);
    fn down(&mut self) -> bool;
    fn up(&mut self) -> bool;
}

pub trait VerticalCursor: OpaqueVerticalCursor
where
    Self: Sized,
{
    type Item;

    fn get(&self) -> &Self::Item;
    fn down_new(&mut self) -> Option<Self>;
}

pub trait VerticalCursorMut: VerticalCursor {
    fn get_mut(&mut self) -> &mut Self::Item;
}
