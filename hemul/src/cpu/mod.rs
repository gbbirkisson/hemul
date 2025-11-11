use std::error::Error;

use self::{
    address::Address,
    instructions::{AddressMode, Cycles, OpCode},
};
use crate::{Addressable, Byte, Interruptible, Resettable, Tickable, Word};
use instructions::Op;
use thiserror::Error;

pub(crate) mod address;
mod instructions;
pub mod snapshot;

pub(crate) type PFlag = bool;

pub const SP_PAGE: Byte = 0x01;
pub const SP_ADDR: Byte = 0xFF;
pub const NMIB: Word = 0xFFFA; // + 0xFFFB
pub const RESB: Word = 0xFFFC; // + 0xFFFD
pub const IRQB: Word = 0xFFFE; // + 0xFFFF

// Status register flag bits
const C_FLAG: Byte = 0b0100_0000; // Carry
const Z_FLAG: Byte = 0b0010_0000; // Zero
const I_FLAG: Byte = 0b0001_0000; // Interrupt Disable
const D_FLAG: Byte = 0b0000_1000; // Decimal Mode
const B_FLAG: Byte = 0b0000_0100; // Break Command
const V_FLAG: Byte = 0b0000_0010; // Overflow
const N_FLAG: Byte = 0b0000_0001; // Negative

/// Cpu mode
pub enum Mode {
    /// Each clock cycle executes a single instruction
    Fast,

    /// Each instruction takes as many clock cycles as the original 6502 used
    Original(u8),
}

#[allow(non_snake_case, dead_code)]
pub struct Cpu<T: Addressable> {
    addr: T,

    /// Program Counter
    PC: Word,
    /// Stack Pointer
    SP: Byte,

    /// Accumulator
    A: Byte,
    /// Index Register X
    X: Byte,
    /// Index Register Y
    Y: Byte,

    /// Carry Flag
    C: PFlag,
    /// Zero Flag
    Z: PFlag,
    /// Interrupt Disable
    I: PFlag,
    /// Decimal Mode
    D: PFlag,
    /// Break Command
    B: PFlag,
    /// Overflow Flag
    V: PFlag,
    /// Negative Flag
    N: PFlag,

    /// The mode the Cpu runs in
    mode: Mode,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub enum CpuError {
    #[error("invalid opcode `{0:#04x}`")]
    BadOpCode(Byte),

    #[error("memory location out of bounds `{0:#06x}`")]
    OutOfBounds(Word),

    #[error("invalid address mode")]
    InvalidAddressMode,
}

impl<T> Cpu<T>
where
    T: Addressable,
{
    pub fn new(addr: T) -> Self {
        Self {
            addr,

            PC: 0,
            SP: 0,

            A: 0,
            X: 0,
            Y: 0,

            C: false,
            Z: false,
            I: false,
            D: false,
            B: false,
            V: false,
            N: false,

            mode: Mode::Fast,
        }
    }

    /// Read byte from address
    fn read(&self, addr: impl Into<Word>) -> Result<Byte, CpuError> {
        let addr = addr.into();
        if self.addr.inside_bounds(addr) {
            Ok(self.addr[addr])
        } else {
            Err(CpuError::OutOfBounds(addr))
        }
    }

    /// Read a word from address and address + 1
    fn read_word(&self, addr: impl Into<Word>) -> Result<Word, CpuError> {
        let a1 = addr.into();
        let a2 = a1 + 1;
        let addr = self.read(a1)?;
        let page = self.read(a2)?;
        Ok(Address::Full(addr, page).into())
    }

    /// Fetch byte from memory that the PC points to and increment the PC
    fn fetch(&mut self) -> Result<Byte, CpuError> {
        let data = self.read(self.PC)?;
        self.PC += 1;
        Ok(data)
    }

    /// Fetch word from memory that the PC points to and increment the PC twice
    fn fetch_word(&mut self) -> Result<Word, CpuError> {
        let data = self.read_word(self.PC)?;
        self.PC += 2;
        Ok(data)
    }

    /// Read next op from memory without changing the PC
    fn read_op(&self) -> Result<Op, CpuError> {
        self.read(self.PC)?.try_into()
    }

    /// Fetch op from memory that the PC points to and increment the PC
    fn fetch_op(&mut self) -> Result<Op, CpuError> {
        let op = self.read_op()?;
        self.PC += 1;
        Ok(op)
    }

    /// Fetch memory address that is referenced by some address mode
    fn fetch_addr(&mut self, mode: &AddressMode) -> Result<Word, CpuError> {
        Ok(match mode {
            AddressMode::Accumulator | AddressMode::Relative | AddressMode::Implicit => {
                return Err(CpuError::InvalidAddressMode);
            }
            AddressMode::Immediate => {
                self.PC += 1;
                self.PC - 1
            }
            AddressMode::ZeroPage => Address::Zero(self.fetch()?).into(),
            AddressMode::ZeroPageX => Address::Zero(self.fetch()?.wrapping_add(self.X)).into(),
            AddressMode::ZeroPageY => Address::Zero(self.fetch()?.wrapping_add(self.Y)).into(),
            AddressMode::Absolute => self.fetch_word()?,
            AddressMode::AbsoluteX => self.fetch_word()? + u16::from(self.X),
            AddressMode::AbsoluteY => self.fetch_word()? + u16::from(self.Y),
            AddressMode::Indirect => todo!(),
            AddressMode::IndexedIndirect => {
                let zero_page_addr = self.fetch()?;
                let addr_with_offset = zero_page_addr.wrapping_add(self.X);
                self.read_word(addr_with_offset)?
            }
            AddressMode::IndirectIndexed => {
                let zero_page_addr = self.fetch()?;
                let target_addr = self.read_word(Address::Zero(zero_page_addr))?;
                target_addr + u16::from(self.Y)
            }
        })
    }

    /// Write byte to address
    fn write(&mut self, addr: impl Into<Word>, value: impl Into<Byte>) -> Result<(), CpuError> {
        let addr = addr.into();
        if self.addr.inside_bounds(addr) {
            self.addr[addr] = value.into();
            Ok(())
        } else {
            Err(CpuError::OutOfBounds(addr))
        }
    }

    /// Push byte onto the stack
    fn stack_push(&mut self, byte: impl Into<Byte>) -> Result<(), CpuError> {
        let addr = Address::from((self.SP, SP_PAGE));
        self.write(addr, byte.into())?;
        self.SP = self.SP.wrapping_sub(1);
        Ok(())
    }

    /// Pop byte from the stack
    fn stack_pop(&mut self) -> Result<Byte, CpuError> {
        self.SP = self.SP.wrapping_add(1);
        let addr = Address::from((self.SP, SP_PAGE));
        let data = self.read(addr)?;
        Ok(data)
    }

    /// Get status register
    fn status_get(&self) -> Byte {
        let mut res = 0;
        if self.C {
            res |= C_FLAG;
        }
        if self.Z {
            res |= Z_FLAG;
        }
        if self.I {
            res |= I_FLAG;
        }
        if self.D {
            res |= D_FLAG;
        }
        if self.B {
            res |= B_FLAG;
        }
        if self.V {
            res |= V_FLAG;
        }
        if self.N {
            res |= N_FLAG;
        }
        res
    }

    /// Set status register
    fn status_set(&mut self, status: Byte) {
        self.C = status & C_FLAG > 0;
        self.Z = status & Z_FLAG > 0;
        self.I = status & I_FLAG > 0;
        self.D = status & D_FLAG > 0;
        self.B = status & B_FLAG > 0;
        self.V = status & V_FLAG > 0;
        self.N = status & N_FLAG > 0;

        assert!(!self.D, "Decimal Mode not supported");
    }

    /// Set mode
    #[allow(dead_code)]
    fn mode_set(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn tick_until_nop(&mut self) -> Result<(), Box<dyn Error>> {
        let mut count = 0;
        loop {
            if matches!(self.read_op()?, Op(OpCode::Nop, _, _)) {
                return Ok(());
            }

            count += 1;
            if count > 2000 {
                return Err("endless Loop".into());
            }

            self.tick()?;
        }
    }

    pub fn tick_for(&mut self, count: usize) -> Result<(), Box<dyn Error>> {
        for _ in 0..count {
            self.tick()?;
        }
        Ok(())
    }
}

macro_rules! flags_zn {
    ($self:ident, $r:expr) => {
        $self.Z = $r == 0;
        $self.N = ($r & 0b1000_0000) > 0;
    };
}

macro_rules! compare {
    ($self:ident, $r:ident, $mode:ident) => {
        let addr = $self.fetch_addr(&$mode)?;
        let data = $self.read(addr)?;
        let res = $self.$r.wrapping_sub(data);
        $self.C = $self.$r >= data;
        $self.Z = $self.$r == data;
        $self.N = (res & 0b1000_0000) > 0;
    };
}

macro_rules! branch {
    ($self:ident, $cond:expr) => {{
        let offset = $self.fetch()?;
        if $cond {
            $self.PC = u16::try_from(i32::from($self.PC) + i32::from(offset))
                .map_err(|e| format!("failed to calculate offset: {e}"))?;

            true
        } else {
            false
        }
    }};
}

impl<T> Tickable for Cpu<T>
where
    T: Addressable,
{
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn tick(&mut self) -> Result<(), Box<dyn Error>> {
        // Burn cycles if we need to
        if let Mode::Original(noop) = self.mode
            && noop > 0
        {
            self.mode = Mode::Original(noop - 1);
            return Ok(());
        }

        let Op(op, mode, cycles) = dbg!(self.fetch_op()?);
        let mut noop = match cycles {
            Cycles::Constant(c) | Cycles::Page(c) | Cycles::Branch(c) => c,
        };

        // We used 1 cycle to fetch the op code, so no need to burn that
        noop -= 1;

        // Execute op code
        match (op, mode) {
            (OpCode::Lda, mode) => {
                let addr = self.fetch_addr(&mode)?;
                self.A = self.read(addr)?;
                flags_zn!(self, self.A);
            }
            (OpCode::Ldx, mode) => {
                let addr = self.fetch_addr(&mode)?;
                self.X = self.read(addr)?;
                flags_zn!(self, self.X);
            }
            (OpCode::Ldy, mode) => {
                let addr = self.fetch_addr(&mode)?;
                self.Y = self.read(addr)?;
                flags_zn!(self, self.Y);
            }
            (OpCode::Sta, mode) => {
                let addr = self.fetch_addr(&mode)?;
                self.write(addr, self.A)?;
            }
            (OpCode::Stx, mode) => {
                let addr = self.fetch_addr(&mode)?;
                self.write(addr, self.X)?;
            }
            (OpCode::Sty, mode) => {
                let addr = self.fetch_addr(&mode)?;
                self.write(addr, self.Y)?;
            }
            (OpCode::Tax, _) => {
                self.X = self.A;
                flags_zn!(self, self.X);
            }
            (OpCode::Tay, _) => {
                self.Y = self.A;
                flags_zn!(self, self.Y);
            }
            (OpCode::Txa, _) => {
                self.A = self.X;
                flags_zn!(self, self.A);
            }
            (OpCode::Tya, _) => {
                self.A = self.Y;
                flags_zn!(self, self.Y);
            }
            (OpCode::Tsx, _) => {
                self.X = self.SP;
                flags_zn!(self, self.X);
            }
            (OpCode::Txs, _) => {
                self.SP = self.X;
            }
            (OpCode::Pha, _) => {
                self.stack_push(self.A)?;
            }
            (OpCode::Php, _) => {
                self.stack_push(self.status_get())?;
            }
            (OpCode::Pla, _) => {
                self.A = self.stack_pop()?;
                flags_zn!(self, self.A);
            }
            (OpCode::Plp, _) => {
                let status = self.stack_pop()?;
                self.status_set(status);
            }
            (OpCode::And, mode) => {
                let addr = self.fetch_addr(&mode)?;
                self.A &= self.read(addr)?;
                flags_zn!(self, self.A);
            }
            (OpCode::Eor, mode) => {
                let addr = self.fetch_addr(&mode)?;
                self.A ^= self.read(addr)?;
                flags_zn!(self, self.A);
            }
            (OpCode::Ora, mode) => {
                let addr = self.fetch_addr(&mode)?;
                self.A |= self.read(addr)?;
                flags_zn!(self, self.A);
            }
            (OpCode::Bit, mode) => {
                let addr = self.fetch_addr(&mode)?;
                let data = self.read(addr)?;
                self.Z = (data & self.A) == 0;
                self.N = (data & 0b1000_0000) > 0;
                self.V = (data & 0b0100_0000) > 0;
            }
            (OpCode::Adc, mode) => {
                let addr = self.fetch_addr(&mode)?;
                let data = self.read(addr)?;
                let (result1, carry1) = self.A.overflowing_add(data);
                let (result2, carry2) = result1.overflowing_add(u8::from(self.C));
                self.A = result2;
                self.C = carry1 || carry2;
                flags_zn!(self, self.A);
            }
            (OpCode::Sbc, mode) => {
                let addr = self.fetch_addr(&mode)?;
                let data = self.read(addr)?;
                let (result1, carry1) = self.A.overflowing_sub(data);
                let (result2, carry2) = result1.overflowing_sub(u8::from(!self.C));
                self.A = result2;
                self.C = !(carry1 || carry2);
                flags_zn!(self, self.A);
            }
            (OpCode::Cmp, mode) => {
                compare!(self, A, mode);
            }
            (OpCode::Cpx, mode) => {
                compare!(self, X, mode);
            }
            (OpCode::Cpy, mode) => {
                compare!(self, Y, mode);
            }
            (OpCode::Inc, mode) => {
                let addr = self.fetch_addr(&mode)?;
                let mut data = self.read(addr)?;
                data = data.wrapping_add(1);
                self.write(addr, data)?;
                flags_zn!(self, data);
            }
            (OpCode::Inx, _) => {
                self.X = self.X.wrapping_add(1);
                flags_zn!(self, self.X);
            }
            (OpCode::Iny, _) => {
                self.Y = self.Y.wrapping_add(1);
                flags_zn!(self, self.Y);
            }
            (OpCode::Dec, mode) => {
                let addr = self.fetch_addr(&mode)?;
                let mut data = self.read(addr)?;
                data = data.wrapping_sub(1);
                self.write(addr, data)?;
                flags_zn!(self, data);
            }
            (OpCode::Dex, _) => {
                self.X = self.X.wrapping_sub(1);
                flags_zn!(self, self.X);
            }
            (OpCode::Dey, _) => {
                self.Y = self.Y.wrapping_sub(1);
                flags_zn!(self, self.Y);
            }
            (OpCode::Asl, mode) => {
                let (addr, mut data) = if matches!(mode, AddressMode::Accumulator) {
                    (None, self.A)
                } else {
                    let addr = self.fetch_addr(&mode)?;
                    let data = self.read(addr)?;
                    (Some(addr), data)
                };

                self.C = data & 0b1000_0000 > 0;
                data <<= 1;

                if let Some(addr) = addr {
                    self.write(addr, data)?;
                } else {
                    self.A = data;
                }

                self.Z = data == 0;
                self.N = data & 0b1000_0000 > 0;
            }
            (OpCode::Lsr, mode) => {
                let (addr, mut data) = if matches!(mode, AddressMode::Accumulator) {
                    (None, self.A)
                } else {
                    let addr = self.fetch_addr(&mode)?;
                    let data = self.read(addr)?;
                    (Some(addr), data)
                };

                self.C = data & 0b0000_0001 > 0;
                data >>= 1;

                if let Some(addr) = addr {
                    self.write(addr, data)?;
                } else {
                    self.A = data;
                }

                // flags_zn!(self, data); < Docs said this, and I dont believe it!
                self.Z = data == 0;
                self.N = data & 0b1000_0000 > 0;
            }
            (OpCode::Rol, mode) => {
                let (addr, mut data) = if matches!(mode, AddressMode::Accumulator) {
                    (None, self.A)
                } else {
                    let addr = self.fetch_addr(&mode)?;
                    let data = self.read(addr)?;
                    (Some(addr), data)
                };

                let old_c = self.C;
                self.C = data & 0b1000_0000 > 0;
                data <<= 1;
                data += u8::from(old_c);

                if let Some(addr) = addr {
                    self.write(addr, data)?;
                } else {
                    self.A = data;
                }

                self.Z = data == 0;
                self.N = data & 0b1000_0000 > 0;
            }
            (OpCode::Ror, mode) => {
                let (addr, mut data) = if matches!(mode, AddressMode::Accumulator) {
                    (None, self.A)
                } else {
                    let addr = self.fetch_addr(&mode)?;
                    let data = self.read(addr)?;
                    (Some(addr), data)
                };

                let old_c = self.C;
                self.C = data & 0b0000_0001 > 0;
                data >>= 1;
                if old_c {
                    data |= 0b1000_0000;
                }

                if let Some(addr) = addr {
                    self.write(addr, data)?;
                } else {
                    self.A = data;
                }

                self.Z = data == 0;
                self.N = data & 0b1000_0000 > 0;
            }
            (OpCode::Jmp, AddressMode::Absolute) => {
                self.PC = self.fetch_word()?;
            }
            (OpCode::Jmp, AddressMode::Indirect) => {
                let addr = self.fetch_word()?;
                self.PC = self.read_word(addr)?;
            }
            (OpCode::Jsr, _) => {
                let new_pc = self.fetch_word()?;
                let Address::Full(addr, page) = Address::from(self.PC - 1) else {
                    return Err("could not construct address from PC".into());
                };
                self.stack_push(page)?;
                self.stack_push(addr)?;
                self.PC = new_pc;
            }
            (OpCode::Rts, _) => {
                let addr = self.stack_pop()?;
                let page = self.stack_pop()?;
                self.PC = Address::Full(addr, page).into();
                self.PC += 1;
            }
            (OpCode::Bcc, _) => {
                if branch!(self, !self.C) {
                    noop += 1;
                }
            }
            (OpCode::Bcs, _) => {
                if branch!(self, self.C) {
                    noop += 1;
                }
            }
            (OpCode::Beq, _) => {
                if branch!(self, self.Z) {
                    noop += 1;
                }
            }
            (OpCode::Bmi, _) => {
                if branch!(self, self.N) {
                    noop += 1;
                }
            }
            (OpCode::Bne, _) => {
                if branch!(self, !self.Z) {
                    noop += 1;
                }
            }
            (OpCode::Bpl, _) => {
                if branch!(self, !self.N) {
                    noop += 1;
                }
            }
            (OpCode::Bvc, _) => {
                if branch!(self, !self.V) {
                    noop += 1;
                }
            }
            (OpCode::Bvs, _) => {
                if branch!(self, self.V) {
                    noop += 1;
                }
            }
            (OpCode::Clc, _) => {
                self.C = false;
            }
            (OpCode::Cld, _) => {
                self.D = false;
            }
            (OpCode::Cli, _) => {
                self.I = false;
            }
            (OpCode::Clv, _) => {
                self.V = false;
            }
            (OpCode::Sec, _) => {
                self.C = true;
            }
            (OpCode::Sed, _) => {
                self.D = true;
                return Err("decimal Mode not supported".into());
            }
            (OpCode::Sei, _) => {
                self.I = true;
            }
            (OpCode::Brk, _) => {
                self.interrupt(0)?;
            }
            (OpCode::Rti, _) => {
                self.I = false;

                let status = self.stack_pop()?;
                self.status_set(status);

                let addr = self.stack_pop()?;
                let page = self.stack_pop()?;
                self.PC = Address::Full(addr, page).into();
                self.PC += 1;
            }
            (OpCode::Nop, _) => {}
            (op, mode) => todo!("{:?}({:?})", op, mode),
        }

        if let Mode::Original(_) = self.mode {
            self.mode = Mode::Original(noop);
        }

        Ok(())
    }
}

impl<T> Interruptible for Cpu<T>
where
    T: Addressable,
{
    fn interrupt(
        &mut self,
        tp: impl Into<crate::Interrupt>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let int_addr = match tp.into() {
            0 => IRQB,
            _ => NMIB,
        };

        if self.I && int_addr == IRQB {
            return Ok(());
        }

        let Address::Full(addr, page) = Address::from(self.PC - 1) else {
            return Err("could not construct address from PC".into());
        };
        self.stack_push(page)?;
        self.stack_push(addr)?;
        self.PC = self.read_word(int_addr)?;

        self.stack_push(self.status_get())?;

        self.I = true;

        Ok(())
    }
}

impl<T> Resettable for Cpu<T>
where
    T: Addressable,
{
    /// Reset processor
    fn reset(&mut self) -> Result<(), Box<dyn Error>> {
        self.SP = SP_ADDR;
        self.PC = self.read_word(RESB)?;

        self.A = 0;
        self.X = 0;
        self.Y = 0;

        // * => Set by software?
        // self.C = false; // *
        // self.Z = false; // *
        self.I = false;
        self.D = false;
        self.B = false;
        // self.V = false; // *
        // self.N = false; // *

        if let Mode::Original(_) = self.mode {
            self.mode = Mode::Original(0);
        }

        Ok(())
    }
}
