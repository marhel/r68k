// Integration with Musashi
extern crate libc;


// Register enum copied from Musashi's m68k_register_t enum
#[repr(C)]
#[derive(Copy, Clone)]
pub enum Register {
	/* Real registers */
	D0,		/* Data registers */
	D1,
	D2,
	D3,
	D4,
	D5,
	D6,
	D7,
	A0,		/* Address registers */
	A1,
	A2,
	A3,
	A4,
	A5,
	A6,
	A7,
	PC,		/* Program Counter */
	SR,		/* Status Register */
	SP,		/* The current Stack Pointer (located in A7) */
	USP,		/* User Stack Pointer */
	ISP,		/* Interrupt Stack Pointer */
	MSP,		/* Master Stack Pointer */
	SFC,		/* Source Function Code */
	DFC,		/* Destination Function Code */
	VBR,		/* Vector Base Register */
	CACR,		/* Cache Control Register */
	CAAR,		/* Cache Address Register */

	/* Assumed registers */
	/* These are cheat registers which emulate the 1-longword prefetch
	 * present in the 68000 and 68010.
	 */
	PrefAddr,	/* Last prefetch address */
	PrefData,	/* Last prefetch data */

	/* Convenience registers */
	PPC,		/* Previous value in the program counter */
	IR,			/* Instruction register */
	CpuType	/* Type of CPU being run */
}

#[repr(C)]
#[derive(Copy, Clone)]
enum CpuType
{
	Invalid,
	M68000,
	M68010,
	M68EC020,
	M68020,
	M68030,		/* Supported by disassembler ONLY */
	M68040		/* Supported by disassembler ONLY */
}

#[link(name = "musashi", kind = "static")]
extern {
	fn m68k_init();
	fn m68k_set_cpu_type(cputype: CpuType);
	fn m68k_pulse_reset();
	fn m68k_execute(num_cycles: i32) -> i32;
	fn m68k_get_reg(context: *mut libc::c_void, regnum: Register) -> u32;
	fn m68k_set_reg(regnum: Register, value: u32);
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum Operation {
	None,
    ReadByte(u32),
    ReadWord(u32),
    ReadLong(u32),
    WriteByte(u32, u32),
    WriteWord(u32, u32),
    WriteLong(u32, u32),
}
static mut musashi_memory:  [u8; 1024] = [0u8; 1024];
// as statics are not allowed to have destructors, allocate a
// big enough array to hold the small number of operations
// expected from executing a very limited number of opcodes
static mut musashi_opcount: usize = 0;
static mut musashi_ops: [Operation; 128] = [Operation::None; 128];

unsafe fn register_op(op: Operation) {
	if(musashi_opcount < musashi_ops.len()) {
		musashi_ops[musashi_opcount] = op;
		musashi_opcount += 1;
	}
}
// callbacks from Musashi
#[no_mangle]
pub extern fn cpu_read_byte(address: u32) -> u32 {
	let op = Operation::ReadByte(address);
	let address = address as usize;
	unsafe {
		register_op(op);
		musashi_memory[address] as u32
	}
}
#[no_mangle]
pub extern fn cpu_read_word(address: u32) -> u32 {
	let op = Operation::ReadWord(address);
	let address = address as usize;
	unsafe {
		register_op(op);
		((musashi_memory[address+0] as u32) << 8
		|(musashi_memory[address+1] as u32) << 0) as u32
	}
}
#[no_mangle]
pub extern fn cpu_read_long(address: u32) -> u32 {
	let op = Operation::ReadLong(address);
	let address = address as usize;
	unsafe {
		register_op(op);
		((musashi_memory[address+0] as u32) << 24
		|(musashi_memory[address+1] as u32) << 16
		|(musashi_memory[address+2] as u32) <<  8
		|(musashi_memory[address+3] as u32) <<  0) as u32
	}
}

#[no_mangle]
pub extern fn cpu_write_byte(address: u32, value: u32) {
	let op = Operation::WriteByte(address, value);
	let address = address as usize;
	unsafe {
		register_op(op);
		musashi_memory[address+0] = (value & 0xff) as u8;
	}
}
#[no_mangle]
pub extern fn cpu_write_word(address: u32, value: u32) {
	let op = Operation::WriteWord(address, value);
	let address = address as usize;
	unsafe {
		register_op(op);
		musashi_memory[address+0] = (value & 0xff00 >> 8) as u8;
		musashi_memory[address+1] = (value & 0x00ff >> 0) as u8;
	}
}
#[no_mangle]
pub extern fn cpu_write_long(address: u32, value: u32) {
	let op = Operation::WriteLong(address, value);
	let address = address as usize;
	unsafe {
		register_op(op);
		musashi_memory[address+0] = (value & 0xff000000 >> 24) as u8;
		musashi_memory[address+1] = (value & 0x00ff0000 >> 16) as u8;
		musashi_memory[address+2] = (value & 0x0000ff00 >>  8) as u8;
		musashi_memory[address+3] = (value & 0x000000ff >>  0) as u8;
	}
}

#[no_mangle]
pub extern fn cpu_pulse_reset() {panic!("pr")}
#[no_mangle]
pub extern fn cpu_long_branch() {println!("lb")}
#[no_mangle]
pub extern fn cpu_set_fc(fc: u32) {println!("set_fc {}", fc)}
#[no_mangle]
pub extern fn cpu_irq_ack(level: i32) -> i32 {panic!("ia")}
#[no_mangle]
pub extern fn cpu_instr_callback() {println!("ic")}

use std::ptr;

pub fn experimental_communication() {
	unsafe {
		m68k_init();
		m68k_set_cpu_type(CpuType::M68000);
		m68k_set_reg(Register::D0, 123);
		println!("D0: {}", m68k_get_reg(ptr::null_mut(), Register::D0));
	}
}

pub fn roundtrip_register(reg: Register, value: u32) -> u32 {
	unsafe {
		m68k_init();
		m68k_set_cpu_type(CpuType::M68000);
		m68k_set_reg(reg, value);
		m68k_get_reg(ptr::null_mut(), reg)
	}
}

use cpu::Core;

pub fn roundtrip_core(core: &mut Core) {
	unsafe {
		m68k_init();
		m68k_set_cpu_type(CpuType::M68000);
		let regs = [Register::D0, Register::D1, Register::D2, Register::D3, Register::D4, Register::D5, Register::D6, Register::D7, Register::A0, Register::A1, Register::A2, Register::A3, Register::A4, Register::A5, Register::A6, Register::A7];
		for (i, &reg) in regs.iter().enumerate() {m68k_set_reg(reg, core.dar[i]);}
		m68k_set_reg(Register::PC, core.pc);
		m68k_set_reg(Register::SP, core.sp);
		m68k_set_reg(Register::SR, core.status_register());

		m68k_execute(1);

		for (i, &reg) in regs.iter().enumerate() {
			core.dar[i] = m68k_get_reg(ptr::null_mut(), reg);
		}
		core.pc = m68k_get_reg(ptr::null_mut(), Register::PC);
		core.sp = m68k_get_reg(ptr::null_mut(), Register::SP);
		core.sr_to_flags(m68k_get_reg(ptr::null_mut(), Register::SR));
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::musashi_memory;
	use cpu::Core;

	#[test]
	fn roundtrip_d0() {
		assert_eq!(256, roundtrip_register(Register::D0, 256));
	}

	#[test]
	fn roundtrip_abcd_rr() {
		let mut cpu = Core::new_mem(0x40, &[0xc3, 0x00]);
		unsafe {
			let offset = cpu.pc as usize;
			for (i,b) in cpu.mem.iter().enumerate() {
				musashi_memory[i] = *b;
			}
		}
		cpu.dar[0] = 0x16;
		cpu.dar[1] = 0x26;
		roundtrip_core(&mut cpu);

		// 16 + 26 is 42
		assert_eq!(0x42, cpu.dar[1]);
	}
}