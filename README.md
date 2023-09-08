<h1>hemul</h1>

Emulation of the 6502 micro processor

<!-- vim-markdown-toc GFM -->

* [Requirements](#requirements)
* [Resources](#resources)
* [Running the VM](#running-the-vm)
* [TODO](#todo)

<!-- vim-markdown-toc -->

## Requirements

You will need these binaries in your path to do testing:

- [xa](https://linux.die.net/man/1/xa)
- [hexdump](https://man7.org/linux/man-pages/man1/hexdump.1.html)

## Resources

- https://www.youtube.com/watch?v=qJgsuQoy9bc&t=1042s
- https://www.nesdev.org/obelisk-6502-guide/
- http://www.6502.org/tutorials/decimal_mode.html
- https://wiki.cdot.senecacollege.ca/wiki/6502_Math
- https://www.youtube.com/watch?v=LnzuMJLZRdU&list=PLowKtXNTBypFbtuVMUVXNR0z1mu7dp7eH

## Running the VM

With assemly code:

```console
$ cat << EOF | cargo run -p hemul-vm -- -b - -a
    ; 1 + 2
    LDA     #01
    ADC     #02
    STA     $0402
    NOP
EOF
```

## TODO

* Create a testing macro
    - [x] Assemble program
    - [x] Load to memory
    - [x] Run ...

* Create snapshot mechanism
    - [x] Pull out snapshot
    - [x] Assert on snapshot, nice macro for this maybe?
    - [x] Print nice on exceptions, like hexdump?

* CI
    - [x] Add github actions

* Implement:
    - [ ] All the opcodes
    - [x] A bus
    - [x] Clock crystal
    - [ ] A serial port with socat: [ref](https://www.baeldung.com/linux/make-virtual-serial-port)

* Other
    - [ ] Setup syntax highlighting for ASM: [ref](https://www.youtube.com/watch?v=v3o9YaHBM4Q&t)


cat << EOF | cargo run -- -b - -a
    ; 1 + 2
    LDA     #01
    ADC     #02
    STA     $0402
    NOP
EOF
