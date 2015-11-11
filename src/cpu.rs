pub struct Core {
	pub pc: u32,
}

impl Core {
	pub fn new() -> Core {
		Core { pc: 0 }
	}

	pub fn jump(&mut self, pc: u32) {
    	self.pc = pc;
	}
}

#[cfg(test)]
mod tests {
    use super::Core;
    //use super::*;

	#[test]
	fn a_jump_changes_pc() {
		let mut cpu = Core { pc: 0 };
		cpu.jump(128);
	    assert_eq!(128, cpu.pc);
	}
}