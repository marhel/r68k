#![macro_use]
use super::super::Core;
use cpu::{CFLAG_SET, ZFLAG_SET, XFLAG_SET, NFLAG_SET, ZFLAG_CLEAR, VFLAG_CLEAR, CFLAG_CLEAR, XFLAG_CLEAR, NFLAG_CLEAR};
use std::num::Wrapping;

macro_rules! ir_dx {
    ($e:ident) => (($e.ir() >> 9 & 7) as usize);
}
macro_rules! ir_dy {
    ($e:ident) => (($e.ir() & 7) as usize);
}
macro_rules! ir_ax {
    ($e:ident) => (8+($e.ir() >> 9 & 7) as usize);
}
macro_rules! ir_ay {
    ($e:ident) => (8+($e.ir() & 7) as usize);
}
macro_rules! dx {
    ($e:ident) => (*$e.dx());
}
macro_rules! dy {
    ($e:ident) => (*$e.dy());
}
macro_rules! ax {
    ($e:ident) => (*$e.ax());
}
macro_rules! ay {
    ($e:ident) => (*$e.ay());
}
macro_rules! pc {
    ($e:ident) => (*$e.pc());
}
macro_rules! ir {
    ($e:ident) => ($e.ir());
}
macro_rules! dar {
    ($e:ident) => ($e.dar());
}
macro_rules! inactive_usp {
    ($e:ident) => (*$e.inactive_usp());
}
macro_rules! c_flag {
    ($e:ident) => (*$e.c_flag());
}
macro_rules! v_flag {
    ($e:ident) => (*$e.v_flag());
}
macro_rules! n_flag {
    ($e:ident) => (*$e.n_flag());
}
macro_rules! s_flag {
    ($e:ident) => (*$e.s_flag());
}
macro_rules! x_flag {
    ($e:ident) => (*$e.x_flag());
}
macro_rules! not_z_flag {
    ($e:ident) => (*$e.not_z_flag());
}
macro_rules! sp {
    ($e:ident) => ($e.dar()[15]);
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
    ($e:expr) => (($e & 0x8000_0000) > 0)
}
// All instructions are ported from https://github.com/kstenerud/Musashi
pub fn abcd_8<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    // unsigned int res = ((src) & 0x0f) + ((dst) & 0x0f) + ((m68ki_cpu.x_flag>>8)&1);
    let mut res = low_nibble!(src) + low_nibble!(dst) + core.x_flag_as_1();

    // m68ki_cpu.v_flag = ~res;
    v_flag!(core) = !res;

    // if(res > 9)
    //  res += 6;
    if res > 9 {
        res += 6;
    }
    // res += ((src) & 0xf0) + ((dst) & 0xf0);
    res += high_nibble!(src) + high_nibble!(dst);
    // m68ki_cpu.x_flag = m68ki_cpu.c_flag = (res > 0x99) << 8;
    c_flag!(core) = true_is_1!(res > 0x99) << 8;
    x_flag!(core) = c_flag!(core);

    if c_flag!(core) > 0 {
        res = (Wrapping(res) - Wrapping(0xa0)).0;
    }

    // m68ki_cpu.v_flag &= res;
    // m68ki_cpu.n_flag = (res);
    v_flag!(core) &= res;
    n_flag!(core) = res;

    // res = ((res) & 0xff);
    // m68ki_cpu.not_z_flag |= res;
    res = mask_out_above_8!(res);
    not_z_flag!(core) |= res;
    res
}

pub fn add_8<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let dst = mask_out_above_8!(dst);
    let src = mask_out_above_8!(src);

    let res = dst + src;
    // m68ki_cpu.n_flag = (res);
    n_flag!(core) = res;
    // m68ki_cpu.v_flag = ((src^res) & (dst^res));
    v_flag!(core) = (src ^ res) & (dst ^ res);
    // m68ki_cpu.x_flag = m68ki_cpu.c_flag = (res);
    c_flag!(core) = res;
    x_flag!(core) = res;
    // m68ki_cpu.not_z_flag = ((res) & 0xff);
    let res8 = mask_out_above_8!(res);
    not_z_flag!(core) = res8;
    res8
}
pub fn add_16<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let dst = mask_out_above_16!(dst);
    let src = mask_out_above_16!(src);
    let res = dst + src;

    // m68ki_cpu.n_flag = ((res)>>8);
    let res_hi = res >> 8;
    n_flag!(core) = res_hi;
    // m68ki_cpu.v_flag = (((src^res) & (dst^res))>>8);
    v_flag!(core) = ((src ^ res) & (dst ^ res)) >> 8;
    // m68ki_cpu.x_flag = m68ki_cpu.c_flag = ((res)>>8);
    c_flag!(core) = res_hi;
    x_flag!(core) = res_hi;
    // m68ki_cpu.not_z_flag = ((res) & 0xffff);
    let res16 = mask_out_above_16!(res);
    not_z_flag!(core) = res16;

    res16
}
pub fn add_32<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let res: u64 = u64::from(dst) + u64::from(src);

    let res_hi = (res >> 24) as u32;
    n_flag!(core) = res_hi;
    // m68ki_cpu.v_flag = (((src^res) & (dst^res))>>24);
    v_flag!(core) = (((u64::from(src) ^ res) & (u64::from(dst) ^ res)) >> 24) as u32;
     // m68ki_cpu.x_flag = m68ki_cpu.c_flag = (((src & dst) | (~res & (src | dst)))>>23);
    c_flag!(core) = res_hi;
    x_flag!(core) = res_hi;

    let res32 = res as u32;

    not_z_flag!(core) = res32;

    res32
}

pub fn addx_8<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let dst = mask_out_above_8!(dst);
    let src = mask_out_above_8!(src);

    let res = dst + src + core.x_flag_as_1();

    n_flag!(core) = res;
    v_flag!(core) = (src ^ res) & (dst ^ res);
    c_flag!(core) = res;
    x_flag!(core) = res;

    let res8 = mask_out_above_8!(res);
    not_z_flag!(core) |= res8;
    res8
}
pub fn addx_16<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let dst = mask_out_above_16!(dst);
    let src = mask_out_above_16!(src);
    let res = dst + src + core.x_flag_as_1();

    let res_hi = res >> 8;
    n_flag!(core) = res_hi;
    v_flag!(core) = ((src ^ res) & (dst ^ res)) >> 8;
    c_flag!(core) = res_hi;
    x_flag!(core) = res_hi;

    let res16 = mask_out_above_16!(res);
    not_z_flag!(core) |= res16;
    res16
}
pub fn addx_32<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let res: u64 = u64::from(dst) + u64::from(src) + u64::from(core.x_flag_as_1());

    let res_hi = (res >> 24) as u32;
    n_flag!(core) = res_hi;
    v_flag!(core) = (((u64::from(src) ^ res) & (u64::from(dst) ^ res)) >> 24) as u32;
    c_flag!(core) = res_hi;
    x_flag!(core) = res_hi;

    let res32 = res as u32;
    not_z_flag!(core) |= res32;
    res32
}

pub fn and_8<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let dst = mask_out_above_8!(dst);
    let src = mask_out_above_8!(src);
    let res = dst & src;

    not_z_flag!(core) = res;
    n_flag!(core) = res;
    c_flag!(core) = 0;
    v_flag!(core) = 0;

    res
}
pub fn and_16<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let dst = mask_out_above_16!(dst);
    let src = mask_out_above_16!(src);
    let res = dst & src;

    let res_hi = res >> 8;
    not_z_flag!(core) = res;
    n_flag!(core) = res_hi;
    c_flag!(core) = 0;
    v_flag!(core) = 0;

    res
}
pub fn and_32<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let res = dst & src;

    let res_hi = res >> 24;
    not_z_flag!(core) = res;
    n_flag!(core) = res_hi;
    c_flag!(core) = 0;
    v_flag!(core) = 0;

    res
}

pub fn asr_8<T: Core>(core: &mut T, dst: u32, shift: u32) -> u32 {
    let src = mask_out_above_8!(dst);
    let res = src.wrapping_shr(shift);

    if shift != 0 {
        if shift < 8 {
            let res = if msb_8_set!(src) {
                res | SHIFT_8_TABLE[shift as usize]
            } else {
                res
            };
            n_flag!(core) = res;
            not_z_flag!(core) = res;
            v_flag!(core) = VFLAG_CLEAR;
            c_flag!(core) = src.wrapping_shl(9-shift);
            x_flag!(core) = c_flag!(core);
            res
        } else if msb_8_set!(src) {
            c_flag!(core) = CFLAG_SET;
            x_flag!(core) = XFLAG_SET;
            n_flag!(core) = NFLAG_SET;
            not_z_flag!(core) = ZFLAG_CLEAR;
            v_flag!(core) = VFLAG_CLEAR;
            0xff
        } else {
            c_flag!(core) = CFLAG_CLEAR;
            x_flag!(core) = XFLAG_CLEAR;
            n_flag!(core) = NFLAG_CLEAR;
            not_z_flag!(core) = ZFLAG_SET;
            v_flag!(core) = VFLAG_CLEAR;
            0x00
        }
    } else {
        c_flag!(core) = CFLAG_CLEAR;
        n_flag!(core) = src;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        res
    }
}

pub fn asr_16<T: Core>(core: &mut T, dst: u32, shift: u32) -> u32 {
    let src = mask_out_above_16!(dst);
    let res = src.wrapping_shr(shift);
    if shift != 0 {
        if shift < 16 {
            let res = if msb_16_set!(src) {
                res | SHIFT_16_TABLE[shift as usize]
            } else {
                res
            };
            n_flag!(core) = res >> 8;
            not_z_flag!(core) = res;
            v_flag!(core) = VFLAG_CLEAR;
            c_flag!(core) = src.wrapping_shr(shift - 1) << 8;
            x_flag!(core) = c_flag!(core);
            res
        } else if msb_16_set!(src) {
            c_flag!(core) = CFLAG_SET;
            x_flag!(core) = XFLAG_SET;
            n_flag!(core) = NFLAG_SET;
            not_z_flag!(core) = ZFLAG_CLEAR;
            v_flag!(core) = VFLAG_CLEAR;
            0xffff
        } else {
            c_flag!(core) = CFLAG_CLEAR;
            x_flag!(core) = XFLAG_CLEAR;
            n_flag!(core) = NFLAG_CLEAR;
            not_z_flag!(core) = ZFLAG_SET;
            v_flag!(core) = VFLAG_CLEAR;
            0x0000
        }
    } else {
        c_flag!(core) = CFLAG_CLEAR;
        n_flag!(core) = src >> 8;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        res
    }
}

pub fn asr_32<T: Core>(core: &mut T, dst: u32, shift: u32) -> u32 {
    let src = dst;
    let res = src.wrapping_shr(shift);
    if shift != 0 {
        if shift < 32 {
            let res = if msb_32_set!(src) {
                res | SHIFT_32_TABLE[shift as usize]
            } else {
                res
            };
            n_flag!(core) = res >> 24;
            not_z_flag!(core) = res;
            v_flag!(core) = VFLAG_CLEAR;
            c_flag!(core) = src.wrapping_shr(shift - 1) << 8;
            x_flag!(core) = c_flag!(core);
            res
        } else if msb_32_set!(src) {
            c_flag!(core) = CFLAG_SET;
            x_flag!(core) = XFLAG_SET;
            n_flag!(core) = NFLAG_SET;
            not_z_flag!(core) = ZFLAG_CLEAR;
            v_flag!(core) = VFLAG_CLEAR;
            0xffff_ffff
        } else {
            c_flag!(core) = CFLAG_CLEAR;
            x_flag!(core) = XFLAG_CLEAR;
            n_flag!(core) = NFLAG_CLEAR;
            not_z_flag!(core) = ZFLAG_SET;
            v_flag!(core) = VFLAG_CLEAR;
            0x0000_0000
        }
    } else {
        n_flag!(core) = src >> 24;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        c_flag!(core) = CFLAG_CLEAR;
        res
    }
}

pub fn asl_8<T: Core>(core: &mut T, dst: u32, shift: u32) -> u32 {
    let src = mask_out_above_8!(dst);
    let res = mask_out_above_8!(src.wrapping_shl(shift));

    if shift != 0 {
        if shift < 8 {
            n_flag!(core) = res;
            not_z_flag!(core) = res;
            c_flag!(core) = src.wrapping_shl(shift);
            x_flag!(core) = c_flag!(core);
            let src = src & SHIFT_8_TABLE[shift as usize + 1];
            v_flag!(core) = false_is_1!(src == 0 || src == SHIFT_8_TABLE[shift as usize + 1]) << 7;
            res
        } else {
            c_flag!(core) = (if shift == 8 {src & 1} else {0}) << 8;
            x_flag!(core) = c_flag!(core);
            n_flag!(core) = NFLAG_CLEAR;
            not_z_flag!(core) = ZFLAG_SET;
            v_flag!(core) = false_is_1!(src == 0) << 7;
            0x00
        }
    } else {
        c_flag!(core) = CFLAG_CLEAR;
        n_flag!(core) = src;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        res
    }
}

pub fn asl_16<T: Core>(core: &mut T, dst: u32, shift: u32) -> u32 {
    let src = mask_out_above_16!(dst);
    let res = mask_out_above_16!(src.wrapping_shl(shift));
    if shift != 0 {
        if shift < 16 {
            n_flag!(core) = res >> 8;
            not_z_flag!(core) = res;
            c_flag!(core) = src.wrapping_shl(shift) >> 8;
            x_flag!(core) = c_flag!(core);
            let src = src & SHIFT_16_TABLE[shift as usize + 1];
            v_flag!(core) = false_is_1!(src == 0 || src == SHIFT_16_TABLE[shift as usize + 1]) << 7;
            res
        } else {
            c_flag!(core) = (if shift == 16 {src & 1} else {0}) << 8;
            x_flag!(core) = c_flag!(core);
            n_flag!(core) = NFLAG_CLEAR;
            not_z_flag!(core) = ZFLAG_SET;
            v_flag!(core) = false_is_1!(src == 0) << 7;
            0x0000
        }
    } else {
        c_flag!(core) = CFLAG_CLEAR;
        n_flag!(core) = src >> 8;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        res
    }
}

pub fn asl_32<T: Core>(core: &mut T, dst: u32, shift: u32) -> u32 {
    let src = dst;
    let res = src.wrapping_shl(shift);
    if shift != 0 {
        if shift < 32 {
            n_flag!(core) = res >> 24;
            not_z_flag!(core) = res;
            c_flag!(core) = src.wrapping_shr(32 - shift) << 8;
            x_flag!(core) = c_flag!(core);
            let src = src & SHIFT_32_TABLE[shift as usize + 1];
            v_flag!(core) = false_is_1!(src == 0 || src == SHIFT_32_TABLE[shift as usize + 1]) << 7;
            res
        } else {
            c_flag!(core) = (if shift == 32 {src & 1} else {0}) << 8;
            x_flag!(core) = c_flag!(core);
            n_flag!(core) = NFLAG_CLEAR;
            not_z_flag!(core) = ZFLAG_SET;
            v_flag!(core) = false_is_1!(src == 0) << 7;
            0x0000_0000
        }
    } else {
        n_flag!(core) = src >> 24;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        c_flag!(core) = CFLAG_CLEAR;
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
 0x0000_0000, 0x8000_0000, 0xc000_0000, 0xe000_0000, 0xf000_0000, 0xf800_0000,
 0xfc00_0000, 0xfe00_0000, 0xff00_0000, 0xff80_0000, 0xffc0_0000, 0xffe0_0000,
 0xfff0_0000, 0xfff8_0000, 0xfffc_0000, 0xfffe_0000, 0xffff_0000, 0xffff_8000,
 0xffff_c000, 0xffff_e000, 0xffff_f000, 0xffff_f800, 0xffff_fc00, 0xffff_fe00,
 0xffff_ff00, 0xffff_ff80, 0xffff_ffc0, 0xffff_ffe0, 0xffff_fff0, 0xffff_fff8,
 0xffff_fffc, 0xffff_fffe, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff,
 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff,
 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff,
 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff,
 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff,
 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff
];

pub fn cmp_8<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let dst = mask_out_above_8!(dst);
    let src = mask_out_above_8!(src);

    let res = (Wrapping(dst) - Wrapping(src)).0;

    n_flag!(core) = res;
    v_flag!(core) = (src ^ dst) & (res ^ dst);
    c_flag!(core) = res;

    let res8 = mask_out_above_8!(res);
    not_z_flag!(core) = res8;
    res8
}
pub fn cmp_16<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let dst = mask_out_above_16!(dst);
    let src = mask_out_above_16!(src);
    let res = (Wrapping(dst) - Wrapping(src)).0;

    let res_hi = res >> 8;
    n_flag!(core) = res_hi;
    v_flag!(core) = ((src ^ dst) & (res ^ dst)) >> 8;
    c_flag!(core) = res_hi;

    let res16 = mask_out_above_16!(res);
    not_z_flag!(core) = res16;
    res16
}
pub fn cmp_32<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let res = (Wrapping(u64::from(dst)) - Wrapping(u64::from(src))).0;

    let res_hi = (res >> 24) as u32;
    n_flag!(core) = res_hi;
    v_flag!(core) = (((u64::from(src) ^ u64::from(dst)) & (res ^ u64::from(dst))) >> 24) as u32;
    c_flag!(core) = res_hi;

    let res32 = res as u32;
    not_z_flag!(core) = res32;
    res32
}

// Put common implementation of DBcc here
// Put common implementation of DIVS here
pub fn divs_16<T: Core>(core: &mut T, dst: u32, src: i16) {
    if dst == 0x8000_0000 && src == -1 {
        n_flag!(core) = 0;
        v_flag!(core) = 0;
        c_flag!(core) = 0;
        not_z_flag!(core) = 0;
        dx!(core) = 0;
        return;
    }
    let quotient: i32 = (dst as i32) / i32::from(src);
    let remainder: i32 = (dst as i32) % i32::from(src);
    if quotient == i32::from(quotient as i16) {
        not_z_flag!(core) = quotient as u32;
        n_flag!(core) = quotient as u32 >> 8;
        v_flag!(core) = 0;
        c_flag!(core) = 0;
        dx!(core) = ((remainder as u32) << 16) | mask_out_above_16!(quotient as u32);
    } else {
        v_flag!(core) = 0x80;
    }
}

// Put common implementation of DIVU here
pub fn divu_16<T: Core>(core: &mut T, dst: u32, src: u16) {
    let quotient: u32 = dst / u32::from(src);
    let remainder: u32 = dst % u32::from(src);
    if quotient < 0x10000 {
        not_z_flag!(core) = quotient;
        n_flag!(core) = quotient >> 8;
        v_flag!(core) = 0;
        c_flag!(core) = 0;
        dx!(core) = (remainder << 16) | mask_out_above_16!(quotient);
    } else {
        v_flag!(core) = 0x80;
    }
}

// Put common implementation of EOR here
pub fn eor_8<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let dst = mask_out_above_8!(dst);
    let src = mask_out_above_8!(src);
    let res = dst ^ src;

    not_z_flag!(core) = res;
    n_flag!(core) = res;
    c_flag!(core) = 0;
    v_flag!(core) = 0;

    res
}
pub fn eor_16<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let dst = mask_out_above_16!(dst);
    let src = mask_out_above_16!(src);
    let res = dst ^ src;

    let res_hi = res >> 8;
    not_z_flag!(core) = res;
    n_flag!(core) = res_hi;
    c_flag!(core) = 0;
    v_flag!(core) = 0;

    res
}
pub fn eor_32<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let res = dst ^ src;

    let res_hi = res >> 24;
    not_z_flag!(core) = res;
    n_flag!(core) = res_hi;
    c_flag!(core) = 0;
    v_flag!(core) = 0;

    res
}

// No common implementation of EXG needed
// No common implementation of EXT needed
// No common implementation of ILLEGAL needed
// No common implementation of JMP needed
// No common implementation of JSR needed
// No common implementation of LEA needed
// No common implementation of LINK needed

// Put common implementation of LSL, LSR here
pub fn lsr_8<T: Core>(core: &mut T, dst: u32, shift: u32) -> u32 {
    let src = mask_out_above_8!(dst);
    let res = src.wrapping_shr(shift);

    if shift != 0 {
        if shift <= 8 {
            n_flag!(core) = NFLAG_CLEAR;
            not_z_flag!(core) = res;
            v_flag!(core) = VFLAG_CLEAR;
            c_flag!(core) = src.wrapping_shl(9-shift);
            x_flag!(core) = c_flag!(core);
            res
        } else {
            c_flag!(core) = CFLAG_CLEAR;
            x_flag!(core) = XFLAG_CLEAR;
            n_flag!(core) = NFLAG_CLEAR;
            not_z_flag!(core) = ZFLAG_SET;
            v_flag!(core) = VFLAG_CLEAR;
            0x00
        }
    } else {
        c_flag!(core) = CFLAG_CLEAR;
        n_flag!(core) = src;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        res
    }
}

pub fn lsr_16<T: Core>(core: &mut T, dst: u32, shift: u32) -> u32 {
    let src = mask_out_above_16!(dst);
    let res = src.wrapping_shr(shift);
    if shift != 0 {
        if shift <= 16 {
            n_flag!(core) = NFLAG_CLEAR;
            not_z_flag!(core) = res;
            v_flag!(core) = VFLAG_CLEAR;
            c_flag!(core) = src.wrapping_shr(shift - 1) << 8;
            x_flag!(core) = c_flag!(core);
            res
        } else {
            c_flag!(core) = CFLAG_CLEAR;
            x_flag!(core) = XFLAG_CLEAR;
            n_flag!(core) = NFLAG_CLEAR;
            not_z_flag!(core) = ZFLAG_SET;
            v_flag!(core) = VFLAG_CLEAR;
            0x0000
        }
    } else {
        c_flag!(core) = CFLAG_CLEAR;
        n_flag!(core) = src >> 8;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        res
    }
}

pub fn lsr_32<T: Core>(core: &mut T, dst: u32, shift: u32) -> u32 {
    let src = dst;
    let res = src.wrapping_shr(shift);
    if shift != 0 {
        if shift < 32 {
            n_flag!(core) = NFLAG_CLEAR;
            not_z_flag!(core) = res;
            v_flag!(core) = VFLAG_CLEAR;
            c_flag!(core) = src.wrapping_shr(shift - 1) << 8;
            x_flag!(core) = c_flag!(core);
            res
        } else {
            c_flag!(core) = if shift == 32 {((src) & 0x8000_0000)>>23} else {0};
            x_flag!(core) = c_flag!(core);
            n_flag!(core) = NFLAG_CLEAR;
            not_z_flag!(core) = ZFLAG_SET;
            v_flag!(core) = VFLAG_CLEAR;
            0x0000_0000
        }
    } else {
        c_flag!(core) = CFLAG_CLEAR;
        n_flag!(core) = src >> 24;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        res
    }
}

pub fn lsl_8<T: Core>(core: &mut T, dst: u32, shift: u32) -> u32 {
    let src = mask_out_above_8!(dst);
    let res = mask_out_above_8!(src.wrapping_shl(shift));

    if shift != 0 {
        if shift <= 8 {
            n_flag!(core) = res;
            not_z_flag!(core) = res;
            c_flag!(core) = src.wrapping_shl(shift);
            x_flag!(core) = c_flag!(core);
            v_flag!(core) = VFLAG_CLEAR;
            res
        } else {
            c_flag!(core) = CFLAG_CLEAR;
            x_flag!(core) = XFLAG_CLEAR;
            n_flag!(core) = NFLAG_CLEAR;
            not_z_flag!(core) = ZFLAG_SET;
            v_flag!(core) = VFLAG_CLEAR;
            0x00
        }
    } else {
        c_flag!(core) = CFLAG_CLEAR;
        n_flag!(core) = src;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        res
    }
}

pub fn lsl_16<T: Core>(core: &mut T, dst: u32, shift: u32) -> u32 {
    let src = mask_out_above_16!(dst);
    let res = mask_out_above_16!(src.wrapping_shl(shift));
    if shift != 0 {
        if shift <= 16 {
            n_flag!(core) = res >> 8;
            not_z_flag!(core) = res;
            c_flag!(core) = src.wrapping_shl(shift) >> 8;
            x_flag!(core) = c_flag!(core);
            v_flag!(core) = VFLAG_CLEAR;
            res
        } else {
            c_flag!(core) = CFLAG_CLEAR;
            x_flag!(core) = XFLAG_CLEAR;
            n_flag!(core) = NFLAG_CLEAR;
            not_z_flag!(core) = ZFLAG_SET;
            v_flag!(core) = VFLAG_CLEAR;
            0x0000
        }
    } else {
        c_flag!(core) = CFLAG_CLEAR;
        n_flag!(core) = src >> 8;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        res
    }
}

pub fn lsl_32<T: Core>(core: &mut T, dst: u32, shift: u32) -> u32 {
    let src = dst;
    let res = src.wrapping_shl(shift);
    if shift != 0 {
        if shift < 32 {
            n_flag!(core) = res >> 24;
            not_z_flag!(core) = res;
            c_flag!(core) = src.wrapping_shr(32 - shift) << 8;
            x_flag!(core) = c_flag!(core);
            v_flag!(core) = VFLAG_CLEAR;
            res
        } else {
            c_flag!(core) = (if shift == 32 {src & 1} else {0}) << 8;
            x_flag!(core) = c_flag!(core);
            n_flag!(core) = NFLAG_CLEAR;
            not_z_flag!(core) = ZFLAG_SET;
            v_flag!(core) = VFLAG_CLEAR;
            0x0000_0000
        }
    } else {
        n_flag!(core) = src >> 24;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        c_flag!(core) = CFLAG_CLEAR;
        res
    }
}

// Put common implementation of MOVE here
pub fn move_flags<T: Core>(core: &mut T, src: u32, shift: u32) -> u32 {
    n_flag!(core) = src >> shift;
    not_z_flag!(core) = src;
    v_flag!(core) = 0;
    c_flag!(core) = 0;
    src
}

// Put common implementation of MOVEA here
// Put common implementation of MOVE to CCR here
// Put common implementation of MOVE from SR here
// Put common implementation of MOVE to SR here
// Put common implementation of MOVE USP here
// Put common implementation of MOVEM here
// Put common implementation of MOVEP here
// Put common implementation of MOVEQ here
// Put common implementation of MULS here
pub fn muls_16<T: Core>(core: &mut T, dst: i16, src: i16) -> u32 {
    let res = i32::from(dst).wrapping_mul(i32::from(src)) as u32;
    not_z_flag!(core) = res;
    n_flag!(core) = res >> 24;
    v_flag!(core) = 0;
    c_flag!(core) = 0;
    res
}
// Put common implementation of MULU here
pub fn mulu_16<T: Core>(core: &mut T, dst: u16, src: u16) -> u32 {
    let res = u32::from(dst).wrapping_mul(u32::from(src)) as u32;
    not_z_flag!(core) = res;
    n_flag!(core) = res >> 24;
    v_flag!(core) = 0;
    c_flag!(core) = 0;
    res
}
// Put common implementation of NBCD here
pub fn nbcd<T: Core>(core: &mut T, dst: u32) -> Option<u32> {
    let mut res = mask_out_above_8!((0x9a as u32).wrapping_sub(dst).wrapping_sub(core.x_flag_as_1()));
    let answer = if res != 0x9a {
        v_flag!(core) = !res;
        if (res & 0x0f) == 0xa {
            res = (res & 0xf0) + 0x10;
        }

        res &= 0xff;
        v_flag!(core) &= res;

        not_z_flag!(core) |= res;
        c_flag!(core) = CFLAG_SET;
        x_flag!(core) = XFLAG_SET;
        Some(res)
    }
    else
    {
        v_flag!(core) = 0;
        c_flag!(core) = 0;
        x_flag!(core) = 0;
        None
    };
    n_flag!(core) = res;
    answer
}
// Put common implementation of NEG here
// Put common implementation of NEGX here
// Put common implementation of NOP here
// Put common implementation of NOT here
pub fn not_8<T: Core>(core: &mut T, dst: u32) -> u32 {
    let res = mask_out_above_8!(!dst);

    not_z_flag!(core) = res;
    n_flag!(core) = res;
    c_flag!(core) = 0;
    v_flag!(core) = 0;

    res
}
pub fn not_16<T: Core>(core: &mut T, dst: u32) -> u32 {
    let res = mask_out_above_16!(!dst);

    let res_hi = res >> 8;
    not_z_flag!(core) = res;
    n_flag!(core) = res_hi;
    c_flag!(core) = 0;
    v_flag!(core) = 0;

    res
}
pub fn not_32<T: Core>(core: &mut T, dst: u32) -> u32 {
    let res = !dst;

    let res_hi = res >> 24;
    not_z_flag!(core) = res;
    n_flag!(core) = res_hi;
    c_flag!(core) = 0;
    v_flag!(core) = 0;

    res
}

// Put common implementation of OR here
pub fn or_8<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let dst = mask_out_above_8!(dst);
    let src = mask_out_above_8!(src);
    let res = dst | src;

    not_z_flag!(core) = res;
    n_flag!(core) = res;
    c_flag!(core) = 0;
    v_flag!(core) = 0;

    res
}
pub fn or_16<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let dst = mask_out_above_16!(dst);
    let src = mask_out_above_16!(src);
    let res = dst | src;

    let res_hi = res >> 8;
    not_z_flag!(core) = res;
    n_flag!(core) = res_hi;
    c_flag!(core) = 0;
    v_flag!(core) = 0;

    res
}
pub fn or_32<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let res = dst | src;

    let res_hi = res >> 24;
    not_z_flag!(core) = res;
    n_flag!(core) = res_hi;
    c_flag!(core) = 0;
    v_flag!(core) = 0;

    res
}

// Put common implementation of ORI here
// Put common implementation of ORI to CCR here
// Put common implementation of ORI to SR here
// Put common implementation of PEA here
// Put common implementation of RESET here
// Put common implementation of ROL, ROR here
pub fn ror_8<T: Core>(core: &mut T, dst: u32, orig_shift: u32) -> u32 {
    let src = mask_out_above_8!(dst);

    if orig_shift != 0 {
        let shift = orig_shift & 7;
        let res = u32::from((src as u8).rotate_right(shift));
        n_flag!(core) = res;
        not_z_flag!(core) = res;
        v_flag!(core) = VFLAG_CLEAR;
        c_flag!(core) = src.wrapping_shl(8-(shift.wrapping_sub(1) & 7));
        res
    } else {
        c_flag!(core) = CFLAG_CLEAR;
        n_flag!(core) = src;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        src
    }
}

pub fn ror_16<T: Core>(core: &mut T, dst: u32, orig_shift: u32) -> u32 {
    let src = mask_out_above_16!(dst);

    if orig_shift != 0 {
        let shift = orig_shift & 15;
        let res = u32::from((src as u16).rotate_right(shift));
        n_flag!(core) = res >> 8;
        not_z_flag!(core) = res;
        v_flag!(core) = VFLAG_CLEAR;
        c_flag!(core) = src.wrapping_shr(shift.wrapping_sub(1) & 15) << 8;
        res
    } else {
        c_flag!(core) = CFLAG_CLEAR;
        n_flag!(core) = src >> 8;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        src
    }
}

pub fn ror_32<T: Core>(core: &mut T, dst: u32, orig_shift: u32) -> u32 {
    let src = dst;
    if orig_shift != 0 {
        let shift = orig_shift & 31;
        let res = src.rotate_right(shift);
        n_flag!(core) = res >> 24;
        not_z_flag!(core) = res;
        v_flag!(core) = VFLAG_CLEAR;
        c_flag!(core) = src.wrapping_shr(shift.wrapping_sub(1) & 31) << 8;
        res
    } else {
        n_flag!(core) = src >> 24;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        c_flag!(core) = CFLAG_CLEAR;
        src
    }
}

pub fn rol_8<T: Core>(core: &mut T, dst: u32, orig_shift: u32) -> u32 {
    let src = mask_out_above_8!(dst);

    if orig_shift != 0 {
        let shift = orig_shift & 7;
        if shift != 0 {
            let res = u32::from((src as u8).rotate_left(shift));
            n_flag!(core) = res;
            not_z_flag!(core) = res;
            c_flag!(core) = src.wrapping_shl(shift);
            v_flag!(core) = VFLAG_CLEAR;
            res
        } else {
            c_flag!(core) = (src & 1) << 8;
            n_flag!(core) = src;
            not_z_flag!(core) = src;
            v_flag!(core) = VFLAG_CLEAR;
            src
        }
    } else {
        c_flag!(core) = CFLAG_CLEAR;
        n_flag!(core) = src;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        src
    }
}

pub fn rol_16<T: Core>(core: &mut T, dst: u32, orig_shift: u32) -> u32 {
    let src = mask_out_above_16!(dst);
    if orig_shift != 0 {
        let shift = orig_shift & 15;
        if shift != 0 {
            let res = u32::from((src as u16).rotate_left(shift));
            n_flag!(core) = res >> 8;
            not_z_flag!(core) = res;
            c_flag!(core) = src.wrapping_shl(shift) >> 8;
            v_flag!(core) = VFLAG_CLEAR;
            res
        } else {
            c_flag!(core) = (src & 1) << 8;
            n_flag!(core) = src >> 8;
            not_z_flag!(core) = src;
            v_flag!(core) = VFLAG_CLEAR;
            src
        }
    } else {
        c_flag!(core) = CFLAG_CLEAR;
        n_flag!(core) = src >> 8;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        src
    }
}

pub fn rol_32<T: Core>(core: &mut T, dst: u32, orig_shift: u32) -> u32 {
    let src = dst;
    if orig_shift != 0 {
        let shift = orig_shift & 31;
        let res = src.rotate_left(shift);
        n_flag!(core) = res >> 24;
        not_z_flag!(core) = res;
        c_flag!(core) = src.wrapping_shr(32 - shift) << 8;
        v_flag!(core) = VFLAG_CLEAR;
        res
    } else {
        n_flag!(core) = src >> 24;
        not_z_flag!(core) = src;
        v_flag!(core) = VFLAG_CLEAR;
        c_flag!(core) = CFLAG_CLEAR;
        src
    }
}

// Put common implementation of ROXL, ROXR here
pub fn roxr_8<T: Core>(core: &mut T, dst: u32, orig_shift: u32) -> u32 {
    if orig_shift != 0 {
        let shift = orig_shift % 9;
        let src = mask_out_above_8!(dst);
        let x8 = core.x_flag_as_1() << 8;
        let srcx8 = src | x8;
        let res = (srcx8 >> shift) | (srcx8 << (9-shift));
        x_flag!(core) = res;
        c_flag!(core) = x_flag!(core);
        let res = mask_out_above_8!(res);
        n_flag!(core) = res;
        not_z_flag!(core) = res;
        v_flag!(core) = VFLAG_CLEAR;
        res
    } else {
        c_flag!(core) = x_flag!(core);
        n_flag!(core) = dst;
        not_z_flag!(core) = mask_out_above_8!(dst);
        v_flag!(core) = VFLAG_CLEAR;
        dst
    }
}

pub fn roxr_16<T: Core>(core: &mut T, dst: u32, orig_shift: u32) -> u32 {
    if orig_shift != 0 {
        let shift = orig_shift % 17;
        let src = mask_out_above_16!(dst);
        let x16 = core.x_flag_as_1() << 16;
        let srcx16 = src | x16;
        let res = (srcx16 >> shift) | (srcx16 << (17-shift));

        x_flag!(core) = res >> 8;
        c_flag!(core) = x_flag!(core);
        let res = mask_out_above_16!(res);
        n_flag!(core) = res >> 8;
        not_z_flag!(core) = res;
        v_flag!(core) = VFLAG_CLEAR;
        res
    } else {
        c_flag!(core) = x_flag!(core);
        n_flag!(core) = dst >> 8;
        not_z_flag!(core) = mask_out_above_16!(dst);
        v_flag!(core) = VFLAG_CLEAR;
        dst
    }
}

pub fn roxr_32<T: Core>(core: &mut T, dst: u32, orig_shift: u32) -> u32 {
    let src = dst;
    let shift = orig_shift % 33;
    let res = if shift != 0 {
        let x32: u64 = u64::from(core.x_flag_as_1()) << 32;
        let srcx32 = u64::from(src) | x32;
        let res = (srcx32 >> shift) | (srcx32 << (33-shift));
        x_flag!(core) = (res >> 24) as u32;
        res as u32
    } else {
        src
    };
    c_flag!(core) = x_flag!(core);
    n_flag!(core) = res >> 24;
    not_z_flag!(core) = res;
    v_flag!(core) = VFLAG_CLEAR;
    res
}

pub fn roxl_8<T: Core>(core: &mut T, dst: u32, orig_shift: u32) -> u32 {
    if orig_shift != 0 {
        let shift = orig_shift % 9;
        let src = mask_out_above_8!(dst);
        let x8 = core.x_flag_as_1() << 8;
        let srcx8 = src | x8;
        let res = (srcx8 << shift) | (srcx8 >> (9-shift));
        x_flag!(core) = res;
        c_flag!(core) = x_flag!(core);
        let res = mask_out_above_8!(res);
        n_flag!(core) = res;
        not_z_flag!(core) = res;
        v_flag!(core) = VFLAG_CLEAR;
        res
    } else {
        c_flag!(core) = x_flag!(core);
        n_flag!(core) = dst;
        not_z_flag!(core) = mask_out_above_8!(dst);
        v_flag!(core) = VFLAG_CLEAR;
        dst
    }
}

pub fn roxl_16<T: Core>(core: &mut T, dst: u32, orig_shift: u32) -> u32 {
    if orig_shift != 0 {
        let shift = orig_shift % 17;
        let src = mask_out_above_16!(dst);
        let x16 = core.x_flag_as_1() << 16;
        let srcx16 = src | x16;
        let res = (srcx16 << shift) | (srcx16 >> (17-shift));

        x_flag!(core) = res >> 8;
        c_flag!(core) = x_flag!(core);
        let res = mask_out_above_16!(res);
        n_flag!(core) = res >> 8;
        not_z_flag!(core) = res;
        v_flag!(core) = VFLAG_CLEAR;
        res
    } else {
        c_flag!(core) = x_flag!(core);
        n_flag!(core) = dst >> 8;
        not_z_flag!(core) = mask_out_above_16!(dst);
        v_flag!(core) = VFLAG_CLEAR;
        dst
    }
}

pub fn roxl_32<T: Core>(core: &mut T, dst: u32, orig_shift: u32) -> u32 {
    let src = dst;
    let shift = orig_shift % 33;
    let res = if shift != 0 {
        let x32: u64 = u64::from(core.x_flag_as_1()) << 32;
        let srcx32 = u64::from(src) | x32;
        let res = (srcx32 << shift) | (srcx32 >> (33-shift));
        x_flag!(core) = (res >> 24) as u32;
        res as u32
    } else {
        src
    };
    c_flag!(core) = x_flag!(core);
    n_flag!(core) = res >> 24;
    not_z_flag!(core) = res;
    v_flag!(core) = VFLAG_CLEAR;
    res
}

// Put common implementation of RTE here
// Put common implementation of RTR here
// Put common implementation of RTS here

pub fn sbcd_8<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let ln_src = low_nibble!(src);
    let hn_src = high_nibble!(src);
    let ln_dst = low_nibble!(dst);
    let hn_dst = high_nibble!(dst);

    let mut res = ln_dst.wrapping_sub(ln_src).wrapping_sub(core.x_flag_as_1());

    v_flag!(core) = !res;

    if res > 9 {
        res -= 6;
    }
    
    res = res.wrapping_add(hn_dst.wrapping_sub(hn_src));
    c_flag!(core) = true_is_1!(res > 0x99) << 8;
    x_flag!(core) = c_flag!(core);

    if c_flag!(core) > 0 {
        res = res.wrapping_add(0xa0);
    }

    v_flag!(core) &= res;
    n_flag!(core) = res;

    res = mask_out_above_8!(res);
    not_z_flag!(core) |= res;
    res
}

// Put common implementation of Scc here
// Put common implementation of STOP here
// Put common implementation of SUB here

pub fn sub_8<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let dst = mask_out_above_8!(dst);
    let src = mask_out_above_8!(src);

    let res = dst.wrapping_sub(src);
    // m68ki_cpu.n_flag = (res);
    n_flag!(core) = res;
    // m68ki_cpu.v_flag = ((src^res) & (dst^res));
    v_flag!(core) = (src ^ dst) & (res ^ dst);
    // m68ki_cpu.x_flag = m68ki_cpu.c_flag = (res);
    c_flag!(core) = res;
    x_flag!(core) = res;
    // m68ki_cpu.not_z_flag = ((res) & 0xff);
    let res8 = mask_out_above_8!(res);
    not_z_flag!(core) = res8;
    res8
}

pub fn sub_16<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let dst = mask_out_above_16!(dst);
    let src = mask_out_above_16!(src);
    let res = dst.wrapping_sub(src);

    // m68ki_cpu.n_flag = ((res)>>8);
    let res_hi = res >> 8;
    n_flag!(core) = res_hi;
    // m68ki_cpu.v_flag = (((src^res) & (dst^res))>>8);
    v_flag!(core) = ((src ^ dst) & (res ^ dst)) >> 8;
    // m68ki_cpu.x_flag = m68ki_cpu.c_flag = ((res)>>8);
    c_flag!(core) = res_hi;
    x_flag!(core) = res_hi;
    // m68ki_cpu.not_z_flag = ((res) & 0xffff);
    let res16 = mask_out_above_16!(res);
    not_z_flag!(core) = res16;

    res16
}

pub fn sub_32<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let res: u64 = u64::from(dst).wrapping_sub(u64::from(src));

    let res_hi = (res >> 24) as u32;
    n_flag!(core) = res_hi;
    // m68ki_cpu.v_flag = (((src^res) & (dst^res))>>24);
    v_flag!(core) = (((u64::from(src )^ u64::from(dst)) & (res ^ u64::from(dst))) >> 24) as u32;
     // m68ki_cpu.x_flag = m68ki_cpu.c_flag = (((src & dst) | (~res & (src | dst)))>>23);
    c_flag!(core) = res_hi;
    x_flag!(core) = res_hi;

    let res32 = res as u32;

    not_z_flag!(core) = res32;

    res32
}

pub fn subx_8<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let dst = mask_out_above_8!(dst);
    let src = mask_out_above_8!(src);
    let res = dst.wrapping_sub(src).wrapping_sub(core.x_flag_as_1());

    n_flag!(core) = res;
    v_flag!(core) = (src ^ dst) & (res ^ dst);
    c_flag!(core) = res;
    x_flag!(core) = res;

    let res8 = mask_out_above_8!(res);
    not_z_flag!(core) |= res8;
    res8
}

pub fn subx_16<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let dst = mask_out_above_16!(dst);
    let src = mask_out_above_16!(src);
    let res = dst.wrapping_sub(src).wrapping_sub(core.x_flag_as_1());

    let res_hi = res >> 8;
    n_flag!(core) = res_hi;
    v_flag!(core) = ((src ^ dst) & (res ^ dst)) >> 8;
    c_flag!(core) = res_hi;
    x_flag!(core) = res_hi;

    let res16 = mask_out_above_16!(res);
    not_z_flag!(core) |= res16;
    res16
}

pub fn subx_32<T: Core>(core: &mut T, dst: u32, src: u32) -> u32 {
    let res = u64::from(dst).wrapping_sub(u64::from(src)).wrapping_sub(u64::from(core.x_flag_as_1()));

    let res_hi = (res >> 24) as u32;
    n_flag!(core) = res_hi;
    v_flag!(core) = (((u64::from(src) ^ u64::from(dst)) & (res ^ u64::from(dst))) >> 24) as u32;
    c_flag!(core) = res_hi;
    x_flag!(core) = res_hi;

    let res32 = res as u32;
    not_z_flag!(core) |= res32;
    res32
}


// Put common implementation of SWAP here
// Put common implementation of TAS here
// Put common implementation of TRAP here
// Put common implementation of TRAPV here
// Put common implementation of TST here
// Put common implementation of UNLK here

#[cfg(test)]
mod tests {
    use super::super::super::{TestCore, Core};

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
    fn core_with_ir<'a>(ir: u16) -> TestCore {
        let mut ic = TestCore::new(0x40);
        ic.ir = ir;

        for r in 0..16 {
            ic.dar[r] = (r * 0x11) as u32;
        }    
        ic
    }
    #[test]
    fn dx_and_dy() {
        let core: &mut Core = &mut core_with_ir(0b1111_1001_1111_1010);

        assert_eq!(0b1111_1001_1111_1010, ir!(core)); // X=4, Y=2
        assert_eq!(0x22, dy!(core));
        assert_eq!(0x44, dx!(core));
    }
    #[test]
    fn more_dx_and_dy() {
        let core: &mut Core = &mut core_with_ir(0b1111_1011_1111_1110);
        assert_eq!(0b1111_1011_1111_1110, ir!(core)); // X=5, Y=6
        assert_eq!(0x66, dy!(core));
        assert_eq!(0x55, dx!(core));
    }
    #[test]
    fn ax_and_ay() {
        let core: &mut Core = &mut core_with_ir(0b1111_1001_1111_1010);

        assert_eq!(0b1111_1001_1111_1010, ir!(core)); // X=4, Y=2
        assert_eq!(0xAA, ay!(core));
        assert_eq!(0xCC, ax!(core));
    }
    #[test]
    fn more_ax_and_ay() {
        let core: &mut Core = &mut core_with_ir(0b1111_1011_1111_1110);

        assert_eq!(0b1111_1011_1111_1110, ir!(core)); // X=5, Y=6
        assert_eq!(0xEE, ay!(core));
        assert_eq!(0xDD, ax!(core));
    }
}
