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
        let mut child = Command::new("xa")
            .args(["-o", "-", "/dev/stdin"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start xa");

        let mut stdin = child.stdin.take().expect("Failed to get stdin");
        std::thread::spawn(move || {
            stdin
                .write_all(value.as_bytes())
                .expect("Failed to write to stdin");
        });
        let output = child.wait_with_output().expect("Failed to read stdout");

        let mut memory = Memory::new();
        for (i, byte) in output.stdout.into_iter().enumerate() {
            memory[i as u16] = byte;
        }
        memory
    }
}
