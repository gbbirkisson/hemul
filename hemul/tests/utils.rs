extern crate hemul;

#[macro_export]
macro_rules! asm_test {
    ($a:expr) => {{
        use hemul::Snapshottable;
        let mut cpu = hemul::asm!($a);
        let res = cpu.tick_until_nop();
        let snapshot = dbg!(cpu.snapshot());
        assert_eq!(res, Ok(()));
        if let Err(ref e) = snapshot {
            assert!(false, "Failed to create snapshot: {}", e);
        }
        snapshot.unwrap()
    }};
}
