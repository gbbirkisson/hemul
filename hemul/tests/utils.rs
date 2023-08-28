extern crate hemul;

#[macro_export]
macro_rules! asm_test {
    ($a:expr) => {{
        use hemul::Snapshottable;
        let mut cpu = hemul::asm!($a);
        let res = cpu.tick_until_nop();
        assert_eq!(res, Ok(()));
        let snapshot = cpu.snapshot();
        if let Err(ref e) = snapshot {
            assert!(false, "Failed to create snapshot: {}", e);
        }
        let snapshot = snapshot.unwrap();
        dbg!(snapshot)
    }};
}
