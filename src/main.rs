use std::io;

mod cpu;
use cpu::*;

mod device;

mod memory;
use memory::*;

mod prelude;

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
    // let memory: Memory = File::open("a.o65")?.into();
    // let mut cpu = Cpu::new(memory);
    let mut cpu = asm!(
        r#"
    ; 1 + 2
    LDA     #01
    ADC     #02
    STA     $0402
    NOP
        "#
    );
    cpu.tick_until_nop();
    Ok(())
}
