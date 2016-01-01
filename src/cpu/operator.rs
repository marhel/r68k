
use super::effective_address;
use super::Core;

pub fn ay_pd_8(core: &mut Core) -> u32 {
	let ea = effective_address::predecrement_ay(core);
	core.read_data_byte(ea)
}
pub fn ax_pd_8(core: &mut Core) -> (u32, u32) {
	let ea = effective_address::predecrement_ax(core);
	(core.read_data_byte(ea), ea)
}
pub fn ay_pi_8(core: &mut Core) -> u32 {
	let ea = effective_address::postincrement_ay(core);
	core.read_data_byte(ea)
}
pub fn ay_ai_8(core: &mut Core) -> u32 {
	let ea = effective_address::address_indirect_ay(core);
	core.read_data_byte(ea)
}
pub fn ay_di_8(core: &mut Core) -> u32 {
	let ea = effective_address::displacement_ay(core);
	core.read_data_byte(ea)
}
pub fn ay_ix_8(core: &mut Core) -> u32 {
	let ea = effective_address::index_ay(core);
	core.read_data_byte(ea)
}
pub fn aw_8(core: &mut Core) -> u32 {
	let ea = effective_address::absolute_word(core);
	core.read_data_byte(ea)
}
pub fn al_8(core: &mut Core) -> u32 {
	let ea = effective_address::absolute_long(core);
	core.read_data_byte(ea)
}
pub fn pcdi_8(core: &mut Core) -> u32 {
	let ea = effective_address::displacement_pc(core);
	core.read_program_byte(ea)
}
pub fn pcix_8(core: &mut Core) -> u32 {
	let ea = effective_address::index_pc(core);
	core.read_program_byte(ea)
}
pub fn imm_8(core: &mut Core) -> u32 {
	let extension = core.read_imm_u16();
	mask_out_above_8!(extension) as u32
}
pub fn dx(core: &mut Core) -> u32 {
	dx!(core)
}
pub fn dy(core: &mut Core) -> u32 {
	dy!(core)
}
pub fn ax(core: &mut Core) -> u32 {
	ax!(core)
}
pub fn ay(core: &mut Core) -> u32 {
	ay!(core)
}
#[cfg(test)]
mod tests {
	use super::super::Core;
	use super::{ax_pd_8, ay_pd_8};
	use ram::{AddressBus, SUPERVISOR_DATA};

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
		let (dst, ea) = ax_pd_8(core);
		assert_eq!(0x44, dst);
		assert_eq!(512+4*4-1, core.dar[8+4]);
		assert_eq!(512+4*4-1, ea);

		core.ir = 0b1111_1111_1111_1111; // X=7, Y=7
		assert_eq!(512+4*7, core.dar[8+7]);
		let (dst, ea) = ax_pd_8(core);
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
		assert_eq!(0x22, ay_pd_8(core));
		assert_eq!(512+4*2-1, core.dar[8+2]);

		core.ir = 0b1111_1011_1111_1111; // X=5, Y=7
		assert_eq!(512+4*7, core.dar[8+7]);
		assert_eq!(0x77, ay_pd_8(core));
		// A7 is kept even
		assert_eq!(512+4*7-2, core.dar[8+7]);
	}
}
