// Testing of logical operations

use proptest::prelude::*;
use utils::*;

extern crate hemul;

#[path = "utils.rs"]
mod utils;

proptest! {
    #[test]
    fn test_instr_logical_and(a in 0..255u8, b in 0..255u8) {
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    LDA     $2000
    AND     $2002
    NOP
    .org    $2000
    .word   ${}
    .word   ${}
            "#,
            as_hex(a),
            as_hex(b)
            )
        );
        assert_eq!(snapshot.A, a & b);
        assert_eq!(snapshot.Z, a & b == 0);
        assert_eq!(snapshot.N, (a & b) >> 7 == 1);
    }

    #[test]
    fn test_instr_logical_eor(a in 0..255u8, b in 0..255u8) {
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    LDA     $2000
    EOR     $2002
    NOP
    .org    $2000
    .word   ${}
    .word   ${}
            "#,
            as_hex(a),
            as_hex(b)
            )
        );
        assert_eq!(snapshot.A, a ^ b);
        assert_eq!(snapshot.Z, a ^ b == 0);
        assert_eq!(snapshot.N, (a ^ b) >> 7 == 1);
    }

    #[test]
    fn test_instr_logical_ora(a in 0..255u8, b in 0..255u8) {
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    LDA     $2000
    ORA     $2002
    NOP
    .org    $2000
    .word   ${}
    .word   ${}
            "#,
            as_hex(a),
            as_hex(b)
            )
        );
        assert_eq!(snapshot.A, a | b);
        assert_eq!(snapshot.Z, a | b == 0);
        assert_eq!(snapshot.N, (a | b) >> 7 == 1);
    }

    #[test]
    fn test_instr_logical_bit(a in 0..255u8, b in 0..255u8) {
        let snapshot = asm_test!(
            format!(
            r#"
            ;;
    LDA     $2000
    BIT     $2002
    NOP
    .org    $2000
    .word   ${}
    .word   ${}
            "#,
            as_hex(a),
            as_hex(b)
            )
        );
        assert_eq!(snapshot.A, a);
        assert_eq!(snapshot.Z, a & b == 0);
        assert_eq!(snapshot.N, b >> 7 == 1);
        assert_eq!(snapshot.V, b & 0b0100_0000 > 0);
    }
}
