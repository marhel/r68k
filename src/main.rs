extern crate itertools;
mod cpu;

use std::io::Result;
use std::io::prelude::*;
use std::fs::File;


fn main() {
	let mut cpu = cpu::Core::new_mem(0x40, &[0xc3, 0x00]);
	cpu.ophandlers = cpu::ops::instruction_set();
	cpu.dar[0] = 0x16;
	cpu.dar[1] = 0x26;
	cpu.execute1();
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
	try!(writeln!(buffer, "PC:{:08x} SP:{:08x} SR:{:08x} FL:{} {} {}", core.pc, core.sp, core.status_register(), core.flags(), dxs, axs));
	Ok(())
}

// cpu state (all registers, flags etc.)
// initialize
//      m68ki_build_opcode_table();
//		m68ki_jump(0);
//		REG_SP = m68ki_read_imm_32();
//		REG_PC = m68ki_read_imm_32();
//		m68ki_jump(REG_PC);

// execute instruction (modifies CPU state)
/*
			/* Record previous program counter */
			REG_PPC = REG_PC;

			/* Read an instruction and call its handler */
			REG_IR = m68ki_read_imm_16();
			// include all 64k entries?
			m68ki_instruction_jump_table[REG_IR]();
			USE_CYCLES(CYC_INSTRUCTION[REG_IR]);
*/

