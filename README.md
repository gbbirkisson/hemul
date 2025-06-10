<h1>
  <p align="center">
    <a href="https://github.com/gbbirkisson/hemul">
      <img src="https://raw.githubusercontent.com/gbbirkisson/hemul/main/logo.png" alt="Logo" height="128">
    </a>
    <br>hemul
  </p>
</h1>

<p align="center">
Emulation of the 6502 micro processor, because why not 🤷 The project is named h[emu]l after the "Hemul" from the "Moomin" francise.
</p>

<p align="center">
[![GitHub last commit (branch)](https://img.shields.io/github/last-commit/gbbirkisson/hemul/main)](https://github.com/gbbirkisson/hemul/commits/main)
[![CI](https://github.com/gbbirkisson/hemul/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/gbbirkisson/hemul/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/gbbirkisson/hemul/branch/main/graph/badge.svg?token=GFZ3Y0Y2X6)](https://codecov.io/github/gbbirkisson/hemul)
[![GitHub](https://img.shields.io/github/license/gbbirkisson/hemul)](https://github.com/gbbirkisson/hemul/blob/main/LICENSE)
</p>

<!-- vim-markdown-toc GFM -->

* [Requirements](#requirements)
* [Running the emulator](#running-the-emulator)
* [Resources](#resources)

<!-- vim-markdown-toc -->

## Requirements

You will need these binaries in your path to do testing:

- [vasm6502_oldstyle](http://www.compilers.de/vasm.html)
- [hexdump](https://man7.org/linux/man-pages/man1/hexdump.1.html)

## Running the emulator

With assembly code:

```console
$ cat << EOF | cargo run -p hemul-cli -- -b - -a
    ; 1 + 2
    LDA     #01
    ADC     #02
    STA     $0402
    NOP
EOF
```

> [!NOTE]
> You will need `vasm6502_oldstyle` in your PATH to run this command!

## Resources

- https://www.nesdev.org/obelisk-6502-guide/
- http://www.6502.org/tutorials/decimal_mode.html
- https://wiki.cdot.senecacollege.ca/wiki/6502_Math
- https://www.youtube.com/watch?v=LnzuMJLZRdU&list=PLowKtXNTBypFbtuVMUVXNR0z1mu7dp7eH
