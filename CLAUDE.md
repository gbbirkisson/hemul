# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Hemul is a 6502 microprocessor emulator written in Rust. The project is organized as a Rust workspace with two crates:
- `hemul` - Core emulation library containing CPU, memory, bus, and oscillator implementations
- `hemul-cli` - Command-line interface for running assembly code

## Architecture

### Core Components

1. **CPU** (`hemul/src/cpu/`) - 6502 processor implementation
   - Implements all addressing modes and instruction execution
   - Handles interrupts, stack operations, and status flags
   - Supports both Fast mode (1 instruction per tick) and Original mode (cycle-accurate)

2. **Memory** (`hemul/src/memory.rs`) - RAM implementation with configurable size
   - Implements the `Addressable` trait for byte-level access

3. **Bus** (`hemul/src/bus.rs`) - Connects addressable components
   - Maps memory ranges to different devices
   - Allows multiple peripherals on the address space

4. **Oscillator** (`hemul/src/oscillator.rs`) - Clock source
   - Drives CPU and other tickable components at specified frequency
   - Configurable MHz rate

### Key Traits

- `Addressable` - For memory-mapped components
- `Tickable` - For clock-driven components
- `Resettable` - For components that support reset
- `Interruptible` - For interrupt handling
- `Snapshottable` - For state capture/testing

## Development Commands

### Building and Testing
```bash
# Run all tests
make test

# Run specific test
make test TEST=test_instr_arithmetic

# Lint code
make lint

# Full development cycle (test + lint)
make dev
```

### Running the Emulator
```bash
# Run with assembly code
echo "LDA #01; ADC #02; NOP" | cargo run -p hemul-cli -- -b - -a

# Run with binary code
cargo run -p hemul-cli -- -b <binary_file>

# Set CPU frequency
cargo run -p hemul-cli -- -b <file> --mhz 1.79
```

## Testing Approach

Tests use property-based testing with `proptest` and a custom `asm_test!` macro that:
1. Assembles 6502 assembly code using `vasm6502_oldstyle`
2. Loads it into memory
3. Runs until NOP instruction
4. Returns a CPU snapshot for assertions

Example test pattern:
```rust
let snapshot = asm_test!(
    format!(r#"
    LDA     #{}
    ADC     #{}
    NOP
    "#, value1, value2)
);
assert_eq!(snapshot.A, expected_value);
```

## Dependencies

- `vasm6502_oldstyle` - Required in PATH for assembling test code
- `hexdump` - Required for test utilities

## Current Implementation Status

Implemented opcodes include:
- Load/Store: LDA, LDX, LDY, STA, STX, STY
- Transfer: TAX, TAY, TXA, TYA, TSX, TXS
- Stack: PHA, PHP, PLA, PLP
- Arithmetic: ADC, SBC, CMP, CPX, CPY, INC, INX, INY, DEC, DEX, DEY
- Logical: AND, EOR, ORA, BIT
- Shifts: ASL, LSR, ROL, ROR
- Jumps: JMP, JSR, RTS
- Branches: BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS
- Flags: CLC, CLD, CLI, CLV, SEC, SEI
- Interrupts: BRK, RTI
- Other: NOP

Note: Decimal mode (SED) is not supported.
