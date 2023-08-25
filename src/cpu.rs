use crate::device::{Addressable, Tickable};

type Reg16 = u16;
type Reg8 = u8;
type PFlag = bool;

const SP_START: u8 = 0x00ff;
const PC_START: u16 = 0xfffc;

#[allow(non_snake_case, dead_code)]
pub struct Cpu<T: Addressable> {
    addr: T,

    PC: Reg16, // Program Counter
    SP: Reg8,  // Stack Pointer

    A: Reg8, // Accumulator
    X: Reg8, // Index Register X
    Y: Reg8, // Index Register Y

    C: PFlag, // Carry Flag
    Z: PFlag, // Zero Flag
    I: PFlag, // Interupt Disable
    D: PFlag, // Decimal Mode
    B: PFlag, // Break Command
    V: PFlag, // Overflow Flag
    N: PFlag, // Negative Flag

    op: Op,
}

#[derive(Debug, Clone, Copy)]
enum CpuError {
    BadOpCode(u8),
}

#[derive(Debug, Clone)]
enum MemAddr {
    None,
    Half(u8),
    Full(u16),
}

#[derive(Debug, Clone)]
enum Op {
    // Custom Ops
    Error(CpuError),
    Reset(MemAddr),
    None,

    Nop,

    LdaIm,
    AdcIm,
    StaAbs(MemAddr),
}

impl From<u8> for Op {
    fn from(value: u8) -> Self {
        match value {
            0xEA => Op::Nop,
            0xA9 => Op::LdaIm,
            0x69 => Op::AdcIm,
            0x8d => Op::StaAbs(MemAddr::None),
            _ => Op::Error(CpuError::BadOpCode(value)),
        }
    }
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

            op: Op::Reset(MemAddr::None),
        }
    }

    fn reset(&mut self) {
        self.op = Op::Reset(MemAddr::None);
    }

    fn read(&self, addr: u16) -> u8 {
        self.addr[addr]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.addr[addr] = value;
    }

    fn fetch(&mut self) -> u8 {
        let res = self.read(self.PC);
        self.PC += 1;
        res
    }

    pub fn tick_until_nop(&mut self) {
        loop {
            match self.op {
                Op::Nop | Op::Error(_) => break,
                _ => {}
            }
            self.tick();
        }
    }

    pub fn tick_for(&mut self, count: usize) {
        for _ in 0..count {
            self.tick()
        }
    }
}

impl<T> Tickable for Cpu<T>
where
    T: Addressable,
{
    fn tick(&mut self) {
        dbg!(&self.op);
        self.op = match self.op {
            // If the CPU had an error do nothing
            Op::Error(error) => Op::Error(error),

            // Reset processor, this should take 7 ticks, but we are just using 3.
            Op::Reset(MemAddr::None) => {
                self.SP = SP_START;
                self.PC = PC_START;

                // self.A = 0;
                // self.X = 0;
                // self.Y = 0;

                // * => Set by software?
                self.C = false; // *
                self.Z = false; // *
                self.I = true;
                self.D = false;
                self.B = true;
                self.V = false; // *
                self.N = false; // *

                Op::Reset(MemAddr::Half(self.fetch()))
            }
            Op::Reset(MemAddr::Half(addr)) => Op::Reset(MemAddr::from((addr, self.fetch()))),
            Op::Reset(MemAddr::Full(addr)) => {
                self.PC = addr;
                Op::None
            }

            // No Op running, fetch next one
            Op::None => Op::from(self.fetch()),

            // Nop
            Op::Nop => Op::None,

            // Lda
            Op::LdaIm => {
                self.A = self.fetch();
                self.Z = self.A == 0;
                self.N = (self.A & 0b1000000) > 0;
                // TODO SIDE EFFECTS
                Op::None
            }

            // Adc
            Op::AdcIm => {
                self.A += self.fetch();
                // TODO SIDE EFFECTS
                Op::None
            }

            // Sta
            Op::StaAbs(MemAddr::None) => Op::StaAbs(MemAddr::Half(self.fetch())),
            Op::StaAbs(MemAddr::Half(addr)) => Op::StaAbs(MemAddr::from((addr, self.fetch()))),
            Op::StaAbs(MemAddr::Full(addr)) => {
                self.write(addr, self.A);
                Op::None
            }
        };
    }
}

impl From<(u8, u8)> for MemAddr {
    fn from((addr, page): (u8, u8)) -> Self {
        let addr = addr as u16;
        let page = page as u16;
        MemAddr::Full(page << 8 | addr)
    }
}
