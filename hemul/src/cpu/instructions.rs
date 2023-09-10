use crate::{
    cpu::{Addressable, Cpu, Error},
    Byte, Word,
};

use super::IRQB;

pub trait OpParser {
    /// Parses a byte into an tuple that represents the `OpCode` and number of cycles that
    /// instruction takes to execute
    fn parse(&self, value: Byte) -> Result<(Op, u8), Error>;
}

/// The 6502 instruction set
#[derive(Debug, Clone)]
pub enum Op {
    // Load/Store Operations
    // These instructions transfer a single byte between memory and one of the registers. Load
    // operations set the negative (N) and zero (Z) flags depending on the value of transferred.
    // Store operations do not affect the flag settings.
    /// LDA - Load Accumulator
    /// ```text
    /// A,Z,N = M
    /// ```
    /// Loads a byte of memory into the accumulator setting the zero and negative flags as
    /// appropriate.
    Lda(AddressMode),

    /// LDX - Load X Register
    /// ```text
    /// X,Z,N = M
    /// ```
    /// Loads a byte of memory into the X register setting the zero and negative flags as
    /// appropriate.
    Ldx(AddressMode),

    /// LDY - Load Y Register
    /// ```text
    /// Y,Z,N = M
    /// ```
    /// Loads a byte of memory into the Y register setting the zero and negative flags as
    /// appropriate.
    Ldy(AddressMode),

    /// STA - Store Accumulator
    /// ```text
    /// M = A
    /// ```
    /// Stores the contents of the accumulator into memory.
    Sta(AddressMode),

    /// STX - Store X Register
    /// ```text
    /// M = X
    /// ```
    /// Stores the contents of the X register into memory.
    Stx(AddressMode),

    /// STY - Store Y Register
    /// ```text
    /// M = Y
    /// ```
    /// Stores the contents of the Y register into memory.
    Sty(AddressMode),

    // Register Transfers
    // The contents of the X and Y registers can be moved to or from the accumulator, setting the
    // negative (N) and zero (Z) flags as appropriate.
    /// TAX - Transfer Accumulator to X
    /// ```text
    /// X = A
    /// ```
    /// Copies the current contents of the accumulator into the X register and sets the zero and
    /// negative flags as appropriate.
    Tax,

    /// TAY - Transfer Accumulator to Y
    /// ```text
    /// Y = A
    /// ```
    /// Copies the current contents of the accumulator into the Y register and sets the zero and
    /// negative flags as appropriate.
    Tay,

    /// TXA - Transfer X to Accumulator
    /// ```text
    /// A = X
    /// ```
    /// Copies the current contents of the X register into the accumulator and sets the zero and
    /// negative flags as appropriate.
    Txa,

    /// TYA - Transfer Y to Accumulator
    /// ```text
    /// A = X
    /// ```
    /// Copies the current contents of the Y register into the accumulator and sets the zero and
    /// negative flags as appropriate.
    Tya,

    // Stack Operations
    // The 6502 microprocessor supports a 256 byte stack fixed between memory locations $0100 and
    // $01FF. A special 8-bit register, S, is used to keep track of the next free byte of stack
    // space. Pushing a byte on to the stack causes the value to be stored at the current free
    // location (e.g. $0100,S) and then the stack pointer is post decremented. Pull operations
    // reverse this procedure. The stack register can only be accessed by transferring its
    // value to or from the X register. Its value is automatically modified by push/pull
    // instructions, subroutine calls and returns, interrupts and returns from interrupts.
    /// TSX - Transfer Stack Pointer to X
    /// ```text
    /// X = S
    /// ```
    /// Copies the current contents of the stack register into the X register and sets the zero and
    /// negative flags as appropriate.
    Tsx,

    /// TXS - Transfer X to Stack Pointer
    /// ```text
    /// S = X
    /// ```
    /// Copies the current contents of the X register into the stack register.
    Txs,

    /// PHA - Push Accumulator
    /// Pushes a copy of the accumulator on to the stack
    Pha,

    /// PHP - Push Processor Status
    /// Pushes a copy of the status flags on to the stack.
    Php,

    /// PLA - Pull Accumulator
    /// Pulls an 8 bit value from the stack and into the accumulator. The zero and negative flags
    /// are set as appropriate.
    Pla,

    /// PLP - Pull Processor Status
    /// Pulls an 8 bit value from the stack and into the processor flags. The flags will take on
    /// new states as determined by the value pulled.
    Plp,

    // Logical
    // The following instructions perform logical operations on the contents of the accumulator
    // and another value held in memory. The BIT instruction performs a logical AND to test the
    // presence of bits in the memory value to set the flags but does not keep the result.
    /// AND - Logical AND
    /// ```text
    /// A,Z,N = A&M
    /// ```
    /// A logical AND is performed, bit by bit, on the accumulator contents using the contents of a
    /// byte of memory.
    And(AddressMode),

    /// EOR - Exclusive OR
    /// ```text
    /// A,Z,N = A^M
    /// ```
    /// An exclusive OR is performed, bit by bit, on the accumulator contents using the contents of
    /// a byte of memory.
    Eor(AddressMode),

    /// ORA - Logical Inclusive OR
    /// ```text
    /// A,Z,N = A|M
    /// ```
    /// An inclusive OR is performed, bit by bit, on the accumulator contents using the contents of
    /// a byte of memory.
    Ora(AddressMode),

    /// BIT - Bit Test
    /// ```text
    /// A & M, N = M7, V = M6
    /// ```
    /// This instructions is used to test if one or more bits are set in a target memory location.
    /// The mask pattern in A is ANDed with the value in memory to set or clear the zero flag, but
    /// the result is not kept. Bits 7 and 6 of the value from memory are copied into the N and V
    /// flags.
    Bit(AddressMode),

    // Arithmetic
    // The arithmetic operations perform addition and subtraction on the contents of the
    // accumulator. The compare operations allow the comparison of the accumulator and X or Y with
    // memory values.
    /// ADC - Add with Carry
    /// ```text
    /// A,Z,C,N = A+M+C
    /// ```
    /// This instruction adds the contents of a memory location to the accumulator together with
    /// the carry bit. If overflow occurs the carry bit is set, this enables multiple byte addition
    /// to be performed.
    Adc(AddressMode),

    /// SBC - Subtract with Carry
    /// ```text
    /// A,Z,C,N = A-M-(1-C)
    /// ```
    /// This instruction subtracts the contents of a memory location to the accumulator together
    /// with the not of the carry bit. If overflow occurs the carry bit is clear, this enables
    /// multiple byte subtraction to be performed.
    Sbc(AddressMode),

    /// CMP - Compare
    /// ```text
    /// Z,C,N = A-M
    /// ```
    /// This instruction compares the contents of the accumulator with another memory held value
    /// and sets the zero and carry flags as appropriate.
    Cmp(AddressMode),

    /// CPX - Compare X Register
    /// ```text
    /// Z,C,N = X-M
    /// ```
    /// This instruction compares the contents of the X register with another memory held value and
    /// sets the zero and carry flags as appropriate.
    Cpx(AddressMode),

    /// CPY - Compare Y Register
    /// ```text
    /// Z,C,N = Y-M
    /// ```
    /// This instruction compares the contents of the Y register with another memory held value and
    /// sets the zero and carry flags as appropriate.
    Cpy(AddressMode),

    // Increments & Decrements
    // Increment or decrement a memory location or one of the X or Y registers by one setting the
    // negative (N) and zero (Z) flags as appropriate,
    /// INC - Increment Memory
    /// ```text
    /// M,Z,N = M+1
    /// ```
    /// Adds one to the value held at a specified memory location setting the zero and negative
    /// flags as appropriate.
    Inc(AddressMode),

    /// INX - Increment X Register
    /// ```text
    /// X,Z,N = X+1
    /// ```
    /// Adds one to the X register setting the zero and negative flags as appropriate.
    Inx,

    /// INY - Increment Y Register
    /// ```text
    /// Y,Z,N = Y+1
    /// ```
    /// Adds one to the Y register setting the zero and negative flags as appropriate.
    Iny,

    /// DEC - Decrement Memory
    /// ```text
    /// M,Z,N = M-1
    /// ```
    /// Subtracts one from the value held at a specified memory location setting the zero and
    /// negative flags as appropriate.
    Dec(AddressMode),

    /// DEX - Decrement X Register
    /// ```text
    /// X,Z,N = X-1
    /// ```
    /// Subtracts one from the X register setting the zero and negative flags as appropriate.
    Dex,

    /// DEY - Decrement Y Register
    /// ```text
    /// Y,Z,N = Y-1
    /// ```
    /// Subtracts one from the Y register setting the zero and negative flags as appropriate.
    Dey,

    // Shifts
    // Shift instructions cause the bits within either a memory location or the accumulator to be
    // shifted by one bit position. The rotate instructions use the contents if the carry flag (C)
    // to fill the vacant position generated by the shift and to catch the overflowing bit. The
    // arithmetic and logical shifts shift in an appropriate 0 or 1 bit as appropriate but catch
    // the overflow bit in the carry flag (C).
    /// ASL - Arithmetic Shift Left
    /// ```text
    /// A,Z,C,N = M*2 or M,Z,C,N = M*2
    /// ```
    /// This operation shifts all the bits of the accumulator or memory contents one bit left. Bit
    /// 0 is set to 0 and bit 7 is placed in the carry flag. The effect of this operation is to
    /// multiply the memory contents by 2 (ignoring 2's complement considerations), setting the
    /// carry if the result will not fit in 8 bits.
    Asl(AddressMode),
    /// LSR - Logical Shift Right
    /// ```text
    /// A,C,Z,N = A/2 or M,C,Z,N = M/2
    /// ```
    /// Each of the bits in A or M is shift one place to the right. The bit that was in bit 0 is
    /// shifted into the carry flag. Bit 7 is set to zero.
    Lsr(AddressMode),

    /// ROL - Rotate Left
    /// Move each of the bits in either A or M one place to the left. Bit 0 is filled with the
    /// current value of the carry flag whilst the old bit 7 becomes the new carry flag value.
    Rol(AddressMode),

    /// ROR - Rotate Right
    /// Move each of the bits in either A or M one place to the right. Bit 7 is filled with the
    /// current value of the carry flag whilst the old bit 0 becomes the new carry flag value.
    Ror(AddressMode),

    // Jumps & Calls
    // The following instructions modify the program counter causing a break to normal sequential
    // execution. The JSR instruction pushes the old PC onto the stack before changing it to the
    // new location allowing a subsequent RTS to return execution to the instruction after the
    // call.
    /// Sets the program counter to the address specified by the operand.
    Jmp(AddressMode),

    /// JSR - Jump to Subroutine
    /// The JSR instruction pushes the address (minus one) of the return point on to the stack and
    /// then sets the program counter to the target memory address.
    Jsr,
    /// RTS - Return from Subroutine
    /// The RTS instruction is used at the end of a subroutine to return to the calling routine. It
    /// pulls the program counter (minus one) from the stack.
    Rts,

    // Branches
    // Branch instructions break the normal sequential flow of execution by changing the program
    // counter if a specified condition is met. All the conditions are based on examining a single
    // bit within the processor status. Branch instructions use relative address to identify
    // the target instruction if they are executed. As relative addresses are stored using a
    // signed 8 bit byte the target instruction must be within 126 bytes before the branch or 128
    // bytes after the branch.
    /// BCC - Branch if Carry Clear
    /// If the carry flag is clear then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    Bcc,

    /// BCS - Branch if Carry Set
    /// If the carry flag is set then add the relative displacement to the program counter to cause
    /// a branch to a new location.
    Bcs,

    /// BEQ - Branch if Equal
    /// If the zero flag is set then add the relative displacement to the program counter to cause
    /// a branch to a new location.
    Beq,

    /// BMI - Branch if Minus
    /// If the negative flag is set then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    Bmi,

    /// BNE - Branch if Not Equal
    /// If the zero flag is clear then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    Bne,

    /// BPL - Branch if Positive
    /// If the negative flag is clear then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    Bpl,

    /// BVC - Branch if Overflow Clear
    /// If the overflow flag is clear then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    Bvc,

    /// BVS - Branch if Overflow Set
    /// If the overflow flag is set then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    Bvs,

    // Status Flag Changes
    // The following instructions change the values of specific status flags.
    /// CLC - Clear Carry Flag
    /// ```text
    /// C = 0
    /// ```
    /// Set the carry flag to zero.
    Clc,

    /// CLD - Clear Decimal Mode
    /// ```text
    /// D = 0
    /// ```
    /// Sets the decimal mode flag to zero.
    Cld,

    /// CLI - Clear Interrupt Disable
    /// ```text
    /// I = 0
    /// ```
    /// Clears the interrupt disable flag allowing normal interrupt requests to be serviced.
    Cli,

    /// CLV - Clear Overflow Flag
    /// ```text
    /// V = 0
    /// ```
    /// Clears the overflow flag.
    Clv,

    /// SEC - Set Carry Flag
    /// ```text
    /// C = 1
    /// ```
    /// Set the carry flag to one.
    Sec,

    /// SED - Set Decimal Flag
    /// ```text
    /// D = 1
    /// ```
    /// Set the decimal mode flag to one.
    Sed,

    /// SEI - Set Interrupt Disable
    /// ```text
    /// I = 1
    /// ```
    /// Set the interrupt disable flag to one.
    Sei,

    // System Functions
    // The remaining instructions perform useful but rarely used functions.
    /// BRK - Force Interrupt
    /// The BRK instruction forces the generation of an interrupt request. The program counter and
    /// processor status are pushed on the stack then the IRQ interrupt vector at $FFFE/F is loaded
    /// into the PC and the break flag in the status set to one.
    #[allow(dead_code)]
    Brk,

    /// NOP - No Operation
    /// The NOP instruction causes no changes to the processor other than the normal incrementing
    /// of the program counter to the next instruction.
    Nop,

    /// RTI - Return from Interrupt
    /// The RTI instruction is used at the end of an interrupt processing routine. It pulls the
    /// processor flags from the stack followed by the program counter.
    Rti,

    /// Internal instruction that triggers a interrupt
    Interrupt(Word),
}

/// The 6502 processor provides several ways in which memory locations can be addressed. Some
/// instructions support several different modes while others may only support one. In addition the
/// two index registers can not always be used interchangeably. This lack of orthogonality in the
/// instruction set is one of the features that makes the 6502 trickier to program well.
#[derive(Debug, Clone)]
pub enum AddressMode {
    /// For many 6502 instructions the source and destination of the information to be manipulated
    /// is implied directly by the function of the instruction itself and no further operand needs
    /// to be specified. Operations like 'Clear Carry Flag' (CLC) and 'Return from Subroutine'
    /// (RTS) are implicit.
    //Implicit,

    /// Some instructions have an option to operate directly upon the accumulator. The programmer
    /// specifies this by using a special operand value, 'A'. For example:
    ///
    /// ```asm
    /// LSR A           ;Logical shift right one bit
    /// ROR A           ;Rotate right one bit
    /// ```
    Accumulator,

    /// Immediate addressing allows the programmer to directly specify an 8 bit constant within the
    /// instruction. It is indicated by a '#' symbol followed by an numeric expression. For
    /// example:
    ///
    /// ```asm
    /// LDA #10         ;Load 10 ($0A) into the accumulator
    /// LDX #LO LABEL   ;Load the LSB of a 16 bit address into X
    /// LDY #HI LABEL   ;Load the MSB of a 16 bit address into Y
    /// ````
    Immediate,

    /// An instruction using zero page addressing mode has only an 8 bit address operand. This
    /// limits it to addressing only the first 256 bytes of memory (e.g. $0000 to $00FF) where the
    /// most significant byte of the address is always zero. In zero page mode only the least
    /// significant byte of the address is held in the instruction making it shorter by one byte
    /// (important for space saving) and one less memory fetch during execution (important for
    /// speed).
    ///
    /// An assembler will automatically select zero page addressing mode if the operand evaluates
    /// to a zero page address and the instruction supports the mode (not all do).
    ///
    /// ```asm
    /// LDA $00         ;Load accumulator from $00
    /// ASL ANSWER      ;Shift labelled location ANSWER left
    /// ````
    ZeroPage,

    /// The address to be accessed by an instruction using indexed zero page addressing is
    /// calculated by taking the 8 bit zero page address from the instruction and adding the
    /// current value of the X register to it. For example if the X register contains $0F and the
    /// instruction LDA $80,X is executed then the accumulator will be loaded from $008F (e.g. $80
    /// + $0F => $8F).
    ///
    /// NB:
    /// The address calculation wraps around if the sum of the base address and the register exceed
    /// $FF. If we repeat the last example but with $FF in the X register then the accumulator will
    /// be loaded from $007F (e.g. $80 + $FF => $7F) and not $017F.
    ///
    /// ```asm
    /// STY $10,X       ;Save the Y register at location on zero page
    /// AND TEMP,X      ;Logical AND accumulator with a zero page value
    /// ```
    ZeroPageX,

    /// The address to be accessed by an instruction using indexed zero page addressing is
    /// calculated by taking the 8 bit zero page address from the instruction and adding the
    /// current value of the Y register to it. This mode can only be used with the LDX and STX
    /// instructions.
    ///
    /// ```asm
    /// LDX $10,Y       ;Load the X register from a location on zero page
    /// STX TEMP,Y      ;Store the X register in a location on zero page
    /// ```
    ZeroPageY,

    /// Relative addressing mode is used by branch instructions (e.g. BEQ, BNE, etc.) which contain
    /// a signed 8 bit relative offset (e.g. -128 to +127) which is added to program counter if the
    /// condition is true. As the program counter itself is incremented during instruction
    /// execution by two the effective address range for the target instruction must be with -126
    /// to +129 bytes of the branch.
    ///
    /// ```asm
    /// BEQ LABEL       ;Branch if zero flag set to LABEL
    /// BNE *+4         ;Skip over the following 2 byte instruction
    /// ```
    //Relative,

    /// Instructions using absolute addressing contain a full 16 bit address to identify the target
    /// location.
    ///
    /// ```asm
    /// JMP $1234       ;Jump to location $1234
    /// JSR WIBBLE      ;Call subroutine WIBBLE
    /// ```
    Absolute,

    /// The address to be accessed by an instruction using X register indexed absolute addressing
    /// is computed by taking the 16 bit address from the instruction and added the contents of the
    /// X register. For example if X contains $92 then an STA $2000,X instruction will store the
    /// accumulator at $2092 (e.g. $2000 + $92).
    ///
    /// ```asm
    /// STA $3000,X     ;Store accumulator between $3000 and $30FF
    /// ROR CRC,X       ;Rotate right one bit
    /// ```
    AbsoluteX,

    /// The Y register indexed absolute addressing mode is the same as the previous mode only with
    /// the contents of the Y register added to the 16 bit address from the instruction.
    ///
    /// ```asm
    /// AND $4000,Y     ;Perform a logical AND with a byte of memory
    /// STA MEM,Y       ;Store accumulator in memory
    /// ```
    AbsoluteY,

    /// JMP is the only 6502 instruction to support indirection. The instruction contains a 16 bit
    /// address which identifies the location of the least significant byte of another 16 bit
    /// memory address which is the real target of the instruction.
    ///
    /// For example if location $0120 contains $FC and location $0121 contains $BA then the
    /// instruction JMP ($0120) will cause the next instruction execution to occur at $BAFC (e.g.
    /// the contents of $0120 and $0121).
    ///
    /// ```asm
    /// JMP ($FFFC)     ;Force a power on reset
    /// JMP (TARGET)    ;Jump via a labelled memory area
    /// ```
    Indirect,

    /// Indexed indirect addressing is normally used in conjunction with a table of address held on
    /// zero page. The address of the table is taken from the instruction and the X register added
    /// to it (with zero page wrap around) to give the location of the least significant byte of
    /// the target address.
    ///
    /// ```asm
    /// LDA ($40,X)     ;Load a byte indirectly from memory
    /// STA (MEM,X)     ;Store accumulator indirectly into memory
    /// ```
    IndexedIndirect,

    /// Indirect indirect addressing is the most common indirection mode used on the 6502. In
    /// instruction contains the zero page location of the least significant byte of 16 bit
    /// address. The Y register is dynamically added to this value to generated the actual target
    /// address for operation.
    ///
    /// ```asm
    /// LDA ($40),Y     ;Load a byte indirectly from memory
    /// STA (DST),Y     ;Store accumulator indirectly into memory
    /// ```
    IndirectIndexed,
}

impl<T> OpParser for Cpu<T>
where
    T: Addressable,
{
    #[allow(clippy::too_many_lines)]
    fn parse(&self, value: Byte) -> Result<(Op, u8), Error> {
        Ok(match value {
            0xA9 => (Op::Lda(AddressMode::Immediate), 2),
            0xA5 => (Op::Lda(AddressMode::ZeroPage), 3),
            0xB5 => (Op::Lda(AddressMode::ZeroPageX), 4),
            0xAD => (Op::Lda(AddressMode::Absolute), 4),
            0xBD => (Op::Lda(AddressMode::AbsoluteX), 4), // +1 if page is crossed
            0xB9 => (Op::Lda(AddressMode::AbsoluteY), 4), // +1 if page is crossed
            0xA1 => (Op::Lda(AddressMode::IndexedIndirect), 6),
            0xB1 => (Op::Lda(AddressMode::IndirectIndexed), 5), // +1 if page is crossed

            0xA2 => (Op::Ldx(AddressMode::Immediate), 2),
            0xA6 => (Op::Ldx(AddressMode::ZeroPage), 3),
            0xB6 => (Op::Ldx(AddressMode::ZeroPageY), 4),
            0xAE => (Op::Ldx(AddressMode::Absolute), 4),
            0xBE => (Op::Ldx(AddressMode::AbsoluteY), 4), // +1 if page is crossed

            0xA0 => (Op::Ldy(AddressMode::Immediate), 2),
            0xA4 => (Op::Ldy(AddressMode::ZeroPage), 3),
            0xB4 => (Op::Ldy(AddressMode::ZeroPageX), 4),
            0xAC => (Op::Ldy(AddressMode::Absolute), 4),
            0xBC => (Op::Ldy(AddressMode::AbsoluteX), 4), // +1 if page is crossed

            0x85 => (Op::Sta(AddressMode::ZeroPage), 3),
            0x95 => (Op::Sta(AddressMode::ZeroPageX), 4),
            0x8D => (Op::Sta(AddressMode::Absolute), 4),
            0x9D => (Op::Sta(AddressMode::AbsoluteX), 5),
            0x99 => (Op::Sta(AddressMode::AbsoluteY), 5),
            0x81 => (Op::Sta(AddressMode::IndexedIndirect), 6),
            0x91 => (Op::Sta(AddressMode::IndirectIndexed), 6),

            0x86 => (Op::Stx(AddressMode::ZeroPage), 3),
            0x96 => (Op::Stx(AddressMode::ZeroPageY), 4),
            0x8E => (Op::Stx(AddressMode::Absolute), 4),

            0x84 => (Op::Sty(AddressMode::ZeroPage), 3),
            0x94 => (Op::Sty(AddressMode::ZeroPageX), 4),
            0x8C => (Op::Sty(AddressMode::Absolute), 4),

            0xAA => (Op::Tax, 2),
            0xA8 => (Op::Tay, 2),
            0x8A => (Op::Txa, 2),
            0x98 => (Op::Tya, 2),
            0xBA => (Op::Tsx, 2),
            0x9A => (Op::Txs, 2),

            0x48 => (Op::Pha, 3),
            0x08 => (Op::Php, 3),
            0x68 => (Op::Pla, 4),
            0x28 => (Op::Plp, 4),

            0x29 => (Op::And(AddressMode::Immediate), 2),
            0x25 => (Op::And(AddressMode::ZeroPage), 3),
            0x35 => (Op::And(AddressMode::ZeroPageX), 4),
            0x2D => (Op::And(AddressMode::Absolute), 4),
            0x3D => (Op::And(AddressMode::AbsoluteX), 4), // +1 if page is crossed
            0x39 => (Op::And(AddressMode::AbsoluteY), 4), // +1 if page is crossed
            0x21 => (Op::And(AddressMode::IndexedIndirect), 6),
            0x31 => (Op::And(AddressMode::IndirectIndexed), 5), // +1 if page is crossed

            0x49 => (Op::Eor(AddressMode::Immediate), 2),
            0x45 => (Op::Eor(AddressMode::ZeroPage), 3),
            0x55 => (Op::Eor(AddressMode::ZeroPageX), 4),
            0x4D => (Op::Eor(AddressMode::Absolute), 4),
            0x5D => (Op::Eor(AddressMode::AbsoluteX), 4), // +1 if page is crossed
            0x59 => (Op::Eor(AddressMode::AbsoluteY), 4), // +1 if page is crossed
            0x41 => (Op::Eor(AddressMode::IndexedIndirect), 6),
            0x51 => (Op::Eor(AddressMode::IndirectIndexed), 5), // +1 if page is crossed

            0x09 => (Op::Ora(AddressMode::Immediate), 2),
            0x05 => (Op::Ora(AddressMode::ZeroPage), 3),
            0x15 => (Op::Ora(AddressMode::ZeroPageX), 4),
            0x0D => (Op::Ora(AddressMode::Absolute), 4),
            0x1D => (Op::Ora(AddressMode::AbsoluteX), 4), // +1 if page is crossed
            0x19 => (Op::Ora(AddressMode::AbsoluteY), 4), // +1 if page is crossed
            0x01 => (Op::Ora(AddressMode::IndexedIndirect), 6),
            0x11 => (Op::Ora(AddressMode::IndirectIndexed), 5), // +1 if page is crossed

            0x24 => (Op::Bit(AddressMode::ZeroPage), 3),
            0x2C => (Op::Bit(AddressMode::Absolute), 4),

            0x69 => (Op::Adc(AddressMode::Immediate), 2),
            0x65 => (Op::Adc(AddressMode::ZeroPage), 3),
            0x75 => (Op::Adc(AddressMode::ZeroPageX), 4),
            0x6D => (Op::Adc(AddressMode::Absolute), 4),
            0x7D => (Op::Adc(AddressMode::AbsoluteX), 4), // +1 if page is crossed
            0x79 => (Op::Adc(AddressMode::AbsoluteY), 4), // +1 if page is crossed
            0x61 => (Op::Adc(AddressMode::IndexedIndirect), 6),
            0x71 => (Op::Adc(AddressMode::IndirectIndexed), 5), // +1 if page is crossed

            0xE9 => (Op::Sbc(AddressMode::Immediate), 2),
            0xE5 => (Op::Sbc(AddressMode::ZeroPage), 3),
            0xF5 => (Op::Sbc(AddressMode::ZeroPageX), 4),
            0xED => (Op::Sbc(AddressMode::Absolute), 4),
            0xFD => (Op::Sbc(AddressMode::AbsoluteX), 4), // +1 if page is crossed
            0xF9 => (Op::Sbc(AddressMode::AbsoluteY), 4), // +1 if page is crossed
            0xE1 => (Op::Sbc(AddressMode::IndexedIndirect), 6),
            0xF1 => (Op::Sbc(AddressMode::IndirectIndexed), 5), // +1 if page is crossed

            0xC9 => (Op::Cmp(AddressMode::Immediate), 2),
            0xC5 => (Op::Cmp(AddressMode::ZeroPage), 3),
            0xD5 => (Op::Cmp(AddressMode::ZeroPageX), 4),
            0xCD => (Op::Cmp(AddressMode::Absolute), 4),
            0xDD => (Op::Cmp(AddressMode::AbsoluteX), 4), // +1 if page is crossed
            0xD9 => (Op::Cmp(AddressMode::AbsoluteY), 4), // +1 if page is crossed
            0xC1 => (Op::Cmp(AddressMode::IndexedIndirect), 6),
            0xD1 => (Op::Cmp(AddressMode::IndirectIndexed), 5), // +1 if page is crossed

            0xE0 => (Op::Cpx(AddressMode::Immediate), 2),
            0xE4 => (Op::Cpx(AddressMode::ZeroPage), 3),
            0xEC => (Op::Cpx(AddressMode::Absolute), 4),

            0xC0 => (Op::Cpy(AddressMode::Immediate), 2),
            0xC4 => (Op::Cpy(AddressMode::ZeroPage), 3),
            0xCC => (Op::Cpy(AddressMode::Absolute), 4),

            0xE6 => (Op::Inc(AddressMode::ZeroPage), 5),
            0xF6 => (Op::Inc(AddressMode::ZeroPageX), 6),
            0xEE => (Op::Inc(AddressMode::Absolute), 6),
            0xFE => (Op::Inc(AddressMode::AbsoluteX), 7),

            0xE8 => (Op::Inx, 2),
            0xC8 => (Op::Iny, 2),

            0xC6 => (Op::Dec(AddressMode::ZeroPage), 5),
            0xD6 => (Op::Dec(AddressMode::ZeroPageX), 6),
            0xCE => (Op::Dec(AddressMode::Absolute), 6),
            0xDE => (Op::Dec(AddressMode::AbsoluteX), 7),

            0xCA => (Op::Dex, 2),
            0x88 => (Op::Dey, 2),

            0x0A => (Op::Asl(AddressMode::Accumulator), 2),
            0x06 => (Op::Asl(AddressMode::ZeroPage), 5),
            0x16 => (Op::Asl(AddressMode::ZeroPageX), 6),
            0x0E => (Op::Asl(AddressMode::Absolute), 6),
            0x1E => (Op::Asl(AddressMode::AbsoluteX), 7),

            0x4A => (Op::Lsr(AddressMode::Accumulator), 2),
            0x46 => (Op::Lsr(AddressMode::ZeroPage), 5),
            0x56 => (Op::Lsr(AddressMode::ZeroPageX), 6),
            0x4E => (Op::Lsr(AddressMode::Absolute), 6),
            0x5E => (Op::Lsr(AddressMode::AbsoluteX), 7),

            0x2A => (Op::Rol(AddressMode::Accumulator), 2),
            0x26 => (Op::Rol(AddressMode::ZeroPage), 5),
            0x36 => (Op::Rol(AddressMode::ZeroPageX), 6),
            0x2E => (Op::Rol(AddressMode::Absolute), 6),
            0x3E => (Op::Rol(AddressMode::AbsoluteX), 7),

            0x6A => (Op::Ror(AddressMode::Accumulator), 2),
            0x66 => (Op::Ror(AddressMode::ZeroPage), 5),
            0x76 => (Op::Ror(AddressMode::ZeroPageX), 6),
            0x6E => (Op::Ror(AddressMode::Absolute), 6),
            0x7E => (Op::Ror(AddressMode::AbsoluteX), 7),

            0x4C => (Op::Jmp(AddressMode::Absolute), 3),
            0x6C => (Op::Jmp(AddressMode::Indirect), 5),

            0x20 => (Op::Jsr, 6),
            0x60 => (Op::Rts, 6),

            0x90 => (Op::Bcc, 2), // +1 if branch succeeds, +2 if to a new page
            0xB0 => (Op::Bcs, 2), // +1 if branch succeeds, +2 if to a new page
            0xF0 => (Op::Beq, 2), // +1 if branch succeeds, +2 if to a new page
            0x30 => (Op::Bmi, 2), // +1 if branch succeeds, +2 if to a new page
            0xD0 => (Op::Bne, 2), // +1 if branch succeeds, +2 if to a new page
            0x10 => (Op::Bpl, 2), // +1 if branch succeeds, +2 if to a new page
            0x50 => (Op::Bvc, 2), // +1 if branch succeeds, +2 if to a new page
            0x70 => (Op::Bvs, 2), // +1 if branch succeeds, +2 if to a new page

            0x18 => (Op::Clc, 2),
            0xD8 => (Op::Cld, 2),
            0x58 => (Op::Cli, 2),
            0xB8 => (Op::Clv, 2),

            0x38 => (Op::Sec, 2),
            0xF8 => (Op::Sed, 2),
            0x78 => (Op::Sei, 2),

            // 0x00 => (Op::Brk, 7),
            0x00 => (Op::Interrupt(IRQB), 7),
            0xEA => (Op::Nop, 2),
            0x40 => (Op::Rti, 6),

            _ => {
                return Err(Error::BadOpCode(value));
            }
        })
    }
}
