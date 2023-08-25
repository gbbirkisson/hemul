https://www.youtube.com/watch?v=qJgsuQoy9bc&t=1042s
https://web.archive.org/web/20210912192127/http://www.obelisk.me.uk/6502/

xa test.asm

TODO:
* Create a testing macro
    * Assemble program
    * Load to memory
    * Run ...

* Create snapshot mechanism
    * Pull out snapshot
    * Assert on snapshot, nice macro for this maybe?
    * Print nice on exceptions, like hexdump?

* Setup syntax highlighting for ASM
    * https://www.youtube.com/watch?v=v3o9YaHBM4Q&t

* Implement:
    * All the opcodes
    * A bus
    * Clock crystal
    * A serial port with socat:
        https://www.baeldung.com/linux/make-virtual-serial-port
