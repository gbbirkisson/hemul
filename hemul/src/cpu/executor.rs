use crate::{
    cpu::{address::Address, Addressable, Cpu, Error},
    Byte,
};

use super::instructions::{AddressMode, Op};

macro_rules! flags_zn {
    ($self:ident, $r:expr) => {
        $self.Z = $r == 0;
        $self.N = ($r & 0b1000_0000) > 0;
    };
}

macro_rules! branch {
    ($self:ident, $cond:expr) => {
        let offset = $self.fetch()?;
        if $cond {
            $self.PC = u16::try_from(i32::from($self.PC) + i32::from(offset))
                .map_err(|_| Error::Other("Failed to calculate offset".to_string()))?;
        }
    };
}

macro_rules! compare {
    ($self:ident, $r:ident, $mode:ident) => {
        let data = $self.fetch_mode(&$mode)?;
        $self.C = $self.$r >= data;
        $self.Z = $self.$r == data;
        $self.N = (data & 0b1000_0000) > 0;
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
                self.V = (data & 0b0100_0000) > 0;
                self.N = (data & 0b1000_0000) > 0;
            }
            Op::Adc(mode) => {
                // TODO
                let data = self.fetch_mode(&mode)?;
                let (data, carry1) = if self.C {
                    data.overflowing_add(1)
                } else {
                    (data, false)
                };
                let (sum, carry2) = self.A.overflowing_add(data);
                self.A = sum;
                self.C = carry1 || carry2;
                flags_zn!(self, self.A);
            }
            Op::Sbc(_mode) => {
                // TODO
                todo!()
            }
            Op::Cmp(mode) => {
                compare!(self, A, mode);
            }
            Op::Cpx(mode) => {
                compare!(self, X, mode);
            }
            Op::Cpy(mode) => {
                compare!(self, Y, mode);
            }
            Op::Inc(mode) => {
                let data = self.edit_mode(&mode, |data: Byte| data.wrapping_add(1))?;
                flags_zn!(self, data);
            }
            Op::Inx => {
                self.X = self.X.wrapping_add(1);
                flags_zn!(self, self.X);
            }
            Op::Iny => {
                self.Y = self.Y.wrapping_add(1);
                flags_zn!(self, self.Y);
            }
            Op::Dec(mode) => {
                let data = self.edit_mode(&mode, |data: Byte| data.wrapping_sub(1))?;
                flags_zn!(self, data);
            }
            Op::Dex => {
                self.X = self.X.wrapping_sub(1);
                flags_zn!(self, self.X);
            }
            Op::Dey => {
                self.Y = self.Y.wrapping_sub(1);
                flags_zn!(self, self.Y);
            }
            Op::Asl(AddressMode::Accumulator) => {
                // TODO
                todo!()
            }
            Op::Asl(_mode) => {
                // TODO
                todo!()
            }
            Op::Lsr(AddressMode::Accumulator) => {
                // TODO
                todo!()
            }
            Op::Lsr(_mode) => {
                // TODO
                todo!()
            }
            Op::Rol(AddressMode::Accumulator) => {
                // TODO
                todo!()
            }
            Op::Rol(_mode) => {
                // TODO
                todo!()
            }
            Op::Ror(AddressMode::Accumulator) => {
                // TODO
                todo!()
            }
            Op::Ror(_mode) => {
                // TODO
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
                let new_pc = self.fetch_word()?;
                let Address::Full(addr, page) = Address::from(self.PC - 1) else {
                    return Err(Error::Other(
                        "Could not construct address from PC".to_string(),
                    ));
                };
                self.stack_push(page)?;
                self.stack_push(addr)?;
                self.PC = new_pc;
            }
            Op::Rts => {
                let addr = self.stack_pop()?;
                let page = self.stack_pop()?;
                self.PC = Address::Full(addr, page).into();
                self.PC += 1;
            }
            Op::Bcc => {
                branch!(self, !self.C);
            }
            Op::Bcs => {
                branch!(self, self.C);
            }
            Op::Beq => {
                branch!(self, self.Z);
            }
            Op::Bmi => {
                branch!(self, self.N);
            }
            Op::Bne => {
                branch!(self, !self.Z);
            }
            Op::Bpl => {
                branch!(self, !self.N);
            }
            Op::Bvc => {
                branch!(self, !self.V);
            }
            Op::Bvs => {
                branch!(self, self.V);
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
                panic!("Decimal Mode not supported");
            }
            Op::Sei => {
                self.I = true;
            }
            Op::Brk(interupt_addr) => {
                let Address::Full(addr, page) = Address::from(self.PC - 1) else {
                    return Err(Error::Other(
                        "Could not construct address from PC".to_string(),
                    ));
                };
                self.stack_push(page)?;
                self.stack_push(addr)?;
                self.PC = self.read_word(interupt_addr)?;
            }
            Op::Nop => {}
            Op::Rti => {
                let addr = self.stack_pop()?;
                let page = self.stack_pop()?;
                self.PC = Address::Full(addr, page).into();
                self.PC += 1;
            }
            invalid => todo!("{:?}", invalid),
        }
        Ok(())
    }
}
