use clap::Parser;
use clap_stdin::MaybeStdin;
use hemul::{bus::Bus, cpu::Cpu, memory::Memory, oscillator::Oscillator, Tickable, Word};

/// Hemul VM
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Code to execute
    #[arg(short, long)]
    bin: MaybeStdin<String>,

    /// Code passed in is assembly not machine code
    #[arg(short, long)]
    #[clap(default_value_t = false)]
    asm: bool,

    /// Frequency to run at
    #[arg(short, long)]
    #[clap(default_value_t = 1.79)]
    mhz: f64,
}

fn main() {
    let args = Args::parse();

    let memory = if args.asm {
        Memory::from(args.bin.as_str())
    } else {
        Memory::from(args.bin.as_bytes())
    };

    let mut bus = Bus::default();
    bus.connect("memory", 0, Word::MAX, Box::new(memory));

    let cpu = Cpu::new(bus);

    let mut oscillator = Oscillator::from_megahertz(args.mhz);
    oscillator.connect("cpu", Box::new(cpu));

    loop {
        if let Err(e) = oscillator.tick() {
            panic!("{}", e);
        }
    }
}
