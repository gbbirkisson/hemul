use crate::cpu::{address::Address, Addressable, Cpu, Error};

use super::instructions::Op;

macro_rules! flags_zn {
    ($self:ident, $r:expr) => {
        $self.Z = $r == 0;
        $self.N = ($r & 0b0100_0000) > 0;
    };
}

#[allow(clippy::module_name_repetitions)]
pub trait OpExecutor {
    /// Executes a given Op
    fn execute(&mut self, op: Op) -> Result<(), Error>;
}

impl<T> OpExecutor for Cpu<T>
where
    T: Addressable,
{
    #[allow(clippy::too_many_lines)]
    fn execute(&mut self, op: Op) -> Result<(), Error> {
        match op {
            Op::Lda(mode) => {
                let (_, data) = self.fetch_mode(&mode)?;
                self.A = data;
                flags_zn!(self, self.A);
            }
            Op::Ldx(mode) => {
                let (_, data) = self.fetch_mode(&mode)?;
                self.X = data;
                flags_zn!(self, self.X);
            }
            Op::Ldy(mode) => {
                let (_, data) = self.fetch_mode(&mode)?;
                self.Y = data;
                flags_zn!(self, self.Y);
            }
            Op::Sta(mode) => {
                self.write_mode(&mode, self.A)?;
            }
            Op::Stx(mode) => {
                self.write_mode(&mode, self.X)?;
            }
            Op::Sty(mode) => {
                self.write_mode(&mode, self.Y)?;
            }
            Op::Tax => {
                self.X = self.A;
                flags_zn!(self, self.X);
            }
            Op::Tay => {
                self.Y = self.A;
                flags_zn!(self, self.Y);
            }
            Op::Txa => {
                self.A = self.X;
                flags_zn!(self, self.A);
            }
            Op::Tya => {
                self.A = self.Y;
                flags_zn!(self, self.A);
            }
            Op::Tsx => {
                self.X = self.SP;
                flags_zn!(self, self.X);
            }
            Op::Txs => {
                self.SP = self.X;
            }
            Op::Pha => {
                self.stack_push(self.A)?;
            }
            Op::Php => {
                self.stack_push(self.status_get())?;
            }
            Op::Pla => {
                self.A = self.stack_pop()?;
                flags_zn!(self, self.A);
            }
            Op::Plp => {
                let status = self.stack_pop()?;
                self.status_set(status);
            }
            Op::And(_mode) => {
                todo!()
            }
            Op::Eor(_mode) => {
                todo!()
            }
            Op::Ora(_mode) => {
                todo!()
            }
            Op::Bit(_mode) => {
                todo!()
            }
            Op::Adc(mode) => {
                let (_, data) = self.fetch_mode(&mode)?;
                self.A += data;
                flags_zn!(self, self.A);
                // TODO SIDE EFFECTS
            }
            Op::Sbc(_mode) => {
                todo!()
            }
            Op::Cmp(_mode) => {
                todo!()
            }
            Op::Cpx(_mode) => {
                todo!()
            }
            Op::Cpy(_mode) => {
                todo!()
            }
            Op::Inc(mode) => {
                let (addr, mut data) = self.fetch_mode(&mode)?;
                data += 1;
                flags_zn!(self, data);
                self.write(addr, data)?;
            }

            Op::Jsr => {
                let Address::Full(addr, page) = Address::from(self.PC - 1) else {
                    panic!()
                };
                self.stack_push(page)?;
                self.stack_push(addr)?;
                self.PC = self.fetch_word()?;
            }

            Op::Nop => {}

            invalid => todo!("{:?}", invalid),
        }
        Ok(())
    }
}
