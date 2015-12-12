// Integration with Musashi
extern crate libc;


// Register enum copied from Musashi's m68k_register_t enum
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(dead_code)]
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
#[allow(dead_code)]
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
use ram::{Operation, AddressBus, AddressSpace, SUPERVISOR_PROGRAM, SUPERVISOR_DATA, USER_PROGRAM, USER_DATA};
static mut musashi_memory:  [u8; 1024] = [0u8; 1024];
// as statics are not allowed to have destructors, allocate a
// big enough array to hold the small number of operations
// expected from executing a very limited number of opcodes
static mut musashi_ops: [Operation; 128] = [Operation::None; 128];
static mut musashi_opcount: usize = 0;
static mut musashi_address_space: AddressSpace = SUPERVISOR_PROGRAM;

unsafe fn register_op(op: Operation) {
	if musashi_opcount < musashi_ops.len() {
		musashi_ops[musashi_opcount] = op;
		musashi_opcount += 1;
	}
}
// callbacks from Musashi
#[no_mangle]
pub extern fn cpu_read_byte(address: u32) -> u32 {
	unsafe {
		let op = Operation::ReadByte(musashi_address_space, address);
		let address = address as usize;
		register_op(op);
		musashi_memory[address] as u32
	}
}
#[no_mangle]
pub extern fn cpu_read_word(address: u32) -> u32 {
	unsafe {
		let op = Operation::ReadWord(musashi_address_space, address);
		let address = address as usize;
		register_op(op);
		((musashi_memory[address+0] as u32) << 8
		|(musashi_memory[address+1] as u32) << 0) as u32
	}
}
#[no_mangle]
pub extern fn cpu_read_long(address: u32) -> u32 {
	unsafe {
		let op = Operation::ReadLong(musashi_address_space, address);
		let address = address as usize;
		register_op(op);
		((musashi_memory[address+0] as u32) << 24
		|(musashi_memory[address+1] as u32) << 16
		|(musashi_memory[address+2] as u32) <<  8
		|(musashi_memory[address+3] as u32) <<  0) as u32
	}
}

#[no_mangle]
pub extern fn cpu_write_byte(address: u32, value: u32) {
	unsafe {
		let op = Operation::WriteByte(musashi_address_space, address, value);
		let address = address as usize;
		register_op(op);
		musashi_memory[address+0] = (value & 0xff) as u8;
	}
}
#[no_mangle]
pub extern fn cpu_write_word(address: u32, value: u32) {
	unsafe {
		let op = Operation::WriteWord(musashi_address_space, address, value);
		let address = address as usize;
		register_op(op);
		musashi_memory[address+0] = (value & 0xff00 >> 8) as u8;
		musashi_memory[address+1] = (value & 0x00ff >> 0) as u8;
	}
}
#[no_mangle]
pub extern fn cpu_write_long(address: u32, value: u32) {
	unsafe {
		let op = Operation::WriteLong(musashi_address_space, address, value);
		let address = address as usize;
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
pub extern fn cpu_long_branch() {}
#[no_mangle]
pub extern fn cpu_set_fc(fc: u32) {
	unsafe {
		musashi_address_space = match fc {
			1 => USER_DATA,
			2 => USER_PROGRAM,
			5 => SUPERVISOR_DATA,
			6 => SUPERVISOR_PROGRAM,
			_ => panic!("unknown fc: {}", fc),
		};
		// println!("set_fc {:?}", musashi_address_space);
	}
}
#[allow(unused_variables)]
#[no_mangle]
pub extern fn cpu_irq_ack(level: i32) -> i32 {panic!("ia")}
#[no_mangle]
pub extern fn cpu_instr_callback() {}

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

pub fn execute1(core: &mut Core) {
	unsafe {
		m68k_init();
		m68k_set_cpu_type(CpuType::M68000);
		m68k_pulse_reset();
		// Resetting opcount, because m68k_pulse_reset causes irrelevant
		// reads from 0x00000000 to set PC/SP, a jump to PC and
		// resetting of state. But we don't want to test those ops.
		musashi_opcount = 0;
		let regs = [Register::D0, Register::D1, Register::D2, Register::D3, Register::D4, Register::D5, Register::D6, Register::D7, Register::A0, Register::A1, Register::A2, Register::A3, Register::A4, Register::A5, Register::A6, Register::A7];
		m68k_set_reg(Register::PC, core.pc);
		m68k_set_reg(Register::USP, core.inactive_usp);
		m68k_set_reg(Register::SR, core.status_register());
		for (i, &reg) in regs.iter().enumerate() { m68k_set_reg(reg, core.dar[i]); }
		for i in 0..1024usize {
			musashi_memory[i] = core.mem.read_byte(SUPERVISOR_PROGRAM, i as u32) as u8;
		}
		m68k_execute(1);

		for (i, &reg) in regs.iter().enumerate() {
			core.dar[i] = m68k_get_reg(ptr::null_mut(), reg);
		}
		core.pc = m68k_get_reg(ptr::null_mut(), Register::PC);
		core.inactive_usp = m68k_get_reg(ptr::null_mut(), Register::USP);
		core.sr_to_flags(m68k_get_reg(ptr::null_mut(), Register::SR));
	}
}
extern crate quickcheck;

#[cfg(test)]
mod tests {
	use super::*;
	use ram::SUPERVISOR_PROGRAM;
	use super::musashi_ops;
	use super::musashi_opcount;
	use ram::Operation;
	use cpu::Core;

	use musashi::quickcheck::*;
	#[derive(Copy, Clone, Debug, PartialEq)]
	struct Bitpattern(u32);
	impl Arbitrary for Bitpattern {
		fn arbitrary<G: Gen>(g: &mut G) -> Bitpattern {
			// let m : u32 = Arbitrary::arbitrary(g);
			// let mut mask: u32 = 0xF; //((m & 0xF) | (m >> 4) & 0xF) as u32;
			// let mut i : u32 = Arbitrary::arbitrary(g);
			// let mut sum: u32 = 0;
			// println!("{}/{} when {}", i, mask, g.size());
			// // 0b11001100 => 0xFF00FF00
			// while i > 0 {
			// 	sum += if i & 1 == 1 { mask } else { 0 };
			// 	i >>= 1;
			// 	mask <<= 4;
			// }

			// when size 256, could generate any 32 bit pattern
			let i1: u32 = Arbitrary::arbitrary(g);
			let i2: u32 = Arbitrary::arbitrary(g);
			let i3: u32 = Arbitrary::arbitrary(g);
			let i4: u32 = Arbitrary::arbitrary(g);
			let sum: u32 = (i1 << 24) | (i2 << 16) | (i3 << 8) | i4;
			// println!("{:b} when {}", i4, g.size());
			Bitpattern(sum)
		}
		fn shrink(&self) -> Box<Iterator<Item=Self>> {
			match *self {
				Bitpattern(x) => {
					let xs = x.shrink(); // should shrink Bitpattern by clearing bits, not setting new ones
					let tagged = xs //.inspect(|x|println!("{}", x))
					.map(Bitpattern);
					Box::new(tagged)
				}
			}
		}
	}

	impl Arbitrary for Register {
		fn arbitrary<G: Gen>(g: &mut G) -> Register {
			let regs = [Register::D0, Register::D1, Register::D2, Register::D3, Register::D4, Register::D5, Register::D6, Register::D7, Register::A0, Register::A1, Register::A2, Register::A3, Register::A4, Register::A5, Register::A6, 
			// Register::A7, Register::SP, Register::SR, Register::PC
			];
			//println!("{}",i);
			if let Some(&reg) = g.choose(&regs) {
				reg
			} else {
				unreachable!();
			}
		}
	}

	extern crate rand;

	use itertools::{Itertools, assert_equal};

	// struct OpSeq {
	// 	mask: u32,
	// 	matching: u32,
	// 	current_op: u32,
	// }
	// impl OpSeq {
	// 	fn new(mask: u32, matching: u32) -> OpSeq {
	// 		OpSeq { mask: mask, matching: matching, current_op: 0 }
	// 	}
	// }
	// impl Iterator for OpSeq {
	// 	type Item = u32;
	// 	fn next(&mut self) -> Option<u32> {
	// 		if self.current_op == 0x10000 {
	// 			None
	// 		} else {
	// 			while (self.current_op & self.mask) != self.matching && self.current_op < 0x10000 {
	// 				self.current_op += 1;
	// 			}
	// 			if self.current_op == 0x10000 {
	// 				return None;
	// 			}
	// 			let res = Some(self.current_op);
	// 			self.current_op += 1;
	// 			res
	// 		}
	// 	}
	// }

	fn opcodes(mask: u32, matching: u32) -> Vec<u16> {
		(0..0x10000u32)
			.filter(|opcode| (opcode & mask) == matching)
			.map(|v|v as u16).collect::<Vec<u16>>()
	}
	macro_rules! opcodes {
	  ($mask:expr , $matching:expr) => {(0..0x10000).filter(|opcode| (opcode & $mask) == $matching)}
	}

	#[test]
	fn opcodes_from_mask_and_matching(){
		let mut opseq = Vec::new();
		opseq.extend(opcodes!(0xf1f8, 0xc100));
		assert_eq!(64, opseq.len());
		let ops = opseq.iter().unique();
		assert_eq!(64, ops.count());
		if let Some(&min) = opseq.iter().min() {
			assert_eq!(0b1100000100000000, min);
		}
		if let Some(&max) = opseq.iter().max() {
			assert_eq!(0b1100111100000111, max);
		}
	}

	static mut opcode_under_test: u16 = 0;

	fn hammer_cores(rs: Vec<(Register, Bitpattern)>) -> bool {
		let pc = 0x40;
		let mem = unsafe {
			[((opcode_under_test >> 8) & 0xff) as u8, (opcode_under_test & 0xff) as u8]
		};
		let mut musashi = Core::new_mem(pc, &mem);

		for r in rs {
			match r {
				(Register::D0, Bitpattern(bp)) => musashi.dar[0] = bp,
				(Register::D1, Bitpattern(bp)) => musashi.dar[1] = bp,
				(Register::D2, Bitpattern(bp)) => musashi.dar[2] = bp,
				(Register::D3, Bitpattern(bp)) => musashi.dar[3] = bp,
				(Register::D4, Bitpattern(bp)) => musashi.dar[4] = bp,
				(Register::D5, Bitpattern(bp)) => musashi.dar[5] = bp,
				(Register::D6, Bitpattern(bp)) => musashi.dar[6] = bp,
				(Register::D7, Bitpattern(bp)) => musashi.dar[7] = bp,
				(Register::A0, Bitpattern(bp)) => musashi.dar[0+8] = bp,
				(Register::A1, Bitpattern(bp)) => musashi.dar[1+8] = bp,
				(Register::A2, Bitpattern(bp)) => musashi.dar[2+8] = bp,
				(Register::A3, Bitpattern(bp)) => musashi.dar[3+8] = bp,
				(Register::A4, Bitpattern(bp)) => musashi.dar[4+8] = bp,
				(Register::A5, Bitpattern(bp)) => musashi.dar[5+8] = bp,
				(Register::A6, Bitpattern(bp)) => musashi.dar[6+8] = bp,
				(Register::A7, Bitpattern(bp)) => musashi.dar[7+8] = bp,
				(Register::USP, Bitpattern(bp)) => musashi.inactive_usp = bp,
				(Register::SR, Bitpattern(bp)) => musashi.sr_to_flags(bp),
				_ => {
					panic!("No idea how to set {:?}", r.0)
				},
			}
		}

		let mut r68k = musashi.clone(); // so very self-aware!
		execute1(&mut musashi);
		r68k.execute1();

		assert_cores_equal(&musashi, &r68k, pc)
	}

	#[test]
	#[ignore]
	fn test_core_with_quickcheck() {
		for opcode in opcodes(0xf1f8, 0xc100)
		{
			println!("Will hammer {:b}", opcode);
			unsafe {
				opcode_under_test = opcode;
			}
			QuickCheck::new()
			.gen(StdGen::new(rand::thread_rng(), 256))
			.tests(10)
			.quickcheck(hammer_cores as fn(Vec<(Register, Bitpattern)>) -> bool);
		}
	}

	fn get_ops() -> Vec<Operation> {
		let mut res: Vec<Operation> = vec![];
		unsafe {
			for i in 0..musashi_opcount {
				res.push(musashi_ops[i]);
			}
		}
		res
	}

	macro_rules! core_eq {
		($left:ident , $right:ident . $field:ident [ $index:expr ]) => ({
			match (&($left.$field[$index]), &($right.$field[$index])) {
				(left_val, right_val) => {
					if !(*left_val == *right_val) {
						println!("core incoherence: `{}[{}]` differs \
							   ({}: `0x{:x}`, {}: `0x{:x}`)", stringify!($field), $index, stringify!($left), left_val, stringify!($right), right_val);
						return false;
					}
				}
			}
		});
		($left:ident , $right:ident . $field:ident () ?) => ({
			match (&($left.$field()), &($right.$field())) {
				(left_val, right_val) => {
					if !(*left_val == *right_val) {
						println!("core incoherence: `{}()` differs \
							   ({}: `{:?}`, {}: `{:?}`)", stringify!($field), stringify!($left), left_val, stringify!($right), right_val);
						return false;
					}
				}
			}
		});
		($left:ident , $right:ident . $field:ident ()) => ({
			match (&($left.$field()), &($right.$field())) {
				(left_val, right_val) => {
					if !(*left_val == *right_val) {
						println!("core incoherence: `{}()` differs \
							   ({}: `0x{:x}`, {}: `0x{:x}`)", stringify!($field), stringify!($left), left_val, stringify!($right), right_val);
						return false;
					}
				}
			}
		});
		($left:ident , $right:ident . $field:ident) => ({
			match (&($left.$field), &($right.$field)) {
				(left_val, right_val) => {
					if !(*left_val == *right_val) {
						println!("core incoherence: `{}` differs \
							   ({}: `0x{:x}`, {}: `0x{:x}`)", stringify!($field), stringify!($left), left_val, stringify!($right), right_val);
						return false;
					}
				}
			}
		})
	}
	fn assert_cores_equal(musashi: &Core, r68k: &Core, pc: u32) -> bool {
		assert_eq!(get_ops().len(), r68k.mem.logger.ops().len());
		assert_equal(get_ops(), r68k.mem.logger.ops());

		core_eq!(musashi, r68k.pc);
		core_eq!(musashi, r68k.inactive_usp);
		for i in (0..16).rev() {
			core_eq!(musashi, r68k.dar[i]);
		}
		core_eq!(musashi, r68k.flags() ?);
		core_eq!(musashi, r68k.status_register());
		true
	}

	#[test]
	fn roundtrip_d0() {
		assert_eq!(256, roundtrip_register(Register::D0, 256));
	}

	#[test]
	fn roundtrip_abcd_rr() {
		let pc = 0x40;
		let mut cpu = Core::new_mem(pc, &[0xc1, 0x01]);
		cpu.dar[0] = 0x17;
		cpu.dar[1] = 0x27;
		cpu.dar[5] = 0x55555;
		execute1(&mut cpu);

		// 17 + 27 is 44
		assert_eq!(0x44, cpu.dar[0]);
		assert_eq!(0x27, cpu.dar[1]);
		assert_eq!(0x55555, cpu.dar[5]);

		let ops = get_ops();
		assert_eq!(1, ops.len());
		assert_eq!(Operation::ReadLong(SUPERVISOR_PROGRAM, pc), ops[0]);
	}

	#[test]
	fn compare_abcd_rr() {
		let pc = 0x40;
		let mut musashi = Core::new_mem(pc, &[0xc3, 0x01]);
		musashi.dar[0] = 0x16;
		musashi.dar[1] = 0x26;

		let mut r68k = musashi.clone(); // so very self-aware!
		execute1(&mut musashi);
		r68k.execute1();

		assert_cores_equal(&musashi, &r68k, pc);
	}
}