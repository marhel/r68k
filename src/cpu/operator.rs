
use super::effective_address;
use super::{Core, Result};

pub fn ea_ay_pd_8(core: &mut Core) -> Result<(u32, u32)> {
	let ea = effective_address::predecrement_ay_8(core);
	core.read_data_byte(ea).map(|val| (val, ea))
}
pub fn ea_ax_pd_8(core: &mut Core) -> Result<(u32, u32)> {
	let ea = effective_address::predecrement_ax_8(core);
	core.read_data_byte(ea).map(|val| (val, ea))
}
pub fn ea_ay_pi_8(core: &mut Core) -> Result<(u32, u32)> {
	let ea = effective_address::postincrement_ay_8(core);
	core.read_data_byte(ea).map(|val| (val, ea))
}
pub fn ea_ay_ai_8(core: &mut Core) -> Result<(u32, u32)> {
	let ea = effective_address::address_indirect_ay(core);
	core.read_data_byte(ea).map(|val| (val, ea))
}
pub fn ea_ay_di_8(core: &mut Core) -> Result<(u32, u32)> {
	let ea = try!(effective_address::displacement_ay(core));
	core.read_data_byte(ea).map(|val| (val, ea))
}
pub fn ea_ay_ix_8(core: &mut Core) -> Result<(u32, u32)> {
	let ea = try!(effective_address::index_ay(core));
	core.read_data_byte(ea).map(|val| (val, ea))
}
pub fn ea_aw_8(core: &mut Core) -> Result<(u32, u32)> {
	let ea = try!(effective_address::absolute_word(core));
	core.read_data_byte(ea).map(|val| (val, ea))
}
pub fn ea_al_8(core: &mut Core) -> Result<(u32, u32)> {
	let ea = try!(effective_address::absolute_long(core));
	core.read_data_byte(ea).map(|val| (val, ea))
}

pub fn ay_pd_8(core: &mut Core) -> Result<u32> {
	let ea = effective_address::predecrement_ay_8(core);
	core.read_data_byte(ea)
}
pub fn ay_pi_8(core: &mut Core) -> Result<u32> {
	let ea = effective_address::postincrement_ay_8(core);
	core.read_data_byte(ea)
}
pub fn ay_ai_8(core: &mut Core) -> Result<u32> {
	let ea = effective_address::address_indirect_ay(core);
	core.read_data_byte(ea)
}
pub fn ay_di_8(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::displacement_ay(core));
	core.read_data_byte(ea)
}
pub fn ay_ix_8(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::index_ay(core));
	core.read_data_byte(ea)
}
pub fn aw_8(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::absolute_word(core));
	core.read_data_byte(ea)
}
pub fn al_8(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::absolute_long(core));
	core.read_data_byte(ea)
}
pub fn pcdi_8(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::displacement_pc(core));
	core.read_program_byte(ea)
}
pub fn pcix_8(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::index_pc(core));
	core.read_program_byte(ea)
}
pub fn imm_8(core: &mut Core) -> Result<u32> {
	let extension = try!(core.read_imm_u16());
	Ok(mask_out_above_8!(extension) as u32)
}


pub fn ea_ay_pd_16(core: &mut Core) -> Result<(u32, u32)> {
	let ea = effective_address::predecrement_ay_16(core);
	core.read_data_word(ea).map(|val| (val, ea))
}
pub fn ea_ay_pi_16(core: &mut Core) -> Result<(u32, u32)> {
	let ea = effective_address::postincrement_ay_16(core);
	core.read_data_word(ea).map(|val| (val, ea))
}
pub fn ea_ay_ai_16(core: &mut Core) -> Result<(u32, u32)> {
	let ea = effective_address::address_indirect_ay(core);
	core.read_data_word(ea).map(|val| (val, ea))
}
pub fn ea_ay_di_16(core: &mut Core) -> Result<(u32, u32)> {
	let ea = try!(effective_address::displacement_ay(core));
	core.read_data_word(ea).map(|val| (val, ea))
}
pub fn ea_ay_ix_16(core: &mut Core) -> Result<(u32, u32)> {
	let ea = try!(effective_address::index_ay(core));
	core.read_data_word(ea).map(|val| (val, ea))
}
pub fn ea_aw_16(core: &mut Core) -> Result<(u32, u32)> {
	let ea = try!(effective_address::absolute_word(core));
	core.read_data_word(ea).map(|val| (val, ea))
}
pub fn ea_al_16(core: &mut Core) -> Result<(u32, u32)> {
	let ea = try!(effective_address::absolute_long(core));
	core.read_data_word(ea).map(|val| (val, ea))
}

pub fn ay_pd_16(core: &mut Core) -> Result<u32> {
	let ea = effective_address::predecrement_ay_16(core);
	core.read_data_word(ea)
}
pub fn ay_pi_16(core: &mut Core) -> Result<u32> {
	let ea = effective_address::postincrement_ay_16(core);
	core.read_data_word(ea)
}
pub fn ay_ai_16(core: &mut Core) -> Result<u32> {
	let ea = effective_address::address_indirect_ay(core);
	core.read_data_word(ea)
}
pub fn ay_di_16(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::displacement_ay(core));
	core.read_data_word(ea)
}
pub fn ay_ix_16(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::index_ay(core));
	core.read_data_word(ea)
}
pub fn aw_16(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::absolute_word(core));
	core.read_data_word(ea)
}
pub fn al_16(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::absolute_long(core));
	core.read_data_word(ea)
}
pub fn pcdi_16(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::displacement_pc(core));
	core.read_program_word(ea)
}
pub fn pcix_16(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::index_pc(core));
	core.read_program_word(ea)
}
pub fn imm_16(core: &mut Core) -> Result<u32> {
	let extension = try!(core.read_imm_i16());
	Ok(extension as u32)
}

pub fn ea_ay_pd_32(core: &mut Core) -> Result<(u32, u32)> {
	let ea = effective_address::predecrement_ay_32(core);
	core.read_data_long(ea).map(|val| (val, ea))
}
pub fn ea_ay_pi_32(core: &mut Core) -> Result<(u32, u32)> {
	let ea = effective_address::postincrement_ay_32(core);
	core.read_data_long(ea).map(|val| (val, ea))
}
pub fn ea_ay_ai_32(core: &mut Core) -> Result<(u32, u32)> {
	let ea = effective_address::address_indirect_ay(core);
	core.read_data_long(ea).map(|val| (val, ea))
}
pub fn ea_ay_di_32(core: &mut Core) -> Result<(u32, u32)> {
	let ea = try!(effective_address::displacement_ay(core));
	core.read_data_long(ea).map(|val| (val, ea))
}
pub fn ea_ay_ix_32(core: &mut Core) -> Result<(u32, u32)> {
	let ea = try!(effective_address::index_ay(core));
	core.read_data_long(ea).map(|val| (val, ea))
}
pub fn ea_aw_32(core: &mut Core) -> Result<(u32, u32)> {
	let ea = try!(effective_address::absolute_word(core));
	core.read_data_long(ea).map(|val| (val, ea))
}
pub fn ea_al_32(core: &mut Core) -> Result<(u32, u32)> {
	let ea = try!(effective_address::absolute_long(core));
	core.read_data_long(ea).map(|val| (val, ea))
}

pub fn ay_pd_32(core: &mut Core) -> Result<u32> {
	let ea = effective_address::predecrement_ay_32(core);
	core.read_data_long(ea)
}
pub fn ay_pi_32(core: &mut Core) -> Result<u32> {
	let ea = effective_address::postincrement_ay_32(core);
	core.read_data_long(ea)
}
pub fn ay_ai_32(core: &mut Core) -> Result<u32> {
	let ea = effective_address::address_indirect_ay(core);
	core.read_data_long(ea)
}
pub fn ay_di_32(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::displacement_ay(core));
	core.read_data_long(ea)
}
pub fn ay_ix_32(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::index_ay(core));
	core.read_data_long(ea)
}
pub fn aw_32(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::absolute_word(core));
	core.read_data_long(ea)
}
pub fn al_32(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::absolute_long(core));
	core.read_data_long(ea)
}
pub fn pcdi_32(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::displacement_pc(core));
	core.read_program_long(ea)
}
pub fn pcix_32(core: &mut Core) -> Result<u32> {
	let ea = try!(effective_address::index_pc(core));
	core.read_program_long(ea)
}
pub fn imm_32(core: &mut Core) -> Result<u32> {
	let extension = try!(core.read_imm_u32());
	Ok(extension)
}
pub fn dx(core: &mut Core) -> Result<u32> {
	Ok(dx!(core))
}
pub fn dy(core: &mut Core) -> Result<u32> {
	Ok(dy!(core))
}
pub fn ay(core: &mut Core) -> Result<u32> {
	Ok(ay!(core))
}
pub fn ax(core: &mut Core) -> Result<u32> {
	Ok(ax!(core))
}
#[cfg(test)]
mod tests {
	use super::super::Core;
	use super::super::Exception::AddressError;
	use super::{ea_ax_pd_8, ay_pd_8, ay_ai_16};
	use ram::{AddressBus, SUPERVISOR_DATA};

	#[test]
	fn test_ax_predecrement_8() {
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
		let (dst, ea) = ea_ax_pd_8(core).unwrap();
		assert_eq!(0x44, dst);
		assert_eq!(512+4*4-1, core.dar[8+4]);
		assert_eq!(512+4*4-1, ea);

		core.ir = 0b1111_1111_1111_1111; // X=7, Y=7
		assert_eq!(512+4*7, core.dar[8+7]);
		let (dst, ea) = ea_ax_pd_8(core).unwrap();
		assert_eq!(0x77, dst);
		// A7 is kept even
		assert_eq!(512+4*7-2, core.dar[8+7]);
		assert_eq!(512+4*7-2, ea);
	}
	#[test]
	fn test_ay_predecrement_8() {
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
		assert_eq!(0x22, ay_pd_8(core).unwrap());
		assert_eq!(512+4*2-1, core.dar[8+2]);

		core.ir = 0b1111_1011_1111_1111; // X=5, Y=7
		assert_eq!(512+4*7, core.dar[8+7]);
		assert_eq!(0x77, ay_pd_8(core).unwrap());
		// A7 is kept even
		assert_eq!(512+4*7-2, core.dar[8+7]);
	}
	#[test]
	fn test_address_error_on_odd_addresses() {
		let mut core = Core::new(0x40);
		core.dar[8] = 0x11; // odd address
		core.ir = 0b1111_1001_1111_1000; // X=4, Y=0
		match ay_ai_16(&mut core) {
			Err(AddressError{..}) => (), // good!
			_ => panic!("Unexpected"),
		};
	}
	#[test]
	fn test_no_address_error_on_even_addresses() {
		let mut core = Core::new(0x40);
		core.dar[8] = 0x12; // even address
		core.ir = 0b1111_1001_1111_1000; // X=4, Y=0
		match ay_ai_16(&mut core) {
			Ok(_) => (), // good!
			_ => panic!("Unexpected"),
		};
	}
}
