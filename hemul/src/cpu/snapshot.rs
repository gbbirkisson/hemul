use crate::{Addressable, Snapshottable};

use super::{Byte, Cpu, PFlag, Word};
use std::{
    error::Error,
    io::prelude::*,
    process::{Command, Stdio},
};

#[allow(non_snake_case, dead_code)]
pub struct Snapshot {
    /// Memory dump
    pub dump: Vec<Byte>,

    /// Program Counter
    pub PC: Word,
    /// Stack Pointer
    pub SP: Byte,

    /// Accumulator
    pub A: Byte,
    /// Index Register X
    pub X: Byte,
    /// Index Register Y
    pub Y: Byte,

    /// Carry Flag
    pub C: PFlag,
    /// Zero Flag
    pub Z: PFlag,
    /// Interrupt Disable
    pub I: PFlag,
    /// Decimal Mode
    pub D: PFlag,
    /// Break Command
    pub B: PFlag,
    /// Overflow Flag
    pub V: PFlag,
    /// Negative Flag
    pub N: PFlag,
}

impl std::fmt::Debug for Snapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "PC\t\tSP\tA\tX\tY\tCZIDBVN")?;
        write!(
            f,
            "{:#06x}\t{:#04x}\t{:#04x}\t{:#04x}\t{:#04x}\t{}{}{}{}{}{}{}",
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

        #[allow(clippy::zombie_processes)]
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

    fn snapshot(&self) -> Result<Self::Snapshot, Box<dyn Error>> {
        Ok(Snapshot {
            dump: self.addr.snapshot()?,

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
}
