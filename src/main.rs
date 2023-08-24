use std::fs::File;
use std::io;

mod cpu;
use cpu::*;

mod memory;
use memory::*;

impl From<&'static str> for Cpu<Memory> {
    fn from(_value: &'static str) -> Self {
        todo!()
    }
}

fn main() -> io::Result<()> {
    //     let mut cpu: Cpu<Memory> = r"
    // ; 1 + 2
    // LDA     #01
    // ADC     #02
    // STA     $0402
    //     ".into();
    let memory: Memory = File::open("a.o65")?.into();
    let mut cpu = Cpu::new(memory);
    cpu.tick_for(8);
    Ok(())
}
