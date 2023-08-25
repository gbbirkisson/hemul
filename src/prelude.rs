#[macro_export]
macro_rules! asm {
    ($a:expr) => {
        crate::cpu::Cpu::new(crate::memory::Memory::from($a))
    };
}

#[macro_export]
macro_rules! asm_test {
    ($a:expr) => {{
        use device::Tickable;
        let mut cpu = asm!($a);
        cpu.tick_until_nop();
        cpu.tick();
        cpu.snapshot()
    }};
}
