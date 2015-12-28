#![macro_use]
use super::Core;
macro_rules! ir_dx {
	($e:ident) => (($e.ir >> 9 & 7) as usize);
}
macro_rules! ir_dy {
	($e:ident) => (($e.ir & 7) as usize);
}
macro_rules! ir_ax {
	($e:ident) => (8+($e.ir >> 9 & 7) as usize);
}
macro_rules! ir_ay {
	($e:ident) => (8+($e.ir & 7) as usize);
}
macro_rules! dx {
	($e:ident) => ($e.dar[ir_dx!($e)]);
}
macro_rules! dy {
	($e:ident) => ($e.dar[ir_dy!($e)]);
}
macro_rules! ax {
	($e:ident) => ($e.dar[ir_ax!($e)]);
}
macro_rules! ay {
	($e:ident) => ($e.dar[ir_ay!($e)]);
}
macro_rules! mask_out_above_8 {
	($e:expr) => ($e & 0xff)
}
macro_rules! mask_out_below_8 {
	($e:expr) => ($e & !0xff)
}
macro_rules! low_nibble {
	($e:expr) => ($e & 0x0f);
}
macro_rules! high_nibble {
	($e:expr) => ($e & 0xf0);
}
macro_rules! true1 {
	($e:expr) => (if $e {1} else {0})
}
macro_rules! not1 {
	($e:expr) => (true1!($e == 0))
}

pub mod fake {
	use super::super::Core;

	pub fn set_d0(core: &mut Core) {
		core.dar[0] = 0xabcd;
	}

	pub fn set_d1(core: &mut Core) {
		core.dar[1] = 0xbcde;
	}

	pub fn set_dx(core: &mut Core) {
		dx!(core) = 0xcdef;
	}

	use super::super::InstructionSet;
	use super::illegal;
	const SET_DX_0: usize = 0b0100_0000_0000_0000;

	pub fn instruction_set() -> InstructionSet {
		// Covers all possible IR values (64k entries)
		let mut handler: InstructionSet = Vec::with_capacity(0x10000);
		for _ in 0..0x10000 { handler.push(illegal); }
		handler[0xA] = set_d0;
		handler[0xB] = set_d1;
		for i in 0..8 {
			let opcode = SET_DX_0 | (i << 9);
			// println!("{:x}", opcode);
			handler[opcode] = set_dx;
		}
		handler
	}
}

pub fn illegal(core: &mut Core) {
	panic!("Illegal instruction {:04x} at {:08x}", core.ir, core.pc-2);
}

use std::num::Wrapping;

pub fn abcd_8_common(core: &mut Core, dst: u32, src: u32) -> u32 {
	let mut res = low_nibble!(src) + low_nibble!(dst) + core.x_flag_as_1();

	// m68ki_cpu.v_flag = ~res;
	core.v_flag = !res;

	// if(res > 9)
	//  res += 6;
	if res > 9 {
		res += 6;
	}
	// res += ((src) & 0xf0) + ((dst) & 0xf0);
	res += high_nibble!(src) + high_nibble!(dst);
	// m68ki_cpu.x_flag = m68ki_cpu.c_flag = (res > 0x99) << 8;
	core.c_flag = true1!(res > 0x99) << 8;
	core.x_flag = core.c_flag;

	if core.c_flag > 0 {
		res = (Wrapping(res) - Wrapping(0xa0)).0;
	}

	// m68ki_cpu.v_flag &= res;
	// m68ki_cpu.n_flag = (res);
	core.v_flag &= res;
	core.n_flag = res;

	// res = ((res) & 0xff);
	// m68ki_cpu.not_z_flag |= res;
	res = mask_out_above_8!(res);
	core.not_z_flag |= res;
	res
}

// All instructions are ported from https://github.com/kstenerud/Musashi
// First real instruction
pub fn abcd_8_rr(core: &mut Core) {
	// unsigned int* r_dst = &(m68ki_cpu.dar[(m68ki_cpu.ir >> 9) & 7]);
	// unsigned int src = (m68ki_cpu.dar[m68ki_cpu.ir & 7]);
	// unsigned int dst = *r_dst;
	// unsigned int res = ((src) & 0x0f) + ((dst) & 0x0f) + ((m68ki_cpu.x_flag>>8)&1);
	let dst = dx!(core);
	let src = dy!(core);
	let res = abcd_8_common(core, dst, src);
	// *r_dst = ((*r_dst) & ~0xff) | res;
	dx!(core) = mask_out_below_8!(dst) | res;
}

use ram::{AddressBus, SUPERVISOR_DATA, USER_DATA};
fn ea_predecrement_8(core: &mut Core, reg_ndx: usize) -> u32 {
	// pre-decrement
	core.dar[reg_ndx] -= match reg_ndx {
		15 => 2, // A7 is kept even
		 _ => 1
	};
	core.dar[reg_ndx]
}
fn ea_ay_pd_8(core: &mut Core) -> u32 {
	let reg_ndx = ir_ay!(core);
	ea_predecrement_8(core, reg_ndx)
}
fn ea_ax_pd_8(core: &mut Core) -> u32 {
	let reg_ndx = ir_ax!(core);
	ea_predecrement_8(core, reg_ndx)
}
fn oper_ay_pd_8(core: &mut Core) -> u32 {
	let ea = ea_ay_pd_8(core);
	let address_space = if core.s_flag != 0 {SUPERVISOR_DATA} else {USER_DATA};
	core.mem.read_byte(address_space, ea)
}
fn oper_ax_pd_8(core: &mut Core) -> (u32, u32) {
	let ea = ea_ax_pd_8(core);
	let address_space = if core.s_flag != 0 {SUPERVISOR_DATA} else {USER_DATA};
	(core.mem.read_byte(address_space, ea), ea)
}

// Second real instruction
pub fn abcd_8_mm(core: &mut Core) {
	// unsigned int src = OPER_AY_PD_8();
	let src = oper_ay_pd_8(core);
	// unsigned int ea = (--((m68ki_cpu.dar+8)[(m68ki_cpu.ir >> 9) & 7]));
	// unsigned int dst = m68ki_read_8_fc (ea, m68ki_cpu.s_flag | m68ki_address_space);
	let (dst, ea) = oper_ax_pd_8(core);

	let res = abcd_8_common(core, dst, src);

	// m68ki_write_8_fc (ea, m68ki_cpu.s_flag | 1, res);		*/
	core.mem.write_byte(SUPERVISOR_DATA, ea, res);
}

use super::Handler;
#[allow(dead_code)]
struct OpcodeHandler {
	mask: u32,
	matching: u32,
	name: String,
	handler: Handler
}

use super::InstructionSet;
macro_rules! op_entry {
    ($mask:expr, $matching:expr, $handler:ident) => (OpcodeHandler { mask: $mask, matching: $matching, handler: $handler, name: stringify!($handler).to_string() })
}
pub const MASK_OUT_X_Y: u32 = 0b1111000111111000; // masks out X and Y register bits (????xxx??????yyy)
pub const OP_ABCD_8_RR: u32 = 0xc100;
pub const OP_ABCD_8_MM: u32 = 0xc108;
pub fn instruction_set() -> InstructionSet {
	// Covers all possible IR values (64k entries)
	let mut handler: InstructionSet = Vec::with_capacity(0x10000);
	for _ in 0..0x10000 { handler.push(illegal); }
	//let handler = [illegal].iter().cycle().take(0x10000).collect::<InstructionSet>();
	// (0..0x10000).map(|_| illegal).collect::<InstructionSet>();
	// the optable contains opcode mask, matching mask and the corresponding handler + name
	let optable = vec![
		op_entry!(MASK_OUT_X_Y, OP_ABCD_8_RR, abcd_8_rr),
		op_entry!(MASK_OUT_X_Y, OP_ABCD_8_MM, abcd_8_mm),
	];
	for op in optable {
		for opcode in 0..0x10000 {
			if (opcode & op.mask) == op.matching {
				// println!("{:16b}: {}", opcode, op.name);
				handler[opcode as usize] = op.handler;
			}
		}
	}
	handler
}

#[cfg(test)]
mod tests {
	use super::super::Core;
	use super::{oper_ax_pd_8, oper_ay_pd_8};
	use ram::{AddressBus, SUPERVISOR_DATA};

	#[test]
	fn low_nibble() {
		assert_eq!(0x0a, low_nibble!(0xba));
	}
	#[test]
	fn high_nibble() {
		assert_eq!(0xb0, high_nibble!(0xba));
	}
	#[test]
	fn mask_out_below_8() {
		assert_eq!(0x2bcdef00, mask_out_below_8!(0x2bcdef73));
	}
	#[test]
	fn mask_out_above_8() {
		assert_eq!(0xf1, mask_out_above_8!(0x2bcdeff1));
	}
	#[test]
	fn dx_and_dy() {
		let mut core = Core::new(0x40);
		core.dar[0] = 0x00;
		core.dar[1] = 0x11;
		core.dar[2] = 0x22;
		core.dar[3] = 0x33;
		core.dar[4] = 0x44;
		core.dar[5] = 0x55;
		core.dar[6] = 0x66;
		core.dar[7] = 0x77;

		core.ir = 0b1111_1001_1111_1010; // X=4, Y=2
		assert_eq!(0x22, dy!(core));
		assert_eq!(0x44, dx!(core));

		core.ir = 0b1111_1011_1111_1110; // X=5, Y=6
		assert_eq!(0x66, dy!(core));
		assert_eq!(0x55, dx!(core));
	}
	#[test]
	fn predecrement_ax() {
		let mut core = Core::new(0x40);
		for i in 0..8 {
			let addr: u32 = 0x200 + 4*i;
			core.dar[8+i as usize] = addr;
			// write just before where A0-A7 points
			let adjustment = if i == 7 {2} else {1};
			core.mem.write_byte(SUPERVISOR_DATA, addr - adjustment, 0x11*i);
		}
		core.ir = 0b1111_1001_1111_1010; // X=4, Y=2
		let core = &mut core;

		assert_eq!(512+4*4, core.dar[8+4]);
		let (dst, ea) = oper_ax_pd_8(core);
		assert_eq!(0x44, dst);
		assert_eq!(512+4*4-1, core.dar[8+4]);
		assert_eq!(512+4*4-1, ea);

		core.ir = 0b1111_1111_1111_1111; // X=7, Y=7
		assert_eq!(512+4*7, core.dar[8+7]);
		let (dst, ea) = oper_ax_pd_8(core);
		assert_eq!(0x77, dst);
		// A7 is kept even
		assert_eq!(512+4*7-2, core.dar[8+7]);
		assert_eq!(512+4*7-2, ea);
	}
	#[test]
	fn predecrement_ay() {
		let mut core = Core::new(0x40);
		for i in 0..8 {
			let addr: u32 = 0x200 + 4*i;
			core.dar[8+i as usize] = addr;
			// write just before where A0-A7 points
			let adjustment = if i == 7 {2} else {1};
			core.mem.write_byte(SUPERVISOR_DATA, addr - adjustment, 0x11*i);
		}

		core.ir = 0b1111_1001_1111_1010; // X=4, Y=2
		let core = &mut core;
		assert_eq!(512+4*2, core.dar[8+2]);
		assert_eq!(0x22, oper_ay_pd_8(core));
		assert_eq!(512+4*2-1, core.dar[8+2]);

		core.ir = 0b1111_1011_1111_1111; // X=5, Y=7
		assert_eq!(512+4*7, core.dar[8+7]);
		assert_eq!(0x77, oper_ay_pd_8(core));
		// A7 is kept even
		assert_eq!(512+4*7-2, core.dar[8+7]);
	}
}
