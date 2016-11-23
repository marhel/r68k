# r68k

r68k is a m68k emulator written in Rust, as a port of [Karl Stenerud's Musashi](https://github.com/kstenerud/Musashi). Musashi "has been successfully running in the MAME project (www.mame.net) for years
and so has had time to mature." - so unlike most other emulators Musashi is of proven quality to run complex real-world m68k software, which I thought was a good foundation to build on.

The end goal for r68k is to have a complete m68k emulation lib, comparable with Musashi in speed and functionality, which I might later use to build a _virtual retro computer_ in Rust - a project I have not even started on yet (or, well, I have some code, but I got tired of C++, and was inspired enough by Rust to start over). It's built from the start to be able to operate multiple cores independently, with their own RAM, because eventually I want to emulate _several_ independent retro computers in the same process, for an even more ambitious, will-never-see-the-light-of-day, still-very-fuzzy-around-the-edges first-person-hacking gaming idea vaugely inspired by [0x10c](https://en.wikipedia.org/wiki/0x10c). There. I said it. Now mock me! But at least I'll get to learn Rust. And failure. But Rust is good!

## Project Layout

    common => r68k_common
        constants       common opcode constants
    emu => r68k_emu
        cpu             Motorola 68000 emulation
        musashi         Musashi integration tests
    tools => r68k_tools
        assembler       simple assembler
        disassembler    simple disassembler
        srecords        support for Motorola SRecord format

## Status
The current status of the r68k emulator is incomplete - all instructions are implemented and verified against Musashi, support for autovectored, autoresetting interrupts are in place, but infrastrucure such as host callbacks are yet to be implemented, and it's therefore not quite usable at this point. Sorry!

However, if this makes you disappointed, you can consider helping out!

The assembler, disassembler and srecord-support is still in very early stages, and only a minority of the instructions are supported at this point. SRecord support is write only.

## Testing philosophy
All 64k possible opcodes have been A/B-tested against Musashi using [BurntSushi's QuickCheck for Rust](https://github.com/BurntSushi/quickcheck). There's about 54 000 valid opcodes for the m68k (and the remaining 11 500 does not represent valid instructions).

Using QuickCheck means we first generate a *randomized* CPU state (including random values for all D and A registers, and the status register which controls Supervisor mode, and includes condition codes such as overflow, zero etc), then both Musashi and r68k is carefully put in this state, and then the instruction under test is executed, and then the resulting state is compared for any differences. All memory accesses made by either emulator are also compared for any differences, including number and order of accesses, the address used, operation size (8, 16 or 32 bits), as well as the value read/written and address space used (user/supervisor + data/program). Then this process is repeated many times for each opcode implemented.

In effect, each instruction is compared thoroughly (with random values) to Musashi, using all combinations possible of the allowed source and destination addressing modes and registers. The number of clock cycles consumed is also reported by Musashi after execution, and is also compared to r68k.

If during execution any exceptions are encountered, such as privilege violations, illegal instruction traps or address errors (word or long accesses on odd addresses) then the actions (and cycles) taken by the emulators are also compared in the same way.

Randomized testing tends to immediately discover any differences in the implementation, and tests _not_ failing gives a fair bit of confidence that the implementation is correct - or at least behaves like Musashi, which (unlike r68k) is in fact battle-tested. I'm also frequently referencing the [M68000 Programmer's Reference Manual](https://www.nxp.com/files/archives/doc/ref_manual/M68000PRM.pdf) and [M68000 User's Manual](http://cache.freescale.com/files/32bit/doc/ref_manual/MC68000UM.pdf).
