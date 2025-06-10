use std::{
    error::Error,
    ops::{Index, IndexMut},
};

pub mod bus;
pub mod cpu;
pub mod memory;
pub mod oscillator;

pub type Word = u16;
pub type Byte = u8;

pub trait Addressable: Index<Word, Output = Byte> + IndexMut<Word, Output = Byte> {
    fn inside_bounds(&self, addr: Word) -> bool;
}

pub trait Tickable {
    fn tick(&mut self) -> Result<(), Box<dyn Error>>;
}

pub trait Resettable {
    fn reset(&mut self) -> Result<(), Box<dyn Error>>;
}

pub type Interrupt = u8;
pub trait Interruptible {
    fn interrupt(&mut self, tp: impl Into<Interrupt>) -> Result<(), Box<dyn Error>>;
}

pub trait Snapshottable {
    type Snapshot;

    fn snapshot(&self) -> Result<Self::Snapshot, Box<dyn Error>>;
}

#[macro_export]
macro_rules! asm {
    ($a:expr) => {{
        use hemul::Resettable;
        let mut cpu = hemul::cpu::Cpu::new(hemul::memory::Memory::from($a));
        let _ = cpu.reset();
        cpu
    }};
}
