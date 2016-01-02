use super::Core;
use std::num::Wrapping;
use ram::ADDRBUS_MASK;

pub fn absolute_word(core: &mut Core) -> u32 {
	let ea = core.read_imm_i16() as u32;
	ea & ADDRBUS_MASK
}
pub fn absolute_long(core: &mut Core) -> u32 {
	let ea = core.read_imm_u32();
	ea & ADDRBUS_MASK
}
pub fn predecrement_ay_8(core: &mut Core) -> u32 {
	let reg_ndx = ir_ay!(core);
	predecrement_8(core, reg_ndx)
}
pub fn postincrement_ay_8(core: &mut Core) -> u32 {
	let reg_ndx = ir_ay!(core);
	postincrement_8(core, reg_ndx)
}
pub fn predecrement_ay_16(core: &mut Core) -> u32 {
	let reg_ndx = ir_ay!(core);
	predecrement_16(core, reg_ndx)
}
pub fn postincrement_ay_16(core: &mut Core) -> u32 {
	let reg_ndx = ir_ay!(core);
	postincrement_16(core, reg_ndx)
}
pub fn address_indirect_ay(core: &mut Core) -> u32 {
	let reg_ndx = ir_ay!(core);
	core.dar[reg_ndx] & ADDRBUS_MASK
}
pub fn displacement_ay(core: &mut Core) -> u32 {
	let reg_val = core.dar[ir_ay!(core)];
	displacement(core, reg_val)
}
pub fn displacement_pc(core: &mut Core) -> u32 {
	let old_pc = core.pc;
	displacement(core, old_pc)
}
pub fn index_ay(core: &mut Core) -> u32 {
	let reg_val = core.dar[ir_ay!(core)];
	index(core, reg_val)
}
pub fn index_pc(core: &mut Core) -> u32 {
	let pc = core.pc;
	index(core, pc)
}
pub fn predecrement_ax_8(core: &mut Core) -> u32 {
	let reg_ndx = ir_ax!(core);
	predecrement_8(core, reg_ndx)
}
pub fn predecrement_ax_16(core: &mut Core) -> u32 {
	let reg_ndx = ir_ax!(core);
	predecrement_16(core, reg_ndx)
}

fn predecrement_8(core: &mut Core, reg_ndx: usize) -> u32 {
	// pre-decrement
	core.dar[reg_ndx] = (Wrapping(core.dar[reg_ndx]) - match reg_ndx {
		15 => Wrapping(2), // A7 is kept even
		 _ => Wrapping(1)
	}).0;
	core.dar[reg_ndx] & ADDRBUS_MASK
}
fn postincrement_8(core: &mut Core, reg_ndx: usize) -> u32 {
	// post-increment
	let ea = core.dar[reg_ndx];
	core.dar[reg_ndx] = (Wrapping(core.dar[reg_ndx]) + match reg_ndx {
		15 => Wrapping(2), // A7 is kept even
		 _ => Wrapping(1)
	}).0;
	ea & ADDRBUS_MASK
}
fn predecrement_16(core: &mut Core, reg_ndx: usize) -> u32 {
	// pre-decrement
	core.dar[reg_ndx] = (Wrapping(core.dar[reg_ndx]) - Wrapping(2)).0;
	core.dar[reg_ndx] & ADDRBUS_MASK
}
fn postincrement_16(core: &mut Core, reg_ndx: usize) -> u32 {
	// post-increment
	let ea = core.dar[reg_ndx];
	core.dar[reg_ndx] = (Wrapping(core.dar[reg_ndx]) + Wrapping(2)).0;
	ea & ADDRBUS_MASK
}
fn displacement(core: &mut Core, reg_val: u32) -> u32 {
	let displacement = core.read_imm_i16();
	let ea = (Wrapping(reg_val) + Wrapping(displacement as u32)).0;
	ea & ADDRBUS_MASK
}
// Brief Extension Word format (see M68000 PRM section 2.1)
const LONG_INDEX_MASK: u16 = 0x0800;
fn index(core: &mut Core, reg_val: u32) -> u32 {
	let extension = core.read_imm_u16();
	let xreg_ndx = (extension>>12) as usize;
	let xn = core.dar[xreg_ndx];
	let xn = if (extension & LONG_INDEX_MASK) > 0 {xn} else {(xn as i16) as u32};

  	let index = extension as i8;
	let ea = (Wrapping(reg_val) + Wrapping(xn) + Wrapping(index as u32)).0;
	ea & ADDRBUS_MASK
}

#[cfg(test)]
mod tests {
	use super::super::Core;
	use super::super::effective_address::{predecrement_8, postincrement_8};
	use ram::{ADDRBUS_MASK};

	#[test]
	fn predecrement_wraps() {
		let mut core = Core::new(0x40);
		for i in 0..8 {
			// pre-decrement should wrap to 0xFFFFFFFF
			// but the following read should be from address 0x00FFFFFF
			// i.e. limited by the 24-bit address bus width
			core.dar[8+i as usize] = 0;
		}
		let ea = predecrement_8(&mut core, 8+0);
		assert_eq!(0x00FFFFFF, ea);
	}
	#[test]
	fn predecrement_wraps_a7_by_two() {
		let mut core = Core::new(0x40);
		for i in 0..8 {
			// pre-decrement should wrap to 0xFFFFFFFF
			// but the following read should be from address 0x00FFFFFF
			// i.e. limited by the 24-bit address bus width
			core.dar[8+i as usize] = 0;
		}
		let ea = predecrement_8(&mut core, 8+7);
		// a7 is kept even
		assert_eq!(0x00FFFFFE, ea);
	}
	#[test]
	fn postincrement_wraps() {
		let mut core = Core::new(0x40);
		for i in 0..8 {
			// pre-decrement should wrap to 0xFFFFFFFF
			// but the following read should be from address 0x00FFFFFF
			// i.e. limited by the 24-bit address bus width
			core.dar[8+i as usize] = 0xFFFFFFFF;
		}
		let ea = postincrement_8(&mut core, 8+0);
		assert_eq!(0xFFFFFFFF & ADDRBUS_MASK, ea);
		assert_eq!(0x0, core.dar[8+0]);
	}
	#[test]
	fn postincrement_wraps_a7_by_two() {
		let mut core = Core::new(0x40);
		for i in 0..8 {
			// pre-decrement should wrap to 0xFFFFFFFF
			// but the following read should be from address 0x00FFFFFF
			// i.e. limited by the 24-bit address bus width
			core.dar[8+i as usize] = 0xFFFFFFFE;
		}
		let ea = postincrement_8(&mut core, 8+7);
		// a7 is kept even
		assert_eq!(0xFFFFFFFE & ADDRBUS_MASK, ea);
		assert_eq!(0x0, core.dar[8+7]);
	}
}
