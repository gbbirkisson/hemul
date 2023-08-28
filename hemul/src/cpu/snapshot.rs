use crate::{Addressable, Snapshottable};

use super::{Byte, Cpu, Op, PFlag, Word};
use std::io::prelude::*;
use std::process::{Command, Stdio};

#[allow(non_snake_case, dead_code)]
pub struct Snapshot {
    pub dump: Vec<Byte>,

    pub PC: Word, // Program Counter
    pub SP: Byte, // Stack Pointer

    pub A: Byte, // Accumulator
    pub X: Byte, // Index Register X
    pub Y: Byte, // Index Register Y

    pub C: PFlag, // Carry Flag
    pub Z: PFlag, // Zero Flag
    pub I: PFlag, // Interupt Disable
    pub D: PFlag, // Decimal Mode
    pub B: PFlag, // Break Command
    pub V: PFlag, // Overflow Flag
    pub N: PFlag, // Negative Flag
}

impl std::fmt::Debug for Snapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "PC\tSP\tA\tX\tY\tCZIDBVN")?;
        write!(
            f,
            "{:#01x}\t{:#01x}\t{:#01x}\t{:#01x}\t{:#01x}\t{}{}{}{}{}{}{}",
            self.PC,
            self.SP,
            self.A,
            self.X,
            self.Y,
            i32::from(self.C),
            i32::from(self.Z),
            i32::from(self.I),
            i32::from(self.D),
            i32::from(self.B),
            i32::from(self.V),
            i32::from(self.N),
        )?;
        write!(f, "\n\n")?;

        let child = Command::new("hexdump")
            .args(["-C"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start hexdump");

        child
            .stdin
            .expect("Failed to get stdin")
            .write_all(&self.dump[..])
            .expect("Failed to write to stdin");

        let mut hexdump = String::new();

        child
            .stdout
            .expect("Failed to get stdout")
            .read_to_string(&mut hexdump)
            .expect("Failed to read stdout");

        write!(f, "{hexdump}")
    }
}

impl<T> Snapshottable for Cpu<T>
where
    T: Addressable + Snapshottable<Snapshot = Vec<u8>>,
{
    type Snapshot = Snapshot;
    type Error = String;

    fn snapshot(&self) -> Result<Self::Snapshot, Self::Error> {
        match self.op {
            Op::None => Ok(Snapshot {
                dump: self
                    .addr
                    .snapshot()
                    .map_err(|_| "Failed to snap addresses")?,

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
            }),
            _ => Err("Op not None".to_string()),
        }
    }
}
