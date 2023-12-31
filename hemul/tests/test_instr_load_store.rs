// Testing of load and store operations. Here we also test access modes extensively

use proptest::prelude::*;
use utils::*;

extern crate hemul;

#[path = "utils.rs"]
mod utils;

fn zn_tests() -> impl Strategy<Value = (u8, bool, bool)> {
    prop_oneof![
        //    Num   Z     N
        Just((0x00, true, false)),
        Just((0x7F, false, false)),
        Just((0xFF, false, true)),
    ]
}

proptest! {
    #[test]
    fn test_instr_load_immediate(reg in registers(), (val, z, n) in zn_tests()) {
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
    fn test_instr_load_zero_page(reg in registers(), (val, z, n) in zn_tests()) {
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
    fn test_instr_load_zero_page_x(reg in registers(), (val, z, n) in zn_tests()) {
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
    fn test_instr_load_zero_page_y((val, z, n) in zn_tests()) {
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
    fn test_instr_load_absolute(reg in registers(), (val, z, n) in zn_tests()) {
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
    fn test_instr_load_absolute_x(reg in registers(), (val, z, n) in zn_tests()) {
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
    fn test_instr_load_absolute_y(reg in registers(), (val, z, n) in zn_tests()) {
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
    fn test_instr_load_indexed_indirect((val, z, n) in zn_tests()) {
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
    fn test_instr_load_indirect_indexed((val, z, n) in zn_tests()) {
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

    #[test]
    fn test_instr_store_zero_page(reg in registers()) {
        let snapshot = asm_test!(
            format!(
                r#"
                ;;
    LD{}    #$66
    ST{}    $88
    NOP
                "#,
                reg,
                reg,
            )
        );
        assert_eq!(snapshot.dump[0x0088], 0x66);
    }

    #[test]
    fn test_instr_store_zero_page_x(reg in registers()) {
        prop_assume!(reg != "X");

        let snapshot = asm_test!(
            format!(
                r#"
                ;;
    LDX     #$05
    LD{}    #$66
    ST{}    $88,X
    NOP
                "#,
                reg,
                reg,
            )
        );
        assert_eq!(snapshot.dump[0x008D], 0x66);
    }

    #[test]
    fn test_instr_store_zero_page_y(reg in registers()) {
        prop_assume!(reg == "X");

        let snapshot = asm_test!(
            format!(
                r#"
                ;;
    LDY     #$0A
    LDX     #$66
    STX     $88,Y
    NOP
                "#,
            )
        );
        assert_eq!(snapshot.dump[0x0092], 0x66);
    }

    #[test]
    fn test_instr_store_absolute(reg in registers()) {
        let snapshot = asm_test!(
            format!(
                r#"
                ;;
    LD{}    #$66
    ST{}    $1234
    NOP
                "#,
                reg,
                reg,
            )
        );
        assert_eq!(snapshot.dump[0x1234], 0x66);
    }

    #[test]
    fn test_instr_store_absolute_xy(reg in registers()) {
        prop_assume!(reg != "A");
        let snapshot = asm_test!(
            format!(
                r#"
                ;;
    LD{}    #$20
    LDA     #$66
    STA     $2000,{}
    NOP
                "#,
                reg,
                reg,
            )
        );
        assert_eq!(snapshot.dump[0x2020], 0x66);
    }

    #[test]
    fn test_instr_store_indexed_indirect(reg in registers()) {
        prop_assume!(reg == "A");
        let snapshot = asm_test!(
                r#"
                ;;
    LDX     #$20
    LDA     #$66
    STA     ($20,X)
    NOP
    .org    $0040
    .word   $1234
                "#
        );
        assert_eq!(snapshot.dump[0x1234], 0x66);
    }

    #[test]
    fn test_instr_store_indirect_indexed(reg in registers()) {
        prop_assume!(reg == "A");
        let snapshot = asm_test!(
                r#"
                ;;
    LDY     #$20
    LDA     #$66
    STA     ($20),Y
    NOP
    .org    $0020
    .word   $1234
                "#
        );
        assert_eq!(snapshot.dump[0x1254], 0x66);
    }
}
