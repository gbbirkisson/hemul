// Testing of register transfers and stack operations

use proptest::prelude::*;
use utils::*;

extern crate hemul;

#[path = "utils.rs"]
mod utils;

proptest! {
    #[test]
    fn test_trans_a_to_r(reg in registers()) {
        prop_assume!(reg != "A");

        let snapshot = asm_test!(
            format!(
                r#"
                ;;
    LDA     #$88
    TA{}
    NOP
                "#,
                reg,
            )
        );
        assert_eq!(register_value("A", &snapshot), 0x88);
        assert_eq!(register_value(reg, &snapshot), 0x88);
    }


    #[test]
    fn test_trans_r_to_a(reg in registers()) {
        prop_assume!(reg != "A");

        let snapshot = asm_test!(
            format!(
                r#"
                ;;
    LD{}    #$77
    T{}A
    NOP
                "#,
                reg,
                reg,
            )
        );
        assert_eq!(register_value("A", &snapshot), 0x77);
        assert_eq!(register_value(reg, &snapshot), 0x77);
    }
}

#[test]
fn test_trans_s_to_x() {
    let snapshot = asm_test!(
        r#"
        ;;
    TSX
    NOP
        "#
    );
    assert_eq!(snapshot.X, 0xFF);
}

#[test]
fn test_trans_x_to_s() {
    let snapshot = asm_test!(
        r#"
        ;;
    LDX #$55
    TXS
    NOP
        "#
    );
    assert_eq!(snapshot.SP, 0x55);
}

#[test]
fn test_stack_push_a() {
    let snapshot = asm_test!(
        r#"
        ;;
    LDA #$DE
    PHA
    NOP
        "#
    );
    assert_eq!(snapshot.dump[0x01FF], 0xDE);
}

#[test]
fn test_stack_push_s() {
    let snapshot = asm_test!(
        r#"
        ;;
    LDA #$FF
    PHP
    NOP
        "#
    );
    //                                   CZIDBVN
    assert_eq!(snapshot.dump[0x01FF], 0b00000001);
}

#[test]
fn test_stack_pull_a() {
    let snapshot = asm_test!(
        r#"
        ;;
    PLA
    NOP
    .org    $0100
    .word   $1234
        "#
    );
    assert_eq!(snapshot.A, 0x34);
}

#[test]
fn test_stack_pull_s() {
    let snapshot = asm_test!(
        r#"
        ;;
    PLP
    NOP
    .org    $0100
    .word   $FFFF
        "#
    );
    assert!(snapshot.C);
    assert!(snapshot.Z);
    assert!(snapshot.I);
    assert!(snapshot.D);
    assert!(snapshot.B);
    assert!(snapshot.V);
    assert!(snapshot.N);
}
