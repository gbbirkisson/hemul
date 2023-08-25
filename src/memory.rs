use std::fs::File;
use std::io::prelude::*;
use std::ops::{Index, IndexMut};
use std::process::{Command, Stdio};

use crate::device::Addressable;

pub struct Memory(Vec<u8>);

impl Memory {
    pub fn new() -> Self {
        Self(vec![0; std::u16::MAX as usize])
    }
}

impl Addressable for Memory {}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl From<File> for Memory {
    fn from(mut f: File) -> Self {
        let mut memory = Memory::new();
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

impl From<&'static str> for Memory {
    fn from(value: &'static str) -> Self {
        let child = Command::new("xa")
            .args(["-o", "-", "/dev/stdin"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start xa");

        child
            .stdin
            .expect("Failed to get stdin")
            .write_all(value.as_bytes())
            .expect("Failed to write to stdin");

        let mut memory = Memory::new();

        child
            .stdout
            .expect("Failed to get stdout")
            .read(&mut memory.0[..])
            .expect("Failed to read stdout");

        memory
    }
}
