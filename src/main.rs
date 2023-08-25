mod cpu;
mod device;
mod memory;
mod prelude;

fn main() {
    let snapshot = asm_test!(
        r#"
    ; 1 + 2
    LDA     #01
    ADC     #02
    STA     $0402
    NOP
        "#
    );
}
