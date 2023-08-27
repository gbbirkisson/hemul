use self::snapshots::Snapshot;
use crate::device::*;
use instructions::*;

mod instructions;
mod snapshots;

pub type PFlag = bool;

pub(crate) const SP: Byte = 0xFF;
pub(crate) const NMIB: (Word, Word) = (0xFFFA, 0xFFFB);
pub(crate) const RESB: (Word, Word) = (0xFFFC, 0xFFFD);
pub(crate) const IRQB: (Word, Word) = (0xFFFE, 0xFFFF);

#[allow(non_snake_case, dead_code)]
pub struct Cpu<T: Addressable> {
    addr: T,

    PC: Word, // Program Counter
    SP: Byte,  // Stack Pointer

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

    st: State,
    op: Op,
}

#[derive(Debug)]
pub enum CpuError {
    BadOpCode(Byte),
}

#[derive(Debug)]
enum Interupt {
    IRQB,
    NMIB,
}

#[derive(Debug)]
enum State {
    None,
    Reset,
    Interupt(Interupt),
    Error(CpuError),
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
        self.st = State::Reset
    }

    fn read(&self, addr: Word) -> Byte {
        self.addr[addr]
    }

    fn write(&mut self, addr: Word, value: Byte) {
        self.addr[addr] = value;
    }

    fn fetch(&mut self) -> Byte {
        let res = self.read(self.PC);
        self.PC += 1;
        res
    }

    pub fn snapshot(&self) -> Option<Snapshot> {
        match self.op {
            Op::None => {
                let mut dump = Vec::with_capacity(std::u16::MAX as usize);
                for addr in 0..std::u16::MAX {
                    dump.push(self.read(addr));
                }
                Some(Snapshot {
                    dump,
                    PC: self.PC,
                    SP: self.SP,

                    A: self.A,
                    X: self.X,
                    Y: self.Y,

                    C: self.C,
                    Z: self.Z,
                    I: self.I,
                    D: self.D,
                    B: self.B,
                    V: self.V,
                    N: self.N,
                })
            }
            _ => None,
        }
    }

    pub fn tick_until_nop(&mut self) {
        loop {
            match (&self.st, &self.op) {
                (State::Error(_), _) => break,
                (_, Op::Nop) => break,
                _ => {}
            }
            self.tick();
        }
    }

    pub fn tick_for(&mut self, count: usize) {
        for _ in 0..count {
            self.tick();
        }
    }
}

impl<T> Tickable for Cpu<T>
where
    T: Addressable,
{
    fn tick(&mut self) {
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
            (State::Interupt(Interupt::IRQB), _) => todo!(),
            (State::Interupt(Interupt::NMIB), _) => todo!(),
            (State::Error(_), _) => {
                // Do nothing
            },

            // Handle opcodes
            (_, _) => {
                self.op = self.handle(self.op.clone());
            }
        }
    }
}
