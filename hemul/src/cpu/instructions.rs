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

            Op::CycleBurn(curr, total, op) => {
                if curr == total - 2 {
                    *op
                } else {
                    Op::CycleBurn(curr + 1, total, op)
                }
            }

            Op::Nop => Op::None,

            Op::LdaIm => {
                self.A = self.fetch()?;
                self.Z = self.A == 0;
                self.N = (self.A & 0b100_0000) > 0;
                Op::None
            }

            Op::LdxIm => {
                self.X = self.fetch()?;
                self.Z = self.A == 0;
                self.N = (self.A & 0b100_0000) > 0;
                Op::None
            }

            Op::AdcIm => {
                self.A += self.fetch()?;
                // TODO SIDE EFFECTS
                Op::None
            }

            Op::Txs => {
                self.SP = self.X;
                Op::None
            }

            Op::Jsr => {
                todo!()
            }

            Op::StaAbs => {
                let data = self.fetch_word()?;
                self.write(data, self.A)?;
                Op::None
            }
        })
    }
}
