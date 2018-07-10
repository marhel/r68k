use super::{Core, Result};
use std::num::Wrapping;

pub fn absolute_word<T: Core>(core: &mut T) -> Result<u32> {
    core.read_imm_i16().map(|res| res as u32)
}
pub fn absolute_long<T: Core>(core: &mut T) -> Result<u32> {
    core.read_imm_u32()
}
pub fn predecrement_ay_8<T: Core>(core: &mut T) -> Result<u32> {
    let reg_ndx = ir_ay!(core);
    Ok(predecrement_8(core, reg_ndx))
}
pub fn postincrement_ay_8<T: Core>(core: &mut T) -> Result<u32> {
    let reg_ndx = ir_ay!(core);
    Ok(postincrement_8(core, reg_ndx))
}
pub fn predecrement_ay_16<T: Core>(core: &mut T) -> Result<u32> {
    let reg_ndx = ir_ay!(core);
    Ok(predecrement_16(core, reg_ndx))
}
pub fn postincrement_ay_16<T: Core>(core: &mut T) -> Result<u32> {
    let reg_ndx = ir_ay!(core);
    Ok(postincrement_16(core, reg_ndx))
}
pub fn predecrement_ay_32<T: Core>(core: &mut T) -> Result<u32> {
    let reg_ndx = ir_ay!(core);
    Ok(predecrement_32(core, reg_ndx))
}
pub fn postincrement_ay_32<T: Core>(core: &mut T) -> Result<u32> {
    let reg_ndx = ir_ay!(core);
    Ok(postincrement_32(core, reg_ndx))
}
pub fn address_indirect_ay<T: Core>(core: &mut T) -> Result<u32> {
    Ok(ay!(core))
}
pub fn address_indirect_ax<T: Core>(core: &mut T) -> Result<u32> {
    Ok(ax!(core))
}
pub fn displacement_ay<T: Core>(core: &mut T) -> Result<u32> {
    let reg_val = ay!(core);
    displacement(core, reg_val)
}
pub fn displacement_ax<T: Core>(core: &mut T) -> Result<u32> {
    let reg_val = ax!(core);
    displacement(core, reg_val)
}
pub fn displacement_pc<T: Core>(core: &mut T) -> Result<u32> {
    let old_pc = pc!(core);
    displacement(core, old_pc)
}
pub fn index_ay<T: Core>(core: &mut T) -> Result<u32> {
    let reg_val = ay!(core);
    index(core, reg_val)
}
pub fn index_ax<T: Core>(core: &mut T) -> Result<u32> {
    let reg_val = ax!(core);
    index(core, reg_val)
}
pub fn index_pc<T: Core>(core: &mut T) -> Result<u32> {
    let pc = pc!(core);
    index(core, pc)
}
pub fn predecrement_ax_8<T: Core>(core: &mut T) -> Result<u32> {
    let reg_ndx = ir_ax!(core);
    Ok(predecrement_8(core, reg_ndx))
}
pub fn predecrement_ax_16<T: Core>(core: &mut T) -> Result<u32> {
    let reg_ndx = ir_ax!(core);
    Ok(predecrement_16(core, reg_ndx))
}
pub fn predecrement_ax_32<T: Core>(core: &mut T) -> Result<u32> {
    let reg_ndx = ir_ax!(core);
    Ok(predecrement_32(core, reg_ndx))
}
pub fn postincrement_ax_8<T: Core>(core: &mut T) -> Result<u32> {
    let reg_ndx = ir_ax!(core);
    Ok(postincrement_8(core, reg_ndx))
}
pub fn postincrement_ax_16<T: Core>(core: &mut T) -> Result<u32> {
    let reg_ndx = ir_ax!(core);
    Ok(postincrement_16(core, reg_ndx))
}
pub fn postincrement_ax_32<T: Core>(core: &mut T) -> Result<u32> {
    let reg_ndx = ir_ax!(core);
    Ok(postincrement_32(core, reg_ndx))
}

fn predecrement_8<T: Core>(core: &mut T, reg_ndx: usize) -> u32 {
    // pre-decrement
    dar!(core)[reg_ndx] = (Wrapping(dar!(core)[reg_ndx]) - match reg_ndx {
        15 => Wrapping(2), // A7 is kept even
         _ => Wrapping(1)
    }).0;
    dar!(core)[reg_ndx]
}
fn postincrement_8<T: Core>(core: &mut T, reg_ndx: usize) -> u32 {
    // post-increment
    let ea = dar!(core)[reg_ndx];
    dar!(core)[reg_ndx] = (Wrapping(dar!(core)[reg_ndx]) + match reg_ndx {
        15 => Wrapping(2), // A7 is kept even
         _ => Wrapping(1)
    }).0;
    ea
}
fn predecrement_16<T: Core>(core: &mut T, reg_ndx: usize) -> u32 {
    // pre-decrement
    dar!(core)[reg_ndx] = (Wrapping(dar!(core)[reg_ndx]) - Wrapping(2)).0;
    dar!(core)[reg_ndx]
}
fn postincrement_16<T: Core>(core: &mut T, reg_ndx: usize) -> u32 {
    // post-increment
    let ea = dar!(core)[reg_ndx];
    dar!(core)[reg_ndx] = (Wrapping(dar!(core)[reg_ndx]) + Wrapping(2)).0;
    ea
}
fn predecrement_32<T: Core>(core: &mut T, reg_ndx: usize) -> u32 {
    // pre-decrement
    dar!(core)[reg_ndx] = (Wrapping(dar!(core)[reg_ndx]) - Wrapping(4)).0;
    dar!(core)[reg_ndx]
}
fn postincrement_32<T: Core>(core: &mut T, reg_ndx: usize) -> u32 {
    // post-increment
    let ea = dar!(core)[reg_ndx];
    dar!(core)[reg_ndx] = (Wrapping(dar!(core)[reg_ndx]) + Wrapping(4)).0;
    ea
}
pub fn displacement<T: Core>(core: &mut T, reg_val: u32) -> Result<u32> {
    let displacement = try!(core.read_imm_i16());
    let ea = (Wrapping(reg_val) + Wrapping(displacement as u32)).0;
    Ok(ea)
}
// Brief Extension Word format (see M68000 PRM section 2.1)
const LONG_INDEX_MASK: u16 = 0x0800;
fn index<T: Core>(core: &mut T, reg_val: u32) -> Result<u32> {
    let extension = try!(core.read_imm_u16());
    // top four bits = (D/A RRR) matches our register array layout
    let xreg_ndx = (extension>>12) as usize;
    let xn = dar!(core)[xreg_ndx];
    let xn = if (extension & LONG_INDEX_MASK) > 0 {xn} else {(xn as i16) as u32};

      let index = extension as i8;
    let ea = (Wrapping(reg_val) + Wrapping(xn) + Wrapping(index as u32)).0;
    Ok(ea)
}

#[cfg(test)]
mod tests {
    use super::super::TestCore;
    use super::super::effective_address::{predecrement_8, postincrement_8};

    #[test]
    fn predecrement_wraps() {
        let mut core = TestCore::new(0x40);
        for i in 0..8 {
            // pre-decrement should wrap to 0xFFFFFFFF
            core.dar[8+i as usize] = 0;
        }
        let ea = predecrement_8(&mut core, 8+0);
        assert_eq!(0xFFFFFFFF, ea);
    }
    #[test]
    fn predecrement_8_wraps_a7_by_two() {
        let mut core = TestCore::new(0x40);
        for i in 0..8 {
            // pre-decrement should wrap to 0xFFFFFFFF
            core.dar[8+i as usize] = 0;
        }
        let ea = predecrement_8(&mut core, 8+7);
        // a7 is kept even
        assert_eq!(0xFFFFFFFE, ea);
    }
    #[test]
    fn postincrement_wraps() {
        let mut core = TestCore::new(0x40);
        for i in 0..8 {
            // pre-decrement should wrap to 0xFFFFFFFF
            core.dar[8+i as usize] = 0xFFFFFFFF;
        }
        let ea = postincrement_8(&mut core, 8+0);
        assert_eq!(0xFFFFFFFF, ea);
        assert_eq!(0x0, core.dar[8+0]);
    }
    #[test]
    fn postincrement_8_wraps_a7_by_two() {
        let mut core = TestCore::new(0x40);
        for i in 0..8 {
            // pre-decrement should wrap to 0xFFFFFFFF
            core.dar[8+i as usize] = 0xFFFFFFFE;
        }
        let ea = postincrement_8(&mut core, 8+7);
        // a7 is kept even
        assert_eq!(0xFFFFFFFE, ea);
        assert_eq!(0x0, core.dar[8+7]);
    }
}
