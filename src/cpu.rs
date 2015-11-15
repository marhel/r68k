pub type InstructionSet = [fn(&mut Core);0x10000];
pub type Handler = fn(&mut Core);

pub struct Core {
	pub pc: u32,
	pub sp: u32,
	pub ir: u16,
	pub dar: [u32; 16],
	pub ophandlers: InstructionSet,
	pub x_flag: u32,
	pub c_flag: u32,
	pub v_flag: u32,
	pub n_flag: u32,
	pub not_z_flag: u32,

	// Memory should probably be located elsewhere
	pub mem: [u8; 1024],
}

#[macro_use]
mod ops {
	use super::Core;

	macro_rules! ir_dx {
		($e:ident) => (($e.ir >> 9 & 7) as usize);
	}
	macro_rules! ir_dy {
		($e:ident) => (($e.ir & 7) as usize);
	}
	macro_rules! dx {
		($e:ident) => ($e.dar[ir_dx!($e)]);
	}
	macro_rules! dy {
		($e:ident) => ($e.dar[ir_dy!($e)]);
	}
	macro_rules! mask_out_above_8 {
		($e:expr) => ($e & 0xff)
	}
	macro_rules! mask_out_below_8 {
		($e:expr) => ($e & !0xff)
	}
	macro_rules! low_nibble {
		($e:expr) => ($e & 0x0f);
	}
	macro_rules! high_nibble {
		($e:expr) => ($e & 0xf0);
	}
	macro_rules! true1 {
		($e:expr) => (if $e {1} else {0})
	}

	pub mod fake {
		use super::super::Core;

		pub fn set_d0(core: &mut Core) {
			core.dar[0] = 0xabcd;
		}

		pub fn set_d1(core: &mut Core) {
			core.dar[1] = 0xbcde;
		}

		pub fn set_dx(core: &mut Core) {
			dx!(core) = 0xcdef;
		}

		use super::super::InstructionSet;
		use super::super::Handler;
		use super::illegal;
		const SET_DX_0: usize = 0b0100_0000_0000_0000;

		pub fn instruction_set() -> InstructionSet {
			// Covers all possible IR values (64k entries)
			let mut handler:[Handler; 0x10000] = [illegal; 0x10000];

			// a few fake instructions
			handler[0xA] = set_d0;
			handler[0xB] = set_d1;

			for i in 0..8 {
				let opcode = SET_DX_0 | (i << 9);
				println!("{:x}", opcode);
				handler[opcode] = set_dx;
			}
			handler
		}
	}

	pub fn illegal(core: &mut Core) {
		panic!("Illegal instruction {:04x} at {:08x}", core.ir, core.pc-2);
	}

	// First real instruction, ported from https://github.com/kstenerud/Musashi
	pub fn abcd_8_rr(core: &mut Core) {
		// unsigned int* r_dst = &(m68ki_cpu.dar[(m68ki_cpu.ir >> 9) & 7]);
		// unsigned int src = (m68ki_cpu.dar[m68ki_cpu.ir & 7]);
		// unsigned int dst = *r_dst;
		// unsigned int res = ((src) & 0x0f) + ((dst) & 0x0f) + ((m68ki_cpu.x_flag>>8)&1);
		let dst = dx!(core);
		let src = dy!(core);
		let mut res = low_nibble!(src) + low_nibble!(dst) + core.x_flag_as_1();

		// m68ki_cpu.v_flag = ~res;
		core.v_flag = !res;

		// if(res > 9)
		//  res += 6;
		// res += ((src) & 0xf0) + ((dst) & 0xf0);
		// m68ki_cpu.x_flag = m68ki_cpu.c_flag = (res > 0x99) << 8;
		if res > 9 {
			res += 6;
		}
		res += high_nibble!(src) + high_nibble!(dst);
		core.c_flag = true1!(res > 0x99) << 8;
		core.x_flag = core.c_flag;

		if core.c_flag > 0 {
			res -= 0xa0;
		}

		// m68ki_cpu.v_flag &= res;
		// m68ki_cpu.n_flag = (res);
		core.v_flag &= res;
		core.n_flag = res;

		// res = ((res) & 0xff);
		// m68ki_cpu.not_z_flag |= res;
		res = mask_out_above_8!(res);
		core.not_z_flag |= res;

		// *r_dst = ((*r_dst) & ~0xff) | res;
		dx!(core) = mask_out_below_8!(dst) | res;
	}

	use super::Handler;
	struct OpcodeHandler {
		mask: u32,
		matching: u32,
		name: String,
		handler: Handler
	}

	use super::InstructionSet;
	macro_rules! op_entry {
	    ($mask:expr, $matching:expr, $handler:ident) => (OpcodeHandler { mask: $mask, matching: $matching, handler: $handler, name: stringify!($handler).to_string() })
	}
	pub fn instruction_set() -> InstructionSet {
		// Covers all possible IR values (64k entries)
		let mut handler:[Handler; 0x10000] = [illegal; 0x10000];
		// the optable contains opcode mask, matching mask and the corresponding handler + name
		let optable = vec![op_entry!(0xf1f8, 0xc100, abcd_8_rr)];
		for op in optable {
			for opcode in 0..0x10000 {
				if (opcode & op.mask) == op.matching {
					println!("{:16b}: {}", opcode, op.name);
					handler[opcode as usize] = op.handler;
				}
			}
		}
		handler
	}
}

impl Core {
	pub fn new(base: u32) -> Core {
		Core { pc: base, sp: 0, ir: 0, dar: [0u32; 16], mem: [0u8; 1024], ophandlers: ops::fake::instruction_set(), x_flag: 0, v_flag: 0, c_flag: 0, n_flag: 0, not_z_flag: 0xffffffff}
	}
	pub fn new_mem(base: u32, contents: &[u8]) -> Core {
		let mut m = [0u8; 1024];
		let mut b = base as usize;
		for byte in contents {
			m[b] = *byte;
			b+=1;
		}
		Core { pc: base, sp: 0, ir: 0, dar: [0u32; 16], mem: m, ophandlers: ops::fake::instruction_set(), x_flag: 0, v_flag: 0, c_flag: 0, n_flag: 0, not_z_flag: 0xffffffff }
	}
	pub fn reset(&mut self) {
		self.jump(0);
		self.sp = self.read_imm_32();
		let new_pc = self.read_imm_32();
		self.jump(new_pc);
	}
	pub fn x_flag_as_1(&self) -> u32 {
		(self.x_flag>>8)&1
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
	pub fn execute1(&mut self) {
		// Read an instruction from PC (increments PC by 2)
		self.ir = self.read_imm_16();
		let opcode = self.ir as usize;

		// Call instruction handler to mutate Core accordingly
		self.ophandlers[opcode](self);

		// TODO: Perform CPU-cycle accounting for this instruction
	}
}

#[cfg(test)]
mod tests {
	use super::Core;
	use super::ops; //::instruction_set;

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
		cpu.execute1();
	}

	#[test]
	fn execute_can_execute_instruction_handler_0a() {
		let mut cpu = Core::new_mem(0xbd, &[0x00, 0x0A, 1u8,0u8, 0u8,0u8,0u8,128u8]);
		cpu.execute1();
		assert_eq!(0xabcd, cpu.dar[0]);
		assert_eq!(0x0000, cpu.dar[1]);
	}

	#[test]
	fn execute_can_execute_instruction_handler_0b() {
		let mut cpu = Core::new_mem(0xbd, &[0x00, 0x0B, 1u8,0u8, 0u8,0u8,0u8,128u8]);
		cpu.execute1();
		assert_eq!(0x0000, cpu.dar[0]);
		assert_eq!(0xbcde, cpu.dar[1]);
	}


	#[test]
	fn execute_can_execute_set_dx() {
		// first byte 40 is register D0
		// 42 == D1
		// 44 == D2
		// 46 == D3
		// 48 == D4
		// 4a == D5
		// 4c == D6
		// 4e == D7
		let mut cpu = Core::new_mem(0x40, &[0x4c, 0x00, 1u8, 0u8]);
		cpu.execute1();
		assert_eq!(0xcdef, cpu.dar[6]);
	}

	#[test]
	fn low_nibble() {
		assert_eq!(0x0a, low_nibble!(0xba));
	}
	#[test]
	fn high_nibble() {
		assert_eq!(0xb0, high_nibble!(0xba));
	}
	#[test]
	fn mask_out_below_8() {
		assert_eq!(0x2bcdef00, mask_out_below_8!(0x2bcdef73));
	}
	#[test]
	fn mask_out_above_8() {
		assert_eq!(0xf1, mask_out_above_8!(0x2bcdeff1));
	}
	#[test]
	fn dx_and_dy() {
		let mut core = Core::new(0x40);
		core.dar[0] = 0x00;
		core.dar[1] = 0x11;
		core.dar[2] = 0x22;
		core.dar[3] = 0x33;
		core.dar[4] = 0x44;
		core.dar[5] = 0x55;
		core.dar[6] = 0x66;
		core.dar[7] = 0x77;

		core.ir = 0b1111_1001_1111_1010;
		assert_eq!(0x22, dy!(core));
		assert_eq!(0x44, dx!(core));

		core.ir = 0b1111_1011_1111_1110;
		assert_eq!(0x66, dy!(core));
		assert_eq!(0x55, dx!(core));
	}

	#[test]
	fn array_elems() {
		let mut arr = [1, 2, 3, 4];
		let mut marr = &mut arr;
		let mut elem: &mut i32 = &mut (marr[1]);
		// let mut elem2: &mut i32 = &mut (arr[2]);
		assert_eq!(2, *elem);
		*elem = 200;
		assert_eq!(200, *elem);
		// assert_eq!(200, &mut marr[1]);
	}

	#[test]
	fn abcd_8_rr() {
		// opcodes c100 - c107, c300 - c307, etc.
		// or more generally c[13579bdf]0[0-7]
		// where [13579bdf] is DX (dest regno) and [0-7] is DY (src regno)
		// so c300 means D1 = D0 + D1 in BCD
		let mut cpu = Core::new_mem(0x40, &[0xc3, 0x00]);
		cpu.ophandlers = ops::instruction_set();

		cpu.dar[0] = 0x16;
		cpu.dar[1] = 0x26;
		cpu.execute1();

		// 16 + 26 is 42
		assert_eq!(0x42, cpu.dar[1]);
	}
}