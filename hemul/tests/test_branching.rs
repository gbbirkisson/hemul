use proptest::prelude::*;

extern crate hemul;

#[path = "utils.rs"]
mod utils;

proptest! {

    #[test]
    fn test_branching(_ in 0..1) {
        let snapshot = asm_test!(
            r#"
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
}
