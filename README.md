# r68k

r68k is a emulator for the m68k written in Rust, as a port of [Karl Stenerud's Musashi](https://github.com/kstenerud/Musashi). Musashi "has been successfully running in the MAME project (www.mame.net) for years
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

## The Processor
The [Motorola 68000](https://en.wikipedia.org/wiki/Motorola_68000) CPU, introduced in 1979, was a very successful CPU that powered several of the most classic personal computers of the 1980s, such as the Apple Macintosh, Commodore Amiga and Atari ST, as well as the first SUN and Apollo UNIX workstations. It was used in several arcade machines and game consoles such as the Sega Genesis/Mega Drive, and was also found in the Apple LaserWriter and HP LaserJet printers.

 It typically ran at 8MHz and could address up to 16MB of RAM. However, home computers in the mid 1980s typically only had 512KB.

## Usage
Note that the emulator is not a full computer system emulation, it's just a CPU connected to some memory, so on its own it doesn't do anything interesting.
You will have to load its memory with some program, which is just a series of bytes representing valid instructions and data, and tell it to start executing those. The CPU starts fetching instructions from memory, executes them one by one, which affects the state of the CPU. Some instructions also write to memory, and you can observe and act on these effects. 

One can build a simple computer emulation on top of r68k.

## CPU Emulator

The current status of the r68k emulator is usable. Please note that it only implements the original 68000 instruction set. It does not support instructions specific to newer CPUs in the 68k family (such as the 68010, 68020 or 68040) at this time.

- all instructions are implemented and verified against [Musashi](https://github.com/kstenerud/Musashi)
- support for autovectored, autoresetting interrupts are in place
- STOP and HALT states are properly emulated
- host callbacks for RESET and exception overrides are implemented
- A memory (RAM) implementation is in place

The main emulation TODOs are:
- adding a memory implementation with support for memory mapping (letting your program react to reads from and writes to certain addresses). It is possible, however, for the user to implement this themselves if needed
- add more hooks to simplify integrating the emulator in a larger emulated system
- Add user/API-documentation and usage examples

## Disassembler
The Disassembler support the full instruction set, and has been verified against the emulator so that all valid opcodes can be disassembled, and no invalid opcodes are incorrectly recognized by the disassembler.
The Disassembler currently has no command line interface, but can be used programmatically to disassemble a chunk of memory, one instruction at a time.

The main disassembly TODOs are:
- unifying the implementation of memory used by the disassembler, assembler and emulator, in order to simplify disassembling the currently executing code on the fly 
- adding a command line interface
- Add user/API-documentation and usage examples

## Assembler
The Assembler supports the full instruction set. It can assemble all valid instructions (which has been verified by disassembling all 64K possible opcodes, making sure that all valid opcodes assemble back to the same sequence of bytes.

The parser is based on [the Pest PEG parser generator](https://github.com/dragostis/pest) and supports the full instruction set, and a few directives (but documentation of supported assembler directives is still missing).

The main disassembly TODOs are:
- support using symbols such as constants and labels as operands (now has no symbol table, and so requires all operands to be registers or numeric literals)
- support instruction aliases, such as allowing the user to use *ADD*, but automatically use *ADDA* if the destination is an address register, and *ADDI* or *ADDQ* if the source is immediate data
- support assembling directly into the emulator memory.
- improve validation - the assembler in some cases now allows assembly of addressing modes that are in fact invalid for the particular instruction, which will cause an invalid instruction exception if run on the emulator
- adding a command line interface
- Add user/API-documentation and usage examples

## S-record support
The [Motorola S-record format](https://en.wikipedia.org/wiki/SREC_(file_format))is a format for representing 
binary data in a simple ASCII-text format, typically used to contain a "memory image" of microprocessor programs. They contain the compiled microprocessor instructions 
and data, along the absolute memory addresses where they are to be stored. These files are often produced by a compiler or assembler and then used to upload a program directly into microprocessor memory.

The S-record-support is still in a very early stage, and is write only.

## Testing philosophy
All 64k possible opcodes have been A/B-tested against Musashi using [BurntSushi's QuickCheck for Rust](https://github.com/BurntSushi/quickcheck). There's about 54 000 valid opcodes for the m68k (and the remaining 11 500 does not represent valid instructions).

Using QuickCheck means we first generate a *randomized* CPU state (including random values for all D and A registers, and the status register which controls Supervisor mode, and includes condition codes such as overflow, zero etc), then both Musashi and r68k is carefully put in this state, and then the instruction under test is executed, and then the resulting state is compared for any differences. All memory accesses made by either emulator are also compared for any differences, including number and order of accesses, the address used, operation size (8, 16 or 32 bits), as well as the value read/written and address space used (user/supervisor + data/program). Then this process is repeated many times for each opcode implemented.

In effect, each instruction is compared thoroughly (with random values) to Musashi, using all combinations possible of the allowed source and destination addressing modes and registers. The number of clock cycles consumed is also reported by Musashi after execution, and is also compared to r68k.

If during execution any exceptions are encountered, such as privilege violations, illegal instruction traps or address errors (word or long accesses on odd addresses) then the actions (and cycles) taken by the emulators are also compared in the same way.

Randomized testing tends to immediately discover any differences in the implementation, and tests _not_ failing gives a fair bit of confidence that the implementation is correct - or at least behaves like Musashi, which (unlike r68k) is in fact battle-tested. I'm also frequently referencing the [M68000 Programmer's Reference Manual](https://www.nxp.com/files/archives/doc/ref_manual/M68000PRM.pdf) and [M68000 User's Manual](http://cache.freescale.com/files/32bit/doc/ref_manual/MC68000UM.pdf).
