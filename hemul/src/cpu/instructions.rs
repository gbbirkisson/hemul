use super::CpuError;
use crate::Byte;

/// The 6502 instruction set
#[derive(Debug, Clone)]
pub enum OpCode {
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
    Lda,

    /// LDX - Load X Register
    /// ```text
    /// X,Z,N = M
    /// ```
    /// Loads a byte of memory into the X register setting the zero and negative flags as
    /// appropriate.
    Ldx,

    /// LDY - Load Y Register
    /// ```text
    /// Y,Z,N = M
    /// ```
    /// Loads a byte of memory into the Y register setting the zero and negative flags as
    /// appropriate.
    Ldy,

    /// STA - Store Accumulator
    /// ```text
    /// M = A
    /// ```
    /// Stores the contents of the accumulator into memory.
    Sta,

    /// STX - Store X Register
    /// ```text
    /// M = X
    /// ```
    /// Stores the contents of the X register into memory.
    Stx,

    /// STY - Store Y Register
    /// ```text
    /// M = Y
    /// ```
    /// Stores the contents of the Y register into memory.
    Sty,

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
    And,

    /// EOR - Exclusive OR
    /// ```text
    /// A,Z,N = A^M
    /// ```
    /// An exclusive OR is performed, bit by bit, on the accumulator contents using the contents of
    /// a byte of memory.
    Eor,

    /// ORA - Logical Inclusive OR
    /// ```text
    /// A,Z,N = A|M
    /// ```
    /// An inclusive OR is performed, bit by bit, on the accumulator contents using the contents of
    /// a byte of memory.
    Ora,

    /// BIT - Bit Test
    /// ```text
    /// A & M, N = M7, V = M6
    /// ```
    /// This instructions is used to test if one or more bits are set in a target memory location.
    /// The mask pattern in A is ANDed with the value in memory to set or clear the zero flag, but
    /// the result is not kept. Bits 7 and 6 of the value from memory are copied into the N and V
    /// flags.
    Bit,

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
    Adc,

    /// SBC - Subtract with Carry
    /// ```text
    /// A,Z,C,N = A-M-(1-C)
    /// ```
    /// This instruction subtracts the contents of a memory location to the accumulator together
    /// with the not of the carry bit. If overflow occurs the carry bit is clear, this enables
    /// multiple byte subtraction to be performed.
    Sbc,

    /// CMP - Compare
    /// ```text
    /// Z,C,N = A-M
    /// ```
    /// This instruction compares the contents of the accumulator with another memory held value
    /// and sets the zero and carry flags as appropriate.
    Cmp,

    /// CPX - Compare X Register
    /// ```text
    /// Z,C,N = X-M
    /// ```
    /// This instruction compares the contents of the X register with another memory held value and
    /// sets the zero and carry flags as appropriate.
    Cpx,

    /// CPY - Compare Y Register
    /// ```text
    /// Z,C,N = Y-M
    /// ```
    /// This instruction compares the contents of the Y register with another memory held value and
    /// sets the zero and carry flags as appropriate.
    Cpy,

    // Increments & Decrements
    // Increment or decrement a memory location or one of the X or Y registers by one setting the
    // negative (N) and zero (Z) flags as appropriate,
    /// INC - Increment Memory
    /// ```text
    /// M,Z,N = M+1
    /// ```
    /// Adds one to the value held at a specified memory location setting the zero and negative
    /// flags as appropriate.
    Inc,

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
    Dec,

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
    Asl,
    /// LSR - Logical Shift Right
    /// ```text
    /// A,C,Z,N = A/2 or M,C,Z,N = M/2
    /// ```
    /// Each of the bits in A or M is shift one place to the right. The bit that was in bit 0 is
    /// shifted into the carry flag. Bit 7 is set to zero.
    Lsr,

    /// ROL - Rotate Left
    /// Move each of the bits in either A or M one place to the left. Bit 0 is filled with the
    /// current value of the carry flag whilst the old bit 7 becomes the new carry flag value.
    Rol,

    /// ROR - Rotate Right
    /// Move each of the bits in either A or M one place to the right. Bit 7 is filled with the
    /// current value of the carry flag whilst the old bit 0 becomes the new carry flag value.
    Ror,

    // Jumps & Calls
    // The following instructions modify the program counter causing a break to normal sequential
    // execution. The JSR instruction pushes the old PC onto the stack before changing it to the
    // new location allowing a subsequent RTS to return execution to the instruction after the
    // call.
    /// Sets the program counter to the address specified by the operand.
    Jmp,

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
    Brk,

    /// NOP - No Operation
    /// The NOP instruction causes no changes to the processor other than the normal incrementing
    /// of the program counter to the next instruction.
    Nop,

    /// RTI - Return from Interrupt
    /// The RTI instruction is used at the end of an interrupt processing routine. It pulls the
    /// processor flags from the stack followed by the program counter.
    Rti,
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
    Implicit,

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
    Relative,

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

/// Denotes how many cycles a particular instruction takes
#[derive(Debug, Clone)]
pub enum Cycles {
    /// The instruction always takes constant time
    Constant(u8),

    /// The instruction takes some time +1 if page is crossed
    Page(u8),

    /// The instruction take some time +1 if branch succeeds, +2 if page is crossed
    Branch(u8),
}

/// Container for instruction, address mode and number of cycles used for a given combo
#[derive(Clone)]
pub struct Op(pub OpCode, pub AddressMode, pub Cycles);

impl std::fmt::Debug for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            AddressMode::Implicit | AddressMode::Relative => {
                write!(f, "{:?}", self.0)
            }
            _ => {
                write!(f, "{:?}({:?})", self.0, self.1)
            }
        }
    }
}

macro_rules! op {
    ($op_code:ident, $address_mode:ident, $cycles: literal) => {
        Op(
            OpCode::$op_code,
            AddressMode::$address_mode,
            Cycles::Constant($cycles),
        )
    };
    ($op_code:ident, $address_mode:ident, $cycles: expr) => {
        Op(OpCode::$op_code, AddressMode::$address_mode, $cycles)
    };
}

impl TryFrom<Byte> for Op {
    type Error = CpuError;

    #[allow(clippy::too_many_lines)]
    fn try_from(value: Byte) -> Result<Self, Self::Error> {
        Ok(match value {
            0xA9 => op!(Lda, Immediate, 2),
            0xA5 => op!(Lda, ZeroPage, 3),
            0xB5 => op!(Lda, ZeroPageX, 4),
            0xAD => op!(Lda, Absolute, 4),
            0xBD => op!(Lda, AbsoluteX, Cycles::Page(4)), // +1 if page is crossed
            0xB9 => op!(Lda, AbsoluteY, Cycles::Page(4)), // +1 if page is crossed
            0xA1 => op!(Lda, IndexedIndirect, 6),
            0xB1 => op!(Lda, IndirectIndexed, Cycles::Page(5)), // +1 if page is crossed

            0xA2 => op!(Ldx, Immediate, 2),
            0xA6 => op!(Ldx, ZeroPage, 3),
            0xB6 => op!(Ldx, ZeroPageY, 4),
            0xAE => op!(Ldx, Absolute, 4),
            0xBE => op!(Ldx, AbsoluteY, Cycles::Page(4)), // +1 if page is crossed

            0xA0 => op!(Ldy, Immediate, 2),
            0xA4 => op!(Ldy, ZeroPage, 3),
            0xB4 => op!(Ldy, ZeroPageX, 4),
            0xAC => op!(Ldy, Absolute, 4),
            0xBC => op!(Ldy, AbsoluteX, Cycles::Page(4)), // +1 if page is crossed

            0x85 => op!(Sta, ZeroPage, 3),
            0x95 => op!(Sta, ZeroPageX, 4),
            0x8D => op!(Sta, Absolute, 4),
            0x9D => op!(Sta, AbsoluteX, 5),
            0x99 => op!(Sta, AbsoluteY, 5),
            0x81 => op!(Sta, IndexedIndirect, 6),
            0x91 => op!(Sta, IndirectIndexed, 6),

            0x86 => op!(Stx, ZeroPage, 3),
            0x96 => op!(Stx, ZeroPageY, 4),
            0x8E => op!(Stx, Absolute, 4),

            0x84 => op!(Sty, ZeroPage, 3),
            0x94 => op!(Sty, ZeroPageX, 4),
            0x8C => op!(Sty, Absolute, 4),

            0xAA => op!(Tax, Implicit, 2),
            0xA8 => op!(Tay, Implicit, 2),
            0x8A => op!(Txa, Implicit, 2),
            0x98 => op!(Tya, Implicit, 2),
            0xBA => op!(Tsx, Implicit, 2),
            0x9A => op!(Txs, Implicit, 2),

            0x48 => op!(Pha, Implicit, 3),
            0x08 => op!(Php, Implicit, 3),
            0x68 => op!(Pla, Implicit, 4),
            0x28 => op!(Plp, Implicit, 4),

            0x29 => op!(And, Immediate, 2),
            0x25 => op!(And, ZeroPage, 3),
            0x35 => op!(And, ZeroPageX, 4),
            0x2D => op!(And, Absolute, 4),
            0x3D => op!(And, AbsoluteX, Cycles::Page(4)), // +1 if page is crossed
            0x39 => op!(And, AbsoluteY, Cycles::Page(4)), // +1 if page is crossed
            0x21 => op!(And, IndexedIndirect, 6),
            0x31 => op!(And, IndirectIndexed, Cycles::Page(5)), // +1 if page is crossed

            0x49 => op!(Eor, Immediate, 2),
            0x45 => op!(Eor, ZeroPage, 3),
            0x55 => op!(Eor, ZeroPageX, 4),
            0x4D => op!(Eor, Absolute, 4),
            0x5D => op!(Eor, AbsoluteX, Cycles::Page(4)), // +1 if page is crossed
            0x59 => op!(Eor, AbsoluteY, Cycles::Page(4)), // +1 if page is crossed
            0x41 => op!(Eor, IndexedIndirect, 6),
            0x51 => op!(Eor, IndirectIndexed, Cycles::Page(5)), // +1 if page is crossed

            0x09 => op!(Ora, Immediate, 2),
            0x05 => op!(Ora, ZeroPage, 3),
            0x15 => op!(Ora, ZeroPageX, 4),
            0x0D => op!(Ora, Absolute, 4),
            0x1D => op!(Ora, AbsoluteX, Cycles::Page(4)), // +1 if page is crossed
            0x19 => op!(Ora, AbsoluteY, Cycles::Page(4)), // +1 if page is crossed
            0x01 => op!(Ora, IndexedIndirect, 6),
            0x11 => op!(Ora, IndirectIndexed, Cycles::Page(5)), // +1 if page is crossed

            0x24 => op!(Bit, ZeroPage, 3),
            0x2C => op!(Bit, Absolute, 4),

            0x69 => op!(Adc, Immediate, 2),
            0x65 => op!(Adc, ZeroPage, 3),
            0x75 => op!(Adc, ZeroPageX, 4),
            0x6D => op!(Adc, Absolute, 4),
            0x7D => op!(Adc, AbsoluteX, Cycles::Page(4)), // +1 if page is crossed
            0x79 => op!(Adc, AbsoluteY, Cycles::Page(4)), // +1 if page is crossed
            0x61 => op!(Adc, IndexedIndirect, 6),
            0x71 => op!(Adc, IndirectIndexed, Cycles::Page(5)), // +1 if page is crossed

            0xE9 => op!(Sbc, Immediate, 2),
            0xE5 => op!(Sbc, ZeroPage, 3),
            0xF5 => op!(Sbc, ZeroPageX, 4),
            0xED => op!(Sbc, Absolute, 4),
            0xFD => op!(Sbc, AbsoluteX, Cycles::Page(4)), // +1 if page is crossed
            0xF9 => op!(Sbc, AbsoluteY, Cycles::Page(4)), // +1 if page is crossed
            0xE1 => op!(Sbc, IndexedIndirect, 6),
            0xF1 => op!(Sbc, IndirectIndexed, Cycles::Page(5)), // +1 if page is crossed

            0xC9 => op!(Cmp, Immediate, 2),
            0xC5 => op!(Cmp, ZeroPage, 3),
            0xD5 => op!(Cmp, ZeroPageX, 4),
            0xCD => op!(Cmp, Absolute, 4),
            0xDD => op!(Cmp, AbsoluteX, Cycles::Page(4)), // +1 if page is crossed
            0xD9 => op!(Cmp, AbsoluteY, Cycles::Page(4)), // +1 if page is crossed
            0xC1 => op!(Cmp, IndexedIndirect, 6),
            0xD1 => op!(Cmp, IndirectIndexed, Cycles::Page(5)), // +1 if page is crossed

            0xE0 => op!(Cpx, Immediate, 2),
            0xE4 => op!(Cpx, ZeroPage, 3),
            0xEC => op!(Cpx, Absolute, 4),

            0xC0 => op!(Cpy, Immediate, 2),
            0xC4 => op!(Cpy, ZeroPage, 3),
            0xCC => op!(Cpy, Absolute, 4),

            0xE6 => op!(Inc, ZeroPage, 5),
            0xF6 => op!(Inc, ZeroPageX, 6),
            0xEE => op!(Inc, Absolute, 6),
            0xFE => op!(Inc, AbsoluteX, 7),

            0xE8 => op!(Inx, Implicit, 2),
            0xC8 => op!(Iny, Implicit, 2),

            0xC6 => op!(Dec, ZeroPage, 5),
            0xD6 => op!(Dec, ZeroPageX, 6),
            0xCE => op!(Dec, Absolute, 6),
            0xDE => op!(Dec, AbsoluteX, 7),

            0xCA => op!(Dex, Implicit, 2),
            0x88 => op!(Dey, Implicit, 2),

            0x0A => op!(Asl, Accumulator, 2),
            0x06 => op!(Asl, ZeroPage, 5),
            0x16 => op!(Asl, ZeroPageX, 6),
            0x0E => op!(Asl, Absolute, 6),
            0x1E => op!(Asl, AbsoluteX, 7),

            0x4A => op!(Lsr, Accumulator, 2),
            0x46 => op!(Lsr, ZeroPage, 5),
            0x56 => op!(Lsr, ZeroPageX, 6),
            0x4E => op!(Lsr, Absolute, 6),
            0x5E => op!(Lsr, AbsoluteX, 7),

            0x2A => op!(Rol, Accumulator, 2),
            0x26 => op!(Rol, ZeroPage, 5),
            0x36 => op!(Rol, ZeroPageX, 6),
            0x2E => op!(Rol, Absolute, 6),
            0x3E => op!(Rol, AbsoluteX, 7),

            0x6A => op!(Ror, Accumulator, 2),
            0x66 => op!(Ror, ZeroPage, 5),
            0x76 => op!(Ror, ZeroPageX, 6),
            0x6E => op!(Ror, Absolute, 6),
            0x7E => op!(Ror, AbsoluteX, 7),

            0x4C => op!(Jmp, Absolute, 3),
            0x6C => op!(Jmp, Indirect, 5),

            0x20 => op!(Jsr, Implicit, 6),
            0x60 => op!(Rts, Implicit, 6),

            // All these have +1 if branch succeeds, +2 if to a new page
            0x90 => op!(Bcc, Relative, Cycles::Branch(2)),
            0xB0 => op!(Bcs, Relative, Cycles::Branch(2)),
            0xF0 => op!(Beq, Relative, Cycles::Branch(2)),
            0x30 => op!(Bmi, Relative, Cycles::Branch(2)),
            0xD0 => op!(Bne, Relative, Cycles::Branch(2)),
            0x10 => op!(Bpl, Relative, Cycles::Branch(2)),
            0x50 => op!(Bvc, Relative, Cycles::Branch(2)),
            0x70 => op!(Bvs, Relative, Cycles::Branch(2)),

            0x18 => op!(Clc, Implicit, 2),
            0xD8 => op!(Cld, Implicit, 2),
            0x58 => op!(Cli, Implicit, 2),
            0xB8 => op!(Clv, Implicit, 2),

            0x38 => op!(Sec, Implicit, 2),
            0xF8 => op!(Sed, Implicit, 2),
            0x78 => op!(Sei, Implicit, 2),

            0x00 => op!(Brk, Implicit, 7),
            0xEA => op!(Nop, Implicit, 2),
            0x40 => op!(Rti, Implicit, 6),

            _ => {
                return Err(CpuError::BadOpCode(value));
            }
        })
    }
}
