use crate::cpu::address::Address;
use crate::cpu::{Addressable, Cpu, Error};
use crate::Byte;

pub trait OpHandler {
    fn handle(&mut self, op: Op) -> Result<Op, Error>;
}

impl TryFrom<Byte> for Op {
    type Error = Error;

    fn try_from(value: Byte) -> Result<Self, Self::Error> {
        match value {
            0xEA => Ok(Self::Nop),
            0xA9 => Ok(Self::LdaIm),
            0x69 => Ok(Self::AdcIm),
            0x8d => Ok(Self::StaAbs(None)),
            _ => Err(Error::BadOpCode(value)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Op {
    None,

    Nop,

    LdaIm,
    AdcIm,
    StaAbs(Option<Address>),
}

impl<T> OpHandler for Cpu<T>
where
    T: Addressable,
{
    fn handle(&mut self, op: Op) -> Result<Op, Error> {
        Ok(match op {
            // None => Try to load next instruction
            Op::None => Op::try_from(self.fetch())?,

            // Nop
            Op::Nop => Op::None,

            // Lda
            Op::LdaIm => {
                self.A = self.fetch();
                self.Z = self.A == 0;
                self.N = (self.A & 0b100_0000) > 0;
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
            Op::StaAbs(None) => Op::StaAbs(Some(Address::Short(self.fetch()))),
            Op::StaAbs(Some(Address::Short(addr))) => {
                Op::StaAbs(Some(Address::Full(addr, self.fetch())))
            }
            Op::StaAbs(Some(addr)) => {
                self.write(addr, self.A);
                Op::None
            }
        })
    }
}
