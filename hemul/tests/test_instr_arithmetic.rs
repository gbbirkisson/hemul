// Testing of arithmetic operations

use proptest::prelude::*;
use utils::*;

extern crate hemul;

#[path = "utils.rs"]
mod utils;

proptest! {
    #[test]
    fn test_instr_arithmetic_adc_no_carry(a in 0..255u8, b in 0..255u8) {
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    LDA     $2000
    ADC     $2002
    NOP
    .org    $2000
    .word   ${}
    .word   ${}
            "#,
            as_hex(a),
            as_hex(b)
            )
        );
        let (res, carry) = a.overflowing_add(b);
        assert_eq!(snapshot.A, res);
        assert_eq!(snapshot.Z, res == 0);
        assert_eq!(snapshot.N, res >> 7 == 1);
        assert_eq!(snapshot.C, carry);
    }

    #[test]
    fn test_instr_arithmetic_adc_with_carry(a in 0..255u8, b in 0..255u8) {
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    SEC
    LDA     $2000
    ADC     $2002
    NOP
    .org    $2000
    .word   ${}
    .word   ${}
            "#,
            as_hex(a),
            as_hex(b)
            )
        );
        let (res_almost, carry1) = a.overflowing_add(b);
        let (res, carry2) = res_almost.overflowing_add(1);
        assert_eq!(snapshot.A, res);
        assert_eq!(snapshot.Z, res == 0);
        assert_eq!(snapshot.N, res >> 7 == 1);
        assert_eq!(snapshot.C, carry1 | carry2);
    }

    #[test]
    fn test_instr_arithmetic_sbc_no_carry(a in 0..255u8, b in 0..255u8) {
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    LDA     $2000
    SBC     $2002
    NOP
    .org    $2000
    .word   ${}
    .word   ${}
            "#,
            as_hex(a),
            as_hex(b)
            )
        );
        let (res_almost, carry1) = a.overflowing_sub(b);
        let (res, carry2) = res_almost.overflowing_sub(1);
        assert_eq!(snapshot.A, res);
        assert_eq!(snapshot.Z, res == 0);
        assert_eq!(snapshot.N, res >> 7 == 1);
        assert_eq!(snapshot.C, !(carry1 | carry2));
    }

    #[test]
    fn test_instr_arithmetic_sbc_with_carry(a in 0..255u8, b in 0..255u8) {
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    SEC
    LDA     $2000
    SBC     $2002
    NOP
    .org    $2000
    .word   ${}
    .word   ${}
            "#,
            as_hex(a),
            as_hex(b)
            )
        );
        let (res, carry) = a.overflowing_sub(b);
        assert_eq!(snapshot.A, res);
        assert_eq!(snapshot.Z, res == 0);
        assert_eq!(snapshot.N, res >> 7 == 1);
        assert_eq!(snapshot.C, !carry);
    }

    #[test]
    fn test_instr_arithmetic_compare(reg in registers(), a in 0..255u8, b in 0..255u8) {
        let instr = match reg {
            "A" => "CMP",
            "X" => "CPX",
            "Y" => "CPY",
            _ => unreachable!(),
        };
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    LD{}    $2000
    {}      $2002
    NOP
    .org    $2000
    .word   ${}
    .word   ${}
            "#,
            reg,
            instr,
            as_hex(a),
            as_hex(b)
            )
        );
        assert_eq!(register_value(reg, &snapshot), a);
        assert_eq!(snapshot.Z, a == b);
        assert_eq!(snapshot.N, a.wrapping_sub(b) >> 7 == 1);
        assert_eq!(snapshot.C, a >= b);
    }

    #[test]
    fn test_instr_arithmetic_inc_memory(a in 0..255u8) {
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    INC     $2000
    NOP
    .org    $2000
    .word   ${}
            "#,
            as_hex(a),
            )
        );

        let res = a.wrapping_add(1);
        assert_eq!(snapshot.dump[0x2000], res);
        assert_eq!(snapshot.Z, res == 0);
        assert_eq!(snapshot.N, res >> 7 == 1);
    }

    #[test]
    fn test_instr_arithmetic_inc_registers(reg in registers(), a in 0..255u8) {
        prop_assume!(reg != "A");
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    LD{}    $2000
    IN{}
    NOP
    .org    $2000
    .word   ${}
            "#,
            reg,
            reg,
            as_hex(a),
            )
        );

        let res = a.wrapping_add(1);
        assert_eq!(register_value(reg, &snapshot), res);
        assert_eq!(snapshot.Z, res == 0);
        assert_eq!(snapshot.N, res >> 7 == 1);
    }

    #[test]
    fn test_instr_arithmetic_dec_memory(a in 0..255u8) {
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    DEC     $2000
    NOP
    .org    $2000
    .word   ${}
            "#,
            as_hex(a),
            )
        );

        let res = a.wrapping_sub(1);
        assert_eq!(snapshot.dump[0x2000], res);
        assert_eq!(snapshot.Z, res == 0);
        assert_eq!(snapshot.N, res >> 7 == 1);
    }

    #[test]
    fn test_instr_arithmetic_dec_registers(reg in registers(), a in 0..255u8) {
        prop_assume!(reg != "A");
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    LD{}    $2000
    DE{}
    NOP
    .org    $2000
    .word   ${}
            "#,
            reg,
            reg,
            as_hex(a),
            )
        );

        let res = a.wrapping_sub(1);
        assert_eq!(register_value(reg, &snapshot), res);
        assert_eq!(snapshot.Z, res == 0);
        assert_eq!(snapshot.N, res >> 7 == 1);
    }
}
