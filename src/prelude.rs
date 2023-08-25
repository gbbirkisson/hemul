#[macro_export]
macro_rules! asm {
    ($a:expr) => {
        hemul::cpu::Cpu::new(hemul::memory::Memory::from($a))
    };
}

#[macro_export]
macro_rules! asm_test {
    ($a:expr) => {{
        use hemul::device::Tickable;
        let mut cpu = hemul::cpu::Cpu::new(hemul::memory::Memory::from($a));
        cpu.tick_until_nop();
        cpu.tick();
        let snapshot = cpu.snapshot();
        assert!(snapshot.is_some());
        let snapshot = snapshot.unwrap();
        dbg!(snapshot)
    }};
}
