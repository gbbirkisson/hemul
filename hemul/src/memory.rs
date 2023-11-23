use std::{
    error::Error,
    fs::File,
    io::prelude::*,
    ops::{Index, IndexMut},
    process::{Command, Stdio},
};

use crate::{Addressable, Byte, Snapshottable, Word};

pub struct Memory(Vec<Byte>);

impl Memory {
    pub fn using(data: Vec<Byte>) -> Self {
        Self(data)
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self(vec![0; Word::MAX as usize + 1])
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

    fn snapshot(&self) -> Result<Self::Snapshot, Box<dyn Error>> {
        Ok(self.0.clone())
    }
}

impl From<File> for Memory {
    fn from(mut f: File) -> Self {
        let mut memory = Self::default();
        let mut offset = 0;
        let buf_len = 10;
        loop {
            let read = f
                .read(&mut memory.0[offset..offset + buf_len])
                .expect("failed to read file");
            if read < buf_len {
                break;
            }
            offset += read;
        }
        memory
    }
}

impl From<String> for Memory {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<&str> for Memory {
    fn from(value: &str) -> Self {
        // Fallback to bin in repo
        let bin = match Command::new("vasm6502_oldstyle")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(_) => "vasm6502_oldstyle",
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    "../bin/vasm6502_oldstyle"
                } else {
                    panic!("failed running vasm6502_oldstyle");
                }
            }
        };

        // let child = Command::new("xa")
        //     .args(["-o", "-", "/dev/stdin"])
        let child = Command::new(bin)
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
            .expect("failed to start assembler");

        child
            .stdin
            .expect("failed to get stdin")
            .write_all(value.as_bytes())
            .expect("failed to write to stdin");

        let mut data = Vec::new();
        let _ = child
            .stdout
            .expect("failed to get stdout")
            .read_to_end(&mut data)
            .expect("failed to read stdout");
        data.resize(Word::MAX as usize + 1, 0);
        Self::using(data)
    }
}

impl From<&[u8]> for Memory {
    fn from(value: &[u8]) -> Self {
        let mut memory = Self::default();
        for (i, b) in value.iter().enumerate() {
            memory.0[i] = *b;
        }
        memory
    }
}
