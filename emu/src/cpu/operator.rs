
use super::effective_address;
use super::{Core, Result};

pub fn ea_ay_pd_8<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::predecrement_ay_8(core)
    .and_then(|ea| core.read_data_byte(ea).map(|val| (val, ea)))
}
pub fn ea_ax_pd_8<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::predecrement_ax_8(core)
    .and_then(|ea| core.read_data_byte(ea).map(|val| (val, ea)))
}
pub fn ea_ay_pi_8<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::postincrement_ay_8(core)
    .and_then(|ea| core.read_data_byte(ea).map(|val| (val, ea)))
}
pub fn ea_ay_ai_8<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::address_indirect_ay(core)
    .and_then(|ea| core.read_data_byte(ea).map(|val| (val, ea)))
}
pub fn ea_ay_di_8<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::displacement_ay(core)
    .and_then(|ea| core.read_data_byte(ea).map(|val| (val, ea)))
}
pub fn ea_ay_ix_8<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::index_ay(core)
    .and_then(|ea| core.read_data_byte(ea).map(|val| (val, ea)))
}
pub fn ea_aw_8<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::absolute_word(core)
    .and_then(|ea| core.read_data_byte(ea).map(|val| (val, ea)))
}
pub fn ea_al_8<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::absolute_long(core)
    .and_then(|ea| core.read_data_byte(ea).map(|val| (val, ea)))
}

pub fn ay_pd_8<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::predecrement_ay_8(core)
    .and_then(|ea| core.read_data_byte(ea))
}
pub fn ay_pi_8<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::postincrement_ay_8(core)
    .and_then(|ea| core.read_data_byte(ea))
}
pub fn ax_pi_8<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::postincrement_ax_8(core)
    .and_then(|ea| core.read_data_byte(ea))
}
pub fn ay_ai_8<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::address_indirect_ay(core)
    .and_then(|ea| core.read_data_byte(ea))
}
pub fn ay_di_8<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::displacement_ay(core)
    .and_then(|ea| core.read_data_byte(ea))
}
pub fn ay_ix_8<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::index_ay(core)
    .and_then(|ea| core.read_data_byte(ea))
}
pub fn aw_8<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::absolute_word(core)
    .and_then(|ea| core.read_data_byte(ea))
}
pub fn al_8<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::absolute_long(core)
    .and_then(|ea| core.read_data_byte(ea))
}
pub fn pcdi_8<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::displacement_pc(core)
    .and_then(|ea| core.read_program_byte(ea))
}
pub fn pcix_8<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::index_pc(core)
    .and_then(|ea| core.read_program_byte(ea))
}
pub fn imm_8<T: Core>(core: &mut T) -> Result<u32> {
    core.read_imm_u16()
    .map(|extension| u32::from(mask_out_above_8!(extension)))
}

pub fn ea_ay_pd_16<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::predecrement_ay_16(core)
    .and_then(|ea| core.read_data_word(ea).map(|val| (val, ea)))
}
pub fn ea_ax_pd_16<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::predecrement_ax_16(core)
    .and_then(|ea| core.read_data_word(ea).map(|val| (val, ea)))
}
pub fn ea_ay_pi_16<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::postincrement_ay_16(core)
    .and_then(|ea| core.read_data_word(ea).map(|val| (val, ea)))
}
pub fn ea_ay_ai_16<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::address_indirect_ay(core)
    .and_then(|ea| core.read_data_word(ea).map(|val| (val, ea)))
}
pub fn ea_ay_di_16<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::displacement_ay(core)
    .and_then(|ea| core.read_data_word(ea).map(|val| (val, ea)))
}
pub fn ea_ay_ix_16<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::index_ay(core)
    .and_then(|ea| core.read_data_word(ea).map(|val| (val, ea)))
}
pub fn ea_aw_16<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::absolute_word(core)
    .and_then(|ea| core.read_data_word(ea).map(|val| (val, ea)))
}
pub fn ea_al_16<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::absolute_long(core)
    .and_then(|ea| core.read_data_word(ea).map(|val| (val, ea)))
}

pub fn ay_pd_16<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::predecrement_ay_16(core)
    .and_then(|ea| core.read_data_word(ea))
}
pub fn ay_pi_16<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::postincrement_ay_16(core)
    .and_then(|ea| core.read_data_word(ea))
}
pub fn ax_pi_16<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::postincrement_ax_16(core)
    .and_then(|ea| core.read_data_word(ea))
}
pub fn ay_ai_16<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::address_indirect_ay(core)
    .and_then(|ea| core.read_data_word(ea))
}
pub fn ay_di_16<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::displacement_ay(core)
    .and_then(|ea| core.read_data_word(ea))
}
pub fn ay_ix_16<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::index_ay(core)
    .and_then(|ea| core.read_data_word(ea))
}
pub fn aw_16<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::absolute_word(core)
    .and_then(|ea| core.read_data_word(ea))
}
pub fn al_16<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::absolute_long(core)
    .and_then(|ea| core.read_data_word(ea))
}
pub fn pcdi_16<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::displacement_pc(core)
    .and_then(|ea| core.read_program_word(ea))
}
pub fn pcix_16<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::index_pc(core)
    .and_then(|ea| core.read_program_word(ea))
}
pub fn imm_16<T: Core>(core: &mut T) -> Result<u32> {
    core.read_imm_i16()
    .map(|extension| extension as u32)
}

pub fn ea_ay_pd_32<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::predecrement_ay_32(core)
    .and_then(|ea| core.read_data_long(ea).map(|val| (val, ea)))
}
pub fn ea_ax_pd_32<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::predecrement_ax_32(core)
    .and_then(|ea| core.read_data_long(ea).map(|val| (val, ea)))
}
pub fn ea_ay_pi_32<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::postincrement_ay_32(core)
    .and_then(|ea| core.read_data_long(ea).map(|val| (val, ea)))
}
pub fn ea_ay_ai_32<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::address_indirect_ay(core)
    .and_then(|ea| core.read_data_long(ea).map(|val| (val, ea)))
}
pub fn ea_ay_di_32<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::displacement_ay(core)
    .and_then(|ea| core.read_data_long(ea).map(|val| (val, ea)))
}
pub fn ea_ay_ix_32<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::index_ay(core)
    .and_then(|ea| core.read_data_long(ea).map(|val| (val, ea)))
}
pub fn ea_aw_32<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::absolute_word(core)
    .and_then(|ea| core.read_data_long(ea).map(|val| (val, ea)))
}
pub fn ea_al_32<T: Core>(core: &mut T) -> Result<(u32, u32)> {
    effective_address::absolute_long(core)
    .and_then(|ea| core.read_data_long(ea).map(|val| (val, ea)))
}

pub fn ay_pd_32<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::predecrement_ay_32(core)
    .and_then(|ea| core.read_data_long(ea))
}
pub fn ay_pi_32<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::postincrement_ay_32(core)
    .and_then(|ea| core.read_data_long(ea))
}
pub fn ax_pi_32<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::postincrement_ax_32(core)
    .and_then(|ea| core.read_data_long(ea))
}
pub fn ay_ai_32<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::address_indirect_ay(core)
    .and_then(|ea| core.read_data_long(ea))
}
pub fn ay_di_32<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::displacement_ay(core)
    .and_then(|ea| core.read_data_long(ea))
}
pub fn ay_ix_32<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::index_ay(core)
    .and_then(|ea| core.read_data_long(ea))
}
pub fn aw_32<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::absolute_word(core)
    .and_then(|ea| core.read_data_long(ea))
}
pub fn al_32<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::absolute_long(core)
    .and_then(|ea| core.read_data_long(ea))
}
pub fn pcdi_32<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::displacement_pc(core)
    .and_then(|ea| core.read_program_long(ea))
}
pub fn pcix_32<T: Core>(core: &mut T) -> Result<u32> {
    effective_address::index_pc(core)
    .and_then(|ea| core.read_program_long(ea))
}
pub fn imm_32<T: Core>(core: &mut T) -> Result<u32> {
    core.read_imm_u32()
}
pub fn dx<T: Core>(core: &mut T) -> Result<u32> {
    Ok(dx!(core))
}
pub fn dy<T: Core>(core: &mut T) -> Result<u32> {
    Ok(dy!(core))
}
pub fn ay<T: Core>(core: &mut T) -> Result<u32> {
    Ok(ay!(core))
}
pub fn ax<T: Core>(core: &mut T) -> Result<u32> {
    Ok(ax!(core))
}
pub fn quick<T: Core>(core: &mut T) -> Result<u32> {
    Ok((((u32::from(ir!(core)) >> 9) - 1) & 7) + 1)
}

#[cfg(test)]
mod tests {
    use super::super::TestCore;
    use super::super::Exception::AddressError;
    use super::{ea_ax_pd_8, ay_pd_8, ay_ai_16};
    use ram::{AddressBus, SUPERVISOR_DATA};

    #[test]
    fn test_ax_predecrement_8() {
        let mut core = TestCore::new(0x40);
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
        let mut core = TestCore::new(0x40);
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
        let mut core = TestCore::new(0x40);
        core.dar[8] = 0x11; // odd address
        core.ir = 0b1111_1001_1111_1000; // X=4, Y=0
        match ay_ai_16(&mut core) {
            Err(AddressError{..}) => (), // good!
            _ => panic!("Unexpected"),
        };
    }
    #[test]
    fn test_no_address_error_on_even_addresses() {
        let mut core = TestCore::new(0x40);
        core.dar[8] = 0x12; // even address
        core.ir = 0b1111_1001_1111_1000; // X=4, Y=0
        match ay_ai_16(&mut core) {
            Ok(_) => (), // good!
            _ => panic!("Unexpected"),
        };
    }
}
