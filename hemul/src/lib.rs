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

pub type ResetError = String;
pub trait Resetable {
    fn reset(&mut self) -> Result<(), ResetError>;
}

pub type Interupt = u8;
pub type InteruptError = String;
pub trait Interuptable {
    fn interupt(&mut self, tp: impl Into<Interupt>) -> Result<(), InteruptError>;
}

pub trait Snapshottable {
    type Snapshot;
    type Error;

    fn snapshot(&self) -> Result<Self::Snapshot, Self::Error>;
}

#[macro_export]
macro_rules! asm {
    ($a:expr) => {{
        use hemul::Resetable;
        let mut cpu = hemul::cpu::Cpu::new(hemul::memory::Memory::from($a));
        let _ = cpu.reset();
        cpu
    }};
}
