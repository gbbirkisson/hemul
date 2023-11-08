// Testing of branches and status flag changes

use proptest::prelude::*;
use utils::*;

extern crate hemul;

#[path = "utils.rs"]
mod utils;

pub fn cases() -> impl Strategy<Value = (&'static str, u8, bool)> {
    prop_oneof![
        //    OP     S: CZI DBVN, Branch?
        Just(("BCC", 0b1011_0111, true)),
        Just(("BCC", 0b0100_0000, false)),
        Just(("BCS", 0b1011_0111, false)),
        Just(("BCS", 0b0100_0000, true)),
        Just(("BEQ", 0b1101_0111, false)),
        Just(("BEQ", 0b0010_0000, true)),
        Just(("BMI", 0b1111_0110, false)),
        Just(("BMI", 0b0000_0001, true)),
        Just(("BNE", 0b1101_0111, true)),
        Just(("BNE", 0b0010_0000, false)),
        Just(("BPL", 0b1111_0110, true)),
        Just(("BPL", 0b0000_0001, false)),
        Just(("BVC", 0b1111_0101, true)),
        Just(("BVC", 0b0000_0010, false)),
        Just(("BVS", 0b1111_0101, false)),
        Just(("BVS", 0b0000_0010, true)),
    ]
}

proptest! {
    #[test]
    fn test_instr_branch((op, status, branch) in cases()) {
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    LDA     $2000
    PHA
    PLP
    {}      branch
    LDX     #$02
    NOP
branch:
    LDX     #$01
    NOP
    .org    $2000
    .word   ${}
            "#,
            op,
            as_hex(status),
            )
        );
        assert_eq!(snapshot.X, if branch { 0x01 } else { 0x02 })
    }
}

#[test]
fn test_instr_flag_clc() {
    let snapshot = asm_test!(
        r#"
        ;;
    LDA #%11110111
    PHA
    PLP
    CLC
    NOP
        "#
    );
    assert!(!snapshot.C);
}

#[test]
fn test_instr_flag_cli() {
    let snapshot = asm_test!(
        r#"
        ;;
    LDA #%11110111
    PHA
    PLP
    CLI
    NOP
        "#
    );
    assert!(!snapshot.I);
}

#[test]
fn test_instr_flag_clv() {
    let snapshot = asm_test!(
        r#"
        ;;
    LDA #%11110111
    PHA
    PLP
    CLV
    NOP
        "#
    );
    assert!(!snapshot.V);
}

#[test]
fn test_instr_flag_sec() {
    let snapshot = asm_test!(
        r#"
        ;;
    LDA #%00000000
    PHA
    PLP
    SEC
    NOP
        "#
    );
    assert!(snapshot.C);
}

#[test]
fn test_instr_flag_sei() {
    let snapshot = asm_test!(
        r#"
        ;;
    LDA #%00000000
    PHA
    PLP
    SEI
    NOP
        "#
    );
    assert!(snapshot.I);
}
