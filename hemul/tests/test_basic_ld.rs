use hemul::{cpu::snapshot::Snapshot, Byte};
use proptest::prelude::*;

extern crate hemul;

#[path = "utils.rs"]
mod utils;

fn registers() -> impl Strategy<Value = &'static str> {
    prop_oneof![Just("A"), Just("X"), Just("Y"),]
}

fn zn_tests() -> impl Strategy<Value = (u8, bool, bool)> {
    prop_oneof![
        //    Num   Z     N
        Just((0x00, true, false)),
        Just((0x7F, false, false)),
        Just((0xFF, false, true)),
    ]
}

fn register_value(register: &str, snapshot: &Snapshot) -> Byte {
    match register {
        "A" => snapshot.A,
        "X" => snapshot.X,
        "Y" => snapshot.Y,
        _ => panic!("Invalid register"),
    }
}

fn as_hex(n: u8) -> String {
    format!("{:#06x}", n).replace("0x", "")
}

proptest! {
    #[test]
    fn test_basic_ld_immediate(reg in registers(), (val, z, n) in zn_tests()) {
        let snapshot = asm_test!(
            format!(
                r#"
                ;;
    LD{}    #%{:0>8b}
    NOP
                "#,
                reg,
                val
            )
        );
        assert_eq!(register_value(reg, &snapshot), val);
        assert_eq!(snapshot.Z, z);
        assert_eq!(snapshot.N, n);
    }

    #[test]
    fn test_basic_ld_zero_page(reg in registers(), (val, z, n) in zn_tests()) {
        let snapshot = asm_test!(
            format!(
                r#"
                ;;
    LD{}    $88
    NOP
    .org    $0088
    .word   ${}
                "#,
                reg,
                as_hex(val),
            )
        );
        assert_eq!(register_value(reg, &snapshot), val);
        assert_eq!(snapshot.Z, z);
        assert_eq!(snapshot.N, n);
    }

    #[test]
    fn test_basic_ld_zero_page_x(reg in registers(), (val, z, n) in zn_tests()) {
        prop_assume!(reg != "X");

        let snapshot = asm_test!(
            format!(
                r#"
                ;;
    LDX     #$05
    LD{}    $88,X
    NOP
    .org    $008D
    .word   ${}
                "#,
                reg,
                as_hex(val),
            )
        );
        assert_eq!(register_value(reg, &snapshot), val);
        assert_eq!(snapshot.Z, z);
        assert_eq!(snapshot.N, n);
    }

    #[test]
    fn test_basic_ld_zero_page_y((val, z, n) in zn_tests()) {
        let snapshot = asm_test!(
            format!(
                r#"
                ;;
    LDY     #$FF
    LDX     $88,Y
    NOP
    .org    $0087
    .word   ${}
                "#,
                as_hex(val),
            )
        );
        assert_eq!(snapshot.X, val);
        assert_eq!(snapshot.Z, z);
        assert_eq!(snapshot.N, n);
    }

    #[test]
    fn test_basic_ld_absolute(reg in registers(), (val, z, n) in zn_tests()) {
        let snapshot = asm_test!(
            format!(
                r#"
                ;;
    LD{}    $1234
    NOP
    .org    $1234
    .word   ${}
                "#,
                reg,
                as_hex(val),
            )
        );
        assert_eq!(register_value(reg, &snapshot), val);
        assert_eq!(snapshot.Z, z);
        assert_eq!(snapshot.N, n);
    }

    #[test]
    fn test_basic_ld_absolute_x(reg in registers(), (val, z, n) in zn_tests()) {
        prop_assume!(reg != "X");

        let snapshot = asm_test!(
            format!(
                r#"
                ;;
    LDX     #$F6
    LD{}    $1234,X
    NOP
    .org    $132A
    .word   ${}
                "#,
                reg,
                as_hex(val),
            )
        );
        assert_eq!(register_value(reg, &snapshot), val);
        assert_eq!(snapshot.Z, z);
        assert_eq!(snapshot.N, n);
    }

    #[test]
    fn test_basic_ld_absolute_y(reg in registers(), (val, z, n) in zn_tests()) {
        prop_assume!(reg != "Y");

        let snapshot = asm_test!(
            format!(
                r#"
                ;;
    LDY     #$56
    LD{}    $8245,Y
    NOP
    .org    $829B
    .word   ${}
                "#,
                reg,
                as_hex(val),
            )
        );
        assert_eq!(register_value(reg, &snapshot), val);
        assert_eq!(snapshot.Z, z);
        assert_eq!(snapshot.N, n);
    }

    #[test]
    fn test_basic_ld_indexed_indirect((val, z, n) in zn_tests()) {
        let snapshot = asm_test!(
            format!(
                r#"
                ;;
    LDA     ($40,X)
    NOP
    .org    $0040
    .word   $1234
    .org    $1234
    .word   ${}
                "#,
                as_hex(val),
            )
        );
        assert_eq!(snapshot.A, val);
        assert_eq!(snapshot.Z, z);
        assert_eq!(snapshot.N, n);
    }

    #[test]
    fn test_basic_ld_indirect_indexed((val, z, n) in zn_tests()) {
        let snapshot = asm_test!(
            format!(
                r#"
                ;;
    LDY     #$10
    LDA     ($86),Y
    NOP
    .org    $0086
    .word   $4028
    .org    $4038
    .word   ${}
                "#,
                as_hex(val),
            )
        );
        assert_eq!(snapshot.A, val);
        assert_eq!(snapshot.Z, z);
        assert_eq!(snapshot.N, n);
    }
}
