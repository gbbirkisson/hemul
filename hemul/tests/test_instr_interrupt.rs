// Testing of interrupt instructions

extern crate hemul;

#[path = "utils.rs"]
mod utils;

#[test]
fn test_instr_interrupt() {
    let snapshot = asm_test!(
        r#"
        ;;
    BRK
    LDX     #$42
    NOP
    .org    $8000
    LDY     #$43
    RTS
    .org    $FFFE
    .word   $8000
        "#
    );
    assert_eq!(snapshot.X, 0x42);
    assert_eq!(snapshot.Y, 0x43);
}
