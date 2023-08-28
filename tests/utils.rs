#[macro_export]
macro_rules! asm_test {
    ($a:expr) => {{
        let mut cpu = hemul::asm!($a);
        let res = cpu.tick_until_nop();
        assert_eq!(res, Ok(()));
        let snapshot = cpu.snapshot();
        assert!(snapshot.is_some());
        let snapshot = snapshot.unwrap();
        dbg!(snapshot)
    }};
}
