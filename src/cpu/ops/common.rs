#![macro_use]
use super::super::Core;
use cpu::{CFLAG_SET, ZFLAG_SET, XFLAG_SET, NFLAG_SET, ZFLAG_CLEAR, VFLAG_CLEAR, CFLAG_CLEAR, XFLAG_CLEAR, NFLAG_CLEAR};
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
macro_rules! true_is_1 {
	($e:expr) => (if $e {1} else {0})
}
macro_rules! false_is_1 {
	($e:expr) => (if $e {0} else {1})
}
macro_rules! not1 {
	($e:expr) => (true_is_1!($e == 0))
}
macro_rules! msb_8_set {
	($e:expr) => (($e & 0x80) > 0)
}
macro_rules! msb_16_set {
	($e:expr) => (($e & 0x8000) > 0)
}
macro_rules! msb_32_set {
	($e:expr) => (($e & 0x80000000) > 0)
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
	core.c_flag = true_is_1!(res > 0x99) << 8;
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

pub fn asr_8(core: &mut Core, dst: u32, shift: u32) -> u32 {
	let src = mask_out_above_8!(dst);
	let res = src.wrapping_shr(shift);

	if shift != 0 {
		if shift < 8 {
			let res = if msb_8_set!(src) {
				res | SHIFT_8_TABLE[shift as usize]
			} else {
				res
			};
			core.n_flag = res;
			core.not_z_flag = res;
			core.v_flag = VFLAG_CLEAR;
			core.c_flag = src.wrapping_shl(9-shift);
			core.x_flag = core.c_flag;
			res
		} else {
			if msb_8_set!(src) {
				core.c_flag = CFLAG_SET;
				core.x_flag = XFLAG_SET;
				core.n_flag = NFLAG_SET;
				core.not_z_flag = ZFLAG_CLEAR;
				core.v_flag = VFLAG_CLEAR;
				0xff
			} else {
				core.c_flag = CFLAG_CLEAR;
				core.x_flag = XFLAG_CLEAR;
				core.n_flag = NFLAG_CLEAR;
				core.not_z_flag = ZFLAG_SET;
				core.v_flag = VFLAG_CLEAR;
				0x00
			}
		}
	} else {
		core.c_flag = CFLAG_CLEAR;
		core.n_flag = src;
		core.not_z_flag = src;
		core.v_flag = VFLAG_CLEAR;
		res
	}
}

pub fn asr_16(core: &mut Core, dst: u32, shift: u32) -> u32 {
	let src = mask_out_above_16!(dst);
	let res = src.wrapping_shr(shift);
	if shift != 0 {
		if shift < 16 {
			let res = if msb_16_set!(src) {
				res | SHIFT_16_TABLE[shift as usize]
			} else {
				res
			};
			core.n_flag = res >> 8;
			core.not_z_flag = res;
			core.v_flag = VFLAG_CLEAR;
			core.c_flag = src.wrapping_shr(shift - 1) << 8;
			core.x_flag = core.c_flag;
			res
		} else {
			if msb_16_set!(src) {
				core.c_flag = CFLAG_SET;
				core.x_flag = XFLAG_SET;
				core.n_flag = NFLAG_SET;
				core.not_z_flag = ZFLAG_CLEAR;
				core.v_flag = VFLAG_CLEAR;
				0xffff
			} else {
				core.c_flag = CFLAG_CLEAR;
				core.x_flag = XFLAG_CLEAR;
				core.n_flag = NFLAG_CLEAR;
				core.not_z_flag = ZFLAG_SET;
				core.v_flag = VFLAG_CLEAR;
				0x0000
			}
		}
	} else {
		core.c_flag = CFLAG_CLEAR;
		core.n_flag = src >> 8;
		core.not_z_flag = src;
		core.v_flag = VFLAG_CLEAR;
		res
	}
}

pub fn asr_32(core: &mut Core, dst: u32, shift: u32) -> u32 {
	let src = dst;
	let res = src.wrapping_shr(shift);
	if shift != 0 {
		if shift < 32 {
			let res = if msb_32_set!(src) {
				res | SHIFT_32_TABLE[shift as usize]
			} else {
				res
			};
			core.n_flag = res >> 24;
			core.not_z_flag = res;
			core.v_flag = VFLAG_CLEAR;
			core.c_flag = src.wrapping_shr(shift - 1) << 8;
			core.x_flag = core.c_flag;
			res
		} else {
			if msb_32_set!(src) {
				core.c_flag = CFLAG_SET;
				core.x_flag = XFLAG_SET;
				core.n_flag = NFLAG_SET;
				core.not_z_flag = ZFLAG_CLEAR;
				core.v_flag = VFLAG_CLEAR;
				0xffffffff
			} else {
				core.c_flag = CFLAG_CLEAR;
				core.x_flag = XFLAG_CLEAR;
				core.n_flag = NFLAG_CLEAR;
				core.not_z_flag = ZFLAG_SET;
				core.v_flag = VFLAG_CLEAR;
				0x00000000
			}
		}
	} else {
		core.n_flag = src >> 24;
		core.not_z_flag = src;
		core.v_flag = VFLAG_CLEAR;
		core.c_flag = CFLAG_CLEAR;
		res
	}
}

pub fn asl_8(core: &mut Core, dst: u32, shift: u32) -> u32 {
	let src = mask_out_above_8!(dst);
	let res = mask_out_above_8!(src.wrapping_shl(shift));

	if shift != 0 {
		if shift < 8 {
			core.n_flag = res;
			core.not_z_flag = res;
			core.c_flag = src.wrapping_shl(shift);
			core.x_flag = core.c_flag;
			let src = src & SHIFT_8_TABLE[shift as usize + 1];
			core.v_flag = false_is_1!(src == 0 || src == SHIFT_8_TABLE[shift as usize + 1]) << 7;
			res
		} else {
			core.c_flag = (if shift == 8 {src & 1} else {0}) << 8;
			core.x_flag = core.c_flag;
			core.n_flag = NFLAG_CLEAR;
			core.not_z_flag = ZFLAG_SET;
			core.v_flag = false_is_1!(src == 0) << 7;
			0x00
		}
	} else {
		core.c_flag = CFLAG_CLEAR;
		core.n_flag = src;
		core.not_z_flag = src;
		core.v_flag = VFLAG_CLEAR;
		res
	}
}

pub fn asl_16(core: &mut Core, dst: u32, shift: u32) -> u32 {
	let src = mask_out_above_16!(dst);
	let res = mask_out_above_16!(src.wrapping_shl(shift));
	if shift != 0 {
		if shift < 16 {
			core.n_flag = res >> 8;
			core.not_z_flag = res;
			core.c_flag = src.wrapping_shl(shift) >> 8;
			core.x_flag = core.c_flag;
			let src = src & SHIFT_16_TABLE[shift as usize + 1];
			core.v_flag = false_is_1!(src == 0 || src == SHIFT_16_TABLE[shift as usize + 1]) << 7;
			res
		} else {
			core.c_flag = (if shift == 16 {src & 1} else {0}) << 8;
			core.x_flag = core.c_flag;
			core.n_flag = NFLAG_CLEAR;
			core.not_z_flag = ZFLAG_SET;
			core.v_flag = false_is_1!(src == 0) << 7;
			0x0000
		}
	} else {
		core.c_flag = CFLAG_CLEAR;
		core.n_flag = src >> 8;
		core.not_z_flag = src;
		core.v_flag = VFLAG_CLEAR;
		res
	}
}

pub fn asl_32(core: &mut Core, dst: u32, shift: u32) -> u32 {
	let src = dst;
	let res = src.wrapping_shl(shift);
	if shift != 0 {
		if shift < 32 {
			core.n_flag = res >> 24;
			core.not_z_flag = res;
			core.c_flag = src.wrapping_shr(32 - shift) << 8;
			core.x_flag = core.c_flag;
			let src = src & SHIFT_32_TABLE[shift as usize + 1];
			core.v_flag = false_is_1!(src == 0 || src == SHIFT_32_TABLE[shift as usize + 1]) << 7;
			res
		} else {
			core.c_flag = (if shift == 32 {src & 1} else {0}) << 8;
			core.x_flag = core.c_flag;
			core.n_flag = NFLAG_CLEAR;
			core.not_z_flag = ZFLAG_SET;
			core.v_flag = false_is_1!(src == 0) << 7;
			0x00000000
		}
	} else {
		core.n_flag = src >> 24;
		core.not_z_flag = src;
		core.v_flag = VFLAG_CLEAR;
		core.c_flag = CFLAG_CLEAR;
		res
	}
}

static SHIFT_8_TABLE:  [u32; 65] = [
 0x00, 0x80, 0xc0, 0xe0, 0xf0, 0xf8, 0xfc, 0xfe, 0xff, 0xff, 0xff, 0xff,
 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
 0xff, 0xff, 0xff, 0xff, 0xff
];

static SHIFT_16_TABLE: [u32; 65] = [
 0x0000, 0x8000, 0xc000, 0xe000, 0xf000, 0xf800, 0xfc00, 0xfe00, 0xff00,
 0xff80, 0xffc0, 0xffe0, 0xfff0, 0xfff8, 0xfffc, 0xfffe, 0xffff, 0xffff,
 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
 0xffff, 0xffff
];

static SHIFT_32_TABLE: [u32; 65] = [
 0x00000000, 0x80000000, 0xc0000000, 0xe0000000, 0xf0000000, 0xf8000000,
 0xfc000000, 0xfe000000, 0xff000000, 0xff800000, 0xffc00000, 0xffe00000,
 0xfff00000, 0xfff80000, 0xfffc0000, 0xfffe0000, 0xffff0000, 0xffff8000,
 0xffffc000, 0xffffe000, 0xfffff000, 0xfffff800, 0xfffffc00, 0xfffffe00,
 0xffffff00, 0xffffff80, 0xffffffc0, 0xffffffe0, 0xfffffff0, 0xfffffff8,
 0xfffffffc, 0xfffffffe, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff
];

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
