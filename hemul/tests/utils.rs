use hemul::{Byte, cpu::snapshot::Snapshot};
use proptest::prelude::*;

extern crate hemul;

#[allow(dead_code)]
pub fn registers() -> impl Strategy<Value = &'static str> {
    prop_oneof![Just("A"), Just("X"), Just("Y"),]
}

#[allow(dead_code, clippy::missing_panics_doc)]
pub fn register_value(register: &str, snapshot: &Snapshot) -> Byte {
    match register {
        "A" => snapshot.A,
        "X" => snapshot.X,
        "Y" => snapshot.Y,
        _ => panic!("Invalid register"),
    }
}

#[allow(dead_code)]
pub fn as_hex(n: u8) -> String {
    format!("{n:#06x}").replace("0x", "")
}

#[macro_export]
macro_rules! asm_test {
    ($a:expr) => {{
        use hemul::{Resettable, Snapshottable};
        let mut cpu = hemul::asm!($a);
        cpu.reset().expect("Resetting CPU failed");
        let res = cpu.tick_until_nop();
        let snapshot = dbg!(cpu.snapshot());
        assert!(res.is_ok());
        if let Err(ref e) = snapshot {
            assert!(false, "Failed to create snapshot: {}", e);
        }
        snapshot.unwrap()
    }};
}
