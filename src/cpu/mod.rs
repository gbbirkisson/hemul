use self::address::Address;
use crate::{Addressable, Byte, Snapshottable, Tickable, Word};
use instructions::{Op, OpHandler};

pub(crate) mod address;
mod instructions;
pub mod snapshots;

pub(crate) type PFlag = bool;

pub(crate) const SP: Byte = 0xFF;
#[allow(dead_code)]
pub(crate) const NMIB: (Word, Word) = (0xFFFA, 0xFFFB);
pub(crate) const RESB: (Word, Word) = (0xFFFC, 0xFFFD);
#[allow(dead_code)]
pub(crate) const IRQB: (Word, Word) = (0xFFFE, 0xFFFF);

#[allow(non_snake_case, dead_code)]
pub struct Cpu<T: Addressable + Snapshottable> {
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
    T: Addressable + Snapshottable,
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

    fn write(&mut self, addr: impl Into<Word>, value: impl Into<Byte>) {
        self.addr[addr.into()] = value.into();
    }

    fn read(&self, addr: impl Into<Word>) -> Byte {
        self.addr[addr.into()]
    }

    fn fetch(&mut self) -> Byte {
        let res = self.read(self.PC);
        self.PC += 1;
        res
    }

    pub fn tick_until_nop(&mut self) -> Result<(), Error> {
        loop {
            if matches!(&self.op, Op::Nop) {
                return self.tick();
            }
            self.tick()?;
        }
    }

    pub fn tick_for(&mut self, count: usize) -> Result<(), Error> {
        for _ in 0..count {
            self.tick()?;
        }
        Ok(())
    }
}

impl<T> Tickable for Cpu<T>
where
    T: Addressable + Snapshottable,
{
    type Error = Error;

    fn tick(&mut self) -> Result<(), Self::Error> {
        dbg!(&self.st, &self.op);
        match (&self.st, &self.op) {
            // Handle special states
            (State::Reset, _) => {
                self.SP = SP;

                self.PC = Address::from((self.read(RESB.0), self.read(RESB.1))).into();

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
