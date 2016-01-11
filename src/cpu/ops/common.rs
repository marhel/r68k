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

pub fn addx_8(core: &mut Core, dst: u32, src: u32) -> u32 {
	let dst = mask_out_above_8!(dst);
	let src = mask_out_above_8!(src);

	let res = dst + src + core.x_flag_as_1();

	core.n_flag = res;
	core.v_flag = (src ^ res) & (dst ^ res);
	core.c_flag = res;
	core.x_flag = res;

	let res8 = mask_out_above_8!(res);
	core.not_z_flag |= res8;
	res8
}
pub fn addx_16(core: &mut Core, dst: u32, src: u32) -> u32 {
	let dst = mask_out_above_16!(dst);
	let src = mask_out_above_16!(src);
	let res = dst + src + core.x_flag_as_1();

	let res_hi = res >> 8;
	core.n_flag = res_hi;
	core.v_flag = ((src ^ res) & (dst ^ res)) >> 8;
	core.c_flag = res_hi;
	core.x_flag = res_hi;

	let res16 = mask_out_above_16!(res);
	core.not_z_flag |= res16;
	res16
}
pub fn addx_32(core: &mut Core, dst: u32, src: u32) -> u32 {
	let res: u64 = (dst as u64) + (src as u64) + core.x_flag_as_1() as u64;

	let res_hi = (res >> 24) as u32;
	core.n_flag = res_hi;
	core.v_flag = (((src as u64 ^ res) & (dst as u64 ^ res)) >> 24) as u32;
	core.c_flag = res_hi;
	core.x_flag = res_hi;

	let res32 = res as u32;
	core.not_z_flag |= res32;
	res32
}

pub fn and_8(core: &mut Core, dst: u32, src: u32) -> u32 {
	let dst = mask_out_above_8!(dst);
	let src = mask_out_above_8!(src);
	let res = dst & src;

	core.not_z_flag = res;
	core.n_flag = res;
	core.c_flag = 0;
	core.v_flag = 0;

	res
}
pub fn and_16(core: &mut Core, dst: u32, src: u32) -> u32 {
	let dst = mask_out_above_16!(dst);
	let src = mask_out_above_16!(src);
	let res = dst & src;

	let res_hi = res >> 8;
	core.not_z_flag = res;
	core.n_flag = res_hi;
	core.c_flag = 0;
	core.v_flag = 0;

	res
}
pub fn and_32(core: &mut Core, dst: u32, src: u32) -> u32 {
	let res = dst & src;

	let res_hi = res >> 24;
	core.not_z_flag = res;
	core.n_flag = res_hi;
	core.c_flag = 0;
	core.v_flag = 0;

	res
}

#[cfg(test)]
mod tests {
	use super::super::super::Core;

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
