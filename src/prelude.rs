#[macro_export]
macro_rules! asm {
    ($a:expr) => {
        crate::cpu::Cpu::new(crate::memory::Memory::from($a))
    };
}
