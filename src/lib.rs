use std::ops::{Index, IndexMut};

pub mod cpu;
pub mod memory;

pub type Word = u16;
pub type Byte = u8;

pub trait Addressable: Index<Word, Output = Byte> + IndexMut<Word, Output = Byte> {}

pub trait Tickable {
    type Error;

    fn tick(&mut self) -> Result<(), Self::Error>;
}

pub type Interupt = u8;

pub trait Interuptable {
    type Error;

    fn interupt(&mut self, tp: impl Into<Interupt>) -> Result<(), Self::Error>;
}

#[macro_export]
macro_rules! asm {
    ($a:expr) => {
        hemul::cpu::Cpu::new(hemul::memory::Memory::from($a))
    };
}
