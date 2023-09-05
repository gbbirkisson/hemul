use crate::{
    cpu::{address::Address, Addressable, Cpu, Error},
    Byte,
};

use super::instructions::{AddressMode, Op};

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
        #[allow(clippy::match_wildcard_for_single_variants)]
        match op {
            Op::Lda(mode) => {
                self.A = self.fetch_mode(&mode)?;
                flags_zn!(self, self.A);
            }
            Op::Ldx(mode) => {
                self.X = self.fetch_mode(&mode)?;
                flags_zn!(self, self.X);
            }
            Op::Ldy(mode) => {
                self.Y = self.fetch_mode(&mode)?;
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
            Op::And(mode) => {
                self.A &= self.fetch_mode(&mode)?;
                flags_zn!(self, self.A);
            }
            Op::Eor(mode) => {
                self.A ^= self.fetch_mode(&mode)?;
                flags_zn!(self, self.A);
            }
            Op::Ora(mode) => {
                self.A |= self.fetch_mode(&mode)?;
                flags_zn!(self, self.A);
            }
            Op::Bit(mode) => {
                let data = self.fetch_mode(&mode)?;
                self.Z = (data & self.A) == 0;
                self.V = (data & 0b0010_0000) != 0;
                self.N = (data & 0b0100_0000) != 0;
            }
            Op::Adc(mode) => {
                let data = self.fetch_mode(&mode)?;
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
                let data = self.edit_mode(&mode, |data: Byte| data + 1)?;
                flags_zn!(self, data);
            }
            Op::Inx => {
                self.X += 1;
                flags_zn!(self, self.X);
            }
            Op::Iny => {
                self.X += 1;
                flags_zn!(self, self.Y);
            }
            Op::Dec(mode) => {
                let data = self.edit_mode(&mode, |data: Byte| data - 1)?;
                flags_zn!(self, data);
            }
            Op::Dex => {
                self.X -= 1;
                flags_zn!(self, self.X);
            }
            Op::Dey => {
                self.Y -= 1;
                flags_zn!(self, self.Y);
            }
            Op::Asl(_mode) => {
                todo!()
            }
            Op::Lsr(_mode) => {
                todo!()
            }
            Op::Rol(_mode) => {
                todo!()
            }
            Op::Ror(_mode) => {
                todo!()
            }
            Op::Jmp(AddressMode::Absolute) => {
                self.PC = self.fetch_word()?;
            }
            Op::Jmp(AddressMode::Indirect) => {
                let addr = self.fetch_word()?;
                self.PC = self.read_word(addr)?;
            }
            Op::Jsr => {
                let Address::Full(addr, page) = Address::from(self.PC - 1) else {
                    return Err(Error::Other(
                        "Could not construct address from PC".to_string(),
                    ));
                };
                self.stack_push(page)?;
                self.stack_push(addr)?;
                self.PC = self.fetch_word()?;
            }
            Op::Rts => {
                let addr = self.stack_pop()?;
                let page = self.stack_pop()?;
                self.PC = Address::Full(addr, page).into();
            }
            Op::Bcc => {
                let addr = self.fetch_word()?;
                if !self.C {
                    self.PC = addr;
                }
            }
            Op::Bcs => {
                let addr = self.fetch_word()?;
                if self.C {
                    self.PC = addr;
                }
            }
            Op::Beq => {
                let addr = self.fetch_word()?;
                if self.Z {
                    self.PC = addr;
                }
            }
            Op::Bmi => {
                let addr = self.fetch_word()?;
                if self.N {
                    self.PC = addr;
                }
            }
            Op::Bne => {
                let addr = self.fetch_word()?;
                if !self.Z {
                    self.PC = addr;
                }
            }
            Op::Bpl => {
                let addr = self.fetch_word()?;
                if !self.N {
                    self.PC = addr;
                }
            }
            Op::Bvc => {
                let addr = self.fetch_word()?;
                if !self.V {
                    self.PC = addr;
                }
            }
            Op::Bvs => {
                let addr = self.fetch_word()?;
                if self.V {
                    self.PC = addr;
                }
            }
            Op::Clc => {
                self.C = false;
            }
            Op::Cld => {
                self.D = false;
            }
            Op::Cli => {
                self.I = false;
            }
            Op::Clv => {
                self.V = false;
            }
            Op::Sec => {
                self.C = true;
            }
            Op::Sed => {
                self.D = true;
            }
            Op::Sei => {
                self.I = true;
            }
            Op::Brk => {
                todo!()
            }
            Op::Nop => {}
            Op::Rti => {
                todo!()
            }
            invalid => todo!("{:?}", invalid),
        }
        Ok(())
    }
}
