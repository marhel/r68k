// Integration with Musashi
extern crate libc;


// Register enum copied from Musashi's m68k_register_t enum
#[repr(C)]
enum Register {
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

#[link(name = "musashi", kind = "static")]
extern {
	fn m68k_init();
	fn m68k_set_cpu_type(cputype: u32);
	fn m68k_pulse_reset();
	fn m68k_execute(num_cycles: i32) -> i32;
	fn m68k_get_reg(context: *mut libc::c_void, regnum: Register) -> u32;
	fn m68k_set_reg(regnum: Register, value: u32);
}

// callbacks from Musashi
#[no_mangle]
pub extern fn cpu_read_byte(address: u32) -> u32 {panic!("tjoo")}
#[no_mangle]
pub extern fn cpu_read_word(address: u32) -> u32 {panic!("tjoo")}
#[no_mangle]
pub extern fn cpu_read_long(address: u32) -> u32 {panic!("tjoo")}
#[no_mangle]
pub extern fn cpu_write_byte(address: u32, value: u32) {panic!("tjoo")}
#[no_mangle]
pub extern fn cpu_write_word(address: u32, value: u32) {panic!("tjoo")}
#[no_mangle]
pub extern fn cpu_write_long(address: u32, value: u32) {panic!("tjoo")}
#[no_mangle]
pub extern fn cpu_pulse_reset() {panic!("tjoo")}
#[no_mangle]
pub extern fn cpu_long_branch() {panic!("tjoo")}
#[no_mangle]
pub extern fn cpu_set_fc(fc: u32) {panic!("tjoo")}
#[no_mangle]
pub extern fn cpu_irq_ack(level: i32) -> i32 {panic!("tjoo")}

use std::ptr;

pub fn experimental_communication() {
	unsafe {
		m68k_init();
		m68k_set_reg(Register::D0, 123);
		println!("D0: {}", m68k_get_reg(ptr::null_mut(), Register::D0));
	}
}