use std::ops::{Index, IndexMut};

pub mod bus;
pub mod cpu;
pub mod memory;
pub mod oscillator;

pub type Word = u16;
pub type Byte = u8;

pub trait Addressable: Index<Word, Output = Byte> + IndexMut<Word, Output = Byte> {
    fn inside_bounds(&self, addr: Word) -> bool;
}

pub type TickError = String;

pub trait Tickable {
    fn tick(&mut self) -> Result<(), TickError>;
}

pub type Interupt = u8;

pub trait Interuptable {
    type Error;

    fn interupt(&mut self, tp: impl Into<Interupt>) -> Result<(), Self::Error>;
}

pub trait Snapshottable {
    type Snapshot;
    type Error;

    fn snapshot(&self) -> Result<Self::Snapshot, Self::Error>;
}

#[macro_export]
macro_rules! asm {
    ($a:expr) => {
        hemul::cpu::Cpu::new(hemul::memory::Memory::from($a))
    };
}
