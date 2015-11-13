pub struct Core {
	pub pc: u32,
	pub sp: u32,
	pub ir: u16,
	pub dar: [u32; 16],

	// Memory should probably be located elsewhere
	pub mem: [u8; 1024],
}

fn op_illegal(core: &mut Core) {
	panic!("Illegal instruction {:>0irwidth$x} at {:>0pcwidth$x}", core.ir, core.pc-2, irwidth=4, pcwidth=8);
}

fn op_set_d0(core: &mut Core) {
	core.dar[0] = 0xabcd;
}

fn op_set_d1(core: &mut Core) {
	core.dar[1] = 0xbcde;
}

impl Core {
	pub fn new(base: u32) -> Core {
		Core { pc: base, sp: 0, ir: 0, dar: [0u32; 16], mem: [0u8; 1024] }
	}
	pub fn new_mem(base: u32, contents: &[u8]) -> Core {
		let mut m = [0u8; 1024];
		let mut b = base as usize;
		for byte in contents {
			m[b] = *byte;
			b+=1;
		}
		Core { pc: base, sp: 0, ir: 0, dar: [0u32; 16], mem: m }
	}
	pub fn reset(&mut self) {
		self.jump(0);
		self.sp = self.read_imm_32();
		let new_pc = self.read_imm_32();
		self.jump(new_pc);
	}
	pub fn read_imm_32(&mut self) -> u32 {
		let b = self.pc as usize;
		self.pc += 4;
		return ((self.mem[b+0] as u32) << 24 
			|   (self.mem[b+1] as u32) << 16
			|   (self.mem[b+2] as u32) <<  8
			|   (self.mem[b+3] as u32) <<  0
			) as u32;
	}
	pub fn read_imm_16(&mut self) -> u16 {
		let b = self.pc as usize;
		self.pc += 2;
		return ((self.mem[b+0] as u16) << 8 
			|   (self.mem[b+1] as u16) << 0
			) as u16;
	}
	pub fn jump(&mut self, pc: u32) {
    	self.pc = pc;
	}
	pub fn execute(&mut self) {
		// Read an instruction from PC (increments PC by 2) 
		self.ir = self.read_imm_16();
		// Covers all possible IR values (64k entries)
		let mut instruction_handler:[fn(&mut Core);0x10000] = [op_illegal; 0x10000];
		// a few fake instructions
		instruction_handler[0xA] = op_set_d0;
		instruction_handler[0xB] = op_set_d1;
		let op_index = self.ir as usize;

		// Call instruction handler to mutate Core accordingly
		instruction_handler[op_index](self);

		// TODO: Perform CPU-cycle accounting for this instruction
	}
}

#[cfg(test)]
mod tests {
    use super::Core;

	#[test]
	fn new_sets_pc() {
		let cpu = Core::new(256);
	    assert_eq!(256, cpu.pc);
	}

	#[test]
	fn new_mem_sets_pc_and_mem() {
		let base = 128;
		let cpu = Core::new_mem(base, &[1u8, 2u8, 3u8, 4u8, 5u8, 6u8]);
	    assert_eq!(128, cpu.pc);
	    assert_eq!(1, cpu.mem[128]);
	    assert_eq!(2, cpu.mem[129]);
	}

	#[test]
	fn a_jump_changes_pc() {
		let mut cpu = Core::new(0);
		cpu.jump(128);
	    assert_eq!(128, cpu.pc);
	}

	#[test]
	fn an_imm_read32_changes_pc() {
		let base = 128;
		let mut cpu = Core::new(base);
		cpu.read_imm_32();
	    assert_eq!(base+4, cpu.pc);
	}

	#[test]
	fn an_imm_read32_reads_from_pc() {
		let base = 128;
		let mut cpu = Core::new_mem(base, &[2u8, 1u8, 3u8, 4u8]);
		let val = cpu.read_imm_32();
	    assert_eq!((2<<24)+(1<<16)+(3<<8)+4, val);
	}


	#[test]
	fn an_imm_read16_changes_pc() {
		let base = 128;
		let mut cpu = Core::new(base);
		cpu.read_imm_16();
	    assert_eq!(base+2, cpu.pc);
	}

	#[test]
	fn an_imm_read16_reads_from_pc() {
		let base = 128;
		let mut cpu = Core::new_mem(base, &[2u8, 1u8, 3u8, 4u8]);
		let val = cpu.read_imm_16();
	    assert_eq!((2<<8)+(1<<0), val);
	}

	#[test]
	fn a_reset_reads_sp_and_pc_from_0() {
		let mut cpu = Core::new_mem(0, &[0u8,0u8,1u8,0u8, 0u8,0u8,0u8,128u8]);
		cpu.reset();
	    assert_eq!(256, cpu.sp);
	    assert_eq!(128, cpu.pc);
	}

	#[test]
	#[should_panic(expected = "instruction bad1")]
	fn execute_reads_from_pc_and_panics_on_illegal_instruction() {
		let mut cpu = Core::new_mem(0xbd, &[0xba,0xd1,1u8,0u8, 0u8,0u8,0u8,128u8]);
		cpu.execute();
	}

	#[test]
	fn execute_can_execute_instruction_handler_0a() {
		let mut cpu = Core::new_mem(0xbd, &[0x00, 0x0A, 1u8,0u8, 0u8,0u8,0u8,128u8]);
		cpu.execute();
	    assert_eq!(0xabcd, cpu.dar[0]);
	    assert_eq!(0x0000, cpu.dar[1]);
	}

	#[test]
	fn execute_can_execute_instruction_handler_0b() {
		let mut cpu = Core::new_mem(0xbd, &[0x00, 0x0B, 1u8,0u8, 0u8,0u8,0u8,128u8]);
		cpu.execute();
	    assert_eq!(0x0000, cpu.dar[0]);
	    assert_eq!(0xbcde, cpu.dar[1]);
	}
}