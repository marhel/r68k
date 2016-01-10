#![macro_use]
use super::super::Core;
use std::num::Wrapping;

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

// All instructions are ported from https://github.com/kstenerud/Musashi
pub fn abcd_8(core: &mut Core, dst: u32, src: u32) -> u32 {
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

pub fn add_8(core: &mut Core, dst: u32, src: u32) -> u32 {
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
pub fn add_16(core: &mut Core, dst: u32, src: u32) -> u32 {
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
pub fn add_32(core: &mut Core, dst: u32, src: u32) -> u32 {
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
