extern crate hemul;

use hemul::asm_test;

#[test]
fn simple_addition() {
    let snapshot = asm_test!(
        r#"
    ; 1 + 2
    LDA     #01
    ADC     #02
    STA     $0402
    NOP
        "#
    );
    assert_eq!(snapshot.dump[0x0402], 3);
}
