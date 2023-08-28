use crate::cpu::address::Address;
use crate::cpu::{Addressable, Cpu, Error};
use crate::Byte;

pub trait OpHandler {
    fn handle(&mut self, op: Op) -> Result<Op, Error>;
}

#[derive(Debug, Clone)]
pub enum Op {
    None,
    CycleBurn(u8, u8, Box<Op>),

    Nop,

    LdaIm,
    LdxIm,

    AdcIm,

    Txs,

    Jsr,
    Pha,

    StaAbs,
}

impl TryFrom<Byte> for Op {
    type Error = Error;

    fn try_from(value: Byte) -> Result<Self, Self::Error> {
        match value {
            0xEA => Ok(Self::Nop),

            0xA9 => Ok(Self::LdaIm),
            0xA2 => Ok(Self::LdxIm),

            0x69 => Ok(Self::AdcIm),

            0x9A => Ok(Self::Txs),

            0x20 => Ok(Self::CycleBurn(0, 6, Box::new(Self::Jsr))),
            0x48 => Ok(Self::CycleBurn(0, 3, Box::new(Self::Pha))),

            0x8d => Ok(Self::CycleBurn(0, 3, Box::new(Self::StaAbs))),

            _ => Err(Error::BadOpCode(value)),
        }
    }
}

impl<T> OpHandler for Cpu<T>
where
    T: Addressable,
{
    fn handle(&mut self, op: Op) -> Result<Op, Error> {
        Ok(match op {
            // None => Try to load next instruction
            Op::None => Op::try_from(self.fetch()?)?,

            // Burn cycles to simulate how many each operation takes
            Op::CycleBurn(curr, total, op) => {
                if curr == total - 2 {
                    *op
                } else {
                    Op::CycleBurn(curr + 1, total, op)
                }
            }

            // NOP - No Operation
            // The NOP instruction causes no changes to the processor other than the normal
            // incrementing of the program counter to the next instruction.
            Op::Nop => Op::None,

            // LDA - Load Accumulator
            // Loads a byte of memory into the accumulator setting the zero and negative flags
            // as appropriate.
            Op::LdaIm => {
                self.A = self.fetch()?;
                self.Z = self.A == 0;
                self.N = (self.A & 0b100_0000) > 0;
                Op::None
            }

            // LDX - Load X Register
            // Loads a byte of memory into the X register setting the zero and negative flags
            // as appropriate.
            Op::LdxIm => {
                self.X = self.fetch()?;
                self.Z = self.A == 0;
                self.N = (self.A & 0b100_0000) > 0;
                Op::None
            }

            // ADC - Add with Carry
            // This instruction adds the contents of a memory location to the accumulator
            // together with the carry bit. If overflow occurs the carry bit is set, this
            // enables multiple byte addition to be performed.
            Op::AdcIm => {
                self.A += self.fetch()?;
                // TODO SIDE EFFECTS
                Op::None
            }

            // TXS - Transfer X to Stack Pointer
            // Copies the current contents of the X register into the stack register.
            Op::Txs => {
                self.SP = self.X;
                Op::None
            }

            // JSR - Jump to Subroutine
            // The JSR instruction pushes the address (minus one) of the return point on to
            // the stack and then sets the program counter to the target memory address.
            Op::Jsr => {
                let Address::Full(addr, page) = Address::from(self.PC - 1) else { panic!() };
                self.stack_push(page)?;
                self.stack_push(addr)?;
                self.PC = self.fetch_word()?;
                Op::None
            }

            // PHA - Push Accumulator
            // Pushes a copy of the accumulator on to the stack
            Op::Pha => {
                self.stack_push(self.A)?;
                Op::None
            }

            // STA - Store Accumulator
            // Stores the contents of the accumulator into memory.
            Op::StaAbs => {
                let data = self.fetch_word()?;
                self.write(data, self.A)?;
                Op::None
            }
        })
    }
}
