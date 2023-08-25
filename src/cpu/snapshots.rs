use super::{PFlag, Reg16, Reg8};

#[allow(non_snake_case, dead_code)]
#[derive(Debug)]
pub struct Snapshot {
    pub dump: Vec<u8>,

    pub PC: Reg16, // Program Counter
    pub SP: Reg16, // Stack Pointer

    pub A: Reg8, // Accumulator
    pub X: Reg8, // Index Register X
    pub Y: Reg8, // Index Register Y

    pub C: PFlag, // Carry Flag
    pub Z: PFlag, // Zero Flag
    pub I: PFlag, // Interupt Disable
    pub D: PFlag, // Decimal Mode
    pub B: PFlag, // Break Command
    pub V: PFlag, // Overflow Flag
    pub N: PFlag, // Negative Flag
}
