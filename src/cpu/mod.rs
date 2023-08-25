use self::snapshots::Snapshot;
use crate::device::{Addressable, Tickable};
use instructions::{MemAddr, Op};

mod instructions;
mod snapshots;

pub type Reg16 = u16;
pub type Reg8 = u8;
pub type PFlag = bool;

pub(crate) const SP_START: u16 = 0x00ff;
pub(crate) const PC_START: u16 = 0xfffc;

#[allow(non_snake_case, dead_code)]
pub struct Cpu<T: Addressable> {
    addr: T,

    PC: Reg16, // Program Counter
    SP: Reg16, // Stack Pointer

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
            match self.op {
                Op::Nop | Op::Error(_) => break,
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
