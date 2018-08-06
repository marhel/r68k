#![macro_use]
use super::{Core, Cycles, Result, EXCEPTION_CHK, EXCEPTION_UNIMPLEMENTED_1010, EXCEPTION_UNIMPLEMENTED_1111, EXCEPTION_ZERO_DIVIDE, EXCEPTION_TRAP_BASE, EXCEPTION_TRAPV};
use super::Exception::*;

mod common;
pub mod handlers;
pub mod opcodes;

pub mod fake {
    use super::super::{Core, Cycles, Result};

    pub fn set_d0<T: Core>(core: &mut T) -> Result<Cycles> {
        dar!(core)[0] = 0xabcd;
        Ok(Cycles(2))
    }

    pub fn set_d1<T: Core>(core: &mut T) -> Result<Cycles> {
        dar!(core)[1] = 0xbcde;
        Ok(Cycles(2))
    }

    pub fn set_dx<T: Core>(core: &mut T) -> Result<Cycles> {
        dx!(core) = 0xcdef;
        Ok(Cycles(2))
    }

    use super::super::InstructionSet;
    use super::illegal;
    const SET_DX_0: usize = 0b0100_0000_0000_0000;

    pub fn instruction_set<T: Core>() -> InstructionSet<T> {
        // Covers all possible IR values (64k entries)
        let mut handler: InstructionSet<T> = Vec::with_capacity(0x10000);
        for _ in 0..0x10000 { handler.push(illegal::<T>); }
        handler[0xA] = set_d0::<T>;
        handler[0xB] = set_d1::<T>;
        for i in 0..8 {
            let opcode = SET_DX_0 | (i << 9);
            // println!("{:x}", opcode);
            handler[opcode] = set_dx::<T>;
        }
        handler
    }
}

macro_rules! impl_op {
    (-, $common:ident, $name:ident, $src:ident, dx, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let dst = dx!(core);
            let _ = common::$common(core, dst, src);
            Ok(Cycles($cycles))
        });
    (-, $common:ident, $name:ident, $src:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let dst = try!(operator::$dst(core));
            let _ = common::$common(core, dst, src);
            Ok(Cycles($cycles))
        });
    (8, $common:ident, $name:ident, $src:ident, dx, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let dst = dx!(core);
            let res = common::$common(core, dst, src);
            dx!(core) = mask_out_below_8!(dst) | res;
            Ok(Cycles($cycles))
        });
    (8, $common:ident, $name:ident, $src:ident, dy, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let dst = dy!(core);
            let res = common::$common(core, dst, src);
            dy!(core) = mask_out_below_8!(dst) | res;
            Ok(Cycles($cycles))
        });
    (16, $common:ident, $name:ident, $src:ident, dx, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let dst = dx!(core);
            let res = common::$common(core, dst, src);
            dx!(core) = mask_out_below_16!(dst) | res;
            Ok(Cycles($cycles))
        });
    (16, $common:ident, $name:ident, $src:ident, dy, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let dst = dy!(core);
            let res = common::$common(core, dst, src);
            dy!(core) = mask_out_below_16!(dst) | res;
            Ok(Cycles($cycles))
        });
    (32, $common:ident, $name:ident, $src:ident, dx, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let dst = dx!(core);
            let res = common::$common(core, dst, src);
            dx!(core) = res;
            Ok(Cycles($cycles))
        });
    (32, $common:ident, $name:ident, $src:ident, dy, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let dst = dy!(core);
            let res = common::$common(core, dst, src);
            dy!(core) = res;
            Ok(Cycles($cycles))
        });
    (8, $common:ident, $name:ident, $src:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let (dst, ea) = try!(operator::$dst(core));
            let res = common::$common(core, dst, src);
            try!(core.write_data_byte(ea, mask_out_below_8!(dst) | res));
            Ok(Cycles($cycles))
        });
    (16, $common:ident, $name:ident, $src:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let (dst, ea) = try!(operator::$dst(core));
            let res = common::$common(core, dst, src);
            try!(core.write_data_word(ea, mask_out_below_16!(dst) | res));
            Ok(Cycles($cycles))
        });
    (32, $common:ident, $name:ident, $src:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let (dst, ea) = try!(operator::$dst(core));
            let res = common::$common(core, dst, src);
            try!(core.write_data_long(ea, res));
            Ok(Cycles($cycles))
        })
}
macro_rules! impl_shift_op {
    (8, $common:ident, $name:ident, $shift_src:ident, dy, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let shift = try!(operator::$shift_src(core)) & 0x3f; // mod 64
            let dst = dy!(core);
            let res = common::$common(core, dst, shift);
            dy!(core) = mask_out_below_8!(dst) | res;
            Ok(Cycles($cycles + 2 * shift as i32))
        });
    (16, $common:ident, $name:ident, 1, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let shift = 1;
            let (dst, ea) = try!(operator::$dst(core));
            let res = common::$common(core, dst, shift);
            try!(core.write_data_word(ea, mask_out_below_16!(dst) | res));
            Ok(Cycles($cycles))
        });
    (16, $common:ident, $name:ident, $shift_src:ident, dy, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let shift = try!(operator::$shift_src(core)) & 0x3f; // mod 64
            let dst = dy!(core);
            let res = common::$common(core, dst, shift);
            dy!(core) = mask_out_below_16!(dst) | res;
            Ok(Cycles($cycles + 2 * shift as i32))
        });
    (32, $common:ident, $name:ident, $shift_src:ident, dy, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let shift = try!(operator::$shift_src(core)) & 0x3f; // mod 64
            let dst = dy!(core);
            let res = common::$common(core, dst, shift);
            dy!(core) = res;
            Ok(Cycles($cycles + 2 * shift as i32))
        });
}

pub fn illegal<T: Core>(core: &mut T) -> Result<Cycles> {
    let illegal_exception = IllegalInstruction(ir!(core), pc!(core).wrapping_sub(2));
    // println!("Exception: {}", illegal_exception);
    Err(illegal_exception)
}
use super::InstructionSet;
pub fn instruction_set<T: Core>() -> InstructionSet<T> {
    handlers::InstructionSetGenerator::new().generate()
}
use std::num::Wrapping;
use super::operator;

pub fn unimplemented_1010<T: Core>(core: &mut T) -> Result<Cycles> {
    Err(UnimplementedInstruction(ir!(core), pc!(core).wrapping_sub(2), EXCEPTION_UNIMPLEMENTED_1010))
}

pub fn unimplemented_1111<T: Core>(core: &mut T) -> Result<Cycles> {
    Err(UnimplementedInstruction(ir!(core), pc!(core).wrapping_sub(2), EXCEPTION_UNIMPLEMENTED_1111))
}

impl_op!(8, abcd_8, abcd_8_rr, dy, dx, 6);
impl_op!(8, abcd_8, abcd_8_mm, ay_pd_8, ea_ax_pd_8, 18);

macro_rules! add_8_er {
    ($name:ident, $src:ident, $cycles:expr) => (impl_op!(8, add_8, $name, $src, dx, $cycles);)
}
macro_rules! add_8_re {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(8, add_8, $name, dx, $dst, $cycles);)
}
macro_rules! add_16_er {
    ($name:ident, $src:ident, $cycles:expr) => (impl_op!(16, add_16, $name, $src, dx, $cycles);)
}
macro_rules! add_16_re {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(16, add_16, $name, dx, $dst, $cycles);)
}
macro_rules! add_32_er {
    ($name:ident, $src:ident, $cycles:expr) => (impl_op!(32, add_32, $name, $src, dx, $cycles);)
}
macro_rules! add_32_re {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(32, add_32, $name, dx, $dst, $cycles);)
}
add_8_er!(add_8_er_dn, dy, 4);
// add_8_er!(..., ay) not present - for word and long only
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

// add_8_re!(..., dy) not present
// add_8_re!(..., ay) not present
add_8_re!(add_8_re_ai, ea_ay_ai_8,  12);
add_8_re!(add_8_re_pi, ea_ay_pi_8,  12);
add_8_re!(add_8_re_pd, ea_ay_pd_8,  14);
add_8_re!(add_8_re_di, ea_ay_di_8,  16);
add_8_re!(add_8_re_ix, ea_ay_ix_8,  18);
add_8_re!(add_8_re_aw, ea_aw_8,     16);
add_8_re!(add_8_re_al, ea_al_8,     20);
// add_8_re!(..., pcdi) not present
// add_8_re!(..., pcix) not present
// add_8_re!(..., imm) not present

add_16_er!(add_16_er_dn, dy,         4);
add_16_er!(add_16_er_an, ay,         4);
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

// add_16_re!(..., dy) not present
// add_16_re!(..., ay) not present
add_16_re!(add_16_re_ai, ea_ay_ai_16,  12);
add_16_re!(add_16_re_pi, ea_ay_pi_16,  12);
add_16_re!(add_16_re_pd, ea_ay_pd_16,  14);
add_16_re!(add_16_re_di, ea_ay_di_16,  16);
add_16_re!(add_16_re_ix, ea_ay_ix_16,  18);
add_16_re!(add_16_re_aw, ea_aw_16,     16);
add_16_re!(add_16_re_al, ea_al_16,     20);
// add_16_re!(..., pcdi) not present
// add_16_re!(..., pcix) not present
// add_16_re!(..., imm) not present

add_32_er!(add_32_er_dn, dy,         6);
add_32_er!(add_32_er_an, ay,         6);
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

// add_32_re!(..., dy) not present
// add_32_re!(..., ay) not present
add_32_re!(add_32_re_ai, ea_ay_ai_32,  12+8);
add_32_re!(add_32_re_pi, ea_ay_pi_32,  12+8);
add_32_re!(add_32_re_pd, ea_ay_pd_32,  14+8);
add_32_re!(add_32_re_di, ea_ay_di_32,  16+8);
add_32_re!(add_32_re_ix, ea_ay_ix_32,  18+8);
add_32_re!(add_32_re_aw, ea_aw_32,     16+8);
add_32_re!(add_32_re_al, ea_al_32,     20+8);
// add_32_re!(..., pcdi) not present
// add_32_re!(..., pcix) not present
// add_32_re!(..., imm) not present

macro_rules! adda_16 {
    ($name:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            // we must evaluate AY (src) first
            // as the PI/PD addressing modes will change AX (if AX=AY)
            let src = try!(operator::$src(core));
            let dst = try!(operator::ax(core));
            ax!(core) = (Wrapping(dst) + Wrapping(src as i16 as u32)).0;
            Ok(Cycles($cycles))
        })
}
macro_rules! adda_32 {
    ($name:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            // we must evaluate AY (src) first
            // as the PI/PD addressing modes will change AX (if AX=AY)
            let src = try!(operator::$src(core));
            let dst = try!(operator::ax(core));
            ax!(core) = (Wrapping(dst) + Wrapping(src)).0;
            Ok(Cycles($cycles))
        })
}
adda_16!(adda_16_dn, dy,         4+4);
adda_16!(adda_16_an, ay,         4+4);
adda_16!(adda_16_ai, ay_ai_16,   8+4);
adda_16!(adda_16_pi, ay_pi_16,   8+4);
adda_16!(adda_16_pd, ay_pd_16,  10+4);
adda_16!(adda_16_di, ay_di_16,  12+4);
adda_16!(adda_16_ix, ay_ix_16,  14+4);
adda_16!(adda_16_aw, aw_16,     12+4);
adda_16!(adda_16_al, al_16,     16+4);
adda_16!(adda_16_pcdi, pcdi_16, 12+4);
adda_16!(adda_16_pcix, pcix_16, 14+4);
adda_16!(adda_16_imm, imm_16,   10+4);

adda_32!(adda_32_dn, dy,         6);
adda_32!(adda_32_an, ay,         6);
adda_32!(adda_32_ai, ay_ai_32,  14);
adda_32!(adda_32_pi, ay_pi_32,  14);
adda_32!(adda_32_pd, ay_pd_32,  16);
adda_32!(adda_32_di, ay_di_32,  18);
adda_32!(adda_32_ix, ay_ix_32,  20);
adda_32!(adda_32_aw, aw_32,     18);
adda_32!(adda_32_al, al_32,     22);
adda_32!(adda_32_pcdi, pcdi_32, 18);
adda_32!(adda_32_pcix, pcix_32, 20);
adda_32!(adda_32_imm, imm_32,   16);

macro_rules! addi_8 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(8, add_8, $name, imm_8, $dst, $cycles);)
}
macro_rules! addi_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(16, add_16, $name, imm_16, $dst, $cycles);)
}
macro_rules! addi_32 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(32, add_32, $name, imm_32, $dst, $cycles);)
}
addi_8!(addi_8_dn, dy,  8);
// addi_8!(..., ay) not present
addi_8!(addi_8_ai, ea_ay_ai_8,  12+4);
addi_8!(addi_8_pi, ea_ay_pi_8,  12+4);
addi_8!(addi_8_pd, ea_ay_pd_8,  12+6);
addi_8!(addi_8_di, ea_ay_di_8,  12+8);
addi_8!(addi_8_ix, ea_ay_ix_8,  12+10);
addi_8!(addi_8_aw, ea_aw_8,     12+8);
addi_8!(addi_8_al, ea_al_8,     12+12);
// addi_8!(..., pcdi) not present
// addi_8!(..., pcix) not present
// addi_8!(..., imm) not present

addi_16!(addi_16_dn, dy,  8);
// addi_16!(..., ay) not present
addi_16!(addi_16_ai, ea_ay_ai_16,  12+4);
addi_16!(addi_16_pi, ea_ay_pi_16,  12+4);
addi_16!(addi_16_pd, ea_ay_pd_16,  12+6);
addi_16!(addi_16_di, ea_ay_di_16,  12+8);
addi_16!(addi_16_ix, ea_ay_ix_16,  12+10);
addi_16!(addi_16_aw, ea_aw_16,     12+8);
addi_16!(addi_16_al, ea_al_16,     12+12);
// addi_16!(..., pcdi) not present
// addi_16!(..., pcix) not present
// addi_16!(..., imm) not present

addi_32!(addi_32_dn, dy,  16);
// addi_32!(..., ay) not present
addi_32!(addi_32_ai, ea_ay_ai_32,  20+8);
addi_32!(addi_32_pi, ea_ay_pi_32,  20+8);
addi_32!(addi_32_pd, ea_ay_pd_32,  20+10);
addi_32!(addi_32_di, ea_ay_di_32,  20+12);
addi_32!(addi_32_ix, ea_ay_ix_32,  20+14);
addi_32!(addi_32_aw, ea_aw_32,     20+12);
addi_32!(addi_32_al, ea_al_32,     20+16);
// addi_32!(..., pcdi) not present
// addi_32!(..., pcix) not present
// addi_32!(..., imm) not present

macro_rules! addq_8 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(8, add_8, $name, quick, $dst, $cycles);)
}
macro_rules! addq_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(16, add_16, $name, quick, $dst, $cycles);)
}
macro_rules! addq_32 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(32, add_32, $name, quick, $dst, $cycles);)
}

addq_8!(addq_8_dn, dy, 4);
// addq_8!(..., ay) not present - word and long only
addq_8!(addq_8_ai, ea_ay_ai_8,  8+4);
addq_8!(addq_8_pi, ea_ay_pi_8,  8+4);
addq_8!(addq_8_pd, ea_ay_pd_8,  8+6);
addq_8!(addq_8_di, ea_ay_di_8,  8+8);
addq_8!(addq_8_ix, ea_ay_ix_8,  8+10);
addq_8!(addq_8_aw, ea_aw_8,     8+8);
addq_8!(addq_8_al, ea_al_8,     8+12);
// addq_8!(..., pcdi) not present
// addq_8!(..., pcix) not present
// addq_8!(..., imm) not present

addq_16!(addq_16_dn, dy,  4);
pub fn addq_16_an<T: Core>(core: &mut T) -> Result<Cycles> {
    let src = try!(operator::quick(core));
    let dst = ay!(core);
    // When adding to address registers, the condition codes are not
    // altered, and the entire destination address register is used
    // regardless of the operation size.
    ay!(core) = (Wrapping(dst) + Wrapping(src)).0;
    Ok(Cycles(4))
}
addq_16!(addq_16_ai, ea_ay_ai_16,  8+4);
addq_16!(addq_16_pi, ea_ay_pi_16,  8+4);
addq_16!(addq_16_pd, ea_ay_pd_16,  8+6);
addq_16!(addq_16_di, ea_ay_di_16,  8+8);
addq_16!(addq_16_ix, ea_ay_ix_16,  8+10);
addq_16!(addq_16_aw, ea_aw_16,     8+8);
addq_16!(addq_16_al, ea_al_16,     8+12);
// addq_16!(..., pcdi) not present
// addq_16!(..., pcix) not present
// addq_16!(..., imm) not present

addq_32!(addq_32_dn, dy,  8);
pub fn addq_32_an<T: Core>(core: &mut T) -> Result<Cycles> {
    let src = try!(operator::quick(core));
    let dst = ay!(core);
    // When adding to address registers, the condition codes are not
    // altered, and the entire destination address register is used
    // regardless of the operation size.
    ay!(core) = (Wrapping(dst) + Wrapping(src)).0;
    Ok(Cycles(8))
}
addq_32!(addq_32_ai, ea_ay_ai_32,  12+8);
addq_32!(addq_32_pi, ea_ay_pi_32,  12+8);
addq_32!(addq_32_pd, ea_ay_pd_32,  12+10);
addq_32!(addq_32_di, ea_ay_di_32,  12+12);
addq_32!(addq_32_ix, ea_ay_ix_32,  12+14);
addq_32!(addq_32_aw, ea_aw_32,     12+12);
addq_32!(addq_32_al, ea_al_32,     12+16);
// addq_32!(..., pcdi) not present
// addq_32!(..., pcix) not present
// addq_32!(..., imm) not present

impl_op!( 8, addx_8,  addx_8_rr, dy, dx, 4);
impl_op!( 8, addx_8,  addx_8_mm, ay_pd_8, ea_ax_pd_8, 18);
impl_op!(16, addx_16, addx_16_rr, dy, dx, 4);
impl_op!(16, addx_16, addx_16_mm, ay_pd_16, ea_ax_pd_16, 18);
impl_op!(32, addx_32, addx_32_rr, dy, dx, 8);
impl_op!(32, addx_32, addx_32_mm, ay_pd_32, ea_ax_pd_32, 30);

macro_rules! and_8_er {
    ($name:ident, $src:ident, $cycles:expr) => (impl_op!(8, and_8, $name, $src, dx, $cycles);)
}
macro_rules! and_8_re {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(8, and_8, $name, dx, $dst, $cycles);)
}
macro_rules! and_16_er {
    ($name:ident, $src:ident, $cycles:expr) => (impl_op!(16, and_16, $name, $src, dx, $cycles);)
}
macro_rules! and_16_re {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(16, and_16, $name, dx, $dst, $cycles);)
}
macro_rules! and_32_er {
    ($name:ident, $src:ident, $cycles:expr) => (impl_op!(32, and_32, $name, $src, dx, $cycles);)
}
macro_rules! and_32_re {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(32, and_32, $name, dx, $dst, $cycles);)
}

and_8_er!(and_8_er_dn, dy, 4);
// and_8_er!(..., ay) not present
and_8_er!(and_8_er_ai, ay_ai_8,   8);
and_8_er!(and_8_er_pi, ay_pi_8,   8);
and_8_er!(and_8_er_pd, ay_pd_8,  10);
and_8_er!(and_8_er_di, ay_di_8,  12);
and_8_er!(and_8_er_ix, ay_ix_8,  14);
and_8_er!(and_8_er_aw, aw_8,     12);
and_8_er!(and_8_er_al, al_8,     16);
and_8_er!(and_8_er_pcdi, pcdi_8, 12);
and_8_er!(and_8_er_pcix, pcix_8, 14);
and_8_er!(and_8_er_imm, imm_8,   10);

// and_8_re!(..., dy) not present
// and_8_re!(..., ay) not present
and_8_re!(and_8_re_ai, ea_ay_ai_8,  12);
and_8_re!(and_8_re_pi, ea_ay_pi_8,  12);
and_8_re!(and_8_re_pd, ea_ay_pd_8,  14);
and_8_re!(and_8_re_di, ea_ay_di_8,  16);
and_8_re!(and_8_re_ix, ea_ay_ix_8,  18);
and_8_re!(and_8_re_aw, ea_aw_8,     16);
and_8_re!(and_8_re_al, ea_al_8,     20);
// and_8_re!(..., pcdi) not present
// and_8_re!(..., pcix) not present
// and_8_re!(..., imm) not present

and_16_er!(and_16_er_dn,   dy,       4);
// and_16_er!(..., ay) not present
and_16_er!(and_16_er_ai,   ay_ai_16, 8);
and_16_er!(and_16_er_pi,   ay_pi_16, 8);
and_16_er!(and_16_er_pd,   ay_pd_16, 10);
and_16_er!(and_16_er_di,   ay_di_16, 12);
and_16_er!(and_16_er_ix,   ay_ix_16, 14);
and_16_er!(and_16_er_aw,   aw_16,    12);
and_16_er!(and_16_er_al,   al_16,    16);
and_16_er!(and_16_er_pcdi, pcdi_16,  12);
and_16_er!(and_16_er_pcix, pcix_16,  14);
and_16_er!(and_16_er_imm,  imm_16,   10);

// and_16_re!(..., dy) not present
// and_16_re!(..., ay) not present
and_16_re!(and_16_re_ai, ea_ay_ai_16,  12);
and_16_re!(and_16_re_pi, ea_ay_pi_16,  12);
and_16_re!(and_16_re_pd, ea_ay_pd_16,  14);
and_16_re!(and_16_re_di, ea_ay_di_16,  16);
and_16_re!(and_16_re_ix, ea_ay_ix_16,  18);
and_16_re!(and_16_re_aw, ea_aw_16,     16);
and_16_re!(and_16_re_al, ea_al_16,     20);
// and_16_re!(..., pcdi) not present
// and_16_re!(..., pcix) not present
// and_16_re!(..., imm) not present

and_32_er!(and_32_er_dn,   dy,        6);
// and_32_er!(..., ay) not present
and_32_er!(and_32_er_ai,   ay_ai_32, 14);
and_32_er!(and_32_er_pi,   ay_pi_32, 14);
and_32_er!(and_32_er_pd,   ay_pd_32, 16);
and_32_er!(and_32_er_di,   ay_di_32, 18);
and_32_er!(and_32_er_ix,   ay_ix_32, 20);
and_32_er!(and_32_er_aw,   aw_32,    18);
and_32_er!(and_32_er_al,   al_32,    22);
and_32_er!(and_32_er_pcdi, pcdi_32,  18);
and_32_er!(and_32_er_pcix, pcix_32,  20);
and_32_er!(and_32_er_imm,  imm_32,   16);

// and_32_re!(..., dy) not present
// and_32_re!(..., ay) not present
and_32_re!(and_32_re_ai, ea_ay_ai_32,  12+8);
and_32_re!(and_32_re_pi, ea_ay_pi_32,  12+8);
and_32_re!(and_32_re_pd, ea_ay_pd_32,  14+8);
and_32_re!(and_32_re_di, ea_ay_di_32,  16+8);
and_32_re!(and_32_re_ix, ea_ay_ix_32,  18+8);
and_32_re!(and_32_re_aw, ea_aw_32,     16+8);
and_32_re!(and_32_re_al, ea_al_32,     20+8);
// and_32_re!(..., pcdi) not present
// and_32_re!(..., pcix) not present
// and_32_re!(..., imm) not present

macro_rules! andi_8 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(8, and_8, $name, imm_8, $dst, $cycles);)
}
macro_rules! andi_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(16, and_16, $name, imm_16, $dst, $cycles);)
}
macro_rules! andi_32 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(32, and_32, $name, imm_32, $dst, $cycles);)
}
andi_8!(andi_8_dn, dy,  8);
// andi_8_re!(..., ay) not present
andi_8!(andi_8_ai, ea_ay_ai_8,  12+4);
andi_8!(andi_8_pi, ea_ay_pi_8,  12+4);
andi_8!(andi_8_pd, ea_ay_pd_8,  12+6);
andi_8!(andi_8_di, ea_ay_di_8,  12+8);
andi_8!(andi_8_ix, ea_ay_ix_8,  12+10);
andi_8!(andi_8_aw, ea_aw_8,     12+8);
andi_8!(andi_8_al, ea_al_8,     12+12);
// andi_8!(..., pcdi) not present
// andi_8!(..., pcix) not present
// andi_8!(..., imm) not present

andi_16!(andi_16_dn, dy,  8);
// andi_16_re!(..., ay) not present
andi_16!(andi_16_ai, ea_ay_ai_16,  12+4);
andi_16!(andi_16_pi, ea_ay_pi_16,  12+4);
andi_16!(andi_16_pd, ea_ay_pd_16,  12+6);
andi_16!(andi_16_di, ea_ay_di_16,  12+8);
andi_16!(andi_16_ix, ea_ay_ix_16,  12+10);
andi_16!(andi_16_aw, ea_aw_16,     12+8);
andi_16!(andi_16_al, ea_al_16,     12+12);
// andi_16!(..., pcdi) not present
// andi_16!(..., pcix) not present
// andi_16!(..., imm) not present

andi_32!(andi_32_dn, dy,  14);
// andi_32_re!(..., ay) not present
andi_32!(andi_32_ai, ea_ay_ai_32,  20+8);
andi_32!(andi_32_pi, ea_ay_pi_32,  20+8);
andi_32!(andi_32_pd, ea_ay_pd_32,  20+10);
andi_32!(andi_32_di, ea_ay_di_32,  20+12);
andi_32!(andi_32_ix, ea_ay_ix_32,  20+14);
andi_32!(andi_32_aw, ea_aw_32,     20+12);
andi_32!(andi_32_al, ea_al_32,     20+16);
// andi_32!(..., pcdi) not present
// andi_32!(..., pcix) not present
// andi_32!(..., imm) not present

pub fn andi_8_toc<T: Core>(core: &mut T) -> Result<Cycles> {
    let dst = core.condition_code_register();
    let src = mask_out_above_8!(try!(operator::imm_8(core))) as u16;
    core.ccr_to_flags(dst & src);
    Ok(Cycles(20))
}
pub fn andi_16_tos<T: Core>(core: &mut T) -> Result<Cycles> {
    if s_flag!(core) != 0 {
        let dst = core.status_register();
        let src = try!(operator::imm_16(core)) as u16;
        core.sr_to_flags(dst & src);
        Ok(Cycles(20))
    } else {
        Err(PrivilegeViolation(ir!(core), pc!(core).wrapping_sub(2)))
    }
}

macro_rules! asr_8 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(8, asr_8, $name, $src, $dst, $cycles);)
}
macro_rules! asr_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(16, asr_16, $name, 1, $dst, $cycles););
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(16, asr_16, $name, $src, $dst, $cycles);)
}
macro_rules! asr_32 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(32, asr_32, $name, $src, $dst, $cycles);)
}

macro_rules! asl_8 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(8, asl_8, $name, $src, $dst, $cycles);)
}
macro_rules! asl_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(16, asl_16, $name, 1, $dst, $cycles););
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(16, asl_16, $name, $src, $dst, $cycles);)
}
macro_rules! asl_32 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(32, asl_32, $name, $src, $dst, $cycles);)
}

asr_8!(asr_8_s,   quick, dy, 6);
asr_16!(asr_16_s, quick, dy, 6);
asr_32!(asr_32_s, quick, dy, 8);
asr_8!(asr_8_r,   dx,    dy, 6);
asr_16!(asr_16_r, dx,    dy, 6);
asr_32!(asr_32_r, dx,    dy, 8);

asl_8!(asl_8_s,   quick, dy, 6);
asl_16!(asl_16_s, quick, dy, 6);
asl_32!(asl_32_s, quick, dy, 8);
asl_8!(asl_8_r,   dx,    dy, 6);
asl_16!(asl_16_r, dx,    dy, 6);
asl_32!(asl_32_r, dx,    dy, 8);

asl_16!(asl_16_ai, ea_ay_ai_16, 12);
asl_16!(asl_16_pi, ea_ay_pi_16, 12);
asl_16!(asl_16_pd, ea_ay_pd_16, 14);
asl_16!(asl_16_di, ea_ay_di_16, 16);
asl_16!(asl_16_ix, ea_ay_ix_16, 18);
asl_16!(asl_16_aw, ea_aw_16,    16);
asl_16!(asl_16_al, ea_al_16,    20);

asr_16!(asr_16_ai, ea_ay_ai_16, 12);
asr_16!(asr_16_pi, ea_ay_pi_16, 12);
asr_16!(asr_16_pd, ea_ay_pd_16, 14);
asr_16!(asr_16_di, ea_ay_di_16, 16);
asr_16!(asr_16_ix, ea_ay_ix_16, 18);
asr_16!(asr_16_aw, ea_aw_16,    16);
asr_16!(asr_16_al, ea_al_16,    20);

macro_rules! branch {
    (8, $name:ident, $cond:ident) => {
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            Ok(if core.$cond()
            {
                let offset = mask_out_above_8!(ir!(core)) as i8;
                core.branch_8(offset);
                Cycles(10)
            } else {
                Cycles(8)
            })
        }
    };
    (16, $name:ident, $cond:ident) => {
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            Ok(if core.$cond()
            {
                let offset = try!(core.read_imm_i16());
                pc!(core) = pc!(core).wrapping_sub(2);
                core.branch_16(offset);
                Cycles(10)
            } else {
                pc!(core) = pc!(core).wrapping_add(2);
                Cycles(12)
            })
        }
    };
    (16, $name:ident, $cond:ident, dy) => {
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            Ok(if !core.$cond()
            {
                let dst = dy!(core);
                let res = mask_out_above_16!(dst.wrapping_sub(1));
                dy!(core) = mask_out_below_16!(dst) | res;
                if res != 0xffff {
                    let offset = try!(core.read_imm_i16());
                    pc!(core) = pc!(core).wrapping_sub(2);
                    core.branch_16(offset);
                    Cycles(10)
                } else {
                    pc!(core) = pc!(core).wrapping_add(2);
                    Cycles(14)
                }
            } else {
                pc!(core) = pc!(core).wrapping_add(2);
                Cycles(12)
            })
        }
    };
}

branch!(8, bhi_8, cond_hi);
branch!(8, bls_8, cond_ls);
branch!(8, bcc_8, cond_cc);
branch!(8, bcs_8, cond_cs);
branch!(8, bne_8, cond_ne);
branch!(8, beq_8, cond_eq);
branch!(8, bvc_8, cond_vc);
branch!(8, bvs_8, cond_vs);
branch!(8, bpl_8, cond_pl);
branch!(8, bmi_8, cond_mi);
branch!(8, bge_8, cond_ge);
branch!(8, blt_8, cond_lt);
branch!(8, bgt_8, cond_gt);
branch!(8, ble_8, cond_le);

branch!(16, bhi_16, cond_hi);
branch!(16, bls_16, cond_ls);
branch!(16, bcc_16, cond_cc);
branch!(16, bcs_16, cond_cs);
branch!(16, bne_16, cond_ne);
branch!(16, beq_16, cond_eq);
branch!(16, bvc_16, cond_vc);
branch!(16, bvs_16, cond_vs);
branch!(16, bpl_16, cond_pl);
branch!(16, bmi_16, cond_mi);
branch!(16, bge_16, cond_ge);
branch!(16, blt_16, cond_lt);
branch!(16, bgt_16, cond_gt);
branch!(16, ble_16, cond_le);

macro_rules! bchg_8 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core)) & 7; // modulo 8
            let (dst, ea) = try!(operator::$dst(core));
            let mask = 1 << src;
            not_z_flag!(core) = dst & mask;
            try!(core.write_data_byte(ea, dst ^ mask));
            Ok(Cycles($cycles))
        });
}

macro_rules! bclr_8 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core)) & 7; // modulo 8
            let (dst, ea) = try!(operator::$dst(core));
            let mask = 1 << src;
            not_z_flag!(core) = dst & mask;
            try!(core.write_data_byte(ea, dst & !mask));
            Ok(Cycles($cycles))
        });
}

macro_rules! bset_8 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core)) & 7; // modulo 8
            let (dst, ea) = try!(operator::$dst(core));
            let mask = 1 << src;
            not_z_flag!(core) = dst & mask;
            try!(core.write_data_byte(ea, dst | mask));
            Ok(Cycles($cycles))
        });
}

macro_rules! btst_8 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core)) & 7; // modulo 8
            let dst = try!(operator::$dst(core));
            let mask = 1 << src;
            not_z_flag!(core) = dst & mask;
            Ok(Cycles($cycles))
        });
}

pub fn bchg_32_r_dn<T: Core>(core: &mut T) -> Result<Cycles> {
    let dst = dy!(core);
    let src = dx!(core);
    let mask = 1 << (src & 0x1f);

    not_z_flag!(core) = dst & mask;
    dy!(core) ^= mask;
    Ok(Cycles(8))
}

pub fn bchg_32_s_dn<T: Core>(core: &mut T) -> Result<Cycles> {
    let dst = dy!(core);
    let src = try!(operator::imm_8(core));
    let mask = 1 << (src & 0x1f);

    not_z_flag!(core) = dst & mask;
    dy!(core) ^= mask;
    Ok(Cycles(12))
}

bchg_8!(bchg_8_r_ai, dx,    ea_ay_ai_8,  8+4 );
bchg_8!(bchg_8_r_pi, dx,    ea_ay_pi_8,  8+4 );
bchg_8!(bchg_8_r_pd, dx,    ea_ay_pd_8,  8+6 );
bchg_8!(bchg_8_r_di, dx,    ea_ay_di_8,  8+8 );
bchg_8!(bchg_8_r_ix, dx,    ea_ay_ix_8,  8+10);
bchg_8!(bchg_8_r_aw, dx,    ea_aw_8,     8+8 );
bchg_8!(bchg_8_r_al, dx,    ea_al_8,     8+12);
bchg_8!(bchg_8_s_ai, imm_8, ea_ay_ai_8, 12+4 );
bchg_8!(bchg_8_s_pi, imm_8, ea_ay_pi_8, 12+4 );
bchg_8!(bchg_8_s_pd, imm_8, ea_ay_pd_8, 12+6 );
bchg_8!(bchg_8_s_di, imm_8, ea_ay_di_8, 12+8 );
bchg_8!(bchg_8_s_ix, imm_8, ea_ay_ix_8, 12+10);
bchg_8!(bchg_8_s_aw, imm_8, ea_aw_8,    12+8 );
bchg_8!(bchg_8_s_al, imm_8, ea_al_8,    12+12);

pub fn bclr_32_r_dn<T: Core>(core: &mut T) -> Result<Cycles> {
    let dst = dy!(core);
    let src = dx!(core);
    let mask = 1 << (src & 0x1f);

    not_z_flag!(core) = dst & mask;
    dy!(core) &= !mask;
    Ok(Cycles(10))
}

pub fn bclr_32_s_dn<T: Core>(core: &mut T) -> Result<Cycles> {
    let dst = dy!(core);
    let src = try!(operator::imm_8(core));
    let mask = 1 << (src & 0x1f);

    not_z_flag!(core) = dst & mask;
    dy!(core) &= !mask;
    Ok(Cycles(14))
}

bclr_8!(bclr_8_r_ai, dx,    ea_ay_ai_8,  8+4 );
bclr_8!(bclr_8_r_pi, dx,    ea_ay_pi_8,  8+4 );
bclr_8!(bclr_8_r_pd, dx,    ea_ay_pd_8,  8+6 );
bclr_8!(bclr_8_r_di, dx,    ea_ay_di_8,  8+8 );
bclr_8!(bclr_8_r_ix, dx,    ea_ay_ix_8,  8+10);
bclr_8!(bclr_8_r_aw, dx,    ea_aw_8,     8+8 );
bclr_8!(bclr_8_r_al, dx,    ea_al_8,     8+12);
bclr_8!(bclr_8_s_ai, imm_8, ea_ay_ai_8, 12+4 );
bclr_8!(bclr_8_s_pi, imm_8, ea_ay_pi_8, 12+4 );
bclr_8!(bclr_8_s_pd, imm_8, ea_ay_pd_8, 12+6 );
bclr_8!(bclr_8_s_di, imm_8, ea_ay_di_8, 12+8 );
bclr_8!(bclr_8_s_ix, imm_8, ea_ay_ix_8, 12+10);
bclr_8!(bclr_8_s_aw, imm_8, ea_aw_8,    12+8 );
bclr_8!(bclr_8_s_al, imm_8, ea_al_8,    12+12);


pub fn bset_32_r_dn<T: Core>(core: &mut T) -> Result<Cycles> {
    let dst = dy!(core);
    let src = dx!(core);
    let mask = 1 << (src & 0x1f);

    not_z_flag!(core) = dst & mask;
    dy!(core) |= mask;
    Ok(Cycles(8))
}

pub fn bset_32_s_dn<T: Core>(core: &mut T) -> Result<Cycles> {
    let dst = dy!(core);
    let src = try!(operator::imm_8(core));
    let mask = 1 << (src & 0x1f);

    not_z_flag!(core) = dst & mask;
    dy!(core) |= mask;
    Ok(Cycles(12))
}

bset_8!(bset_8_r_ai, dx,    ea_ay_ai_8,  8+4 );
bset_8!(bset_8_r_pi, dx,    ea_ay_pi_8,  8+4 );
bset_8!(bset_8_r_pd, dx,    ea_ay_pd_8,  8+6 );
bset_8!(bset_8_r_di, dx,    ea_ay_di_8,  8+8 );
bset_8!(bset_8_r_ix, dx,    ea_ay_ix_8,  8+10);
bset_8!(bset_8_r_aw, dx,    ea_aw_8,     8+8 );
bset_8!(bset_8_r_al, dx,    ea_al_8,     8+12);
bset_8!(bset_8_s_ai, imm_8, ea_ay_ai_8, 12+4 );
bset_8!(bset_8_s_pi, imm_8, ea_ay_pi_8, 12+4 );
bset_8!(bset_8_s_pd, imm_8, ea_ay_pd_8, 12+6 );
bset_8!(bset_8_s_di, imm_8, ea_ay_di_8, 12+8 );
bset_8!(bset_8_s_ix, imm_8, ea_ay_ix_8, 12+10);
bset_8!(bset_8_s_aw, imm_8, ea_aw_8,    12+8 );
bset_8!(bset_8_s_al, imm_8, ea_al_8,    12+12);


pub fn btst_32_r_dn<T: Core>(core: &mut T) -> Result<Cycles> {
    let dst = dy!(core);
    let src = dx!(core);
    let mask = 1 << (src & 0x1f);

    not_z_flag!(core) = dst & mask;
    Ok(Cycles(6))
}

pub fn btst_32_s_dn<T: Core>(core: &mut T) -> Result<Cycles> {
    let dst = dy!(core);
    let src = try!(operator::imm_8(core));
    let mask = 1 << (src & 0x1f);

    not_z_flag!(core) = dst & mask;
    Ok(Cycles(10))
}

btst_8!(btst_8_r_ai,   dx,    ay_ai_8, 4+4 );
btst_8!(btst_8_r_pi,   dx,    ay_pi_8, 4+4 );
btst_8!(btst_8_r_pd,   dx,    ay_pd_8, 4+6 );
btst_8!(btst_8_r_di,   dx,    ay_di_8, 4+8 );
btst_8!(btst_8_r_ix,   dx,    ay_ix_8, 4+10);
btst_8!(btst_8_r_aw,   dx,    aw_8,    4+8 );
btst_8!(btst_8_r_al,   dx,    al_8,    4+12);
btst_8!(btst_8_r_pcdi, dx,    pcdi_8,  4+8);
btst_8!(btst_8_r_pcix, dx,    pcix_8,  4+10);
btst_8!(btst_8_r_imm,  dx,    imm_8,   4+4);
btst_8!(btst_8_s_ai,   imm_8, ay_ai_8, 8+4 );
btst_8!(btst_8_s_pi,   imm_8, ay_pi_8, 8+4 );
btst_8!(btst_8_s_pd,   imm_8, ay_pd_8, 8+6 );
btst_8!(btst_8_s_di,   imm_8, ay_di_8, 8+8 );
btst_8!(btst_8_s_ix,   imm_8, ay_ix_8, 8+10);
btst_8!(btst_8_s_aw,   imm_8, aw_8,    8+8 );
btst_8!(btst_8_s_al,   imm_8, al_8,    8+12);
btst_8!(btst_8_s_pcdi, imm_8, pcdi_8,  8+8);
btst_8!(btst_8_s_pcix, imm_8, pcix_8,  8+10);
btst_8!(btst_8_s_imm,  imm_8, imm_8,   8+4);

pub fn bra_8<T: Core>(core: &mut T) -> Result<Cycles> {
    let offset = mask_out_above_8!(ir!(core)) as i8;
    core.branch_8(offset);
    Ok(Cycles(10))
}

pub fn bra_16<T: Core>(core: &mut T) -> Result<Cycles> {
    let offset = try!(core.read_imm_i16());
    pc!(core) = pc!(core).wrapping_sub(2);
    core.branch_16(offset);
    Ok(Cycles(10))
}

pub fn bsr_8<T: Core>(core: &mut T) -> Result<Cycles> {
    let offset = mask_out_above_8!(ir!(core)) as i8;
    let pc = pc!(core);
    core.push_32(pc);
    core.branch_8(offset);
    Ok(Cycles(18))
}

pub fn bsr_16<T: Core>(core: &mut T) -> Result<Cycles> {
    let offset = try!(core.read_imm_i16());
    let pc = pc!(core);
    core.push_32(pc);
    pc!(core) = pc!(core).wrapping_sub(2);
    core.branch_16(offset);
    Ok(Cycles(18))
}

macro_rules! chk_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = dx!(core) as i16;
            let bound = try!(operator::$dst(core)) as i16;

            not_z_flag!(core) = src as u32 & 0xffff;
            v_flag!(core) = 0;
            c_flag!(core) = 0;

            if src >= 0 && src <= bound
            {
                Ok(Cycles($cycles))
            } else {
                n_flag!(core) = if src < 0 {1 << 7} else {0};
                // 40 cycles for the CHK trap + EA calculation time
                // deduct the 10 base cycles for the instruction, to extract EA cycles.
                Err(Trap(EXCEPTION_CHK, 40 + $cycles - 10))
            }
        });
}
chk_16!(chk_16_ai,   ay_ai_16,  10 +  4);
chk_16!(chk_16_al,   al_16,     10 + 12);
chk_16!(chk_16_aw,   aw_16,     10 +  8);
chk_16!(chk_16_dn,   dy,        10 +  0);
chk_16!(chk_16_di,   ay_di_16,  10 +  8);
chk_16!(chk_16_imm,  imm_16,    10 +  4);
chk_16!(chk_16_ix,   ay_ix_16,  10 + 10);
chk_16!(chk_16_pcdi, pcdi_16,   10 +  8);
chk_16!(chk_16_pcix, pcix_16,   10 + 10);
chk_16!(chk_16_pd,   ay_pd_16,  10 +  6);
chk_16!(chk_16_pi,   ay_pi_16,  10 +  4);

use cpu::effective_address;

macro_rules! clr {
    ($name:ident, $dst:ident, $write_op:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            // The MC68000PRM says: In the MC68000 and MC68008 a memory location is read before it is cleared.
            // We skip this as Musashi doesn't do that either.
            let ea = try!(effective_address::$dst(core));

            try!(core.$write_op(ea, 0));

            n_flag!(core) = 0;
            v_flag!(core) = 0;
            c_flag!(core) = 0;
            not_z_flag!(core) = 0;
            Ok(Cycles($cycles))
        });
}

pub fn clr_8_dn<T: Core>(core: &mut T) -> Result<Cycles> {
    dy!(core) &= 0xffff_ff00;

    n_flag!(core) = 0;
    v_flag!(core) = 0;
    c_flag!(core) = 0;
    not_z_flag!(core) = 0;
    Ok(Cycles(4))
}
clr!(clr_8_ai, address_indirect_ay, write_data_byte, 8+4);
clr!(clr_8_pi, postincrement_ay_8,  write_data_byte, 8+4);
clr!(clr_8_pd, predecrement_ay_8,   write_data_byte, 8+6);
clr!(clr_8_di, displacement_ay,     write_data_byte, 8+8);
clr!(clr_8_ix, index_ay,            write_data_byte, 8+10);
clr!(clr_8_aw, absolute_word,       write_data_byte, 8+8);
clr!(clr_8_al, absolute_long,       write_data_byte, 8+12);

pub fn clr_16_dn<T: Core>(core: &mut T) -> Result<Cycles> {
    dy!(core) &= 0xffff_0000;

    n_flag!(core) = 0;
    v_flag!(core) = 0;
    c_flag!(core) = 0;
    not_z_flag!(core) = 0;
    Ok(Cycles(4))
}
clr!(clr_16_ai, address_indirect_ay, write_data_word, 8+4);
clr!(clr_16_pi, postincrement_ay_16, write_data_word, 8+4);
clr!(clr_16_pd, predecrement_ay_16,  write_data_word, 8+6);
clr!(clr_16_di, displacement_ay,     write_data_word, 8+8);
clr!(clr_16_ix, index_ay,            write_data_word, 8+10);
clr!(clr_16_aw, absolute_word,       write_data_word, 8+8);
clr!(clr_16_al, absolute_long,       write_data_word, 8+12);

pub fn clr_32_dn<T: Core>(core: &mut T) -> Result<Cycles> {
    dy!(core) = 0;

    n_flag!(core) = 0;
    v_flag!(core) = 0;
    c_flag!(core) = 0;
    not_z_flag!(core) = 0;
    Ok(Cycles(6))
}
clr!(clr_32_ai, address_indirect_ay, write_data_long, 12+8);
clr!(clr_32_pi, postincrement_ay_32, write_data_long, 12+8);
clr!(clr_32_pd, predecrement_ay_32,  write_data_long, 12+10);
clr!(clr_32_di, displacement_ay,     write_data_long, 12+12);
clr!(clr_32_ix, index_ay,            write_data_long, 12+14);
clr!(clr_32_aw, absolute_word,       write_data_long, 12+12);
clr!(clr_32_al, absolute_long,       write_data_long, 12+16);

impl_op!(-, cmp_8, cmp_8_dn,   dy,      dx, 4+0);
impl_op!(-, cmp_8, cmp_8_ai,   ay_ai_8, dx, 4+4);
impl_op!(-, cmp_8, cmp_8_pi,   ay_pi_8, dx, 4+4);
impl_op!(-, cmp_8, cmp_8_pd,   ay_pd_8, dx, 4+6);
impl_op!(-, cmp_8, cmp_8_di,   ay_di_8, dx, 4+8);
impl_op!(-, cmp_8, cmp_8_ix,   ay_ix_8, dx, 4+10);
impl_op!(-, cmp_8, cmp_8_aw,   aw_8,    dx, 4+8);
impl_op!(-, cmp_8, cmp_8_al,   al_8,    dx, 4+12);
impl_op!(-, cmp_8, cmp_8_pcdi, pcdi_8,  dx, 4+8);
impl_op!(-, cmp_8, cmp_8_pcix, pcix_8,  dx, 4+10);
impl_op!(-, cmp_8, cmp_8_imm,  imm_8,   dx, 4+4);

impl_op!(-, cmp_16, cmp_16_dn,   dy,       dx, 4+0);
impl_op!(-, cmp_16, cmp_16_an,   ay,       dx, 4+0);
impl_op!(-, cmp_16, cmp_16_ai,   ay_ai_16, dx, 4+4);
impl_op!(-, cmp_16, cmp_16_pi,   ay_pi_16, dx, 4+4);
impl_op!(-, cmp_16, cmp_16_pd,   ay_pd_16, dx, 4+6);
impl_op!(-, cmp_16, cmp_16_di,   ay_di_16, dx, 4+8);
impl_op!(-, cmp_16, cmp_16_ix,   ay_ix_16, dx, 4+10);
impl_op!(-, cmp_16, cmp_16_aw,   aw_16,    dx, 4+8);
impl_op!(-, cmp_16, cmp_16_al,   al_16,    dx, 4+12);
impl_op!(-, cmp_16, cmp_16_pcdi, pcdi_16,  dx, 4+8);
impl_op!(-, cmp_16, cmp_16_pcix, pcix_16,  dx, 4+10);
impl_op!(-, cmp_16, cmp_16_imm,  imm_16,   dx, 4+4);

impl_op!(-, cmp_32, cmp_32_dn,   dy,       dx, 6+0);
impl_op!(-, cmp_32, cmp_32_an,   ay,       dx, 6+0);
impl_op!(-, cmp_32, cmp_32_ai,   ay_ai_32, dx, 6+8);
impl_op!(-, cmp_32, cmp_32_pi,   ay_pi_32, dx, 6+8);
impl_op!(-, cmp_32, cmp_32_pd,   ay_pd_32, dx, 6+10);
impl_op!(-, cmp_32, cmp_32_di,   ay_di_32, dx, 6+12);
impl_op!(-, cmp_32, cmp_32_ix,   ay_ix_32, dx, 6+14);
impl_op!(-, cmp_32, cmp_32_aw,   aw_32,    dx, 6+12);
impl_op!(-, cmp_32, cmp_32_al,   al_32,    dx, 6+16);
impl_op!(-, cmp_32, cmp_32_pcdi, pcdi_32,  dx, 6+12);
impl_op!(-, cmp_32, cmp_32_pcix, pcix_32,  dx, 6+14);
impl_op!(-, cmp_32, cmp_32_imm,  imm_32,   dx, 6+8);

macro_rules! cmpa_16 {
    ($name:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core)) as i16 as u32;
            let dst = try!(operator::ax(core));
            let _ = common::cmp_32(core, dst, src);
            Ok(Cycles($cycles))
        })
}
macro_rules! cmpa_32 {
    ($name:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let dst = try!(operator::ax(core));
            let _ = common::cmp_32(core, dst, src);
            Ok(Cycles($cycles))
        })
}
cmpa_16!(cmpa_16_dn, dy,        6+0);
cmpa_16!(cmpa_16_an, ay,        6+0);
cmpa_16!(cmpa_16_ai, ay_ai_16,  6+4);
cmpa_16!(cmpa_16_pi, ay_pi_16,  6+4);
cmpa_16!(cmpa_16_pd, ay_pd_16,  6+6);
cmpa_16!(cmpa_16_di, ay_di_16,  6+8);
cmpa_16!(cmpa_16_ix, ay_ix_16,  6+10);
cmpa_16!(cmpa_16_aw, aw_16,     6+8);
cmpa_16!(cmpa_16_al, al_16,     6+12);
cmpa_16!(cmpa_16_pcdi, pcdi_16, 6+8);
cmpa_16!(cmpa_16_pcix, pcix_16, 6+10);
cmpa_16!(cmpa_16_imm, imm_16,   6+4);

cmpa_32!(cmpa_32_dn, dy,        6+0);
cmpa_32!(cmpa_32_an, ay,        6+0);
cmpa_32!(cmpa_32_ai, ay_ai_32,  6+8);
cmpa_32!(cmpa_32_pi, ay_pi_32,  6+8);
cmpa_32!(cmpa_32_pd, ay_pd_32,  6+10);
cmpa_32!(cmpa_32_di, ay_di_32,  6+12);
cmpa_32!(cmpa_32_ix, ay_ix_32,  6+14);
cmpa_32!(cmpa_32_aw, aw_32,     6+12);
cmpa_32!(cmpa_32_al, al_32,     6+16);
cmpa_32!(cmpa_32_pcdi, pcdi_32, 6+12);
cmpa_32!(cmpa_32_pcix, pcix_32, 6+14);
cmpa_32!(cmpa_32_imm, imm_32,   6+8);

macro_rules! cmpi_8 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(-, cmp_8, $name, imm_8, $dst, $cycles);)
}
macro_rules! cmpi_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(-, cmp_16, $name, imm_16, $dst, $cycles);)
}
macro_rules! cmpi_32 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(-, cmp_32, $name, imm_32, $dst, $cycles);)
}
cmpi_8!(cmpi_8_dn, dy,          8+0);
// cmpi_8!(..., ay) not present
cmpi_8!(cmpi_8_ai, ay_ai_8,  8+4);
cmpi_8!(cmpi_8_pi, ay_pi_8,  8+4);
cmpi_8!(cmpi_8_pd, ay_pd_8,  8+6);
cmpi_8!(cmpi_8_di, ay_di_8,  8+8);
cmpi_8!(cmpi_8_ix, ay_ix_8,  8+10);
cmpi_8!(cmpi_8_aw, aw_8,     8+8);
cmpi_8!(cmpi_8_al, al_8,     8+12);
// cmpi_8!(cmpi_8_pcdi, pcdi_8, 8+8);  not present on 68000
// cmpi_8!(cmpi_8_pcix, pcix_8, 8+10); not present on 68000
// cmpi_8!(..., imm) not present

cmpi_16!(cmpi_16_dn, dy,           8+0);
// cmpi_16!(..., ay) not present
cmpi_16!(cmpi_16_ai, ay_ai_16,  8+4);
cmpi_16!(cmpi_16_pi, ay_pi_16,  8+4);
cmpi_16!(cmpi_16_pd, ay_pd_16,  8+6);
cmpi_16!(cmpi_16_di, ay_di_16,  8+8);
cmpi_16!(cmpi_16_ix, ay_ix_16,  8+10);
cmpi_16!(cmpi_16_aw, aw_16,     8+8);
cmpi_16!(cmpi_16_al, al_16,     8+12);
// cmpi_16!(cmpi_16_pcdi, pcdi_16, 8+8);  not present on 68000
// cmpi_16!(cmpi_16_pcix, pcix_16, 8+10); not present on 68000
// cmpi_16!(..., imm) not present

cmpi_32!(cmpi_32_dn, dy,           14+0);
// cmpi_32!(..., ay) not present
cmpi_32!(cmpi_32_ai, ay_ai_32,  12+8);
cmpi_32!(cmpi_32_pi, ay_pi_32,  12+8);
cmpi_32!(cmpi_32_pd, ay_pd_32,  12+10);
cmpi_32!(cmpi_32_di, ay_di_32,  12+12);
cmpi_32!(cmpi_32_ix, ay_ix_32,  12+14);
cmpi_32!(cmpi_32_aw, aw_32,     12+12);
cmpi_32!(cmpi_32_al, al_32,     12+16);
// cmpi_32!(cmpi_32_pcdi, pcdi_32, 12+12); not present on 68000
// cmpi_32!(cmpi_32_pcix, pcix_32, 12+14); not present on 68000
// cmpi_32!(..., imm) not present
//

impl_op!(-, cmp_8,  cmpm_8, ay_pi_8, ax_pi_8, 12);
impl_op!(-, cmp_16, cmpm_16, ay_pi_16, ax_pi_16, 12);
impl_op!(-, cmp_32, cmpm_32, ay_pi_32, ax_pi_32, 20);

// Put implementation of DBcc ops here
branch!(16, dbt_16,  cond_t,  dy);
branch!(16, dbf_16,  cond_f,  dy);
branch!(16, dbhi_16, cond_hi, dy);
branch!(16, dbls_16, cond_ls, dy);
branch!(16, dbcc_16, cond_cc, dy);
branch!(16, dbcs_16, cond_cs, dy);
branch!(16, dbne_16, cond_ne, dy);
branch!(16, dbeq_16, cond_eq, dy);
branch!(16, dbvc_16, cond_vc, dy);
branch!(16, dbvs_16, cond_vs, dy);
branch!(16, dbpl_16, cond_pl, dy);
branch!(16, dbmi_16, cond_mi, dy);
branch!(16, dbge_16, cond_ge, dy);
branch!(16, dblt_16, cond_lt, dy);
branch!(16, dbgt_16, cond_gt, dy);
branch!(16, dble_16, cond_le, dy);

// Put implementation of DIVS ops here
macro_rules! div_op {
    ($common:ident, $srctype:ty, $name:ident, $src:ident, $base_cycles:expr, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            // as opposed to ADDA, we execute src op first
            // even though the PI/PD addressing modes will change AX (if AX=AY)
            let src = try!(operator::$src(core)) as $srctype;
            let dst = dx!(core);
            if src != 0 {
                common::$common(core, dst, src);
                Ok(Cycles($cycles))
            } else {
                // 38 cycles for the ZERO_DIVIDE trap + EA calculation time
                // deduct the base cycles for the instruction, to extract EA cycles.
                Err(Trap(EXCEPTION_ZERO_DIVIDE, 38 + ($cycles - $base_cycles)))
            }
        })
}
macro_rules! divs {
    ($name:ident, $src:ident, $cycles:expr) => (div_op!(divs_16, i16, $name, $src, 158, $cycles);)
}
macro_rules! divu {
    ($name:ident, $src:ident, $cycles:expr) => (div_op!(divu_16, u16, $name, $src, 140, $cycles);)
}

divs!(divs_16_dn, dy, 158+0);
// divs_16_an not present
divs!(divs_16_ai, ay_ai_16,  158+4);
divs!(divs_16_pi, ay_pi_16,  158+4);
divs!(divs_16_pd, ay_pd_16,  158+6);
divs!(divs_16_di, ay_di_16,  158+8);
divs!(divs_16_ix, ay_ix_16,  158+10);
divs!(divs_16_aw, aw_16,     158+8);
divs!(divs_16_al, al_16,     158+12);
divs!(divs_16_pcdi, pcdi_16, 158+8);
divs!(divs_16_pcix, pcix_16, 158+10);
divs!(divs_16_imm, imm_16,   158+4);

// Put implementation of DIVU ops here
divu!(divu_16_dn, dy, 140+0);
// divu_16_an not present
divu!(divu_16_ai, ay_ai_16,  140+4);
divu!(divu_16_pi, ay_pi_16,  140+4);
divu!(divu_16_pd, ay_pd_16,  140+6);
divu!(divu_16_di, ay_di_16,  140+8);
divu!(divu_16_ix, ay_ix_16,  140+10);
divu!(divu_16_aw, aw_16,     140+8);
divu!(divu_16_al, al_16,     140+12);
divu!(divu_16_pcdi, pcdi_16, 140+8);
divu!(divu_16_pcix, pcix_16, 140+10);
divu!(divu_16_imm, imm_16,   140+4);

// Put implementation of EOR, EORI, EORI to CCR and EORI to SR ops here
macro_rules! eor_8 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(8, eor_8, $name, dx, $dst, $cycles);)
}
macro_rules! eor_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(16, eor_16, $name, dx, $dst, $cycles);)
}
macro_rules! eor_32 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(32, eor_32, $name, dx, $dst, $cycles);)
}

eor_8!(eor_8_dn, dy,  4);
// eor_8!(..., ay) not present
eor_8!(eor_8_ai, ea_ay_ai_8, 8+4);
eor_8!(eor_8_pi, ea_ay_pi_8, 8+4);
eor_8!(eor_8_pd, ea_ay_pd_8, 8+6);
eor_8!(eor_8_di, ea_ay_di_8, 8+8);
eor_8!(eor_8_ix, ea_ay_ix_8, 8+10);
eor_8!(eor_8_aw, ea_aw_8,    8+8);
eor_8!(eor_8_al, ea_al_8,    8+12);
// eor_8!(..., pcdi) not present
// eor_8!(..., pcix) not present
// eor_8!(..., imm) not present

eor_16!(eor_16_dn, dy,  4);
// eor_16!(..., ay) not present
eor_16!(eor_16_ai, ea_ay_ai_16,  8+4);
eor_16!(eor_16_pi, ea_ay_pi_16,  8+4);
eor_16!(eor_16_pd, ea_ay_pd_16,  8+6);
eor_16!(eor_16_di, ea_ay_di_16,  8+8);
eor_16!(eor_16_ix, ea_ay_ix_16,  8+10);
eor_16!(eor_16_aw, ea_aw_16,     8+8);
eor_16!(eor_16_al, ea_al_16,     8+12);
// eor_16!(..., pcdi) not present
// eor_16!(..., pcix) not present
// eor_16!(..., imm) not present

eor_32!(eor_32_dn, dy,  8);
// eor_32!(..., ay) not present
eor_32!(eor_32_ai, ea_ay_ai_32,  12+8);
eor_32!(eor_32_pi, ea_ay_pi_32,  12+8);
eor_32!(eor_32_pd, ea_ay_pd_32,  12+10);
eor_32!(eor_32_di, ea_ay_di_32,  12+12);
eor_32!(eor_32_ix, ea_ay_ix_32,  12+14);
eor_32!(eor_32_aw, ea_aw_32,     12+12);
eor_32!(eor_32_al, ea_al_32,     12+16);
// eor_32!(..., pcdi) not present
// eor_32!(..., pcix) not present
// eor_32!(..., imm) not present

macro_rules! eori_8 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(8, eor_8, $name, imm_8, $dst, $cycles);)
}
macro_rules! eori_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(16, eor_16, $name, imm_16, $dst, $cycles);)
}
macro_rules! eori_32 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(32, eor_32, $name, imm_32, $dst, $cycles);)
}
eori_8!(eori_8_dn, dy,  8);
// eori_8_re!(..., ay) not present
eori_8!(eori_8_ai, ea_ay_ai_8,  12+4);
eori_8!(eori_8_pi, ea_ay_pi_8,  12+4);
eori_8!(eori_8_pd, ea_ay_pd_8,  12+6);
eori_8!(eori_8_di, ea_ay_di_8,  12+8);
eori_8!(eori_8_ix, ea_ay_ix_8,  12+10);
eori_8!(eori_8_aw, ea_aw_8,     12+8);
eori_8!(eori_8_al, ea_al_8,     12+12);
// eori_8!(..., pcdi) not present
// eori_8!(..., pcix) not present
// eori_8!(..., imm) not present

eori_16!(eori_16_dn, dy,  8);
// eori_16_re!(..., ay) not present
eori_16!(eori_16_ai, ea_ay_ai_16,  12+4);
eori_16!(eori_16_pi, ea_ay_pi_16,  12+4);
eori_16!(eori_16_pd, ea_ay_pd_16,  12+6);
eori_16!(eori_16_di, ea_ay_di_16,  12+8);
eori_16!(eori_16_ix, ea_ay_ix_16,  12+10);
eori_16!(eori_16_aw, ea_aw_16,     12+8);
eori_16!(eori_16_al, ea_al_16,     12+12);
// eori_16!(..., pcdi) not present
// eori_16!(..., pcix) not present
// eori_16!(..., imm) not present

eori_32!(eori_32_dn, dy,  16);
// eori_32_re!(..., ay) not present
eori_32!(eori_32_ai, ea_ay_ai_32,  20+8);
eori_32!(eori_32_pi, ea_ay_pi_32,  20+8);
eori_32!(eori_32_pd, ea_ay_pd_32,  20+10);
eori_32!(eori_32_di, ea_ay_di_32,  20+12);
eori_32!(eori_32_ix, ea_ay_ix_32,  20+14);
eori_32!(eori_32_aw, ea_aw_32,     20+12);
eori_32!(eori_32_al, ea_al_32,     20+16);
// eori_32!(..., pcdi) not present
// eori_32!(..., pcix) not present
// eori_32!(..., imm) not present

pub fn eori_8_toc<T: Core>(core: &mut T) -> Result<Cycles> {
    let dst = core.condition_code_register();
    let src = mask_out_above_8!(try!(operator::imm_8(core))) as u16;
    core.ccr_to_flags(dst ^ src);
    Ok(Cycles(20))
}
pub fn eori_16_tos<T: Core>(core: &mut T) -> Result<Cycles> {
    if s_flag!(core) != 0 {
        let dst = core.status_register();
        let src = try!(operator::imm_16(core)) as u16;
        core.sr_to_flags(dst ^ src);
        Ok(Cycles(20))
    } else {
        Err(PrivilegeViolation(ir!(core), pc!(core).wrapping_sub(2)))
    }
}

// Put implementation of EXG ops here
pub fn exg_32_dd<T: Core>(core: &mut T) -> Result<Cycles> {
    let x = ir_dx!(core);
    let y = ir_dy!(core);
    dar!(core).swap(x, y);
    Ok(Cycles(6))
}
pub fn exg_32_aa<T: Core>(core: &mut T) -> Result<Cycles> {
    let x = ir_ax!(core);
    let y = ir_ay!(core);
    dar!(core).swap(x, y);
    Ok(Cycles(6))
}
pub fn exg_32_da<T: Core>(core: &mut T) -> Result<Cycles> {
    let x = ir_dx!(core);
    let y = ir_ay!(core);
    dar!(core).swap(x, y);
    Ok(Cycles(6))
}

// Put implementation of EXT ops here
pub fn ext_bw<T: Core>(core: &mut T) -> Result<Cycles> {
    let dst = dy!(core);
    let res = mask_out_above_8!(dst) | if (dst & 0x80) > 0 {0xff00} else {0};
    dy!(core) = mask_out_below_16!(dy!(core)) | res;

    n_flag!(core) = res >> 8;
    v_flag!(core) = 0;
    c_flag!(core) = 0;
    not_z_flag!(core) = res;

    Ok(Cycles(4))
}
pub fn ext_wl<T: Core>(core: &mut T) -> Result<Cycles> {
    let dst = dy!(core);
    let res = mask_out_above_16!(dst) | if (dst & 0x8000) > 0 {0xffff_0000} else {0};
    dy!(core) = res;

    n_flag!(core) = res >> 24;
    v_flag!(core) = 0;
    c_flag!(core) = 0;
    not_z_flag!(core) = res;
    Ok(Cycles(4))
}

// Put implementation of ILLEGAL op here

// We differ between the real illegal instruction, and the default case
// at least for now, as it is useful to be able to handle "unintended
// use of possibly unimplemented instruction" differently from actually
// wanting this to happen
pub fn real_illegal<T: Core>(core: &mut T) -> Result<Cycles> {
    Err(IllegalInstruction(ir!(core), pc!(core).wrapping_sub(2)))
}

// Put implementation of JMP ops here
macro_rules! jump {
    ($name:ident, $dst:ident, $push:expr, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let ea = try!(effective_address::$dst(core));
            // using a constant expression will optimize this check away
            if $push {
                let pc = pc!(core);
                core.push_32(pc);
            }
            core.jump(ea);
            Ok(Cycles($cycles))
        })
}
// TODO: Musashi sometimes uses extra cycles, due to special casing when
// the instruction jumps back on itself
jump!(jmp_32_ai, address_indirect_ay, false, 8);
jump!(jmp_32_di, displacement_ay, false, 10);
jump!(jmp_32_ix, index_ay, false, 14); // TODO: Musashi uses 12
jump!(jmp_32_aw, absolute_word, false, 10);
jump!(jmp_32_al, absolute_long, false, 12);
jump!(jmp_32_pcdi, displacement_pc, false, 10);
jump!(jmp_32_pcix, index_pc, false, 14);

// Put implementation of JSR ops here
jump!(jsr_32_ai, address_indirect_ay, true, 16);
jump!(jsr_32_di, displacement_ay, true, 18);
jump!(jsr_32_ix, index_ay, true, 22);
jump!(jsr_32_aw, absolute_word, true, 18);
jump!(jsr_32_al, absolute_long, true, 20);
jump!(jsr_32_pcdi, displacement_pc, true, 18);
jump!(jsr_32_pcix, index_pc, true, 22);

// Put implementation of LEA ops here
macro_rules! lea {
    ($name:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let ea = try!(effective_address::$dst(core));
            ax!(core) = ea;
            Ok(Cycles($cycles))
        })
}
lea!(lea_32_ai, address_indirect_ay, 4);
lea!(lea_32_di, displacement_ay, 8);
lea!(lea_32_ix, index_ay, 12);
lea!(lea_32_aw, absolute_word, 8);
lea!(lea_32_al, absolute_long, 12);
lea!(lea_32_pcdi, displacement_pc, 8);
lea!(lea_32_pcix, index_pc, 12);

// Put implementation of LINK ops here
pub fn link_16<T: Core>(core: &mut T) -> Result<Cycles> {
    let sp = if ir_ay!(core) == super::STACK_POINTER_REG {
        core.push_sp()
    } else {
        let ay = ay!(core);
        core.push_32(ay)
    };
    ay!(core) = sp;
    sp!(core) = try!(effective_address::displacement(core, sp));
    Ok(Cycles(16))
}

// Put implementation of LSL, LSR ops here
macro_rules! lsr_8 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(8, lsr_8, $name, $src, $dst, $cycles);)
}
macro_rules! lsr_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(16, lsr_16, $name, 1, $dst, $cycles););
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(16, lsr_16, $name, $src, $dst, $cycles);)
}
macro_rules! lsr_32 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(32, lsr_32, $name, $src, $dst, $cycles);)
}

macro_rules! lsl_8 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(8, lsl_8, $name, $src, $dst, $cycles);)
}
macro_rules! lsl_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(16, lsl_16, $name, 1, $dst, $cycles););
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(16, lsl_16, $name, $src, $dst, $cycles);)
}
macro_rules! lsl_32 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(32, lsl_32, $name, $src, $dst, $cycles);)
}

lsr_8!(lsr_8_s,   quick, dy, 6);
lsr_16!(lsr_16_s, quick, dy, 6);
lsr_32!(lsr_32_s, quick, dy, 8);
lsr_8!(lsr_8_r,   dx,    dy, 6);
lsr_16!(lsr_16_r, dx,    dy, 6);
lsr_32!(lsr_32_r, dx,    dy, 8);

lsl_8!(lsl_8_s,   quick, dy, 6);
lsl_16!(lsl_16_s, quick, dy, 6);
lsl_32!(lsl_32_s, quick, dy, 8);
lsl_8!(lsl_8_r,   dx,    dy, 6);
lsl_16!(lsl_16_r, dx,    dy, 6);
lsl_32!(lsl_32_r, dx,    dy, 8);

lsl_16!(lsl_16_ai, ea_ay_ai_16, 12);
lsl_16!(lsl_16_pi, ea_ay_pi_16, 12);
lsl_16!(lsl_16_pd, ea_ay_pd_16, 14);
lsl_16!(lsl_16_di, ea_ay_di_16, 16);
lsl_16!(lsl_16_ix, ea_ay_ix_16, 18);
lsl_16!(lsl_16_aw, ea_aw_16,    16);
lsl_16!(lsl_16_al, ea_al_16,    20);

lsr_16!(lsr_16_ai, ea_ay_ai_16, 12);
lsr_16!(lsr_16_pi, ea_ay_pi_16, 12);
lsr_16!(lsr_16_pd, ea_ay_pd_16, 14);
lsr_16!(lsr_16_di, ea_ay_di_16, 16);
lsr_16!(lsr_16_ix, ea_ay_ix_16, 18);
lsr_16!(lsr_16_aw, ea_aw_16,    16);
lsr_16!(lsr_16_al, ea_al_16,    20);

// Put implementation of MOVE ops here
macro_rules! impl_move {
    (8, $name:ident, dx, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = mask_out_above_8!(try!(operator::$src(core)));
            dx!(core) = mask_out_below_8!(dx!(core)) | src;
            common::move_flags(core, src, 0);
            Ok(Cycles($cycles))
        });
    (8, $name:ident, $dst:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = mask_out_above_8!(try!(operator::$src(core)));
            let ea = try!(effective_address::$dst(core));
            try!(core.write_data_byte(ea, src));
            common::move_flags(core, src, 0);
            Ok(Cycles($cycles))
        });
    (16, $name:ident, dx, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = mask_out_above_16!(try!(operator::$src(core)));
            dx!(core) = mask_out_below_16!(dx!(core)) | src;
            common::move_flags(core, src, 8);
            Ok(Cycles($cycles))
        });
    (16, $name:ident, $dst:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = mask_out_above_16!(try!(operator::$src(core)));
            let ea = try!(effective_address::$dst(core));
            try!(core.write_data_word(ea, src));
            common::move_flags(core, src, 8);
            Ok(Cycles($cycles))
        });
    (32, $name:ident, dx, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            dx!(core) = src;
            common::move_flags(core, src, 24);
            Ok(Cycles($cycles))
        });
    (32, $name:ident, $dst:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let ea = try!(effective_address::$dst(core));
            try!(core.write_data_long(ea, src));
            common::move_flags(core, src, 24);
            Ok(Cycles($cycles))
        });
}
// move_8_<dest>_<src>
impl_move!(8, move_8_dn_dn, dx, dy, 4);
impl_move!(8, move_8_ai_dn, address_indirect_ax, dy, 8);
impl_move!(8, move_8_pi_dn, postincrement_ax_8, dy, 8);
impl_move!(8, move_8_pd_dn, predecrement_ax_8, dy, 8);
impl_move!(8, move_8_di_dn, displacement_ax, dy, 12);
impl_move!(8, move_8_ix_dn, index_ax, dy, 14);
impl_move!(8, move_8_aw_dn, absolute_word, dy, 12);
impl_move!(8, move_8_al_dn, absolute_long, dy, 16);

impl_move!(8, move_8_dn_ai, dx, ay_ai_8, 4+4);
impl_move!(8, move_8_ai_ai, address_indirect_ax, ay_ai_8, 8+4);
impl_move!(8, move_8_pi_ai, postincrement_ax_8, ay_ai_8, 8+4);
impl_move!(8, move_8_pd_ai, predecrement_ax_8, ay_ai_8, 8+4);
impl_move!(8, move_8_di_ai, displacement_ax, ay_ai_8, 12+4);
impl_move!(8, move_8_ix_ai, index_ax, ay_ai_8, 14+4);
impl_move!(8, move_8_aw_ai, absolute_word, ay_ai_8, 12+4);
impl_move!(8, move_8_al_ai, absolute_long, ay_ai_8, 16+4);

impl_move!(8, move_8_dn_pi, dx, ay_pi_8, 4+4);
impl_move!(8, move_8_ai_pi, address_indirect_ax, ay_pi_8, 8+4);
impl_move!(8, move_8_pi_pi, postincrement_ax_8, ay_pi_8, 8+4);
impl_move!(8, move_8_pd_pi, predecrement_ax_8, ay_pi_8, 8+4);
impl_move!(8, move_8_di_pi, displacement_ax, ay_pi_8, 12+4);
impl_move!(8, move_8_ix_pi, index_ax, ay_pi_8, 14+4);
impl_move!(8, move_8_aw_pi, absolute_word, ay_pi_8, 12+4);
impl_move!(8, move_8_al_pi, absolute_long, ay_pi_8, 16+4);

impl_move!(8, move_8_dn_pd, dx, ay_pd_8, 4+6);
impl_move!(8, move_8_ai_pd, address_indirect_ax, ay_pd_8, 8+6);
impl_move!(8, move_8_pi_pd, postincrement_ax_8, ay_pd_8, 8+6);
impl_move!(8, move_8_pd_pd, predecrement_ax_8, ay_pd_8, 8+6);
impl_move!(8, move_8_di_pd, displacement_ax, ay_pd_8, 12+6);
impl_move!(8, move_8_ix_pd, index_ax, ay_pd_8, 14+6);
impl_move!(8, move_8_aw_pd, absolute_word, ay_pd_8, 12+6);
impl_move!(8, move_8_al_pd, absolute_long, ay_pd_8, 16+6);

impl_move!(8, move_8_dn_di, dx, ay_di_8, 4+8);
impl_move!(8, move_8_ai_di, address_indirect_ax, ay_di_8, 8+8);
impl_move!(8, move_8_pi_di, postincrement_ax_8, ay_di_8, 8+8);
impl_move!(8, move_8_pd_di, predecrement_ax_8, ay_di_8, 8+8);
impl_move!(8, move_8_di_di, displacement_ax, ay_di_8, 12+8);
impl_move!(8, move_8_ix_di, index_ax, ay_di_8, 14+8);
impl_move!(8, move_8_aw_di, absolute_word, ay_di_8, 12+8);
impl_move!(8, move_8_al_di, absolute_long, ay_di_8, 16+8);

impl_move!(8, move_8_dn_ix, dx, ay_ix_8, 4+10);
impl_move!(8, move_8_ai_ix, address_indirect_ax, ay_ix_8, 8+10);
impl_move!(8, move_8_pi_ix, postincrement_ax_8, ay_ix_8, 8+10);
impl_move!(8, move_8_pd_ix, predecrement_ax_8, ay_ix_8, 8+10);
impl_move!(8, move_8_di_ix, displacement_ax, ay_ix_8, 12+10);
impl_move!(8, move_8_ix_ix, index_ax, ay_ix_8, 14+10);
impl_move!(8, move_8_aw_ix, absolute_word, ay_ix_8, 12+10);
impl_move!(8, move_8_al_ix, absolute_long, ay_ix_8, 16+10);

impl_move!(8, move_8_dn_aw, dx, aw_8, 4+8);
impl_move!(8, move_8_ai_aw, address_indirect_ax, aw_8, 8+8);
impl_move!(8, move_8_pi_aw, postincrement_ax_8, aw_8, 8+8);
impl_move!(8, move_8_pd_aw, predecrement_ax_8, aw_8, 8+8);
impl_move!(8, move_8_di_aw, displacement_ax, aw_8, 12+8);
impl_move!(8, move_8_ix_aw, index_ax, aw_8, 14+8);
impl_move!(8, move_8_aw_aw, absolute_word, aw_8, 12+8);
impl_move!(8, move_8_al_aw, absolute_long, aw_8, 16+8);

impl_move!(8, move_8_dn_al, dx, al_8, 4+12);
impl_move!(8, move_8_ai_al, address_indirect_ax, al_8, 8+12);
impl_move!(8, move_8_pi_al, postincrement_ax_8, al_8, 8+12);
impl_move!(8, move_8_pd_al, predecrement_ax_8, al_8, 8+12);
impl_move!(8, move_8_di_al, displacement_ax, al_8, 12+12);
impl_move!(8, move_8_ix_al, index_ax, al_8, 14+12);
impl_move!(8, move_8_aw_al, absolute_word, al_8, 12+12);
impl_move!(8, move_8_al_al, absolute_long, al_8, 16+12);

impl_move!(8, move_8_dn_pcdi, dx, pcdi_8, 4+8);
impl_move!(8, move_8_ai_pcdi, address_indirect_ax, pcdi_8, 8+8);
impl_move!(8, move_8_pi_pcdi, postincrement_ax_8, pcdi_8, 8+8);
impl_move!(8, move_8_pd_pcdi, predecrement_ax_8, pcdi_8, 8+8);
impl_move!(8, move_8_di_pcdi, displacement_ax, pcdi_8, 12+8);
impl_move!(8, move_8_ix_pcdi, index_ax, pcdi_8, 14+8);
impl_move!(8, move_8_aw_pcdi, absolute_word, pcdi_8, 12+8);
impl_move!(8, move_8_al_pcdi, absolute_long, pcdi_8, 16+8);

impl_move!(8, move_8_dn_pcix, dx, pcix_8, 4+10);
impl_move!(8, move_8_ai_pcix, address_indirect_ax, pcix_8, 8+10);
impl_move!(8, move_8_pi_pcix, postincrement_ax_8, pcix_8, 8+10);
impl_move!(8, move_8_pd_pcix, predecrement_ax_8, pcix_8, 8+10);
impl_move!(8, move_8_di_pcix, displacement_ax, pcix_8, 12+10);
impl_move!(8, move_8_ix_pcix, index_ax, pcix_8, 14+10);
impl_move!(8, move_8_aw_pcix, absolute_word, pcix_8, 12+10);
impl_move!(8, move_8_al_pcix, absolute_long, pcix_8, 16+10);

impl_move!(8, move_8_dn_imm, dx, imm_8, 4+4);
impl_move!(8, move_8_ai_imm, address_indirect_ax, imm_8, 8+4);
impl_move!(8, move_8_pi_imm, postincrement_ax_8, imm_8, 8+4);
impl_move!(8, move_8_pd_imm, predecrement_ax_8, imm_8, 8+4);
impl_move!(8, move_8_di_imm, displacement_ax, imm_8, 12+4);
impl_move!(8, move_8_ix_imm, index_ax, imm_8, 14+4);
impl_move!(8, move_8_aw_imm, absolute_word, imm_8, 12+4);
impl_move!(8, move_8_al_imm, absolute_long, imm_8, 16+4);

impl_move!(16, move_16_dn_dn, dx, dy, 4);
impl_move!(16, move_16_ai_dn, address_indirect_ax, dy, 8);
impl_move!(16, move_16_pi_dn, postincrement_ax_16, dy, 8);
impl_move!(16, move_16_pd_dn, predecrement_ax_16, dy, 8);
impl_move!(16, move_16_di_dn, displacement_ax, dy, 12);
impl_move!(16, move_16_ix_dn, index_ax, dy, 14);
impl_move!(16, move_16_aw_dn, absolute_word, dy, 12);
impl_move!(16, move_16_al_dn, absolute_long, dy, 16);

impl_move!(16, move_16_dn_an, dx, ay, 4);
impl_move!(16, move_16_ai_an, address_indirect_ax, ay, 8);
impl_move!(16, move_16_pi_an, postincrement_ax_16, ay, 8);
impl_move!(16, move_16_pd_an, predecrement_ax_16, ay, 8);
impl_move!(16, move_16_di_an, displacement_ax, ay, 12);
impl_move!(16, move_16_ix_an, index_ax, ay, 14);
impl_move!(16, move_16_aw_an, absolute_word, ay, 12);
impl_move!(16, move_16_al_an, absolute_long, ay, 16);

impl_move!(16, move_16_dn_ai, dx, ay_ai_16, 4+4);
impl_move!(16, move_16_ai_ai, address_indirect_ax, ay_ai_16, 8+4);
impl_move!(16, move_16_pi_ai, postincrement_ax_16, ay_ai_16, 8+4);
impl_move!(16, move_16_pd_ai, predecrement_ax_16, ay_ai_16, 8+4);
impl_move!(16, move_16_di_ai, displacement_ax, ay_ai_16, 12+4);
impl_move!(16, move_16_ix_ai, index_ax, ay_ai_16, 14+4);
impl_move!(16, move_16_aw_ai, absolute_word, ay_ai_16, 12+4);
impl_move!(16, move_16_al_ai, absolute_long, ay_ai_16, 16+4);

impl_move!(16, move_16_dn_pi, dx, ay_pi_16, 4+4);
impl_move!(16, move_16_ai_pi, address_indirect_ax, ay_pi_16, 8+4);
impl_move!(16, move_16_pi_pi, postincrement_ax_16, ay_pi_16, 8+4);
impl_move!(16, move_16_pd_pi, predecrement_ax_16, ay_pi_16, 8+4);
impl_move!(16, move_16_di_pi, displacement_ax, ay_pi_16, 12+4);
impl_move!(16, move_16_ix_pi, index_ax, ay_pi_16, 14+4);
impl_move!(16, move_16_aw_pi, absolute_word, ay_pi_16, 12+4);
impl_move!(16, move_16_al_pi, absolute_long, ay_pi_16, 16+4);

impl_move!(16, move_16_dn_pd, dx, ay_pd_16, 4+6);
impl_move!(16, move_16_ai_pd, address_indirect_ax, ay_pd_16, 8+6);
impl_move!(16, move_16_pi_pd, postincrement_ax_16, ay_pd_16, 8+6);
impl_move!(16, move_16_pd_pd, predecrement_ax_16, ay_pd_16, 8+6);
impl_move!(16, move_16_di_pd, displacement_ax, ay_pd_16, 12+6);
impl_move!(16, move_16_ix_pd, index_ax, ay_pd_16, 14+6);
impl_move!(16, move_16_aw_pd, absolute_word, ay_pd_16, 12+6);
impl_move!(16, move_16_al_pd, absolute_long, ay_pd_16, 16+6);

impl_move!(16, move_16_dn_di, dx, ay_di_16, 4+8);
impl_move!(16, move_16_ai_di, address_indirect_ax, ay_di_16, 8+8);
impl_move!(16, move_16_pi_di, postincrement_ax_16, ay_di_16, 8+8);
impl_move!(16, move_16_pd_di, predecrement_ax_16, ay_di_16, 8+8);
impl_move!(16, move_16_di_di, displacement_ax, ay_di_16, 12+8);
impl_move!(16, move_16_ix_di, index_ax, ay_di_16, 14+8);
impl_move!(16, move_16_aw_di, absolute_word, ay_di_16, 12+8);
impl_move!(16, move_16_al_di, absolute_long, ay_di_16, 16+8);

impl_move!(16, move_16_dn_ix, dx, ay_ix_16, 4+10);
impl_move!(16, move_16_ai_ix, address_indirect_ax, ay_ix_16, 8+10);
impl_move!(16, move_16_pi_ix, postincrement_ax_16, ay_ix_16, 8+10);
impl_move!(16, move_16_pd_ix, predecrement_ax_16, ay_ix_16, 8+10);
impl_move!(16, move_16_di_ix, displacement_ax, ay_ix_16, 12+10);
impl_move!(16, move_16_ix_ix, index_ax, ay_ix_16, 14+10);
impl_move!(16, move_16_aw_ix, absolute_word, ay_ix_16, 12+10);
impl_move!(16, move_16_al_ix, absolute_long, ay_ix_16, 16+10);

impl_move!(16, move_16_dn_aw, dx, aw_16, 4+8);
impl_move!(16, move_16_ai_aw, address_indirect_ax, aw_16, 8+8);
impl_move!(16, move_16_pi_aw, postincrement_ax_16, aw_16, 8+8);
impl_move!(16, move_16_pd_aw, predecrement_ax_16, aw_16, 8+8);
impl_move!(16, move_16_di_aw, displacement_ax, aw_16, 12+8);
impl_move!(16, move_16_ix_aw, index_ax, aw_16, 14+8);
impl_move!(16, move_16_aw_aw, absolute_word, aw_16, 12+8);
impl_move!(16, move_16_al_aw, absolute_long, aw_16, 16+8);

impl_move!(16, move_16_dn_al, dx, al_16, 4+12);
impl_move!(16, move_16_ai_al, address_indirect_ax, al_16, 8+12);
impl_move!(16, move_16_pi_al, postincrement_ax_16, al_16, 8+12);
impl_move!(16, move_16_pd_al, predecrement_ax_16, al_16, 8+12);
impl_move!(16, move_16_di_al, displacement_ax, al_16, 12+12);
impl_move!(16, move_16_ix_al, index_ax, al_16, 14+12);
impl_move!(16, move_16_aw_al, absolute_word, al_16, 12+12);
impl_move!(16, move_16_al_al, absolute_long, al_16, 16+12);

impl_move!(16, move_16_dn_pcdi, dx, pcdi_16, 4+8);
impl_move!(16, move_16_ai_pcdi, address_indirect_ax, pcdi_16, 8+8);
impl_move!(16, move_16_pi_pcdi, postincrement_ax_16, pcdi_16, 8+8);
impl_move!(16, move_16_pd_pcdi, predecrement_ax_16, pcdi_16, 8+8);
impl_move!(16, move_16_di_pcdi, displacement_ax, pcdi_16, 12+8);
impl_move!(16, move_16_ix_pcdi, index_ax, pcdi_16, 14+8);
impl_move!(16, move_16_aw_pcdi, absolute_word, pcdi_16, 12+8);
impl_move!(16, move_16_al_pcdi, absolute_long, pcdi_16, 16+8);

impl_move!(16, move_16_dn_pcix, dx, pcix_16, 4+10);
impl_move!(16, move_16_ai_pcix, address_indirect_ax, pcix_16, 8+10);
impl_move!(16, move_16_pi_pcix, postincrement_ax_16, pcix_16, 8+10);
impl_move!(16, move_16_pd_pcix, predecrement_ax_16, pcix_16, 8+10);
impl_move!(16, move_16_di_pcix, displacement_ax, pcix_16, 12+10);
impl_move!(16, move_16_ix_pcix, index_ax, pcix_16, 14+10);
impl_move!(16, move_16_aw_pcix, absolute_word, pcix_16, 12+10);
impl_move!(16, move_16_al_pcix, absolute_long, pcix_16, 16+10);

impl_move!(16, move_16_dn_imm, dx, imm_16, 4+4);
impl_move!(16, move_16_ai_imm, address_indirect_ax, imm_16, 8+4);
impl_move!(16, move_16_pi_imm, postincrement_ax_16, imm_16, 8+4);
impl_move!(16, move_16_pd_imm, predecrement_ax_16, imm_16, 8+4);
impl_move!(16, move_16_di_imm, displacement_ax, imm_16, 12+4);
impl_move!(16, move_16_ix_imm, index_ax, imm_16, 14+4);
impl_move!(16, move_16_aw_imm, absolute_word, imm_16, 12+4);
impl_move!(16, move_16_al_imm, absolute_long, imm_16, 16+4);

impl_move!(32, move_32_dn_dn, dx, dy, 4);
impl_move!(32, move_32_ai_dn, address_indirect_ax, dy, 12);
impl_move!(32, move_32_pi_dn, postincrement_ax_32, dy, 12);
impl_move!(32, move_32_pd_dn, predecrement_ax_32, dy, 12);
impl_move!(32, move_32_di_dn, displacement_ax, dy, 16);
impl_move!(32, move_32_ix_dn, index_ax, dy, 18);
impl_move!(32, move_32_aw_dn, absolute_word, dy, 16);
impl_move!(32, move_32_al_dn, absolute_long, dy, 20);

impl_move!(32, move_32_dn_an, dx, ay, 4);
impl_move!(32, move_32_ai_an, address_indirect_ax, ay, 12);
impl_move!(32, move_32_pi_an, postincrement_ax_32, ay, 12);
impl_move!(32, move_32_pd_an, predecrement_ax_32, ay, 12);
impl_move!(32, move_32_di_an, displacement_ax, ay, 16);
impl_move!(32, move_32_ix_an, index_ax, ay, 18);
impl_move!(32, move_32_aw_an, absolute_word, ay, 16);
impl_move!(32, move_32_al_an, absolute_long, ay, 20);

impl_move!(32, move_32_dn_ai, dx, ay_ai_32, 4+8);
impl_move!(32, move_32_ai_ai, address_indirect_ax, ay_ai_32, 12+8);
impl_move!(32, move_32_pi_ai, postincrement_ax_32, ay_ai_32, 12+8);
impl_move!(32, move_32_pd_ai, predecrement_ax_32, ay_ai_32, 12+8);
impl_move!(32, move_32_di_ai, displacement_ax, ay_ai_32, 16+8);
impl_move!(32, move_32_ix_ai, index_ax, ay_ai_32, 18+8);
impl_move!(32, move_32_aw_ai, absolute_word, ay_ai_32, 16+8);
impl_move!(32, move_32_al_ai, absolute_long, ay_ai_32, 20+8);

impl_move!(32, move_32_dn_pi, dx, ay_pi_32, 4+8);
impl_move!(32, move_32_ai_pi, address_indirect_ax, ay_pi_32, 12+8);
impl_move!(32, move_32_pi_pi, postincrement_ax_32, ay_pi_32, 12+8);
impl_move!(32, move_32_pd_pi, predecrement_ax_32, ay_pi_32, 12+8);
impl_move!(32, move_32_di_pi, displacement_ax, ay_pi_32, 16+8);
impl_move!(32, move_32_ix_pi, index_ax, ay_pi_32, 18+8);
impl_move!(32, move_32_aw_pi, absolute_word, ay_pi_32, 16+8);
impl_move!(32, move_32_al_pi, absolute_long, ay_pi_32, 20+8);

impl_move!(32, move_32_dn_pd, dx, ay_pd_32, 4+10);
impl_move!(32, move_32_ai_pd, address_indirect_ax, ay_pd_32, 12+10);
impl_move!(32, move_32_pi_pd, postincrement_ax_32, ay_pd_32, 12+10);
impl_move!(32, move_32_pd_pd, predecrement_ax_32, ay_pd_32, 12+10);
impl_move!(32, move_32_di_pd, displacement_ax, ay_pd_32, 16+10);
impl_move!(32, move_32_ix_pd, index_ax, ay_pd_32, 18+10);
impl_move!(32, move_32_aw_pd, absolute_word, ay_pd_32, 16+10);
impl_move!(32, move_32_al_pd, absolute_long, ay_pd_32, 20+10);

impl_move!(32, move_32_dn_di, dx, ay_di_32, 4+12);
impl_move!(32, move_32_ai_di, address_indirect_ax, ay_di_32, 12+12);
impl_move!(32, move_32_pi_di, postincrement_ax_32, ay_di_32, 12+12);
impl_move!(32, move_32_pd_di, predecrement_ax_32, ay_di_32, 12+12);
impl_move!(32, move_32_di_di, displacement_ax, ay_di_32, 16+12);
impl_move!(32, move_32_ix_di, index_ax, ay_di_32, 18+12);
impl_move!(32, move_32_aw_di, absolute_word, ay_di_32, 16+12);
impl_move!(32, move_32_al_di, absolute_long, ay_di_32, 20+12);

impl_move!(32, move_32_dn_ix, dx, ay_ix_32, 4+14);
impl_move!(32, move_32_ai_ix, address_indirect_ax, ay_ix_32, 12+14);
impl_move!(32, move_32_pi_ix, postincrement_ax_32, ay_ix_32, 12+14);
impl_move!(32, move_32_pd_ix, predecrement_ax_32, ay_ix_32, 12+14);
impl_move!(32, move_32_di_ix, displacement_ax, ay_ix_32, 16+14);
impl_move!(32, move_32_ix_ix, index_ax, ay_ix_32, 18+14);
impl_move!(32, move_32_aw_ix, absolute_word, ay_ix_32, 16+14);
impl_move!(32, move_32_al_ix, absolute_long, ay_ix_32, 20+14);

impl_move!(32, move_32_dn_aw, dx, aw_32, 4+12);
impl_move!(32, move_32_ai_aw, address_indirect_ax, aw_32, 12+12);
impl_move!(32, move_32_pi_aw, postincrement_ax_32, aw_32, 12+12);
impl_move!(32, move_32_pd_aw, predecrement_ax_32, aw_32, 12+12);
impl_move!(32, move_32_di_aw, displacement_ax, aw_32, 16+12);
impl_move!(32, move_32_ix_aw, index_ax, aw_32, 18+12);
impl_move!(32, move_32_aw_aw, absolute_word, aw_32, 16+12);
impl_move!(32, move_32_al_aw, absolute_long, aw_32, 20+12);

impl_move!(32, move_32_dn_al, dx, al_32, 4+16);
impl_move!(32, move_32_ai_al, address_indirect_ax, al_32, 12+16);
impl_move!(32, move_32_pi_al, postincrement_ax_32, al_32, 12+16);
impl_move!(32, move_32_pd_al, predecrement_ax_32, al_32, 12+16);
impl_move!(32, move_32_di_al, displacement_ax, al_32, 16+16);
impl_move!(32, move_32_ix_al, index_ax, al_32, 18+16);
impl_move!(32, move_32_aw_al, absolute_word, al_32, 16+16);
impl_move!(32, move_32_al_al, absolute_long, al_32, 20+16);

impl_move!(32, move_32_dn_pcdi, dx, pcdi_32, 4+12);
impl_move!(32, move_32_ai_pcdi, address_indirect_ax, pcdi_32, 12+12);
impl_move!(32, move_32_pi_pcdi, postincrement_ax_32, pcdi_32, 12+12);
impl_move!(32, move_32_pd_pcdi, predecrement_ax_32, pcdi_32, 12+12);
impl_move!(32, move_32_di_pcdi, displacement_ax, pcdi_32, 16+12);
impl_move!(32, move_32_ix_pcdi, index_ax, pcdi_32, 18+12);
impl_move!(32, move_32_aw_pcdi, absolute_word, pcdi_32, 16+12);
impl_move!(32, move_32_al_pcdi, absolute_long, pcdi_32, 20+12);

impl_move!(32, move_32_dn_pcix, dx, pcix_32, 4+14);
impl_move!(32, move_32_ai_pcix, address_indirect_ax, pcix_32, 12+14);
impl_move!(32, move_32_pi_pcix, postincrement_ax_32, pcix_32, 12+14);
impl_move!(32, move_32_pd_pcix, predecrement_ax_32, pcix_32, 12+14);
impl_move!(32, move_32_di_pcix, displacement_ax, pcix_32, 16+14);
impl_move!(32, move_32_ix_pcix, index_ax, pcix_32, 18+14);
impl_move!(32, move_32_aw_pcix, absolute_word, pcix_32, 16+14);
impl_move!(32, move_32_al_pcix, absolute_long, pcix_32, 20+14);

impl_move!(32, move_32_dn_imm, dx, imm_32, 4+8);
impl_move!(32, move_32_ai_imm, address_indirect_ax, imm_32, 12+8);
impl_move!(32, move_32_pi_imm, postincrement_ax_32, imm_32, 12+8);
impl_move!(32, move_32_pd_imm, predecrement_ax_32, imm_32, 12+8);
impl_move!(32, move_32_di_imm, displacement_ax, imm_32, 16+8);
impl_move!(32, move_32_ix_imm, index_ax, imm_32, 18+8);
impl_move!(32, move_32_aw_imm, absolute_word, imm_32, 16+8);
impl_move!(32, move_32_al_imm, absolute_long, imm_32, 20+8);

// Put implementation of MOVEA ops here
macro_rules! movea_16 {
    ($name:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            // we must evaluate AY (src) first
            // as the PI/PD addressing modes will change AX (if AX=AY)
            ax!(core) = try!(operator::$src(core)) as i16 as u32;
            Ok(Cycles($cycles))
        })
}
macro_rules! movea_32 {
    ($name:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            // we must evaluate AY (src) first
            // as the PI/PD addressing modes will change AX (if AX=AY)
            ax!(core) = try!(operator::$src(core));
            Ok(Cycles($cycles))
        })
}
movea_16!(movea_16_dn, dy, 4);
movea_16!(movea_16_an, ay, 4);
movea_16!(movea_16_ai, ay_ai_16, 8);
movea_16!(movea_16_pi, ay_pi_16, 8);
movea_16!(movea_16_pd, ay_pd_16, 10);
movea_16!(movea_16_di, ay_di_16, 12);
movea_16!(movea_16_ix, ay_ix_16, 14);
movea_16!(movea_16_aw, aw_16, 12);
movea_16!(movea_16_al, al_16, 16);
movea_16!(movea_16_pcdi, pcdi_16, 12);
movea_16!(movea_16_pcix, pcix_16, 14);
movea_16!(movea_16_imm, imm_16, 8);

movea_32!(movea_32_dn, dy, 4);
movea_32!(movea_32_an, ay, 4);
movea_32!(movea_32_ai, ay_ai_32, 12);
movea_32!(movea_32_pi, ay_pi_32, 12);
movea_32!(movea_32_pd, ay_pd_32, 14);
movea_32!(movea_32_di, ay_di_32, 16);
movea_32!(movea_32_ix, ay_ix_32, 18);
movea_32!(movea_32_aw, aw_32, 16);
movea_32!(movea_32_al, al_32, 20);
movea_32!(movea_32_pcdi, pcdi_32, 16);
movea_32!(movea_32_pcix, pcix_32, 18);
movea_32!(movea_32_imm, imm_32, 12);

// Put implementation of MOVE to CCR ops here
macro_rules! move_toc {
    ($name:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let ccr = try!(operator::$src(core)) as u16;
            core.ccr_to_flags(ccr);
            Ok(Cycles($cycles))
        })
}
move_toc!(move_16_toc_dn, dy, 12);
move_toc!(move_16_toc_ai, ay_ai_16, 12+4);
move_toc!(move_16_toc_pi, ay_pi_16, 12+4);
move_toc!(move_16_toc_pd, ay_pd_16, 12+6);
move_toc!(move_16_toc_di, ay_di_16, 12+8);
move_toc!(move_16_toc_ix, ay_ix_16, 12+10);
move_toc!(move_16_toc_aw, aw_16, 12+8);
move_toc!(move_16_toc_al, al_16, 12+12);
move_toc!(move_16_toc_pcdi, pcdi_16, 12+8);
move_toc!(move_16_toc_pcix, pcix_16, 12+10);
move_toc!(move_16_toc_imm, imm_16, 12+4);

// Put implementation of MOVE from SR ops here
macro_rules! move_frs {
    ($name:ident, dy, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            dy!(core) = mask_out_below_16!(dy!(core)) | u32::from(core.status_register());
            Ok(Cycles($cycles))
        });
    ($name:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
  // unsigned int ea = ((m68ki_cpu.dar+8)[m68ki_cpu.ir & 7]);
  // m68ki_write_16_fc(ea, m68ki_cpu.s_flag | 1, ( m68ki_cpu.t1_flag | m68ki_cpu.t0_flag | (m68ki_cpu.s_flag << 11) | (m68ki_cpu.m_flag << 11) | m68ki_cpu.int_mask | (((m68ki_cpu.x_flag&0x100) >> 4) | ((m68ki_cpu.n_flag&0x80) >> 4) | ((!m68ki_cpu.not_z_flag) << 2) | ((m68ki_cpu.v_flag&0x80) >> 6) | ((m68ki_cpu.c_flag&0x100) >> 8))));
  // return;
            let sr = core.status_register();
            let ea = try!(effective_address::$src(core));
            try!(core.write_data_word(ea, u32::from(sr)));
            Ok(Cycles($cycles))
        })
}
move_frs!(move_16_frs_dn, dy, 6);
move_frs!(move_16_frs_ai, address_indirect_ay, 8+4);
move_frs!(move_16_frs_pi, postincrement_ay_16, 8+4);
move_frs!(move_16_frs_pd, predecrement_ay_16,  8+6);
move_frs!(move_16_frs_di, displacement_ay,     8+8);
move_frs!(move_16_frs_ix, index_ay,            8+10);
move_frs!(move_16_frs_aw, absolute_word,       8+8);
move_frs!(move_16_frs_al, absolute_long,       8+12);

// Put implementation of MOVE to SR ops here
macro_rules! move_tos {
    ($name:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            if s_flag!(core) != 0 {
                let sr = try!(operator::$src(core)) as u16;
                core.sr_to_flags(sr);
                Ok(Cycles($cycles))
            } else {
                Err(PrivilegeViolation(ir!(core), pc!(core).wrapping_sub(2)))
            }
        })
}
move_tos!(move_16_tos_dn, dy, 12);
move_tos!(move_16_tos_ai, ay_ai_16, 12+4);
move_tos!(move_16_tos_pi, ay_pi_16, 12+4);
move_tos!(move_16_tos_pd, ay_pd_16, 12+6);
move_tos!(move_16_tos_di, ay_di_16, 12+8);
move_tos!(move_16_tos_ix, ay_ix_16, 12+10);
move_tos!(move_16_tos_aw, aw_16, 12+8);
move_tos!(move_16_tos_al, al_16, 12+12);
move_tos!(move_16_tos_pcdi, pcdi_16, 12+8);
move_tos!(move_16_tos_pcix, pcix_16, 12+10);
move_tos!(move_16_tos_imm, imm_16, 12+4);

// Put implementation of MOVE USP ops here
pub fn move_32_tou<T: Core>(core: &mut T) -> Result<Cycles> {
    if s_flag!(core) != 0 {
        inactive_usp!(core) = ay!(core);
        Ok(Cycles(4))
    } else {
        Err(PrivilegeViolation(ir!(core), pc!(core).wrapping_sub(2)))
    }
}
pub fn move_32_fru<T: Core>(core: &mut T) -> Result<Cycles> {
    if s_flag!(core) != 0 {
        ay!(core) = inactive_usp!(core);
        Ok(Cycles(4))
    } else {
        Err(PrivilegeViolation(ir!(core), pc!(core).wrapping_sub(2)))
    }
}
// Put implementation of MOVEM ops here
macro_rules! movem_16_re {
    ($name:ident, predecrement_ay_16, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let registers = try!(operator::imm_16(core));
            let mut ea = ay!(core);
            let mut moves = 0;
            for i in 0..16 {
                if registers & (1 << i) > 0 {
                    ea = ea.wrapping_sub(2);
                    let reg_word = dar!(core)[15-i] & 0xffff;
                    try!(core.write_data_word(ea, reg_word));
                    moves += 1;
                }
            }
            ay!(core) = ea;
            Ok(Cycles($cycles + 4 * moves))
        });
    ($name:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let registers = try!(operator::imm_16(core));
            let mut ea = try!(effective_address::$dst(core));
            let mut moves = 0;
            for i in 0..16 {
                if registers & (1 << i) > 0 {
                    let reg_word = dar!(core)[i] & 0xffff;
                    try!(core.write_data_word(ea, reg_word));
                    ea = ea.wrapping_add(2);
                    moves += 1;
                }
            }
            Ok(Cycles($cycles + 4 * moves))
        })
}
macro_rules! movem_16_er {
    ($name:ident, $src:ident, pc, $cycles:expr) => (movem_16_er!($name, $src, read_program_word, $cycles););
    ($name:ident, postincrement_ay_16, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let registers = try!(operator::imm_16(core));
            let mut ea = ay!(core);
            let mut moves = 0;
            for i in 0..16 {
                if registers & (1 << i) > 0 {
                    dar!(core)[i] = try!(core.read_data_word(ea)) as i16 as u32;
                    ea = ea.wrapping_add(2);
                    moves += 1;
                }
            }
            ay!(core) = ea;
            Ok(Cycles($cycles + 4 * moves))
        });
    ($name:ident, $src:ident, $cycles:expr) => (movem_16_er!($name, $src, read_data_word, $cycles););
    ($name:ident, $src:ident, $read_word:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let registers = try!(operator::imm_16(core));
            let mut ea = try!(effective_address::$src(core));
            let mut moves = 0;
            for i in 0..16 {
                if registers & (1 << i) > 0 {
                    dar!(core)[i] = try!(core.$read_word(ea)) as i16 as u32;
                    ea = ea.wrapping_add(2);
                    moves += 1;
                }
            }
            Ok(Cycles($cycles + 4 * moves))
        })
}
macro_rules! movem_32_re {
    ($name:ident, predecrement_ay_32, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let registers = try!(operator::imm_16(core));
            let mut ea = ay!(core);
            let mut moves = 0;
            for i in 0..16 {
                if registers & (1 << i) > 0 {
                    ea = ea.wrapping_sub(4);
                    let reg = dar!(core)[15-i];
                    try!(core.write_data_long(ea, reg));
                    moves += 1;
                }
            }
            ay!(core) = ea;
            Ok(Cycles($cycles + 8 * moves))
        });
    ($name:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let registers = try!(operator::imm_16(core));
            let mut ea = try!(effective_address::$dst(core));
            let mut moves = 0;
            for i in 0..16 {
                if registers & (1 << i) > 0 {
                    let reg = dar!(core)[i];
                    try!(core.write_data_long(ea, reg));
                    ea = ea.wrapping_add(4);
                    moves += 1;
                }
            }
            Ok(Cycles($cycles + 8 * moves))
        })
}
macro_rules! movem_32_er {
    ($name:ident, $src:ident, pc, $cycles:expr) => (movem_32_er!($name, $src, read_program_long, $cycles););
    ($name:ident, postincrement_ay_32, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let registers = try!(operator::imm_16(core));
            let mut ea = ay!(core);
            let mut moves = 0;
            for i in 0..16 {
                if registers & (1 << i) > 0 {
                    dar!(core)[i] = try!(core.read_data_long(ea));
                    ea = ea.wrapping_add(4);
                    moves += 1;
                }
            }
            ay!(core) = ea;
            Ok(Cycles($cycles + 8 * moves))
        });
    ($name:ident, $src:ident, $cycles:expr) => (movem_32_er!($name, $src, read_data_long, $cycles););
    ($name:ident, $src:ident, $read_long:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let registers = try!(operator::imm_16(core));
            let mut ea = try!(effective_address::$src(core));
            let mut moves = 0;
            for i in 0..16 {
                if registers & (1 << i) > 0 {
                    dar!(core)[i] = try!(core.$read_long(ea));
                    ea = ea.wrapping_add(4);
                    moves += 1;
                }
            }
            Ok(Cycles($cycles + 8 * moves))
        })
}

movem_16_re!(movem_16_re_ai, address_indirect_ay, 8);
movem_16_re!(movem_16_re_pd, predecrement_ay_16, 8);
movem_16_re!(movem_16_re_di, displacement_ay, 12);
movem_16_re!(movem_16_re_ix, index_ay, 14);
movem_16_re!(movem_16_re_aw, absolute_word, 12);
movem_16_re!(movem_16_re_al, absolute_long, 16);
movem_16_er!(movem_16_er_ai, address_indirect_ay, 12);
movem_16_er!(movem_16_er_pi, postincrement_ay_16, 12);
movem_16_er!(movem_16_er_di, displacement_ay, 16);
movem_16_er!(movem_16_er_ix, index_ay, 18);
movem_16_er!(movem_16_er_aw, absolute_word, 16);
movem_16_er!(movem_16_er_al, absolute_long, 20);
movem_16_er!(movem_16_er_pcdi, displacement_pc, pc, 16);
movem_16_er!(movem_16_er_pcix, index_pc, pc, 18);
movem_32_re!(movem_32_re_ai, address_indirect_ay, 8);
movem_32_re!(movem_32_re_pd, predecrement_ay_32, 8);
movem_32_re!(movem_32_re_di, displacement_ay, 12);
movem_32_re!(movem_32_re_ix, index_ay, 14);
movem_32_re!(movem_32_re_aw, absolute_word, 12);
movem_32_re!(movem_32_re_al, absolute_long, 16);
movem_32_er!(movem_32_er_ai, address_indirect_ay, 12);
movem_32_er!(movem_32_er_pi, postincrement_ay_32, 12);
movem_32_er!(movem_32_er_di, displacement_ay, 16);
movem_32_er!(movem_32_er_ix, index_ay, 18);
movem_32_er!(movem_32_er_aw, absolute_word, 16);
movem_32_er!(movem_32_er_al, absolute_long, 20);
movem_32_er!(movem_32_er_pcdi, displacement_pc, pc, 16);
movem_32_er!(movem_32_er_pcix, index_pc, pc, 18);

// Put implementation of MOVEP ops here
pub fn movep_16_er<T: Core>(core: &mut T) -> Result<Cycles> {
    let ea = try!(effective_address::displacement_ay(core));
    dx!(core) = mask_out_below_16!(dx!(core))
    | try!(core.read_data_byte(ea)) << 8
    | try!(core.read_data_byte(ea.wrapping_add(2)));
    Ok(Cycles(16))
}
pub fn movep_16_re<T: Core>(core: &mut T) -> Result<Cycles> {
    let ea = try!(effective_address::displacement_ay(core));
    let data = mask_out_above_16!(dx!(core));
    try!(core.write_data_byte(ea, mask_out_above_8!(data >> 8)));
    try!(core.write_data_byte(ea.wrapping_add(2), mask_out_above_8!(data)));
    Ok(Cycles(16))
}
pub fn movep_32_er<T: Core>(core: &mut T) -> Result<Cycles> {
    let ea = try!(effective_address::displacement_ay(core));
    dx!(core) = try!(core.read_data_byte(ea)) << 24
              | try!(core.read_data_byte(ea.wrapping_add(2))) << 16
              | try!(core.read_data_byte(ea.wrapping_add(4))) << 8
              | try!(core.read_data_byte(ea.wrapping_add(6)));
    Ok(Cycles(24))
}
pub fn movep_32_re<T: Core>(core: &mut T) -> Result<Cycles> {
    let ea = try!(effective_address::displacement_ay(core));
    let data = dx!(core);
    try!(core.write_data_byte(ea, mask_out_above_8!(data >> 24)));
    try!(core.write_data_byte(ea.wrapping_add(2), mask_out_above_8!(data >> 16)));
    try!(core.write_data_byte(ea.wrapping_add(4), mask_out_above_8!(data >> 8)));
    try!(core.write_data_byte(ea.wrapping_add(6), mask_out_above_8!(data)));
    Ok(Cycles(24))
}

// Put implementation of MOVEQ ops here
pub fn moveq_32<T: Core>(core: &mut T) -> Result<Cycles> {
    let res = mask_out_above_8!(ir!(core)) as i8 as u32;
    dx!(core) = res;

    n_flag!(core) = (res) >> 24;
    not_z_flag!(core) = res;
    v_flag!(core) = 0;
    c_flag!(core) = 0;

    Ok(Cycles(4))
}

// Put implementation of MULS ops here
macro_rules! mul_op {
    ($common:ident, $srctype:ty, $name:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core)) as $srctype;
            let dst = dx!(core) as $srctype;
            dx!(core) = common::$common(core, dst, src);
            Ok(Cycles($cycles))
        })
}
macro_rules! muls {
    ($name:ident, $src:ident, $cycles:expr) => (mul_op!(muls_16, i16, $name, $src, $cycles);)
}
macro_rules! mulu {
    ($name:ident, $src:ident, $cycles:expr) => (mul_op!(mulu_16, u16, $name, $src, $cycles);)
}
muls!(muls_16_dn, dy, 54+0);
muls!(muls_16_ai, ay_ai_16, 54+4);
muls!(muls_16_pi, ay_pi_16, 54+4);
muls!(muls_16_pd, ay_pd_16, 54+6);
muls!(muls_16_di, ay_di_16, 54+8);
muls!(muls_16_ix, ay_ix_16, 54+10);
muls!(muls_16_aw, aw_16, 54+8);
muls!(muls_16_al, al_16, 54+12);
muls!(muls_16_pcdi, pcdi_16, 54+8);
muls!(muls_16_pcix, pcix_16, 54+10);
muls!(muls_16_imm, imm_16, 54+4);

// Put implementation of MULU ops here
mulu!(mulu_16_dn, dy, 54+0);
mulu!(mulu_16_ai, ay_ai_16, 54+4);
mulu!(mulu_16_pi, ay_pi_16, 54+4);
mulu!(mulu_16_pd, ay_pd_16, 54+6);
mulu!(mulu_16_di, ay_di_16, 54+8);
mulu!(mulu_16_ix, ay_ix_16, 54+10);
mulu!(mulu_16_aw, aw_16, 54+8);
mulu!(mulu_16_al, al_16, 54+12);
mulu!(mulu_16_pcdi, pcdi_16, 54+8);
mulu!(mulu_16_pcix, pcix_16, 54+10);
mulu!(mulu_16_imm, imm_16, 54+4);

// Put implementation of NBCD ops here
macro_rules! nbcd {
    ($name:ident, dy, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let dst = dy!(core);
            if let Some(res) = common::nbcd(core, dst) {
                dy!(core) = mask_out_below_8!(dy!(core)) | res;
            }
            Ok(Cycles($cycles))
    });
    ($name:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let (dst, ea) = try!(operator::$dst(core));
            if let Some(res) = common::nbcd(core, dst) {
                try!(core.write_data_byte(ea, res));
            }
            Ok(Cycles($cycles))
        })
}
nbcd!(nbcd_8_dn, dy, 6);
nbcd!(nbcd_8_ai, ea_ay_ai_8, 8+4);
nbcd!(nbcd_8_pi, ea_ay_pi_8, 8+4);
nbcd!(nbcd_8_pd, ea_ay_pd_8, 8+6);
nbcd!(nbcd_8_di, ea_ay_di_8, 8+8);
nbcd!(nbcd_8_ix, ea_ay_ix_8, 8+10);
nbcd!(nbcd_8_aw, ea_aw_8, 8+8);
nbcd!(nbcd_8_al, ea_al_8, 8+12);

// Put implementation of NEG ops here
macro_rules! negop_8 {
    ($name:ident, $common:ident, dy, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let dst = dy!(core);
            let res = common::$common(core, 0, dst);
            dy!(core) = mask_out_below_8!(dy!(core)) | res;
            Ok(Cycles($cycles))
        });
    ($name:ident, $common:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let (dst, ea) = try!(operator::$dst(core));
            let res = common::$common(core, 0, dst);
            try!(core.write_data_byte(ea, mask_out_above_8!(res)));
            Ok(Cycles($cycles))
        });
}
macro_rules! negop_16 {
    ($name:ident, $common:ident, dy, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let dst = dy!(core);
            let res = common::$common(core, 0, dst);
            dy!(core) = mask_out_below_16!(dy!(core)) | res;
            Ok(Cycles($cycles))
        });
    ($name:ident, $common:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let (dst, ea) = try!(operator::$dst(core));
            let res = common::$common(core, 0, dst);
            try!(core.write_data_word(ea, mask_out_above_16!(res)));
            Ok(Cycles($cycles))
        });
}
macro_rules! negop_32 {
    ($name:ident, $common:ident, dy, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let dst = dy!(core);
            let res = common::$common(core, 0, dst);
            dy!(core) = res;
            Ok(Cycles($cycles))
        });
    ($name:ident, $common:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let (dst, ea) = try!(operator::$dst(core));
            let res = common::$common(core, 0, dst);
            try!(core.write_data_long(ea, res));
            Ok(Cycles($cycles))
        });
}
macro_rules! neg_8 {
    ($name:ident, $dst:ident, $cycles:expr) => (negop_8!($name, sub_8, $dst, $cycles);)
}
macro_rules! neg_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (negop_16!($name, sub_16, $dst, $cycles);)
}
macro_rules! neg_32 {
    ($name:ident, $dst:ident, $cycles:expr) => (negop_32!($name, sub_32, $dst, $cycles);)
}
neg_8!(neg_8_dn, dy, 4);
neg_8!(neg_8_ai, ea_ay_ai_8, 8+4);
neg_8!(neg_8_pi, ea_ay_pi_8, 8+4);
neg_8!(neg_8_pd, ea_ay_pd_8, 8+6);
neg_8!(neg_8_di, ea_ay_di_8, 8+8);
neg_8!(neg_8_ix, ea_ay_ix_8, 8+10);
neg_8!(neg_8_aw, ea_aw_8, 8+8);
neg_8!(neg_8_al, ea_al_8, 8+12);

neg_16!(neg_16_dn, dy, 4);
neg_16!(neg_16_ai, ea_ay_ai_16, 8+4);
neg_16!(neg_16_pi, ea_ay_pi_16, 8+4);
neg_16!(neg_16_pd, ea_ay_pd_16, 8+6);
neg_16!(neg_16_di, ea_ay_di_16, 8+8);
neg_16!(neg_16_ix, ea_ay_ix_16, 8+10);
neg_16!(neg_16_aw, ea_aw_16, 8+8);
neg_16!(neg_16_al, ea_al_16, 8+12);

neg_32!(neg_32_dn, dy, 6);
neg_32!(neg_32_ai, ea_ay_ai_32, 12+8);
neg_32!(neg_32_pi, ea_ay_pi_32, 12+8);
neg_32!(neg_32_pd, ea_ay_pd_32, 12+10);
neg_32!(neg_32_di, ea_ay_di_32, 12+12);
neg_32!(neg_32_ix, ea_ay_ix_32, 12+14);
neg_32!(neg_32_aw, ea_aw_32, 12+12);
neg_32!(neg_32_al, ea_al_32, 12+16);

// Put implementation of NEGX ops here
macro_rules! negx_8 {
    ($name:ident, $dst:ident, $cycles:expr) => (negop_8!($name, subx_8, $dst, $cycles);)
}
macro_rules! negx_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (negop_16!($name, subx_16, $dst, $cycles);)
}
macro_rules! negx_32 {
    ($name:ident, $dst:ident, $cycles:expr) => (negop_32!($name, subx_32, $dst, $cycles);)
}
negx_8!(negx_8_dn, dy, 4);
negx_8!(negx_8_ai, ea_ay_ai_8, 8+4);
negx_8!(negx_8_pi, ea_ay_pi_8, 8+4);
negx_8!(negx_8_pd, ea_ay_pd_8, 8+6);
negx_8!(negx_8_di, ea_ay_di_8, 8+8);
negx_8!(negx_8_ix, ea_ay_ix_8, 8+10);
negx_8!(negx_8_aw, ea_aw_8, 8+8);
negx_8!(negx_8_al, ea_al_8, 8+12);

negx_16!(negx_16_dn, dy, 4);
negx_16!(negx_16_ai, ea_ay_ai_16, 8+4);
negx_16!(negx_16_pi, ea_ay_pi_16, 8+4);
negx_16!(negx_16_pd, ea_ay_pd_16, 8+6);
negx_16!(negx_16_di, ea_ay_di_16, 8+8);
negx_16!(negx_16_ix, ea_ay_ix_16, 8+10);
negx_16!(negx_16_aw, ea_aw_16, 8+8);
negx_16!(negx_16_al, ea_al_16, 8+12);

negx_32!(negx_32_dn, dy, 6);
negx_32!(negx_32_ai, ea_ay_ai_32, 12+8);
negx_32!(negx_32_pi, ea_ay_pi_32, 12+8);
negx_32!(negx_32_pd, ea_ay_pd_32, 12+10);
negx_32!(negx_32_di, ea_ay_di_32, 12+12);
negx_32!(negx_32_ix, ea_ay_ix_32, 12+14);
negx_32!(negx_32_aw, ea_aw_32, 12+12);
negx_32!(negx_32_al, ea_al_32, 12+16);

// Put implementation of NOP ops here
pub fn nop<T: Core>(_core: &mut T) -> Result<Cycles> {
    Ok(Cycles(4))
}

// Put implementation of NOT ops here
macro_rules! notop_8 {
    ($name:ident, $common:ident, dy, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let dst = dy!(core);
            let res = common::$common(core, dst);
            dy!(core) = mask_out_below_8!(dy!(core)) | res;
            Ok(Cycles($cycles))
        });
    ($name:ident, $common:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let (dst, ea) = try!(operator::$dst(core));
            let res = common::$common(core, dst);
            try!(core.write_data_byte(ea, mask_out_above_8!(res)));
            Ok(Cycles($cycles))
        });
}
macro_rules! notop_16 {
    ($name:ident, $common:ident, dy, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let dst = dy!(core);
            let res = common::$common(core, dst);
            dy!(core) = mask_out_below_16!(dy!(core)) | res;
            Ok(Cycles($cycles))
        });
    ($name:ident, $common:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let (dst, ea) = try!(operator::$dst(core));
            let res = common::$common(core, dst);
            try!(core.write_data_word(ea, mask_out_above_16!(res)));
            Ok(Cycles($cycles))
        });
}
macro_rules! notop_32 {
    ($name:ident, $common:ident, dy, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let dst = dy!(core);
            let res = common::$common(core, dst);
            dy!(core) = res;
            Ok(Cycles($cycles))
        });
    ($name:ident, $common:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let (dst, ea) = try!(operator::$dst(core));
            let res = common::$common(core, dst);
            try!(core.write_data_long(ea, res));
            Ok(Cycles($cycles))
        });
}
macro_rules! not_8 {
    ($name:ident, $dst:ident, $cycles:expr) => (notop_8!($name, not_8, $dst, $cycles);)
}
macro_rules! not_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (notop_16!($name, not_16, $dst, $cycles);)
}
macro_rules! not_32 {
    ($name:ident, $dst:ident, $cycles:expr) => (notop_32!($name, not_32, $dst, $cycles);)
}
not_8!(not_8_dn, dy, 4);
not_8!(not_8_ai, ea_ay_ai_8, 8+4);
not_8!(not_8_pi, ea_ay_pi_8, 8+4);
not_8!(not_8_pd, ea_ay_pd_8, 8+6);
not_8!(not_8_di, ea_ay_di_8, 8+8);
not_8!(not_8_ix, ea_ay_ix_8, 8+10);
not_8!(not_8_aw, ea_aw_8, 8+8);
not_8!(not_8_al, ea_al_8, 8+12);

not_16!(not_16_dn, dy, 4);
not_16!(not_16_ai, ea_ay_ai_16, 8+4);
not_16!(not_16_pi, ea_ay_pi_16, 8+4);
not_16!(not_16_pd, ea_ay_pd_16, 8+6);
not_16!(not_16_di, ea_ay_di_16, 8+8);
not_16!(not_16_ix, ea_ay_ix_16, 8+10);
not_16!(not_16_aw, ea_aw_16, 8+8);
not_16!(not_16_al, ea_al_16, 8+12);

not_32!(not_32_dn, dy, 6);
not_32!(not_32_ai, ea_ay_ai_32, 12+8);
not_32!(not_32_pi, ea_ay_pi_32, 12+8);
not_32!(not_32_pd, ea_ay_pd_32, 12+10);
not_32!(not_32_di, ea_ay_di_32, 12+12);
not_32!(not_32_ix, ea_ay_ix_32, 12+14);
not_32!(not_32_aw, ea_aw_32, 12+12);
not_32!(not_32_al, ea_al_32, 12+16);

// Put implementation of OR ops here

macro_rules! or_8_er {
    ($name:ident, $src:ident, $cycles:expr) => (impl_op!(8, or_8, $name, $src, dx, $cycles);)
}
macro_rules! or_8_re {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(8, or_8, $name, dx, $dst, $cycles);)
}
macro_rules! or_16_er {
    ($name:ident, $src:ident, $cycles:expr) => (impl_op!(16, or_16, $name, $src, dx, $cycles);)
}
macro_rules! or_16_re {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(16, or_16, $name, dx, $dst, $cycles);)
}
macro_rules! or_32_er {
    ($name:ident, $src:ident, $cycles:expr) => (impl_op!(32, or_32, $name, $src, dx, $cycles);)
}
macro_rules! or_32_re {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(32, or_32, $name, dx, $dst, $cycles);)
}

or_8_er!(or_8_er_dn, dy, 4);
// or_8_er!(..., ay) not present
or_8_er!(or_8_er_ai, ay_ai_8,   8);
or_8_er!(or_8_er_pi, ay_pi_8,   8);
or_8_er!(or_8_er_pd, ay_pd_8,  10);
or_8_er!(or_8_er_di, ay_di_8,  12);
or_8_er!(or_8_er_ix, ay_ix_8,  14);
or_8_er!(or_8_er_aw, aw_8,     12);
or_8_er!(or_8_er_al, al_8,     16);
or_8_er!(or_8_er_pcdi, pcdi_8, 12);
or_8_er!(or_8_er_pcix, pcix_8, 14);
or_8_er!(or_8_er_imm, imm_8,   10);

// or_8_re!(..., dy) not present
// or_8_re!(..., ay) not present
or_8_re!(or_8_re_ai, ea_ay_ai_8,  12);
or_8_re!(or_8_re_pi, ea_ay_pi_8,  12);
or_8_re!(or_8_re_pd, ea_ay_pd_8,  14);
or_8_re!(or_8_re_di, ea_ay_di_8,  16);
or_8_re!(or_8_re_ix, ea_ay_ix_8,  18);
or_8_re!(or_8_re_aw, ea_aw_8,     16);
or_8_re!(or_8_re_al, ea_al_8,     20);
// or_8_re!(..., pcdi) not present
// or_8_re!(..., pcix) not present
// or_8_re!(..., imm) not present

or_16_er!(or_16_er_dn,   dy,       4);
// or_16_er!(..., ay) not present
or_16_er!(or_16_er_ai,   ay_ai_16, 8);
or_16_er!(or_16_er_pi,   ay_pi_16, 8);
or_16_er!(or_16_er_pd,   ay_pd_16, 10);
or_16_er!(or_16_er_di,   ay_di_16, 12);
or_16_er!(or_16_er_ix,   ay_ix_16, 14);
or_16_er!(or_16_er_aw,   aw_16,    12);
or_16_er!(or_16_er_al,   al_16,    16);
or_16_er!(or_16_er_pcdi, pcdi_16,  12);
or_16_er!(or_16_er_pcix, pcix_16,  14);
or_16_er!(or_16_er_imm,  imm_16,   10);

// or_16_re!(..., dy) not present
// or_16_re!(..., ay) not present
or_16_re!(or_16_re_ai, ea_ay_ai_16,  12);
or_16_re!(or_16_re_pi, ea_ay_pi_16,  12);
or_16_re!(or_16_re_pd, ea_ay_pd_16,  14);
or_16_re!(or_16_re_di, ea_ay_di_16,  16);
or_16_re!(or_16_re_ix, ea_ay_ix_16,  18);
or_16_re!(or_16_re_aw, ea_aw_16,     16);
or_16_re!(or_16_re_al, ea_al_16,     20);
// or_16_re!(..., pcdi) not present
// or_16_re!(..., pcix) not present
// or_16_re!(..., imm) not present

or_32_er!(or_32_er_dn,   dy,        6);
// or_32_er!(..., ay) not present
or_32_er!(or_32_er_ai,   ay_ai_32, 14);
or_32_er!(or_32_er_pi,   ay_pi_32, 14);
or_32_er!(or_32_er_pd,   ay_pd_32, 16);
or_32_er!(or_32_er_di,   ay_di_32, 18);
or_32_er!(or_32_er_ix,   ay_ix_32, 20);
or_32_er!(or_32_er_aw,   aw_32,    18);
or_32_er!(or_32_er_al,   al_32,    22);
or_32_er!(or_32_er_pcdi, pcdi_32,  18);
or_32_er!(or_32_er_pcix, pcix_32,  20);
or_32_er!(or_32_er_imm,  imm_32,   16);

// or_32_re!(..., dy) not present
// or_32_re!(..., ay) not present
or_32_re!(or_32_re_ai, ea_ay_ai_32,  12+8);
or_32_re!(or_32_re_pi, ea_ay_pi_32,  12+8);
or_32_re!(or_32_re_pd, ea_ay_pd_32,  14+8);
or_32_re!(or_32_re_di, ea_ay_di_32,  16+8);
or_32_re!(or_32_re_ix, ea_ay_ix_32,  18+8);
or_32_re!(or_32_re_aw, ea_aw_32,     16+8);
or_32_re!(or_32_re_al, ea_al_32,     20+8);
// or_32_re!(..., pcdi) not present
// or_32_re!(..., pcix) not present
// or_32_re!(..., imm) not present

// Put implementation of ORI ops here
macro_rules! ori_8 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(8, or_8, $name, imm_8, $dst, $cycles);)
}
macro_rules! ori_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(16, or_16, $name, imm_16, $dst, $cycles);)
}
macro_rules! ori_32 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(32, or_32, $name, imm_32, $dst, $cycles);)
}
ori_8!(ori_8_dn, dy,  8);
// ori_8_re!(..., ay) not present
ori_8!(ori_8_ai, ea_ay_ai_8,  12+4);
ori_8!(ori_8_pi, ea_ay_pi_8,  12+4);
ori_8!(ori_8_pd, ea_ay_pd_8,  12+6);
ori_8!(ori_8_di, ea_ay_di_8,  12+8);
ori_8!(ori_8_ix, ea_ay_ix_8,  12+10);
ori_8!(ori_8_aw, ea_aw_8,     12+8);
ori_8!(ori_8_al, ea_al_8,     12+12);
// ori_8!(..., pcdi) not present
// ori_8!(..., pcix) not present
// ori_8!(..., imm) not present

ori_16!(ori_16_dn, dy,  8);
// ori_16_re!(..., ay) not present
ori_16!(ori_16_ai, ea_ay_ai_16,  12+4);
ori_16!(ori_16_pi, ea_ay_pi_16,  12+4);
ori_16!(ori_16_pd, ea_ay_pd_16,  12+6);
ori_16!(ori_16_di, ea_ay_di_16,  12+8);
ori_16!(ori_16_ix, ea_ay_ix_16,  12+10);
ori_16!(ori_16_aw, ea_aw_16,     12+8);
ori_16!(ori_16_al, ea_al_16,     12+12);
// ori_16!(..., pcdi) not present
// ori_16!(..., pcix) not present
// ori_16!(..., imm) not present

ori_32!(ori_32_dn, dy,  16); // 2 more than andi_32_dn
// ori_32_re!(..., ay) not present
ori_32!(ori_32_ai, ea_ay_ai_32,  20+8);
ori_32!(ori_32_pi, ea_ay_pi_32,  20+8);
ori_32!(ori_32_pd, ea_ay_pd_32,  20+10);
ori_32!(ori_32_di, ea_ay_di_32,  20+12);
ori_32!(ori_32_ix, ea_ay_ix_32,  20+14);
ori_32!(ori_32_aw, ea_aw_32,     20+12);
ori_32!(ori_32_al, ea_al_32,     20+16);
// ori_32!(..., pcdi) not present
// ori_32!(..., pcix) not present
// ori_32!(..., imm) not present

// Put implementation of ORI to CCR ops here
pub fn ori_8_toc<T: Core>(core: &mut T) -> Result<Cycles> {
    let dst = core.condition_code_register();
    let src = mask_out_above_8!(try!(operator::imm_8(core))) as u16;
    core.ccr_to_flags(dst | src);
    Ok(Cycles(20))
}
// Put implementation of ORI to SR ops here
pub fn ori_16_tos<T: Core>(core: &mut T) -> Result<Cycles> {
    if s_flag!(core) != 0 {
        let dst = core.status_register();
        let src = try!(operator::imm_16(core)) as u16;
        core.sr_to_flags(dst | src);
        Ok(Cycles(20))
    } else {
        Err(PrivilegeViolation(ir!(core), pc!(core).wrapping_sub(2)))
    }
}

// Put implementation of PEA ops here
macro_rules! pea {
    ($name:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let ea = try!(effective_address::$src(core));
            core.push_32(ea);
            Ok(Cycles($cycles))
        });
}
pea!(pea_32_ai, address_indirect_ay, 12);
pea!(pea_32_di, displacement_ay, 16);
pea!(pea_32_ix, index_ay, 20);
pea!(pea_32_aw, absolute_word, 16);
pea!(pea_32_al, absolute_long, 20);
pea!(pea_32_pcdi, displacement_pc, 16);
pea!(pea_32_pcix, index_pc, 20);

// Put implementation of RESET ops here
pub fn reset<T: Core>(core: &mut T) -> Result<Cycles> {
    if s_flag!(core) != 0 {
        core.reset_external_devices();
        Ok(Cycles(132))
    } else {
        Err(PrivilegeViolation(ir!(core), pc!(core).wrapping_sub(2)))
    }
}

// Put implementation of ROL, ROR ops here
macro_rules! ror_8 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(8, ror_8, $name, $src, $dst, $cycles);)
}
macro_rules! ror_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(16, ror_16, $name, 1, $dst, $cycles););
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(16, ror_16, $name, $src, $dst, $cycles);)
}
macro_rules! ror_32 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(32, ror_32, $name, $src, $dst, $cycles);)
}

macro_rules! rol_8 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(8, rol_8, $name, $src, $dst, $cycles);)
}
macro_rules! rol_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(16, rol_16, $name, 1, $dst, $cycles););
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(16, rol_16, $name, $src, $dst, $cycles);)
}
macro_rules! rol_32 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(32, rol_32, $name, $src, $dst, $cycles);)
}

ror_8!(ror_8_s,   quick, dy, 6);
ror_16!(ror_16_s, quick, dy, 6);
ror_32!(ror_32_s, quick, dy, 8);
ror_8!(ror_8_r,   dx,    dy, 6);
ror_16!(ror_16_r, dx,    dy, 6);
ror_32!(ror_32_r, dx,    dy, 8);

rol_8!(rol_8_s,   quick, dy, 6);
rol_16!(rol_16_s, quick, dy, 6);
rol_32!(rol_32_s, quick, dy, 8);
rol_8!(rol_8_r,   dx,    dy, 6);
rol_16!(rol_16_r, dx,    dy, 6);
rol_32!(rol_32_r, dx,    dy, 8);

rol_16!(rol_16_ai, ea_ay_ai_16, 12);
rol_16!(rol_16_pi, ea_ay_pi_16, 12);
rol_16!(rol_16_pd, ea_ay_pd_16, 14);
rol_16!(rol_16_di, ea_ay_di_16, 16);
rol_16!(rol_16_ix, ea_ay_ix_16, 18);
rol_16!(rol_16_aw, ea_aw_16,    16);
rol_16!(rol_16_al, ea_al_16,    20);

ror_16!(ror_16_ai, ea_ay_ai_16, 12);
ror_16!(ror_16_pi, ea_ay_pi_16, 12);
ror_16!(ror_16_pd, ea_ay_pd_16, 14);
ror_16!(ror_16_di, ea_ay_di_16, 16);
ror_16!(ror_16_ix, ea_ay_ix_16, 18);
ror_16!(ror_16_aw, ea_aw_16,    16);
ror_16!(ror_16_al, ea_al_16,    20);

// Put implementation of ROXL, ROXR ops here
macro_rules! roxr_8 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(8, roxr_8, $name, $src, $dst, $cycles);)
}
macro_rules! roxr_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(16, roxr_16, $name, 1, $dst, $cycles););
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(16, roxr_16, $name, $src, $dst, $cycles);)
}
macro_rules! roxr_32 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(32, roxr_32, $name, $src, $dst, $cycles);)
}

macro_rules! roxl_8 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(8, roxl_8, $name, $src, $dst, $cycles);)
}
macro_rules! roxl_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(16, roxl_16, $name, 1, $dst, $cycles););
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(16, roxl_16, $name, $src, $dst, $cycles);)
}
macro_rules! roxl_32 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (impl_shift_op!(32, roxl_32, $name, $src, $dst, $cycles);)
}

roxr_8!(roxr_8_s,   quick, dy, 6);
roxr_16!(roxr_16_s, quick, dy, 6);
roxr_32!(roxr_32_s, quick, dy, 8);
roxr_8!(roxr_8_r,   dx,    dy, 6);
roxr_16!(roxr_16_r, dx,    dy, 6);
roxr_32!(roxr_32_r, dx,    dy, 8);

roxl_8!(roxl_8_s,   quick, dy, 6);
roxl_16!(roxl_16_s, quick, dy, 6);
roxl_32!(roxl_32_s, quick, dy, 8);
roxl_8!(roxl_8_r,   dx,    dy, 6);
roxl_16!(roxl_16_r, dx,    dy, 6);
roxl_32!(roxl_32_r, dx,    dy, 8);

roxl_16!(roxl_16_ai, ea_ay_ai_16, 12);
roxl_16!(roxl_16_pi, ea_ay_pi_16, 12);
roxl_16!(roxl_16_pd, ea_ay_pd_16, 14);
roxl_16!(roxl_16_di, ea_ay_di_16, 16);
roxl_16!(roxl_16_ix, ea_ay_ix_16, 18);
roxl_16!(roxl_16_aw, ea_aw_16,    16);
roxl_16!(roxl_16_al, ea_al_16,    20);

roxr_16!(roxr_16_ai, ea_ay_ai_16, 12);
roxr_16!(roxr_16_pi, ea_ay_pi_16, 12);
roxr_16!(roxr_16_pd, ea_ay_pd_16, 14);
roxr_16!(roxr_16_di, ea_ay_di_16, 16);
roxr_16!(roxr_16_ix, ea_ay_ix_16, 18);
roxr_16!(roxr_16_aw, ea_aw_16,    16);
roxr_16!(roxr_16_al, ea_al_16,    20);

// Put implementation of RTE ops here
pub fn rte_32<T: Core>(core: &mut T) -> Result<Cycles> {
    if s_flag!(core) != 0 {
        let new_sr = core.pop_16();
        let new_pc = core.pop_32();
        core.jump(new_pc);
        core.sr_to_flags(new_sr);
        core.resume_normal_processing();

        Ok(Cycles(20))
    } else {
        Err(PrivilegeViolation(ir!(core), pc!(core).wrapping_sub(2)))
    }
}

// Put implementation of RTR ops here
pub fn rtr_32<T: Core>(core: &mut T) -> Result<Cycles> {
    let new_ccr = core.pop_16();
    let new_pc = core.pop_32();
    core.jump(new_pc);
    core.ccr_to_flags(new_ccr);
    Ok(Cycles(20))
}

// Put implementation of RTS ops here
pub fn rts_32<T: Core>(core: &mut T) -> Result<Cycles> {
    let new_pc = core.pop_32();
    core.jump(new_pc);
    Ok(Cycles(16))
}

impl_op!(8, sbcd_8, sbcd_8_rr, dy, dx, 6);
impl_op!(8, sbcd_8, sbcd_8_mm, ay_pd_8, ea_ax_pd_8, 18);


macro_rules! sxx_8_dn {
    ($name:ident, $cond:ident) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let cycles = if core.$cond() {
                dy!(core) |= 0xff;
                6
            } else {
                dy!(core) &= 0xffff_ff00;
                4
            };
            Ok(Cycles(cycles))
        }
    );
}

macro_rules! sxx_8 {
    ($name:ident, $cond:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let t = if core.$cond() { 0xffu32 } else { 0u32 };
            let ea = try!(effective_address::$dst(core));
            try!(core.write_data_byte(ea, t));
            Ok(Cycles($cycles))
        }
    );
}

sxx_8_dn!(scc_8_dn, cond_cc);
sxx_8!(scc_8_ai, cond_cc, address_indirect_ay, 12);
sxx_8!(scc_8_al, cond_cc, absolute_long,       20);
sxx_8!(scc_8_aw, cond_cc, absolute_word,       16);
sxx_8!(scc_8_di, cond_cc, displacement_ay,     16);
sxx_8!(scc_8_ix, cond_cc, index_ay,            18);
sxx_8!(scc_8_pd, cond_cc, predecrement_ay_8,   14);
sxx_8!(scc_8_pi, cond_cc, postincrement_ay_8,  12);

sxx_8_dn!(scs_8_dn, cond_cs);
sxx_8!(scs_8_ai, cond_cs, address_indirect_ay, 12);
sxx_8!(scs_8_al, cond_cs, absolute_long,       20);
sxx_8!(scs_8_aw, cond_cs, absolute_word,       16);
sxx_8!(scs_8_di, cond_cs, displacement_ay,     16);
sxx_8!(scs_8_ix, cond_cs, index_ay,            18);
sxx_8!(scs_8_pd, cond_cs, predecrement_ay_8,   14);
sxx_8!(scs_8_pi, cond_cs, postincrement_ay_8,  12);

sxx_8_dn!(seq_8_dn, cond_eq);
sxx_8!(seq_8_ai, cond_eq, address_indirect_ay, 12);
sxx_8!(seq_8_al, cond_eq, absolute_long,       20);
sxx_8!(seq_8_aw, cond_eq, absolute_word,       16);
sxx_8!(seq_8_di, cond_eq, displacement_ay,     16);
sxx_8!(seq_8_ix, cond_eq, index_ay,            18);
sxx_8!(seq_8_pd, cond_eq, predecrement_ay_8,   14);
sxx_8!(seq_8_pi, cond_eq, postincrement_ay_8,  12);

sxx_8_dn!(sf_8_dn, cond_f);
sxx_8!(sf_8_ai, cond_f, address_indirect_ay, 12);
sxx_8!(sf_8_al, cond_f, absolute_long,       20);
sxx_8!(sf_8_aw, cond_f, absolute_word,       16);
sxx_8!(sf_8_di, cond_f, displacement_ay,     16);
sxx_8!(sf_8_ix, cond_f, index_ay,            18);
sxx_8!(sf_8_pd, cond_f, predecrement_ay_8,   14);
sxx_8!(sf_8_pi, cond_f, postincrement_ay_8,  12);

sxx_8_dn!(sge_8_dn, cond_ge);
sxx_8!(sge_8_ai, cond_ge, address_indirect_ay, 12);
sxx_8!(sge_8_al, cond_ge, absolute_long,       20);
sxx_8!(sge_8_aw, cond_ge, absolute_word,       16);
sxx_8!(sge_8_di, cond_ge, displacement_ay,     16);
sxx_8!(sge_8_ix, cond_ge, index_ay,            18);
sxx_8!(sge_8_pd, cond_ge, predecrement_ay_8,   14);
sxx_8!(sge_8_pi, cond_ge, postincrement_ay_8,  12);

sxx_8_dn!(sgt_8_dn, cond_gt);
sxx_8!(sgt_8_ai, cond_gt, address_indirect_ay, 12);
sxx_8!(sgt_8_al, cond_gt, absolute_long,       20);
sxx_8!(sgt_8_aw, cond_gt, absolute_word,       16);
sxx_8!(sgt_8_di, cond_gt, displacement_ay,     16);
sxx_8!(sgt_8_ix, cond_gt, index_ay,            18);
sxx_8!(sgt_8_pd, cond_gt, predecrement_ay_8,   14);
sxx_8!(sgt_8_pi, cond_gt, postincrement_ay_8,  12);

sxx_8_dn!(shi_8_dn, cond_hi);
sxx_8!(shi_8_ai, cond_hi, address_indirect_ay, 12);
sxx_8!(shi_8_al, cond_hi, absolute_long,       20);
sxx_8!(shi_8_aw, cond_hi, absolute_word,       16);
sxx_8!(shi_8_di, cond_hi, displacement_ay,     16);
sxx_8!(shi_8_ix, cond_hi, index_ay,            18);
sxx_8!(shi_8_pd, cond_hi, predecrement_ay_8,   14);
sxx_8!(shi_8_pi, cond_hi, postincrement_ay_8,  12);

sxx_8_dn!(sle_8_dn, cond_le);
sxx_8!(sle_8_ai, cond_le, address_indirect_ay, 12);
sxx_8!(sle_8_al, cond_le, absolute_long,       20);
sxx_8!(sle_8_aw, cond_le, absolute_word,       16);
sxx_8!(sle_8_di, cond_le, displacement_ay,     16);
sxx_8!(sle_8_ix, cond_le, index_ay,            18);
sxx_8!(sle_8_pd, cond_le, predecrement_ay_8,   14);
sxx_8!(sle_8_pi, cond_le, postincrement_ay_8,  12);

sxx_8_dn!(sls_8_dn, cond_ls);
sxx_8!(sls_8_ai, cond_ls, address_indirect_ay, 12);
sxx_8!(sls_8_al, cond_ls, absolute_long,       20);
sxx_8!(sls_8_aw, cond_ls, absolute_word,       16);
sxx_8!(sls_8_di, cond_ls, displacement_ay,     16);
sxx_8!(sls_8_ix, cond_ls, index_ay,            18);
sxx_8!(sls_8_pd, cond_ls, predecrement_ay_8,   14);
sxx_8!(sls_8_pi, cond_ls, postincrement_ay_8,  12);

sxx_8_dn!(slt_8_dn, cond_lt);
sxx_8!(slt_8_ai, cond_lt, address_indirect_ay, 12);
sxx_8!(slt_8_al, cond_lt, absolute_long,       20);
sxx_8!(slt_8_aw, cond_lt, absolute_word,       16);
sxx_8!(slt_8_di, cond_lt, displacement_ay,     16);
sxx_8!(slt_8_ix, cond_lt, index_ay,            18);
sxx_8!(slt_8_pd, cond_lt, predecrement_ay_8,   14);
sxx_8!(slt_8_pi, cond_lt, postincrement_ay_8,  12);

sxx_8_dn!(smi_8_dn, cond_mi);
sxx_8!(smi_8_ai, cond_mi, address_indirect_ay, 12);
sxx_8!(smi_8_al, cond_mi, absolute_long,       20);
sxx_8!(smi_8_aw, cond_mi, absolute_word,       16);
sxx_8!(smi_8_di, cond_mi, displacement_ay,     16);
sxx_8!(smi_8_ix, cond_mi, index_ay,            18);
sxx_8!(smi_8_pd, cond_mi, predecrement_ay_8,   14);
sxx_8!(smi_8_pi, cond_mi, postincrement_ay_8,  12);

sxx_8_dn!(sne_8_dn, cond_ne);
sxx_8!(sne_8_ai, cond_ne, address_indirect_ay, 12);
sxx_8!(sne_8_al, cond_ne, absolute_long,       20);
sxx_8!(sne_8_aw, cond_ne, absolute_word,       16);
sxx_8!(sne_8_di, cond_ne, displacement_ay,     16);
sxx_8!(sne_8_ix, cond_ne, index_ay,            18);
sxx_8!(sne_8_pd, cond_ne, predecrement_ay_8,   14);
sxx_8!(sne_8_pi, cond_ne, postincrement_ay_8,  12);

sxx_8_dn!(spl_8_dn, cond_pl);
sxx_8!(spl_8_ai, cond_pl, address_indirect_ay, 12);
sxx_8!(spl_8_al, cond_pl, absolute_long,       20);
sxx_8!(spl_8_aw, cond_pl, absolute_word,       16);
sxx_8!(spl_8_di, cond_pl, displacement_ay,     16);
sxx_8!(spl_8_ix, cond_pl, index_ay,            18);
sxx_8!(spl_8_pd, cond_pl, predecrement_ay_8,   14);
sxx_8!(spl_8_pi, cond_pl, postincrement_ay_8,  12);

sxx_8_dn!(st_8_dn, cond_t);
sxx_8!(st_8_ai, cond_t, address_indirect_ay, 12);
sxx_8!(st_8_al, cond_t, absolute_long,       20);
sxx_8!(st_8_aw, cond_t, absolute_word,       16);
sxx_8!(st_8_di, cond_t, displacement_ay,     16);
sxx_8!(st_8_ix, cond_t, index_ay,            18);
sxx_8!(st_8_pd, cond_t, predecrement_ay_8,   14);
sxx_8!(st_8_pi, cond_t, postincrement_ay_8,  12);

sxx_8_dn!(svc_8_dn, cond_vc);
sxx_8!(svc_8_ai, cond_vc, address_indirect_ay, 12);
sxx_8!(svc_8_al, cond_vc, absolute_long,       20);
sxx_8!(svc_8_aw, cond_vc, absolute_word,       16);
sxx_8!(svc_8_di, cond_vc, displacement_ay,     16);
sxx_8!(svc_8_ix, cond_vc, index_ay,            18);
sxx_8!(svc_8_pd, cond_vc, predecrement_ay_8,   14);
sxx_8!(svc_8_pi, cond_vc, postincrement_ay_8,  12);

sxx_8_dn!(svs_8_dn, cond_vs);
sxx_8!(svs_8_ai, cond_vs, address_indirect_ay, 12);
sxx_8!(svs_8_al, cond_vs, absolute_long,       20);
sxx_8!(svs_8_aw, cond_vs, absolute_word,       16);
sxx_8!(svs_8_di, cond_vs, displacement_ay,     16);
sxx_8!(svs_8_ix, cond_vs, index_ay,            18);
sxx_8!(svs_8_pd, cond_vs, predecrement_ay_8,   14);
sxx_8!(svs_8_pi, cond_vs, postincrement_ay_8,  12);

// Put implementation of Scc ops here
// Put implementation of STOP ops here
pub fn stop<T: Core>(core: &mut T) -> Result<Cycles> {
    if s_flag!(core) != 0 {
        // Stops the fetching and executing of instructions. A trace,
        // interrupt, or reset exception causes the processor to resume
        // instruction execution. A trace exception occurs if
        // instruction tracing is enabled (T0 = 1, T1 = 0) when the STOP
        // instruction begins execution. If an interrupt request is
        // asserted with a priority higher than the priority level set
        // by the new status register value, an interrupt exception
        // occurs; otherwise, the interrupt request is ignored. External
        // reset always initiates reset exception processing. 

        // Note that a processor in the stopped state is not in the
        // halted state, nor vice versa.
        let sr = try!(core.read_imm_u16());
        core.sr_to_flags(sr);
        core.stop_instruction_processing();

        Ok(Cycles(4))
    } else {
        Err(PrivilegeViolation(ir!(core), pc!(core).wrapping_sub(2)))
    }
}

// Put implementation of SUB ops here

macro_rules! sub_8_er {
    ($name:ident, $src:ident, $cycles:expr) => (impl_op!(8, sub_8, $name, $src, dx, $cycles);)
}
macro_rules! sub_8_re {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(8, sub_8, $name, dx, $dst, $cycles);)
}
macro_rules! sub_16_er {
    ($name:ident, $src:ident, $cycles:expr) => (impl_op!(16, sub_16, $name, $src, dx, $cycles);)
}
macro_rules! sub_16_re {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(16, sub_16, $name, dx, $dst, $cycles);)
}
macro_rules! sub_32_er {
    ($name:ident, $src:ident, $cycles:expr) => (impl_op!(32, sub_32, $name, $src, dx, $cycles);)
}
macro_rules! sub_32_re {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(32, sub_32, $name, dx, $dst, $cycles);)
}
sub_8_er!(sub_8_er_dn, dy, 4);
// sub_8_er!(..., ay) not present - for word and long only
sub_8_er!(sub_8_er_ai, ay_ai_8,   8);
sub_8_er!(sub_8_er_pi, ay_pi_8,   8);
sub_8_er!(sub_8_er_pd, ay_pd_8,  10);
sub_8_er!(sub_8_er_di, ay_di_8,  12);
sub_8_er!(sub_8_er_ix, ay_ix_8,  14);
sub_8_er!(sub_8_er_aw, aw_8,     12);
sub_8_er!(sub_8_er_al, al_8,     16);
sub_8_er!(sub_8_er_pcdi, pcdi_8, 12);
sub_8_er!(sub_8_er_pcix, pcix_8, 14);
sub_8_er!(sub_8_er_imm, imm_8,   10);

// sub_8_re!(..., dy) not present
// sub_8_re!(..., ay) not present
sub_8_re!(sub_8_re_ai, ea_ay_ai_8,  12);
sub_8_re!(sub_8_re_pi, ea_ay_pi_8,  12);
sub_8_re!(sub_8_re_pd, ea_ay_pd_8,  14);
sub_8_re!(sub_8_re_di, ea_ay_di_8,  16);
sub_8_re!(sub_8_re_ix, ea_ay_ix_8,  18);
sub_8_re!(sub_8_re_aw, ea_aw_8,     16);
sub_8_re!(sub_8_re_al, ea_al_8,     20);
// sub_8_re!(..., pcdi) not present
// sub_8_re!(..., pcix) not present
// sub_8_re!(..., imm) not present

sub_16_er!(sub_16_er_dn, dy,         4);
sub_16_er!(sub_16_er_an, ay,         4);
sub_16_er!(sub_16_er_ai, ay_ai_16,   8);
sub_16_er!(sub_16_er_pi, ay_pi_16,   8);
sub_16_er!(sub_16_er_pd, ay_pd_16,  10);
sub_16_er!(sub_16_er_di, ay_di_16,  12);
sub_16_er!(sub_16_er_ix, ay_ix_16,  14);
sub_16_er!(sub_16_er_aw, aw_16,     12);
sub_16_er!(sub_16_er_al, al_16,     16);
sub_16_er!(sub_16_er_pcdi, pcdi_16, 12);
sub_16_er!(sub_16_er_pcix, pcix_16, 14);
sub_16_er!(sub_16_er_imm, imm_16,   10);

// sub_16_re!(..., dy) not present
// sub_16_re!(..., ay) not present
sub_16_re!(sub_16_re_ai, ea_ay_ai_16,  12);
sub_16_re!(sub_16_re_pi, ea_ay_pi_16,  12);
sub_16_re!(sub_16_re_pd, ea_ay_pd_16,  14);
sub_16_re!(sub_16_re_di, ea_ay_di_16,  16);
sub_16_re!(sub_16_re_ix, ea_ay_ix_16,  18);
sub_16_re!(sub_16_re_aw, ea_aw_16,     16);
sub_16_re!(sub_16_re_al, ea_al_16,     20);
// sub_16_re!(..., pcdi) not present
// sub_16_re!(..., pcix) not present
// sub_16_re!(..., imm) not present

sub_32_er!(sub_32_er_dn, dy,         6);
sub_32_er!(sub_32_er_an, ay,         6);
sub_32_er!(sub_32_er_ai, ay_ai_32,  14);
sub_32_er!(sub_32_er_pi, ay_pi_32,  14);
sub_32_er!(sub_32_er_pd, ay_pd_32,  16);
sub_32_er!(sub_32_er_di, ay_di_32,  18);
sub_32_er!(sub_32_er_ix, ay_ix_32,  20);
sub_32_er!(sub_32_er_aw, aw_32,     18);
sub_32_er!(sub_32_er_al, al_32,     22);
sub_32_er!(sub_32_er_pcdi, pcdi_32, 18);
sub_32_er!(sub_32_er_pcix, pcix_32, 20);
sub_32_er!(sub_32_er_imm, imm_32,   16);

// sub_32_re!(..., dy) not present
// sub_32_re!(..., ay) not present
sub_32_re!(sub_32_re_ai, ea_ay_ai_32,  12+8);
sub_32_re!(sub_32_re_pi, ea_ay_pi_32,  12+8);
sub_32_re!(sub_32_re_pd, ea_ay_pd_32,  14+8);
sub_32_re!(sub_32_re_di, ea_ay_di_32,  16+8);
sub_32_re!(sub_32_re_ix, ea_ay_ix_32,  18+8);
sub_32_re!(sub_32_re_aw, ea_aw_32,     16+8);
sub_32_re!(sub_32_re_al, ea_al_32,     20+8);
// sub_32_re!(..., pcdi) not present
// sub_32_re!(..., pcix) not present
// sub_32_re!(..., imm) not present

macro_rules! suba_16 {
    ($name:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            // we must evaluate AY (src) first
            // as the PI/PD addressing modes will change AX (if AX=AY)
            let src = try!(operator::$src(core));
            let dst = try!(operator::ax(core));
            ax!(core) = dst.wrapping_sub(src as i16 as u32);
            Ok(Cycles($cycles))
        })
}
macro_rules! suba_32 {
    ($name:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            // we must evaluate AY (src) first
            // as the PI/PD addressing modes will change AX (if AX=AY)
            let src = try!(operator::$src(core));
            let dst = try!(operator::ax(core));
            ax!(core) = dst.wrapping_sub(src);
            Ok(Cycles($cycles))
        })
}
suba_16!(suba_16_dn, dy,         4+4);
suba_16!(suba_16_an, ay,         4+4);
suba_16!(suba_16_ai, ay_ai_16,   8+4);
suba_16!(suba_16_pi, ay_pi_16,   8+4);
suba_16!(suba_16_pd, ay_pd_16,  10+4);
suba_16!(suba_16_di, ay_di_16,  12+4);
suba_16!(suba_16_ix, ay_ix_16,  14+4);
suba_16!(suba_16_aw, aw_16,     12+4);
suba_16!(suba_16_al, al_16,     16+4);
suba_16!(suba_16_pcdi, pcdi_16, 12+4);
suba_16!(suba_16_pcix, pcix_16, 14+4);
suba_16!(suba_16_imm, imm_16,   10+4);

suba_32!(suba_32_dn, dy,         6);
suba_32!(suba_32_an, ay,         6);
suba_32!(suba_32_ai, ay_ai_32,  14);
suba_32!(suba_32_pi, ay_pi_32,  14);
suba_32!(suba_32_pd, ay_pd_32,  16);
suba_32!(suba_32_di, ay_di_32,  18);
suba_32!(suba_32_ix, ay_ix_32,  20);
suba_32!(suba_32_aw, aw_32,     18);
suba_32!(suba_32_al, al_32,     22);
suba_32!(suba_32_pcdi, pcdi_32, 18);
suba_32!(suba_32_pcix, pcix_32, 20);
suba_32!(suba_32_imm, imm_32,   16);

macro_rules! subi_8 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(8, sub_8, $name, imm_8, $dst, $cycles);)
}
macro_rules! subi_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(16, sub_16, $name, imm_16, $dst, $cycles);)
}
macro_rules! subi_32 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(32, sub_32, $name, imm_32, $dst, $cycles);)
}
subi_8!(subi_8_dn, dy,  8);
// subi_8!(..., ay) not present
subi_8!(subi_8_ai, ea_ay_ai_8,  12+4);
subi_8!(subi_8_pi, ea_ay_pi_8,  12+4);
subi_8!(subi_8_pd, ea_ay_pd_8,  12+6);
subi_8!(subi_8_di, ea_ay_di_8,  12+8);
subi_8!(subi_8_ix, ea_ay_ix_8,  12+10);
subi_8!(subi_8_aw, ea_aw_8,     12+8);
subi_8!(subi_8_al, ea_al_8,     12+12);
// subi_8!(..., pcdi) not present
// subi_8!(..., pcix) not present
// subi_8!(..., imm) not present

subi_16!(subi_16_dn, dy,  8);
// subi_16!(..., ay) not present
subi_16!(subi_16_ai, ea_ay_ai_16,  12+4);
subi_16!(subi_16_pi, ea_ay_pi_16,  12+4);
subi_16!(subi_16_pd, ea_ay_pd_16,  12+6);
subi_16!(subi_16_di, ea_ay_di_16,  12+8);
subi_16!(subi_16_ix, ea_ay_ix_16,  12+10);
subi_16!(subi_16_aw, ea_aw_16,     12+8);
subi_16!(subi_16_al, ea_al_16,     12+12);
// subi_16!(..., pcdi) not present
// subi_16!(..., pcix) not present
// subi_16!(..., imm) not present

subi_32!(subi_32_dn, dy,  16);
// subi_32!(..., ay) not present
subi_32!(subi_32_ai, ea_ay_ai_32,  20+8);
subi_32!(subi_32_pi, ea_ay_pi_32,  20+8);
subi_32!(subi_32_pd, ea_ay_pd_32,  20+10);
subi_32!(subi_32_di, ea_ay_di_32,  20+12);
subi_32!(subi_32_ix, ea_ay_ix_32,  20+14);
subi_32!(subi_32_aw, ea_aw_32,     20+12);
subi_32!(subi_32_al, ea_al_32,     20+16);
// subi_32!(..., pcdi) not present
// subi_32!(..., pcix) not present
// subi_32!(..., imm) not present

macro_rules! subq_8 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(8, sub_8, $name, quick, $dst, $cycles);)
}
macro_rules! subq_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(16, sub_16, $name, quick, $dst, $cycles);)
}
macro_rules! subq_32 {
    ($name:ident, $dst:ident, $cycles:expr) => (impl_op!(32, sub_32, $name, quick, $dst, $cycles);)
}

subq_8!(subq_8_dn, dy, 4);
// subq_8!(..., ay) not present - word and long only
subq_8!(subq_8_ai, ea_ay_ai_8,  8+4);
subq_8!(subq_8_pi, ea_ay_pi_8,  8+4);
subq_8!(subq_8_pd, ea_ay_pd_8,  8+6);
subq_8!(subq_8_di, ea_ay_di_8,  8+8);
subq_8!(subq_8_ix, ea_ay_ix_8,  8+10);
subq_8!(subq_8_aw, ea_aw_8,     8+8);
subq_8!(subq_8_al, ea_al_8,     8+12);
// subq_8!(..., pcdi) not present
// subq_8!(..., pcix) not present
// subq_8!(..., imm) not present

subq_16!(subq_16_dn, dy,  4);
pub fn subq_16_an<T: Core>(core: &mut T) -> Result<Cycles> {
    let src = try!(operator::quick(core));
    let dst = ay!(core);
    // When adding to address registers, the condition codes are not
    // altered, and the entire destination address register is used
    // regardless of the operation size.
    ay!(core) = dst.wrapping_sub(src);
    Ok(Cycles(8))
}
subq_16!(subq_16_ai, ea_ay_ai_16,  8+4);
subq_16!(subq_16_pi, ea_ay_pi_16,  8+4);
subq_16!(subq_16_pd, ea_ay_pd_16,  8+6);
subq_16!(subq_16_di, ea_ay_di_16,  8+8);
subq_16!(subq_16_ix, ea_ay_ix_16,  8+10);
subq_16!(subq_16_aw, ea_aw_16,     8+8);
subq_16!(subq_16_al, ea_al_16,     8+12);
// subq_16!(..., pcdi) not present
// subq_16!(..., pcix) not present
// subq_16!(..., imm) not present

subq_32!(subq_32_dn, dy,  8);
pub fn subq_32_an<T: Core>(core: &mut T) -> Result<Cycles> {
    let src = try!(operator::quick(core));
    let dst = ay!(core);
    // When adding to address registers, the condition codes are not
    // altered, and the entire destination address register is used
    // regardless of the operation size.
    ay!(core) = dst.wrapping_sub(src);
    Ok(Cycles(8))
}
subq_32!(subq_32_ai, ea_ay_ai_32,  12+8);
subq_32!(subq_32_pi, ea_ay_pi_32,  12+8);
subq_32!(subq_32_pd, ea_ay_pd_32,  12+10);
subq_32!(subq_32_di, ea_ay_di_32,  12+12);
subq_32!(subq_32_ix, ea_ay_ix_32,  12+14);
subq_32!(subq_32_aw, ea_aw_32,     12+12);
subq_32!(subq_32_al, ea_al_32,     12+16);
// subq_32!(..., pcdi) not present
// subq_32!(..., pcix) not present
// subq_32!(..., imm) not present

impl_op!( 8, subx_8,  subx_8_rr, dy, dx, 4);
impl_op!( 8, subx_8,  subx_8_mm, ay_pd_8, ea_ax_pd_8, 18);
impl_op!(16, subx_16, subx_16_rr, dy, dx, 4);
impl_op!(16, subx_16, subx_16_mm, ay_pd_16, ea_ax_pd_16, 18);
impl_op!(32, subx_32, subx_32_rr, dy, dx, 8);
impl_op!(32, subx_32, subx_32_mm, ay_pd_32, ea_ax_pd_32, 30);

pub fn swap_32_dn<T: Core>(core: &mut T) -> Result<Cycles> {
    let v = dy!(core);
    let res = ((v & 0x0000_ffff) << 16) | (v >> 16);

    dy!(core) = res;

    n_flag!(core) = res >> 24;
    v_flag!(core) = 0;
    c_flag!(core) = 0;
    not_z_flag!(core) = res;

    Ok(Cycles(4))
}

// Put implementation of TAS ops here TAS uses a read/modify/write bus
// cycle to ensure uninterrupted updating of memory. If interrupted,
// then the write does not proceed. Note that this does not apply to the
// data register direct mode, as there's no memory access.
macro_rules! tas_8 {
    ($name:ident, dy, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let dst = dy!(core);

            not_z_flag!(core) = mask_out_above_8!(dst);
            n_flag!(core) = dst;
            v_flag!(core) = 0;
            c_flag!(core) = 0;

            dy!(core) = dst | 0x80;
            Ok(Cycles($cycles))
        });
    ($name:ident, $dst:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let (dst, ea) = try!(operator::$dst(core));

            not_z_flag!(core) = dst;
            n_flag!(core) = dst;
            v_flag!(core) = 0;
            c_flag!(core) = 0;

            if core.allow_tas_writeback() {
                try!(core.write_data_byte(ea, mask_out_above_8!(dst | 0x80)));
            }
            Ok(Cycles($cycles))
        });
}
tas_8!(tas_8_dn, dy, 4);
tas_8!(tas_8_ai, ea_ay_ai_8, 14+4);
tas_8!(tas_8_pi, ea_ay_pi_8, 14+4);
tas_8!(tas_8_pd, ea_ay_pd_8, 14+6);
tas_8!(tas_8_di, ea_ay_di_8, 14+8);
tas_8!(tas_8_ix, ea_ay_ix_8, 14+10);
tas_8!(tas_8_aw, ea_aw_8, 14+8);
tas_8!(tas_8_al, ea_al_8, 14+12);

// Put implementation of TRAP ops here
pub fn trap<T: Core>(core: &mut T) -> Result<Cycles> {
    Err(Trap(EXCEPTION_TRAP_BASE + low_nibble!(ir!(core)) as u8, 34))
}

// Put implementation of TRAPV ops here
pub fn trapv<T: Core>(core: &mut T) -> Result<Cycles> {
    if v_flag!(core) != 0 {
        Err(Trap(EXCEPTION_TRAPV, 34))
    } else {
        Ok(Cycles(4))
    }
}

// Put implementation of TST ops here
macro_rules! tst_8 {
    ($name:ident, dy, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = mask_out_above_8!(dy!(core));

            not_z_flag!(core) = src;
            n_flag!(core) = src;
            v_flag!(core) = 0;
            c_flag!(core) = 0;

            Ok(Cycles($cycles))
        });
    ($name:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core));

            not_z_flag!(core) = src;
            n_flag!(core) = src;
            v_flag!(core) = 0;
            c_flag!(core) = 0;

            Ok(Cycles($cycles))
        });
}
macro_rules! tst_16 {
    ($name:ident, dy, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = mask_out_above_16!(dy!(core));

            not_z_flag!(core) = src;
            n_flag!(core) = src >> 8;
            v_flag!(core) = 0;
            c_flag!(core) = 0;

            Ok(Cycles($cycles))
        });
    ($name:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core));

            not_z_flag!(core) = src;
            n_flag!(core) = src >> 8;
            v_flag!(core) = 0;
            c_flag!(core) = 0;

            Ok(Cycles($cycles))
        });
}
macro_rules! tst_32 {
    ($name:ident, $src:ident, $cycles:expr) => (
        pub fn $name<T: Core>(core: &mut T) -> Result<Cycles> {
            let src = try!(operator::$src(core));

            not_z_flag!(core) = src;
            n_flag!(core) = src >> 24;
            v_flag!(core) = 0;
            c_flag!(core) = 0;

            Ok(Cycles($cycles))
        });
}
tst_8!(tst_8_dn,   dy,      4);
tst_8!(tst_8_ai,   ay_ai_8, 4+4);
tst_8!(tst_8_pi,   ay_pi_8, 4+4);
tst_8!(tst_8_pd,   ay_pd_8, 4+6);
tst_8!(tst_8_di,   ay_di_8, 4+8);
tst_8!(tst_8_ix,   ay_ix_8, 4+10);
tst_8!(tst_8_aw,   aw_8,    4+8);
tst_8!(tst_8_al,   al_8,    4+12);
// tst_8!(tst_8_pcdi, pcdi_8,  4+8);
// tst_8!(tst_8_pcix, pcix_8,  4+10);
// tst_8!(tst_8_imm,  imm_8,   4+4);

tst_16!(tst_16_dn,   dy,       4);
// tst_16!(tst_16_an,   ay,       4);
tst_16!(tst_16_ai,   ay_ai_16, 4+4);
tst_16!(tst_16_pi,   ay_pi_16, 4+4);
tst_16!(tst_16_pd,   ay_pd_16, 4+6);
tst_16!(tst_16_di,   ay_di_16, 4+8);
tst_16!(tst_16_ix,   ay_ix_16, 4+10);
tst_16!(tst_16_aw,   aw_16,    4+8);
tst_16!(tst_16_al,   al_16,    4+12);
// tst_16!(tst_16_pcdi, pcdi_16,  4+8);
// tst_16!(tst_16_pcix, pcix_16,  4+10);
// tst_16!(tst_16_imm,  imm_16,   4+4);

tst_32!(tst_32_dn,   dy,        4);
// tst_32!(tst_32_an,   ay,        4);
tst_32!(tst_32_ai,   ay_ai_32,  4+8);
tst_32!(tst_32_pi,   ay_pi_32,  4+8);
tst_32!(tst_32_pd,   ay_pd_32,  4+10);
tst_32!(tst_32_di,   ay_di_32,  4+12);
tst_32!(tst_32_ix,   ay_ix_32,  4+14);
tst_32!(tst_32_aw,   aw_32,     4+12);
tst_32!(tst_32_al,   al_32,     4+16);
// tst_32!(tst_32_pcdi, pcdi_32,   4+12);
// tst_32!(tst_32_pcix, pcix_32,   4+14);
// tst_32!(tst_32_imm,  imm_32,    4+8);

// Put implementation of UNLK ops here
pub fn unlk_32<T: Core>(core: &mut T) -> Result<Cycles> {
    let ay = ay!(core);
    sp!(core) = ay;
    ay!(core) = core.pop_32();

    Ok(Cycles(12))
}
