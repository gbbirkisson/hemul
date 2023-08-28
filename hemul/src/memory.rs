use std::fs::File;
use std::io::prelude::*;
use std::ops::{Index, IndexMut};
use std::process::{Command, Stdio};

use crate::{Addressable, Byte, Snapshottable, Word};

pub struct Memory(Vec<Byte>);

impl Memory {
    pub fn new() -> Self {
        Self(vec![0; Word::MAX as usize + 1])
    }

    pub fn using(data: Vec<Byte>) -> Self {
        Self(data)
    }
}

impl Addressable for Memory {
    fn inside_bounds(&self, addr: Word) -> bool {
        self.0.len() > addr.into()
    }
}

impl Index<Word> for Memory {
    type Output = Byte;

    fn index(&self, index: Word) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<Word> for Memory {
    fn index_mut(&mut self, index: Word) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl Snapshottable for Memory {
    type Snapshot = Vec<Byte>;
    type Error = ();

    fn snapshot(&self) -> Result<Self::Snapshot, Self::Error> {
        Ok(self.0.clone())
    }
}

impl From<File> for Memory {
    fn from(mut f: File) -> Self {
        let mut memory = Self::new();
        let mut offset = 0;
        let buf_len = 10;
        loop {
            let read = f
                .read(&mut memory.0[offset..offset + buf_len])
                .expect("Failed to read file");
            if read < buf_len {
                break;
            }
            offset += read;
        }
        memory
    }
}

impl From<&str> for Memory {
    fn from(value: &str) -> Self {
        // let child = Command::new("xa")
        //     .args(["-o", "-", "/dev/stdin"])
        let child = Command::new("vasm6502_oldstyle")
            .args([
                "-Fbin",
                "-dotdir",
                "-o",
                "/dev/stdout",
                "-quiet",
                "/dev/stdin",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start assembler");

        child
            .stdin
            .expect("Failed to get stdin")
            .write_all(value.as_bytes())
            .expect("Failed to write to stdin");

        let mut data = Vec::new();
        let _ = child
            .stdout
            .expect("Failed to get stdout")
            .read_to_end(&mut data)
            .expect("Failed to read stdout");
        data.resize(Word::MAX as usize + 1, 0);
        Self::using(data)
    }
}

impl From<&[u8]> for Memory {
    fn from(value: &[u8]) -> Self {
        let mut memory = Self::new();
        for (i, b) in value.iter().enumerate() {
            memory.0[i] = *b;
        }
        memory
    }
}
