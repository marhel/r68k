#![macro_use]
use super::{Core, Cycles, Result, EXCEPTION_CHK};
use super::Exception::IllegalInstruction;
use super::Exception::Trap;
mod common;
pub mod handlers;

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

macro_rules! impl_op {
    (-, $common:ident, $name:ident, $src:ident, dx, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let dst = dx!(core);
            let _ = common::$common(core, dst, src);
            Ok(Cycles($cycles))
        });
    (8, $common:ident, $name:ident, $src:ident, dx, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let dst = dx!(core);
            let res = common::$common(core, dst, src);
            dx!(core) = mask_out_below_8!(dst) | res;
            Ok(Cycles($cycles))
        });
    (8, $common:ident, $name:ident, $src:ident, dy, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let dst = dy!(core);
            let res = common::$common(core, dst, src);
            dy!(core) = mask_out_below_8!(dst) | res;
            Ok(Cycles($cycles))
        });
    (16, $common:ident, $name:ident, $src:ident, dx, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let dst = dx!(core);
            let res = common::$common(core, dst, src);
            dx!(core) = mask_out_below_16!(dst) | res;
            Ok(Cycles($cycles))
        });
    (16, $common:ident, $name:ident, $src:ident, dy, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let dst = dy!(core);
            let res = common::$common(core, dst, src);
            dy!(core) = mask_out_below_16!(dst) | res;
            Ok(Cycles($cycles))
        });
    (32, $common:ident, $name:ident, $src:ident, dx, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let dst = dx!(core);
            let res = common::$common(core, dst, src);
            dx!(core) = res;
            Ok(Cycles($cycles))
        });
    (32, $common:ident, $name:ident, $src:ident, dy, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let dst = dy!(core);
            let res = common::$common(core, dst, src);
            dy!(core) = res;
            Ok(Cycles($cycles))
        });
    (8, $common:ident, $name:ident, $src:ident, $dst:ident, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let (dst, ea) = try!(operator::$dst(core));
            let res = common::$common(core, dst, src);
            core.write_data_byte(ea, mask_out_below_8!(dst) | res);
            Ok(Cycles($cycles))
        });
    (16, $common:ident, $name:ident, $src:ident, $dst:ident, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let (dst, ea) = try!(operator::$dst(core));
            let res = common::$common(core, dst, src);
            core.write_data_word(ea, mask_out_below_16!(dst) | res);
            Ok(Cycles($cycles))
        });
    (32, $common:ident, $name:ident, $src:ident, $dst:ident, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let src = try!(operator::$src(core));
            let (dst, ea) = try!(operator::$dst(core));
            let res = common::$common(core, dst, src);
            core.write_data_long(ea, res);
            Ok(Cycles($cycles))
        })
}
macro_rules! impl_shift_op {
    (8, $common:ident, $name:ident, $shift_src:ident, dy, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let shift = try!(operator::$shift_src(core)) & 0x3f; // mod 64
            let dst = dy!(core);
            let res = common::$common(core, dst, shift);
            dy!(core) = mask_out_below_8!(dst) | res;
            Ok(Cycles($cycles + 2 * shift as i32))
        });
    (16, $common:ident, $name:ident, 1, $dst:ident, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let shift = 1;
            let (dst, ea) = try!(operator::$dst(core));
            let res = common::$common(core, dst, shift);
            core.write_data_word(ea, mask_out_below_16!(dst) | res);
            Ok(Cycles($cycles))
        });
    (16, $common:ident, $name:ident, $shift_src:ident, dy, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let shift = try!(operator::$shift_src(core)) & 0x3f; // mod 64
            let dst = dy!(core);
            let res = common::$common(core, dst, shift);
            dy!(core) = mask_out_below_16!(dst) | res;
            Ok(Cycles($cycles + 2 * shift as i32))
        });
    (32, $common:ident, $name:ident, $shift_src:ident, dy, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let shift = try!(operator::$shift_src(core)) & 0x3f; // mod 64
            let dst = dy!(core);
            let res = common::$common(core, dst, shift);
            dy!(core) = res;
            Ok(Cycles($cycles + 2 * shift as i32))
        });
}

pub fn illegal(core: &mut Core) -> Result<Cycles> {
    Err(IllegalInstruction(core.ir, core.pc-2))
}
use super::InstructionSet;
pub fn instruction_set() -> InstructionSet {
    handlers::generate()
}
use std::num::Wrapping;
use super::operator;

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
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            // we must load original value from AX before the src
            // as the PI/PD addressing modes will change AX (if AX=AY)
            let dst = try!(operator::ax(core));
            let src = try!(operator::$src(core));
            ax!(core) = (Wrapping(dst) + Wrapping(src as i16 as u32)).0;
            Ok(Cycles($cycles))
        })
}
macro_rules! adda_32 {
    ($name:ident, $src:ident, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            // we must load original value from AX before the src
            // as the PI/PD addressing modes will change AX (if AX=AY)
            let dst = try!(operator::ax(core));
            let src = try!(operator::$src(core));
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
pub fn addq_16_an(core: &mut Core) -> Result<Cycles> {
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
pub fn addq_32_an(core: &mut Core) -> Result<Cycles> {
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

pub fn andi_16_toc(core: &mut Core) -> Result<Cycles> {
    let dst = core.condition_code_register();
    let src = mask_out_above_8!(try!(operator::imm_16(core))) as u16;
    core.ccr_to_flags(dst & src);
    Ok(Cycles(20))
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
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            Ok(if core.$cond()
            {
                let offset = mask_out_above_8!(core.ir) as i8;
                core.branch_8(offset);
                Cycles(10)
            } else {
                Cycles(8)
            })
        }
    };
    (16, $name:ident, $cond:ident) => {
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            Ok(if core.$cond()
            {
                let offset = try!(core.read_imm_i16());
                core.pc -= 2;
                core.branch_16(offset);
                Cycles(10)
            } else {
                core.pc += 2;
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
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let src = try!(operator::$src(core)) & 7; // modulo 8
            let (dst, ea) = try!(operator::$dst(core));
            let mask = 1 << src;
            core.not_z_flag = dst & mask;
            core.write_data_byte(ea, dst ^ mask);
            Ok(Cycles($cycles))
        });
}

macro_rules! bclr_8 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let src = try!(operator::$src(core)) & 7; // modulo 8
            let (dst, ea) = try!(operator::$dst(core));
            let mask = 1 << src;
            core.not_z_flag = dst & mask;
            core.write_data_byte(ea, dst & !mask);
            Ok(Cycles($cycles))
        });
}

macro_rules! bset_8 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let src = try!(operator::$src(core)) & 7; // modulo 8
            let (dst, ea) = try!(operator::$dst(core));
            let mask = 1 << src;
            core.not_z_flag = dst & mask;
            core.write_data_byte(ea, dst | mask);
            Ok(Cycles($cycles))
        });
}

macro_rules! btst_8 {
    ($name:ident, $src:ident, $dst:ident, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let src = try!(operator::$src(core)) & 7; // modulo 8
            let (dst, _) = try!(operator::$dst(core));
            let mask = 1 << src;
            core.not_z_flag = dst & mask;
            Ok(Cycles($cycles))
        });
}

pub fn bchg_32_r_dn(core: &mut Core) -> Result<Cycles> {
    let dst = dy!(core);
    let src = dx!(core);
    let mask = 1 << (src & 0x1f);

    core.not_z_flag = dst & mask;
    dy!(core) ^= mask;
    Ok(Cycles(8))
}

pub fn bchg_32_s_dn(core: &mut Core) -> Result<Cycles> {
    let dst = dy!(core);
    let src = try!(operator::imm_8(core));
    let mask = 1 << (src & 0x1f);

    core.not_z_flag = dst & mask;
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

pub fn bclr_32_r_dn(core: &mut Core) -> Result<Cycles> {
    let dst = dy!(core);
    let src = dx!(core);
    let mask = 1 << (src & 0x1f);

    core.not_z_flag = dst & mask;
    dy!(core) &= !mask;
    Ok(Cycles(10))
}

pub fn bclr_32_s_dn(core: &mut Core) -> Result<Cycles> {
    let dst = dy!(core);
    let src = try!(operator::imm_8(core));
    let mask = 1 << (src & 0x1f);

    core.not_z_flag = dst & mask;
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


pub fn bset_32_r_dn(core: &mut Core) -> Result<Cycles> {
    let dst = dy!(core);
    let src = dx!(core);
    let mask = 1 << (src & 0x1f);

    core.not_z_flag = dst & mask;
    dy!(core) |= mask;
    Ok(Cycles(8))
}

pub fn bset_32_s_dn(core: &mut Core) -> Result<Cycles> {
    let dst = dy!(core);
    let src = try!(operator::imm_8(core));
    let mask = 1 << (src & 0x1f);

    core.not_z_flag = dst & mask;
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


pub fn btst_32_r_dn(core: &mut Core) -> Result<Cycles> {
    let dst = dy!(core);
    let src = dx!(core);
    let mask = 1 << (src & 0x1f);

    core.not_z_flag = dst & mask;
    Ok(Cycles(6))
}

pub fn btst_32_s_dn(core: &mut Core) -> Result<Cycles> {
    let dst = dy!(core);
    let src = try!(operator::imm_8(core));
    let mask = 1 << (src & 0x1f);

    core.not_z_flag = dst & mask;
    Ok(Cycles(10))
}

btst_8!(btst_8_r_ai, dx,    ea_ay_ai_8, 4+4 );
btst_8!(btst_8_r_pi, dx,    ea_ay_pi_8, 4+4 );
btst_8!(btst_8_r_pd, dx,    ea_ay_pd_8, 4+6 );
btst_8!(btst_8_r_di, dx,    ea_ay_di_8, 4+8 );
btst_8!(btst_8_r_ix, dx,    ea_ay_ix_8, 4+10);
btst_8!(btst_8_r_aw, dx,    ea_aw_8,    4+8 );
btst_8!(btst_8_r_al, dx,    ea_al_8,    4+12);
btst_8!(btst_8_s_ai, imm_8, ea_ay_ai_8, 8+4 );
btst_8!(btst_8_s_pi, imm_8, ea_ay_pi_8, 8+4 );
btst_8!(btst_8_s_pd, imm_8, ea_ay_pd_8, 8+6 );
btst_8!(btst_8_s_di, imm_8, ea_ay_di_8, 8+8 );
btst_8!(btst_8_s_ix, imm_8, ea_ay_ix_8, 8+10);
btst_8!(btst_8_s_aw, imm_8, ea_aw_8,    8+8 );
btst_8!(btst_8_s_al, imm_8, ea_al_8,    8+12);

pub fn bra_8(core: &mut Core) -> Result<Cycles> {
    let offset = mask_out_above_8!(core.ir) as i8;
    core.branch_8(offset);
    Ok(Cycles(10))
}

pub fn bra_16(core: &mut Core) -> Result<Cycles> {
    let offset = try!(core.read_imm_i16());
    core.pc -= 2;
    core.branch_16(offset);
    Ok(Cycles(10))
}

pub fn bsr_8(core: &mut Core) -> Result<Cycles> {
    let offset = mask_out_above_8!(core.ir) as i8;
    let pc = core.pc;
    core.push_32(pc);
    core.branch_8(offset);
    Ok(Cycles(18))
}

pub fn bsr_16(core: &mut Core) -> Result<Cycles> {
    let offset = try!(core.read_imm_i16());
    let pc = core.pc;
    core.push_32(pc);
    core.pc -= 2;
    core.branch_16(offset);
    Ok(Cycles(18))
}

macro_rules! chk_16 {
    ($name:ident, $dst:ident, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            let src = dx!(core) as i16;
            let bound = try!(operator::$dst(core)) as i16;

            core.not_z_flag = src as u32 & 0xffff;
            core.v_flag = 0;
            core.c_flag = 0;

            if src >= 0 && src <= bound
            {
                Ok(Cycles($cycles))
            } else {
                core.n_flag = if src < 0 {1 << 7} else {0};
                // unclear if we should deduct the 10 cycles for the instruction, or not?
                // if they are already included in the 40-cycle cost of the exception
                // we include them twice now
                Err(Trap(EXCEPTION_CHK, $cycles))
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

macro_rules! clr_any_try {
    ($name:ident, $dst:ident, $write_op:ident, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            // The MC68000PRM says: In the MC68000 and MC68008 a memory location is read before it is cleared.
            // We skip this as Musashi doesn't do that either.
            let ea = try!(effective_address::$dst(core));

            core.$write_op(ea, 0);

            core.n_flag = 0;
            core.v_flag = 0;
            core.c_flag = 0;
            core.not_z_flag = 0;
            Ok(Cycles($cycles))
        });
}

macro_rules! clr_any {
    ($name:ident, $dst:ident, $write_op:ident, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            // The MC68000PRM says: In the MC68000 and MC68008 a memory location is read before it is cleared.
            // We skip this as Musashi doesn't do that either.
            let ea = effective_address::$dst(core);

            core.$write_op(ea, 0);

            core.n_flag = 0;
            core.v_flag = 0;
            core.c_flag = 0;
            core.not_z_flag = 0;
            Ok(Cycles($cycles))
        });
}

pub fn clr_8_dn(core: &mut Core) -> Result<Cycles> {
    dy!(core) &= 0xffffff00;

    core.n_flag = 0;
    core.v_flag = 0;
    core.c_flag = 0;
    core.not_z_flag = 0;
    Ok(Cycles(4))
}
clr_any!(clr_8_ai,     address_indirect_ay, write_data_byte, 8+4);
clr_any!(clr_8_pi,     postincrement_ay_8,  write_data_byte, 8+4);
clr_any!(clr_8_pd,     predecrement_ay_8,   write_data_byte, 8+6);
clr_any_try!(clr_8_di, displacement_ay,     write_data_byte, 8+8);
clr_any_try!(clr_8_ix, index_ay,            write_data_byte, 8+10);
clr_any_try!(clr_8_aw, absolute_word,       write_data_byte, 8+8);
clr_any_try!(clr_8_al, absolute_long,       write_data_byte, 8+12);

pub fn clr_16_dn(core: &mut Core) -> Result<Cycles> {
    dy!(core) &= 0xffff0000;

    core.n_flag = 0;
    core.v_flag = 0;
    core.c_flag = 0;
    core.not_z_flag = 0;
    Ok(Cycles(4))
}
clr_any!(clr_16_ai,     address_indirect_ay, write_data_word, 8+4);
clr_any!(clr_16_pi,     postincrement_ay_16, write_data_word, 8+4);
clr_any!(clr_16_pd,     predecrement_ay_16,  write_data_word, 8+6);
clr_any_try!(clr_16_di, displacement_ay,     write_data_word, 8+8);
clr_any_try!(clr_16_ix, index_ay,            write_data_word, 8+10);
clr_any_try!(clr_16_aw, absolute_word,       write_data_word, 8+8);
clr_any_try!(clr_16_al, absolute_long,       write_data_word, 8+12);

pub fn clr_32_dn(core: &mut Core) -> Result<Cycles> {
    dy!(core) = 0;

    core.n_flag = 0;
    core.v_flag = 0;
    core.c_flag = 0;
    core.not_z_flag = 0;
    Ok(Cycles(6))
}
clr_any!(clr_32_ai, 	address_indirect_ay, write_data_long, 12+8);
clr_any!(clr_32_pi, 	postincrement_ay_32, write_data_long, 12+8);
clr_any!(clr_32_pd, 	predecrement_ay_32,  write_data_long, 12+10);
clr_any_try!(clr_32_di, displacement_ay,     write_data_long, 12+12);
clr_any_try!(clr_32_ix, index_ay,            write_data_long, 12+14);
clr_any_try!(clr_32_aw, absolute_word,       write_data_long, 12+12);
clr_any_try!(clr_32_al, absolute_long,       write_data_long, 12+16);

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
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            // we must load original value from AX before the src
            // as the PI/PD addressing modes will change AX (if AX=AY)
            let dst = try!(operator::ax(core));
            let src = try!(operator::$src(core)) as i16 as u32;
            let _ = common::cmp_32(core, dst, src);
            Ok(Cycles($cycles))
        })
}
macro_rules! cmpa_32 {
    ($name:ident, $src:ident, $cycles:expr) => (
        pub fn $name(core: &mut Core) -> Result<Cycles> {
            // we must load original value from AX before the src
            // as the PI/PD addressing modes will change AX (if AX=AY)
            let dst = try!(operator::ax(core));
            let src = try!(operator::$src(core));
            let _ = common::cmp_32(core, dst, src);
            Ok(Cycles($cycles))
        })
}
cmpa_16!(cmpa_16_dn, dy,         6+0);
cmpa_16!(cmpa_16_an, ay,         6+0);
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

cmpa_32!(cmpa_32_dn, dy,         6+0);
cmpa_32!(cmpa_32_an, ay,         6+0);
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
