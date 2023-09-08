use proptest::prelude::*;

extern crate hemul;

#[path = "utils.rs"]
mod utils;

proptest! {

    #[test]
    fn test_add_example(_ in 0..1) {
        let snapshot = asm_test!(
            r#"
LOWBYTE  = $4000
HIGHBYTE = $4001
    LDA #$F0      ; A=$F0
    CLC           ; C=0
    ADC #$20      ; Result is $F0+$20+C = $110; therefore A=$10 and C=1
    STA LOWBYTE
    LDA #$30      ; A=$30
    ADC #$01      ; Value is $30+$01+C = $32; therefore A=$32 and C=0
    STA HIGHBYTE
    NOP
            "#
        );
        assert_eq!(snapshot.dump[0x04000], 0x10);
        assert_eq!(snapshot.dump[0x04001], 0x32);
        assert!(!snapshot.C);
    }

    #[test]
    fn test_add_without_carry(a in 0..=255u8, b in 0..=255u8) {
        let snapshot = asm_test!(
            format!(
                r#"
RESULT  = $4000
    LDA     #%{:0>8b}
    ADC     #%{:0>8b}
    STA     RESULT
    NOP
            "#,
                a, b
            )
        );
        assert_eq!((snapshot.dump[0x04000], snapshot.C), a.overflowing_add(b));
    }

    #[test]
    fn test_add_with_carry(a in 0..=255u8, b in 0..=255u8) {
        let snapshot = asm_test!(
            format!(
                r#"
RESULT  = $4000
    SEC
    LDA     #%{:0>8b}
    ADC     #%{:0>8b}
    STA     RESULT
    NOP
            "#,
                a, b
            )
        );
        let (sum, carry1) = a.overflowing_add(b);
        let (sum, carry2) = sum.overflowing_add(1);
        assert_eq!((snapshot.dump[0x04000], snapshot.C), (sum, carry1 || carry2));
    }
}
