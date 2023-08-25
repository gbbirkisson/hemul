use std::fs::File;
use std::io::prelude::*;
use std::ops::{Index, IndexMut};

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
    fn from(s: &'static str) -> Self {
        dbg!(s);
        todo!()
    }
}
