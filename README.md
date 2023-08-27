<h1>hemul</h1>

Emulation of the 6502 micro processor

<!-- vim-markdown-toc GFM -->

* [Requirements](#requirements)
* [Resources](#resources)
* [TODO](#todo)

<!-- vim-markdown-toc -->

## Requirements

You will need these binaries in your path to do testing:

- [xa](https://linux.die.net/man/1/xa)
- [hexdump](https://man7.org/linux/man-pages/man1/hexdump.1.html)

## Resources

- https://www.youtube.com/watch?v=qJgsuQoy9bc&t=1042s
- https://web.archive.org/web/20210912192127/http://www.obelisk.me.uk/6502/
- https://www.youtube.com/watch?v=LnzuMJLZRdU&list=PLowKtXNTBypFbtuVMUVXNR0z1mu7dp7eH

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
    - [ ] A bus
    - [ ] Clock crystal
    - [ ] A serial port with socat: [ref](https://www.baeldung.com/linux/make-virtual-serial-port)

* Other
    - [ ] Setup syntax highlighting for ASM: [ref](https://www.youtube.com/watch?v=v3o9YaHBM4Q&t)

