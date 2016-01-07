#![macro_use]
use super::{Core, Cycles, Result};
use super::Exception::IllegalInstruction;

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
macro_rules! mask_out_above_16 {
	($e:expr) => ($e & 0xffff)
}
macro_rules! mask_out_below_16 {
	($e:expr) => ($e & !0xffff)
}
macro_rules! mask_out_above_32 {
	($e:expr) => ($e & 0xffffffff)
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
	use super::super::{Core, Cycles, Result};

	pub fn set_d0(core: &mut Core) -> Result<Cycles> {
		core.dar[0] = 0xabcd;
		Ok(Cycles(2))
	}

	pub fn set_d1(core: &mut Core) -> Result<Cycles> {
		core.dar[1] = 0xbcde;
		Ok(Cycles(2))
	}

	pub fn set_dx(core: &mut Core) -> Result<Cycles> {
		dx!(core) = 0xcdef;
		Ok(Cycles(2))
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

pub fn illegal(core: &mut Core) -> Result<Cycles> {
	Err(IllegalInstruction(core.ir, core.pc-2))
}

use std::num::Wrapping;
use super::operator;

// All instructions are ported from https://github.com/kstenerud/Musashi
fn abcd_8_common(core: &mut Core, dst: u32, src: u32) -> u32 {
	// unsigned int res = ((src) & 0x0f) + ((dst) & 0x0f) + ((m68ki_cpu.x_flag>>8)&1);
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
pub fn abcd_8_rr(core: &mut Core) -> Result<Cycles> {
	let dst = try!(operator::dx(core));
	let src = try!(operator::dy(core));
	let res = abcd_8_common(core, dst, src);
	dx!(core) = mask_out_below_8!(dst) | res;
	Ok(Cycles(6))
}
pub fn abcd_8_mm(core: &mut Core) -> Result<Cycles> {
	let src = try!(operator::ay_pd_8(core));
	let (dst, ea) = try!(operator::ax_pd_8(core));
	let res = abcd_8_common(core, dst, src);
	core.write_data_byte(ea, res);
	Ok(Cycles(18))
}

fn add_8_common(core: &mut Core, dst: u32, src: u32) -> u32 {
	let dst = mask_out_above_8!(dst);
	let src = mask_out_above_8!(src);

	let res = dst + src;
	// m68ki_cpu.n_flag = (res);
	core.n_flag = res;
	// m68ki_cpu.v_flag = ((src^res) & (dst^res));
	core.v_flag = (src ^ res) & (dst ^ res);
	// m68ki_cpu.x_flag = m68ki_cpu.c_flag = (res);
	core.c_flag = res;
	core.x_flag = res;
	// m68ki_cpu.not_z_flag = ((res) & 0xff);
	let res8 = mask_out_above_8!(res);
	core.not_z_flag = res8;
	res8
}
fn add_16_common(core: &mut Core, dst: u32, src: u32) -> u32 {
	let dst = mask_out_above_16!(dst);
	let src = mask_out_above_16!(src);
	let res = dst + src;

	// m68ki_cpu.n_flag = ((res)>>8);
	let res_hi = res >> 8;
	core.n_flag = res_hi;
	// m68ki_cpu.v_flag = (((src^res) & (dst^res))>>8);
	core.v_flag = ((src ^ res) & (dst ^ res)) >> 8;
	// m68ki_cpu.x_flag = m68ki_cpu.c_flag = ((res)>>8);
	core.c_flag = res_hi;
	core.x_flag = res_hi;
	// m68ki_cpu.not_z_flag = ((res) & 0xffff);
	let res16 = mask_out_above_16!(res);
	core.not_z_flag = res16;

	res16
}
fn add_32_common(core: &mut Core, dst: u32, src: u32) -> u32 {
	let res: u64 = (dst as u64) + (src as u64);

	let res_hi = (res >> 24) as u32;
	core.n_flag = res_hi;
	// m68ki_cpu.v_flag = (((src^res) & (dst^res))>>24);
	core.v_flag = (((src as u64 ^ res) & (dst as u64 ^ res)) >> 24) as u32;
 	// m68ki_cpu.x_flag = m68ki_cpu.c_flag = (((src & dst) | (~res & (src | dst)))>>23);
	core.c_flag = res_hi;
	core.x_flag = res_hi;

	let res32 = res as u32;

	core.not_z_flag = res32;

	res32
}

macro_rules! add_8_er {
	($name:ident, $src:ident, $cycles:expr) => (
		pub fn $name(core: &mut Core) -> Result<Cycles> {
			let dst = try!(operator::dx(core));
			let src = try!(operator::$src(core));
			let res = add_8_common(core, dst, src);
			dx!(core) = mask_out_below_8!(dst) | res;
			Ok(Cycles($cycles))
		})
}
macro_rules! add_16_er {
	($name:ident, $src:ident, $cycles:expr) => (
		pub fn $name(core: &mut Core) -> Result<Cycles> {
			let dst = try!(operator::dx(core));
			let src = try!(operator::$src(core));
			let res = add_16_common(core, dst, src);
			dx!(core) = mask_out_below_16!(dst) | res;
			Ok(Cycles($cycles))
		})
}
macro_rules! add_32_er {
	($name:ident, $src:ident, $cycles:expr) => (
		pub fn $name(core: &mut Core) -> Result<Cycles> {
			let dst = try!(operator::dx(core));
			let src = try!(operator::$src(core));
			let res = add_32_common(core, dst, src);
			dx!(core) = res;
			Ok(Cycles($cycles))
		})
}
add_8_er!(add_8_er_d, dy, 4);
// add_8_er!(add_8_er_a, ay) not present - for word and long only
add_8_er!(add_8_er_ai, ay_ai_8,   8);
add_8_er!(add_8_er_pi, ay_pi_8,   8);
add_8_er!(add_8_er_pd, ay_pd_8,  10);
add_8_er!(add_8_er_di, ay_di_8,  12);
add_8_er!(add_8_er_ix, ay_ix_8,  14);
add_8_er!(add_8_er_aw, aw_8,     12);
add_8_er!(add_8_er_al, al_8,     16);
add_8_er!(add_8_er_pcdi, pcdi_8, 12);
add_8_er!(add_8_er_pcix, pcix_8, 14);
add_8_er!(add_8_er_imm, imm_8,   10);

add_16_er!(add_16_er_d, dy,          4);
add_16_er!(add_16_er_a, ay,          4);
add_16_er!(add_16_er_ai, ay_ai_16,   8);
add_16_er!(add_16_er_pi, ay_pi_16,   8);
add_16_er!(add_16_er_pd, ay_pd_16,  10);
add_16_er!(add_16_er_di, ay_di_16,  12);
add_16_er!(add_16_er_ix, ay_ix_16,  14);
add_16_er!(add_16_er_aw, aw_16,     12);
add_16_er!(add_16_er_al, al_16,     16);
add_16_er!(add_16_er_pcdi, pcdi_16, 12);
add_16_er!(add_16_er_pcix, pcix_16, 14);
add_16_er!(add_16_er_imm, imm_16,   10);

add_32_er!(add_32_er_d, dy,          6);
add_32_er!(add_32_er_a, ay,          6);
add_32_er!(add_32_er_ai, ay_ai_32,  14);
add_32_er!(add_32_er_pi, ay_pi_32,  14);
add_32_er!(add_32_er_pd, ay_pd_32,  16);
add_32_er!(add_32_er_di, ay_di_32,  18);
add_32_er!(add_32_er_ix, ay_ix_32,  20);
add_32_er!(add_32_er_aw, aw_32,     18);
add_32_er!(add_32_er_al, al_32,     22);
add_32_er!(add_32_er_pcdi, pcdi_32, 18);
add_32_er!(add_32_er_pcix, pcix_32, 20);
add_32_er!(add_32_er_imm, imm_32,   16);

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
pub const MASK_OUT_X: u32 = 0b1111000111111111; // masks out X register bits (????xxx?????????)

const OP_ADD   : u32 = 0b1101_0000_0000_0000;

const OPER_D   : u32 = 0x00;
const OPER_A   : u32 = 0x08;
const OPER_AI  : u32 = 0x10;
const OPER_PI  : u32 = 0x18;
const OPER_PD  : u32 = 0x20;
const OPER_DI  : u32 = 0x28;
const OPER_IX  : u32 = 0x30;
const OPER_AW  : u32 = 0x38;
const OPER_AL  : u32 = 0x39;
const OPER_PCDI: u32 = 0x3a;
const OPER_PCIX: u32 = 0x3b;
const OPER_IMM : u32 = 0x3c;

pub const BYTE_SIZED: u32 = 0x00;
pub const WORD_SIZED: u32 = 0x40;
pub const LONG_SIZED: u32 = 0x80;

pub const DEST_DX: u32 = 0x000;
pub const DEST_EA: u32 = 0x100;

// -- OP-constants -------------------------------
pub const OP_ABCD_8_RR: u32 = 0xc100;
pub const OP_ABCD_8_MM: u32 = 0xc108;

pub const OP_ADD_8_ER_D    : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_D;
pub const OP_ADD_8_ER_AI   : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_AI;
pub const OP_ADD_8_ER_PI   : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_PI;
pub const OP_ADD_8_ER_PD   : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_PD;
pub const OP_ADD_8_ER_DI   : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_DI;
pub const OP_ADD_8_ER_IX   : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_IX;
pub const OP_ADD_8_ER_AW   : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_AW;
pub const OP_ADD_8_ER_AL   : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_AL;
pub const OP_ADD_8_ER_PCDI : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_PCDI;
pub const OP_ADD_8_ER_PCIX : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_PCIX;
pub const OP_ADD_8_ER_IMM  : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_IMM;

pub const OP_ADD_16_ER_D   : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_D;
pub const OP_ADD_16_ER_A   : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_A;
pub const OP_ADD_16_ER_AI  : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_AI;
pub const OP_ADD_16_ER_PI  : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_PI;
pub const OP_ADD_16_ER_PD  : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_PD;
pub const OP_ADD_16_ER_DI  : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_DI;
pub const OP_ADD_16_ER_IX  : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_IX;
pub const OP_ADD_16_ER_AW  : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_AW;
pub const OP_ADD_16_ER_AL  : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_AL;
pub const OP_ADD_16_ER_PCDI: u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_PCDI;
pub const OP_ADD_16_ER_PCIX: u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_PCIX;
pub const OP_ADD_16_ER_IMM : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_IMM;

pub const OP_ADD_32_ER_D   : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_D;
pub const OP_ADD_32_ER_A   : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_A;
pub const OP_ADD_32_ER_AI  : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_AI;
pub const OP_ADD_32_ER_PI  : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_PI;
pub const OP_ADD_32_ER_PD  : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_PD;
pub const OP_ADD_32_ER_DI  : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_DI;
pub const OP_ADD_32_ER_IX  : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_IX;
pub const OP_ADD_32_ER_AW  : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_AW;
pub const OP_ADD_32_ER_AL  : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_AL;
pub const OP_ADD_32_ER_PCDI: u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_PCDI;
pub const OP_ADD_32_ER_PCIX: u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_PCIX;
pub const OP_ADD_32_ER_IMM : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_IMM;

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

		op_entry!(MASK_OUT_X_Y, OP_ADD_8_ER_D,    add_8_er_d),
		op_entry!(MASK_OUT_X_Y, OP_ADD_8_ER_AI,   add_8_er_ai),
		op_entry!(MASK_OUT_X_Y, OP_ADD_8_ER_PI,   add_8_er_pi),
		op_entry!(MASK_OUT_X_Y, OP_ADD_8_ER_PD,   add_8_er_pd),
		op_entry!(MASK_OUT_X_Y, OP_ADD_8_ER_DI,   add_8_er_di),
		op_entry!(MASK_OUT_X_Y, OP_ADD_8_ER_IX,   add_8_er_ix),
		op_entry!(MASK_OUT_X,   OP_ADD_8_ER_AW,   add_8_er_aw),
		op_entry!(MASK_OUT_X,   OP_ADD_8_ER_AL,   add_8_er_al),
		op_entry!(MASK_OUT_X,   OP_ADD_8_ER_PCDI, add_8_er_pcdi),
		op_entry!(MASK_OUT_X,   OP_ADD_8_ER_PCIX, add_8_er_pcix),
		op_entry!(MASK_OUT_X,   OP_ADD_8_ER_IMM,  add_8_er_imm),

		op_entry!(MASK_OUT_X_Y, OP_ADD_16_ER_D,    add_16_er_d),
		op_entry!(MASK_OUT_X_Y, OP_ADD_16_ER_A,    add_16_er_a),
		op_entry!(MASK_OUT_X_Y, OP_ADD_16_ER_AI,   add_16_er_ai),
		op_entry!(MASK_OUT_X_Y, OP_ADD_16_ER_PI,   add_16_er_pi),
		op_entry!(MASK_OUT_X_Y, OP_ADD_16_ER_PD,   add_16_er_pd),
		op_entry!(MASK_OUT_X_Y, OP_ADD_16_ER_DI,   add_16_er_di),
		op_entry!(MASK_OUT_X_Y, OP_ADD_16_ER_IX,   add_16_er_ix),
		op_entry!(MASK_OUT_X,   OP_ADD_16_ER_AW,   add_16_er_aw),
		op_entry!(MASK_OUT_X,   OP_ADD_16_ER_AL,   add_16_er_al),
		op_entry!(MASK_OUT_X,   OP_ADD_16_ER_PCDI, add_16_er_pcdi),
		op_entry!(MASK_OUT_X,   OP_ADD_16_ER_PCIX, add_16_er_pcix),
		op_entry!(MASK_OUT_X,   OP_ADD_16_ER_IMM,  add_16_er_imm),

		op_entry!(MASK_OUT_X_Y, OP_ADD_32_ER_D,    add_32_er_d),
		op_entry!(MASK_OUT_X_Y, OP_ADD_32_ER_A,    add_32_er_a),
		op_entry!(MASK_OUT_X_Y, OP_ADD_32_ER_AI,   add_32_er_ai),
		op_entry!(MASK_OUT_X_Y, OP_ADD_32_ER_PI,   add_32_er_pi),
		op_entry!(MASK_OUT_X_Y, OP_ADD_32_ER_PD,   add_32_er_pd),
		op_entry!(MASK_OUT_X_Y, OP_ADD_32_ER_DI,   add_32_er_di),
		op_entry!(MASK_OUT_X_Y, OP_ADD_32_ER_IX,   add_32_er_ix),
		op_entry!(MASK_OUT_X,   OP_ADD_32_ER_AW,   add_32_er_aw),
		op_entry!(MASK_OUT_X,   OP_ADD_32_ER_AL,   add_32_er_al),
		op_entry!(MASK_OUT_X,   OP_ADD_32_ER_PCDI, add_32_er_pcdi),
		op_entry!(MASK_OUT_X,   OP_ADD_32_ER_PCIX, add_32_er_pcix),
		op_entry!(MASK_OUT_X,   OP_ADD_32_ER_IMM,  add_32_er_imm),
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
}
