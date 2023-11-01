extern crate hemul;

#[path = "utils.rs"]
mod utils;

#[test]
fn test_branching() {
    let snapshot = asm_test!(
        r#"
        ;;
    CLC
    BCC skip1
    LDA #$20
skip1:
    SEC
    BCS skip2
    LDA #$20
skip2:
    NOP
        "#
    );
    assert_eq!(snapshot.A, 0x00);
}
