use std::ops::{Index, IndexMut};

pub trait Addressable: Index<u16, Output = u8> + IndexMut<u16, Output = u8> {}

pub trait Tickable {
    fn tick(&mut self);
}
