use self::{address::Address, instructions::AddressMode};
use crate::{Addressable, Byte, TickError, Tickable, Word};
use executor::OpExecutor;
use instructions::{Op, OpParser};

pub(crate) mod address;
mod executor;
mod instructions;
pub mod snapshot;

pub(crate) type PFlag = bool;

pub(crate) const SP_PAGE: Byte = 0x01;
pub(crate) const SP_ADDR: Byte = 0xFF;
#[allow(dead_code)]
pub(crate) const NMIB: (Word, Word) = (0xFFFA, 0xFFFB);
pub(crate) const RESB: (Word, Word) = (0xFFFC, 0xFFFD);
#[allow(dead_code)]
pub(crate) const IRQB: (Word, Word) = (0xFFFE, 0xFFFF);

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
    /// Interupt Disable
    I: PFlag,
    /// Decimal Mode
    D: PFlag,
    /// Break Command
    B: PFlag,
    /// Overflow Flag
    V: PFlag,
    /// Negative Flag
    N: PFlag,

    /// Processor state
    st: Option<State>,
}

#[derive(Debug)]
pub enum Error {
    BadOpCode(Byte),
    OutOfBounds(Word),
    StackOverflow,
    Other(String),
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        match value {
            Error::BadOpCode(code) => format!("BadOpCode: {code:#04x}"),
            Error::OutOfBounds(addr) => format!("OutOfBounds: {addr:#06x}"),
            Error::StackOverflow => "StackOverflow".to_string(),
            Error::Other(msg) => format!("Other: {msg}"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Interupt {
    Irqb,
    Nmib,
}

#[derive(Debug)]
enum State {
    CycleBurn(Op, u8, u8),
    Reset,
    #[allow(dead_code)]
    Interupt(Interupt),
}

#[allow(dead_code)]
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

            st: Some(State::Reset),
        }
    }

    /// Reset processor
    fn reset(&mut self) {
        self.st = Some(State::Reset);
    }

    /// Read byte from address
    fn read(&self, addr: impl Into<Word>) -> Result<(Word, Byte), Error> {
        let addr = addr.into();
        if self.addr.inside_bounds(addr) {
            Ok((addr, self.addr[addr]))
        } else {
            Err(Error::OutOfBounds(addr))
        }
    }

    /// Read a word from address and address + 1
    fn read_word(&self, addr: impl Into<Word>) -> Result<Word, Error> {
        let a1 = addr.into();
        let a2 = a1 + 1;
        let (_, addr) = self.read(a1)?;
        let (_, page) = self.read(a2)?;
        Ok(Address::Full(addr, page).into())
    }

    /// Fetch byte from memory that the PC points to and increment the PC
    fn fetch(&mut self) -> Result<(Word, Byte), Error> {
        let (addr, data) = self.read(self.PC)?;
        self.PC += 1;
        Ok((addr, data))
    }

    /// Fetch word from memory that the PC points to and increment the PC twice
    fn fetch_word(&mut self) -> Result<Word, Error> {
        let data = self.read_word(self.PC)?;
        self.PC += 2;
        Ok(data)
    }

    /// Fetch byte from memory using provided address mode
    fn fetch_mode(&mut self, mode: &AddressMode) -> Result<(Word, Byte), Error> {
        Ok(match mode {
            AddressMode::Accumulator => todo!(),
            AddressMode::Immediate => self.fetch()?,
            AddressMode::ZeroPage => todo!(),
            AddressMode::ZeroPageX => todo!(),
            AddressMode::ZeroPageY => todo!(),
            AddressMode::Absolute => {
                let addr = self.fetch_word()?;
                self.read(addr)?
            }
            AddressMode::AbsoluteX => todo!(),
            AddressMode::AbsoluteY => todo!(),
            AddressMode::Indirect => todo!(),
            AddressMode::IndexedIndirect => todo!(),
            AddressMode::IndirectIndexed => todo!(),
        })
    }

    /// Write byte to address
    fn write(&mut self, addr: impl Into<Word>, value: impl Into<Byte>) -> Result<(), Error> {
        let addr = addr.into();
        if self.addr.inside_bounds(addr) {
            self.addr[addr] = value.into();
            Ok(())
        } else {
            Err(Error::OutOfBounds(addr))
        }
    }

    /// Write byte into memory using provided address mode
    fn write_mode(&mut self, mode: &AddressMode, value: impl Into<Byte>) -> Result<(), Error> {
        match mode {
            AddressMode::Accumulator => todo!(),
            AddressMode::Immediate => todo!(),
            AddressMode::ZeroPage => todo!(),
            AddressMode::ZeroPageX => todo!(),
            AddressMode::ZeroPageY => todo!(),
            AddressMode::Absolute => {
                let data = self.fetch_word()?;
                self.write(data, value)?;
            }
            AddressMode::AbsoluteX => todo!(),
            AddressMode::AbsoluteY => todo!(),
            AddressMode::Indirect => todo!(),
            AddressMode::IndexedIndirect => todo!(),
            AddressMode::IndirectIndexed => todo!(),
        };
        Ok(())
    }

    /// Push byte onto the stack
    fn stack_push(&mut self, byte: impl Into<Byte>) -> Result<(), Error> {
        if self.SP == 0 {
            Err(Error::StackOverflow)
        } else {
            self.write(Address::from((self.SP, SP_PAGE)), byte.into())?;
            self.SP -= 1;
            Ok(())
        }
    }

    /// Pop byte from the stack
    fn stack_pop(&mut self) -> Result<Byte, Error> {
        if self.SP == SP_ADDR {
            Err(Error::StackOverflow)
        } else {
            let (_, data) = self.read(Address::from((self.SP, SP_PAGE)))?;
            self.SP += 1;
            Ok(data)
        }
    }

    /// Get status register
    fn status_get(&self) -> Byte {
        let mut res = 0;
        if self.C {
            res |= 0b0100_0000;
        }
        if self.Z {
            res |= 0b0010_0000;
        }
        if self.I {
            res |= 0b0001_0000;
        }
        if self.D {
            res |= 0b0000_1000;
        }
        if self.B {
            res |= 0b0000_0100;
        }
        if self.V {
            res |= 0b0000_0010;
        }
        if self.N {
            res |= 0b0000_0001;
        }
        res
    }

    /// Set status register
    fn status_set(&mut self, status: Byte) {
        self.C = status & 0b0100_0000 > 0;
        self.Z = status & 0b0010_0000 > 0;
        self.I = status & 0b0001_0000 > 0;
        self.D = status & 0b0000_1000 > 0;
        self.B = status & 0b0000_0100 > 0;
        self.V = status & 0b0000_0010 > 0;
        self.N = status & 0b0000_0001 > 0;
    }

    pub fn tick_until_nop(&mut self) -> Result<(), TickError> {
        loop {
            if matches!(&self.st, Some(State::CycleBurn(Op::Nop, _, _))) {
                return self.tick();
            }
            self.tick()?;
        }
    }

    pub fn tick_for(&mut self, count: usize) -> Result<(), TickError> {
        for _ in 0..count {
            self.tick()?;
        }
        Ok(())
    }
}

impl<T> Tickable for Cpu<T>
where
    T: Addressable,
{
    fn tick(&mut self) -> Result<(), TickError> {
        self.st = match &self.st {
            None => {
                let pc = self.PC;
                let (_, op_code) = self.fetch()?;
                let (op, cycles) = self.parse(op_code)?;
                dbg!(format!("{:#01x} {:?}", pc, op));
                Some(State::CycleBurn(op, 1, cycles))
            }
            Some(state) => match state {
                State::CycleBurn(op, curr, total) => {
                    let op = op.clone();
                    let curr = curr + 1;
                    if curr == *total {
                        self.execute(op)?; // TODO: Print nice debug message with PC
                        None
                    } else {
                        Some(State::CycleBurn(op, curr, *total))
                    }
                }
                State::Interupt(Interupt::Irqb) => todo!(),
                State::Interupt(Interupt::Nmib) => todo!(),
                State::Reset => {
                    self.SP = SP_ADDR;

                    self.PC = dbg!(self.read_word(RESB.0)?);

                    self.A = 0;
                    self.X = 0;
                    self.Y = 0;

                    // * => Set by software?
                    self.C = false; // *
                    self.Z = false; // *
                    self.I = true;
                    self.D = false;
                    self.B = true;
                    self.V = false; // *
                    self.N = false; // *

                    None
                }
            },
        };
        Ok(())
    }
}
