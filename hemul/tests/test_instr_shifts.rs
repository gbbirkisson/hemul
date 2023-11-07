// Testing of logical operations

use proptest::prelude::*;
use utils::*;

extern crate hemul;

#[path = "utils.rs"]
mod utils;

pub fn left() -> impl Strategy<Value = &'static str> {
    prop_oneof![Just("ASL"), Just("ROL")]
}

pub fn right() -> impl Strategy<Value = &'static str> {
    prop_oneof![Just("LSR"), Just("ROR")]
}

pub fn addr_mode() -> impl Strategy<Value = &'static str> {
    prop_oneof![Just("A"), Just("$2000")]
}

proptest! {
    #[test]
    fn test_instr_shifts_left_accumulator(op in left(), mode in addr_mode(), a in 0..255u8) {
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    LDA     $2000
    {}      {}
    NOP
    .org    $2000
    .word   ${}
            "#,
            op,
            mode,
            as_hex(a),
            )
        );
        let res = a << 1;
        if mode == "A" {
            assert_eq!(snapshot.A, res);
        } else {
            assert_eq!(snapshot.dump[0x2000], res);
        }
        assert_eq!(snapshot.Z, res == 0);
        assert_eq!(snapshot.N, res >> 7 == 1);
        assert_eq!(snapshot.C, a >> 7 == 1);
    }

    #[test]
    fn test_instr_shifts_right_accumulator(op in right(), mode in addr_mode(), a in 0..255u8) {
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    LDA     $2000
    {}      {}
    NOP
    .org    $2000
    .word   ${}
            "#,
            op,
            mode,
            as_hex(a),
            )
        );
        let res = a >> 1;
        if mode == "A" {
            assert_eq!(snapshot.A, res);
        } else {
            assert_eq!(snapshot.dump[0x2000], res);
        }
        assert_eq!(snapshot.Z, res == 0);
        assert_eq!(snapshot.N, res >> 7 == 1);
        assert_eq!(snapshot.C, a & 0b0000_0001 == 1);
    }

    #[test]
    fn test_instr_shifts_rol_with_carry(a in 0..255u8) {
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    SEC
    LDA     $2000
    ROL     A
    NOP
    .org    $2000
    .word   ${}
            "#,
            as_hex(a),
            )
        );
        let res = a << 1 | 0b0000_0001;
        assert_eq!(snapshot.A, res);
        assert_eq!(snapshot.Z, res == 0);
        assert_eq!(snapshot.N, res >> 7 == 1);
        assert_eq!(snapshot.C, a >> 7 == 1);
    }

    #[test]
    fn test_instr_shifts_ror_with_carry(a in 0..255u8) {
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    SEC
    LDA     $2000
    ROR     A
    NOP
    .org    $2000
    .word   ${}
            "#,
            as_hex(a),
            )
        );
        let res = a >> 1 | 0b1000_0000;
        assert_eq!(snapshot.A, res);
        assert_eq!(snapshot.Z, res == 0);
        assert_eq!(snapshot.N, res >> 7 == 1);
        assert_eq!(snapshot.C, a & 0b0000_0001 == 1);
    }
}
