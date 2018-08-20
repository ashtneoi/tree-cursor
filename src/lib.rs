pub mod cursor;

pub mod prelude {
    pub use super::{Down, DownMut, TakeChild};
}

#[cfg(test)]
mod tests;

pub trait Down {
    fn down(&self, idx: usize) -> Option<&Self>;
}

pub trait DownMut {
    fn down_mut(&mut self, idx: usize) -> Option<&mut Self>;
}

pub trait TakeChild {
    fn take_child(&mut self, idx: usize) -> Self;
}
