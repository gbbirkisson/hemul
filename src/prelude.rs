#[macro_export]
macro_rules! asm {
    ($a:expr) => {
        Cpu::new(Memory::from($a))
    };
}
