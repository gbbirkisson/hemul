use std::ops::{Index, IndexMut};

type Reg16 = u16;
type Reg8 = u8;
type PFlag = bool;

pub const PC_START: u16 = 0xFF00;
const SP_START: u8 = 0x00FF;

#[allow(non_snake_case)]
pub struct Cpu<T> {
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

    op: Option<Op>,
}

#[derive(Debug, Clone)]
enum MemAddr {
    None,
    Half(u8),
    Full(u16),
}

#[derive(Debug, Clone)]
enum Op {
    LdaIm,
    AdcIm,
    StaAbs(MemAddr),
}

impl From<u8> for Op {
    fn from(value: u8) -> Self {
        match value {
            0xA9 => Op::LdaIm,
            0x69 => Op::AdcIm,
            0x8d => Op::StaAbs(MemAddr::None),
            _ => panic!("Invalid instruction: {:#01x}", value),
        }
    }
}

impl<T> Cpu<T>
where
    T: Index<u16, Output = u8>,
    T: IndexMut<u16, Output = u8>,
{
    pub fn new(addr: T) -> Self {
        let mut cpu = Self {
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

            op: None,
        };
        cpu.reset();
        cpu
    }

    fn reset(&mut self) {
        self.PC = PC_START;
        self.SP = SP_START;

        self.A = 0;
        self.X = 0;
        self.Y = 0;

        self.C = false;
        self.Z = false;
        self.I = false;
        self.D = false;
        self.B = false;
        self.V = false;
        self.N = false;

        self.op = None;
    }

    fn read(&mut self) -> u8 {
        let res = self.addr[self.PC];
        self.PC += 1;
        res
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.addr[addr] = value;
    }

    pub fn tick(&mut self) {
        match dbg!(&self.op) {
            None => {
                self.op = Some(self.read().into());
            }
            Some(op) => {
                self.op = self.handle_op(op.clone());
            }
        }
    }

    pub fn tick_for(&mut self, count: usize) {
        for _ in 0..count {
            self.tick()
        }
    }

    fn handle_op(&mut self, op: Op) -> Option<Op> {
        match op {
            Op::LdaIm => {
                self.A = self.read();
                self.Z = self.A == 0;
                self.N = (self.A & 0b1000000) > 0;
                // TODO SIDE EFFECTS
                None
            }
            Op::AdcIm => {
                self.A += self.read();
                // TODO SIDE EFFECTS
                None
            }
            Op::StaAbs(MemAddr::None) => Some(Op::StaAbs(MemAddr::Half(self.read()))),
            Op::StaAbs(MemAddr::Half(addr1)) => {
                Some(Op::StaAbs(MemAddr::from((addr1, self.read()))))
            }
            Op::StaAbs(MemAddr::Full(addr)) => {
                self.write(addr, self.A);
                None
            }
        }
    }
}

impl From<(u8, u8)> for MemAddr {
    fn from((addr1, addr2): (u8, u8)) -> Self {
        let addr1 = addr1 as u16;
        let addr2 = addr2 as u16;
        MemAddr::Full(addr2 << 8 | addr1)
    }
}
