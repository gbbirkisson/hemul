use self::address::Address;
use crate::{Addressable, Byte, TickError, Tickable, Word};
use instructions::{Op, OpHandler};

pub(crate) mod address;
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

    PC: Word, // Program Counter
    SP: Byte, // Stack Pointer

    A: Byte, // Accumulator
    X: Byte, // Index Register X
    Y: Byte, // Index Register Y

    C: PFlag, // Carry Flag
    Z: PFlag, // Zero Flag
    I: PFlag, // Interupt Disable
    D: PFlag, // Decimal Mode
    B: PFlag, // Break Command
    V: PFlag, // Overflow Flag
    N: PFlag, // Negative Flag

    op: Op,    // Current Op Code
    st: State, // Other state
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    BadOpCode(Byte),
    OutOfBounds(Word),
    StackOverflow,
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        match value {
            Error::BadOpCode(code) => format!("BadOpCode: {code:#04x}"),
            Error::OutOfBounds(addr) => format!("OutOfBounds: {addr:#06x}"),
            Error::StackOverflow => "StackOverflow".to_string(),
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
    None,
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

            op: Op::None,
            st: State::Reset,
        }
    }

    fn reset(&mut self) {
        self.st = State::Reset;
    }

    fn write(&mut self, addr: impl Into<Word>, value: impl Into<Byte>) -> Result<(), Error> {
        let addr = addr.into();
        if self.addr.inside_bounds(addr) {
            self.addr[addr] = value.into();
            Ok(())
        } else {
            Err(Error::OutOfBounds(addr))
        }
    }

    fn read(&self, addr: impl Into<Word>) -> Result<Byte, Error> {
        let addr = addr.into();
        if self.addr.inside_bounds(addr) {
            Ok(self.addr[addr])
        } else {
            Err(Error::OutOfBounds(addr))
        }
    }

    fn fetch(&mut self) -> Result<Byte, Error> {
        let res = self.read(self.PC)?;
        self.PC += 1;
        Ok(res)
    }

    fn fetch_word(&mut self) -> Result<Word, Error> {
        Ok(Address::Full(self.fetch()?, self.fetch()?).into())
    }

    fn stack_push(&mut self, byte: impl Into<Byte>) -> Result<(), Error> {
        if self.SP == 0 {
            Err(Error::StackOverflow)
        } else {
            self.write(Address::from((self.SP, SP_PAGE)), byte.into())?;
            self.SP -= 1;
            Ok(())
        }
    }

    fn stack_pop(&mut self) -> Result<Byte, Error> {
        if self.SP == SP_ADDR {
            Err(Error::StackOverflow)
        } else {
            let res = self.read(Address::from((self.SP, SP_PAGE)))?;
            self.SP += 1;
            Ok(res)
        }
    }

    pub fn tick_until_nop(&mut self) -> Result<(), TickError> {
        loop {
            if matches!(&self.op, Op::Nop) {
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
        if !matches!(&self.st, State::None) {
            dbg!(&self.st);
        }
        if !matches!(&self.op, Op::None | Op::CycleBurn(_, _, _)) {
            dbg!(&self.op);
        }
        match (&self.st, &self.op) {
            // Handle special states
            (State::Reset, _) => {
                self.SP = SP_ADDR;

                self.PC = Address::from((self.read(RESB.0)?, self.read(RESB.1)?)).into();

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

                self.st = State::None;
                self.op = Op::None;
            }
            (State::Interupt(Interupt::Irqb), _) => todo!(),
            (State::Interupt(Interupt::Nmib), _) => todo!(),

            // Handle opcodes
            (_, _) => {
                self.op = self.handle(self.op.clone())?;
            }
        }
        Ok(())
    }
}
