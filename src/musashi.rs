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

// callbacks from Musashi
#[no_mangle]
pub extern fn cpu_read_byte(address: u32) -> u32 {panic!("rb")}
#[no_mangle]
pub extern fn cpu_read_word(address: u32) -> u32 {panic!("rw")}
#[no_mangle]
pub extern fn cpu_read_long(address: u32) -> u32 {panic!("rl")}
#[no_mangle]
pub extern fn cpu_write_byte(address: u32, value: u32) {panic!("wb")}
#[no_mangle]
pub extern fn cpu_write_word(address: u32, value: u32) {panic!("ww")}
#[no_mangle]
pub extern fn cpu_write_long(address: u32, value: u32) {panic!("wl")}
#[no_mangle]
pub extern fn cpu_pulse_reset() {panic!("pr")}
#[no_mangle]
pub extern fn cpu_long_branch() {panic!("lb")}
#[no_mangle]
pub extern fn cpu_set_fc(fc: u32) {panic!("sf")}
#[no_mangle]
pub extern fn cpu_irq_ack(level: i32) -> i32 {panic!("ia")}

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
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn roundtrip_D0() {
		assert_eq!(256, roundtrip_register(Register::D0, 256));
	}
}