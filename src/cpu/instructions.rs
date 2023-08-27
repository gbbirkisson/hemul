use crate::cpu::{Addressable, Cpu, Error, State};
use crate::{Address, Byte};

impl TryFrom<Byte> for Op {
    type Error = Byte;

    fn try_from(value: Byte) -> Result<Self, Self::Error> {
        match value {
            0xEA => Ok(Self::Nop),
            0xA9 => Ok(Self::LdaIm),
            0x69 => Ok(Self::AdcIm),
            0x8d => Ok(Self::StaAbs(None)),
            _ => Err(value),
        }
    }
}

pub trait OpHandler {
    fn handle(&mut self, op: Op) -> Op;
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
    fn handle(&mut self, op: Op) -> Op {
        match op {
            // None => Try to load next instruction
            Op::None => match Op::try_from(self.fetch()) {
                Ok(op) => op,
                Err(e) => {
                    self.st = State::Error(Error::BadOpCode(e));
                    Op::None
                }
            },

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
                self.write(addr.into(), self.A);
                Op::None
            }
        }
    }
}
