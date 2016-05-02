pub mod cpu;
pub mod musashi;
pub mod disassembler;
mod ram;
#[macro_use]
extern crate lazy_static;

extern crate itertools;
use itertools::Itertools;

use std::io::Result;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut cpu = cpu::Core::new_mem(0x40, &[0xc3, 0x00]);
    cpu.ophandlers = cpu::ops::instruction_set();
    cpu.dar[0] = 0x16;
    cpu.dar[1] = 0x26;
    cpu.execute1();
    musashi::experimental_communication();
    println!("Hello, CPU at {}", cpu.pc);
    match write_state(&cpu) {
        Ok(_) => return (),
        Err(e) => panic!(e),
    };
}

fn write_state(core: &cpu::Core) -> Result<()> {
    let mut buffer = try!(File::create("cpustate.txt"));
    let dxs = (0..8).map(|i| format!("D{}:{:08x}", i, core.dar[i])).join(" ");
    let axs = (0..8).map(|i| format!("A{}:{:08x}", i, core.dar[i+8])).join(" ");
    try!(writeln!(buffer, "PC:{:08x} SP:{:08x} SR:{:08x} FL:{} {} {}", core.pc, core.dar[15], core.status_register(), core.flags(), dxs, axs));
    Ok(())
}

