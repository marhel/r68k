mod cpu;

use std::io::Result;
use std::io::prelude::*;
use std::fs::File;


fn main() {
	let cpu = cpu::Core::new(64);
	println!("Hello, CPU at {}", cpu.pc);
	match write_data(&cpu) {
		Ok(_) => return (),
		Err(e) => panic!(e),
	};
}

fn write_data(core: &cpu::Core) -> Result<()> {
	let mut buffer = try!(File::create("cpustate.txt"));
	try!(write!(buffer, "{:04x} {:04x} {:04x}", core.pc, core.sp, core.status_register()));
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

