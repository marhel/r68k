pub struct Core {
	pub pc: u32,
	pub sp: u32,
	pub mem: [u8; 1024],
}

impl Core {
	pub fn new(base: u32) -> Core {
		Core { pc: base, sp: 0, mem: [0u8; 1024] }
	}
	pub fn new_mem(base: u32, contents: &[u8]) -> Core {
		let mut m = [0u8; 1024];
		let mut b = base as usize;
		for byte in contents {
			m[b] = *byte;
			b+=1;
		}
		Core { pc: base, sp: 0, mem: m }
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
	pub fn jump(&mut self, pc: u32) {
    	self.pc = pc;
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
	fn a_reset_reads_sp_and_pc_from_0() {
		let mut cpu = Core::new_mem(0, &[0u8,0u8,1u8,0u8, 0u8,0u8,0u8,128u8]);
		cpu.reset();
	    assert_eq!(256, cpu.sp);
	    assert_eq!(128, cpu.pc);
	}
}