use std::ops::{Index, IndexMut};

pub mod cpu;
pub mod memory;
pub mod prelude;

pub type Word = u16;
pub type Byte = u8;

#[derive(Debug, Clone)]
pub enum Address {
    Short(Byte),
    Full(Byte, Byte),
}

pub trait Addressable: Index<Word, Output = Byte> + IndexMut<Word, Output = Byte> {}

pub trait Tickable {
    fn tick(&mut self);
}

pub type InteruptType = u8;

pub trait Interuptable {
    fn interupt(&mut self, t: InteruptType);
}

#[macro_export]
macro_rules! asm {
    ($a:expr) => {
        hemul::cpu::Cpu::new(hemul::memory::Memory::from($a))
    };
}

#[macro_export]
macro_rules! asm_test {
    ($a:expr) => {{
        use $crate::Tickable;
        let mut cpu = hemul::cpu::Cpu::new(hemul::memory::Memory::from($a));
        cpu.tick_until_nop();
        cpu.tick();
        let snapshot = cpu.snapshot();
        assert!(snapshot.is_some());
        let snapshot = snapshot.unwrap();
        dbg!(snapshot)
    }};
}
