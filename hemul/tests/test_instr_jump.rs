// Testing of jump and call operations

extern crate hemul;

#[path = "utils.rs"]
mod utils;

#[test]
fn test_instr_jmp_absolute() {
    let snapshot = asm_test!(
        r#"
        ;;
    JMP $8000
    .org $8000
    LDA #$20
    NOP
        "#
    );
    assert_eq!(snapshot.A, 0x20);
}

#[test]
fn test_instr_jmp_indirect() {
    let snapshot = asm_test!(
        r#"
        ;;
    JMP ($8000)
    .org $8000
    .word $9000
    .org $9000
    LDA #$20
    NOP
        "#
    );
    assert_eq!(snapshot.A, 0x20);
}

#[test]
fn test_instr_jsr_and_rts() {
    let snapshot = asm_test!(
        r#"
        ;;
    JSR setx
    LDY #$22
    NOP
    LDA #$20
setx:
    LDX #$21
    RTS
    NOP
    LDA #$20
        "#
    );
    assert_eq!(snapshot.A, 0x00);
    assert_eq!(snapshot.X, 0x21);
    assert_eq!(snapshot.Y, 0x22);
}
