type OperandDecoder = fn(u32, &Memory) -> Vec<Operand>;

#[derive(Clone, Copy, Debug, PartialEq)] 
enum Size {
	Unsized, Byte, Word, Long
}

const OP_ADD   : u32 = 0b1101_0000_0000_0000;

const BYTE_SIZED: u32 = 0x00;
const WORD_SIZED: u32 = 0x40;
const LONG_SIZED: u32 = 0x80;

const DEST_DX: u32 = 0x000;
const DEST_EA: u32 = 0x100;

#[derive(Clone, Copy, Debug, PartialEq)] 
enum Operand {
	DataRegisterDirect(u8),
	AddressRegisterDirect(u8),
	AddressRegisterIndirect(u8),
	AddressRegisterIndirectWithPredecrement(u8),
	AddressRegisterIndirectWithPostincrement(u8),
	AddressRegisterIndirectWithDisplacement(u8, i16),
	AddressRegisterIndirectWithIndex(u8, u8, i8),
	PcWithDisplacement(i16),
	PcWithIndex(u8, i8),
	AbsoluteWord(u16),
	AbsoluteLong(u32),
	Immediate(u16),
}

fn xreg(xreg: u8) -> Operand {
    if xreg & 8 > 0 {
        Operand::AddressRegisterDirect(xreg & 7)
    } else {
        Operand::DataRegisterDirect(xreg & 7)
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Size::Unsized => write!(f, ""),
            Size::Byte => write!(f, ".B"),
            Size::Word => write!(f, ".W"),
            Size::Long => write!(f, ".L"),
        }
    }
}
impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::DataRegisterDirect(reg) => write!(f, "D{}", reg),
            Operand::AddressRegisterDirect(reg) => write!(f, "A{}", reg),
            Operand::AddressRegisterIndirect(reg) => write!(f, "(A{})", reg),
            Operand::AddressRegisterIndirectWithPredecrement(reg) => write!(f, "-(A{})", reg),
            Operand::AddressRegisterIndirectWithPostincrement(reg) => write!(f, "(A{})+", reg),
            Operand::AddressRegisterIndirectWithDisplacement(reg, dis) => write!(f, "{}(A{})", dis, reg),
            Operand::AddressRegisterIndirectWithIndex(reg, ireg, dis) => write!(f, "{}(A{},{})", dis, reg, xreg(ireg)),
            Operand::PcWithDisplacement(dis) => write!(f, "{}(PC)", dis),
            Operand::PcWithIndex(ireg, dis) => write!(f, "{}(PC,{})", dis, xreg(ireg)),
            Operand::AbsoluteWord(word) => write!(f, "${:4x}", word),
            Operand::AbsoluteLong(long) => write!(f, "${:8x}", long),
            Operand::Immediate(imm) => write!(f, "#${:8x}", imm),
         }
    }
}
/*
	REG_DIRECT_DATA,		// Register Direct - Data
	REG_DIRECT_ADDR,		// Register Direct - Address

	REGI_ADDR,				// Register Indirect - Address
	REGI_ADDR_POST_INC,		// Register Indirect - Address with Postincrement
	REGI_ADDR_PRE_DEC,		// Register Indirect - Address with Predecrement
	REGI_ADDR_DISP,			// Register Indirect - Address with Displacement

	AREGI_INDEX_8_BIT_DISP,	// Address Register Indirect With Index- 8-bit displacement

	PCI_DISP,				// Program Counter Indirect - with Displacement

	PCI_INDEX_8_BIT_DISP,	// Program Counter Indirect with Index - with 8-Bit Displacement

	ABSOLUTE_DATA_SHORT,	// Absolute Data Addressing  - Short
	ABSOLUTE_DATA_LONG,		// Absolute Data Addressing  - Long
	IMMEDIATE,              // Immediate value
*/
// #[derive(Clone, Copy)] 
pub struct OpcodeInfo {
    mask: u32,
    matching: u32,
    size: Size,
    decoder: OperandDecoder,
    mnemonic: &'static str,
}
#[derive(Debug)] 
pub struct OpcodeInstance {
    mnemonic: &'static str,
    size: Size,
	operands: Vec<Operand>,
}

use std::fmt;
impl fmt::Debug for OpcodeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ => write!(f, "[some fn]"),
        }
    }
}
impl fmt::Display for OpcodeInstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.operands.len() {
            0 => write!(f, "{}{}", self.mnemonic, self.size),
            1 => write!(f, "{}{}\t{}", self.mnemonic, self.size, self.operands[0]),
            2 => write!(f, "{}{}\t{},{}", self.mnemonic, self.size, self.operands[0], self.operands[1]),
            _ => panic!("more than two operands {:?}", self)
        }
    }
}
macro_rules! instruction {
    ($mask:expr, $matching:expr, $size:expr, $mnemonic:expr, $decoder:ident) => (OpcodeInfo { mask: $mask, matching: $matching, size: $size, mnemonic: $mnemonic, decoder: $decoder})
}
fn get_ea(pc: u32, mem: &Memory) -> Operand {
	let opcode = mem.read_word(pc);
	let mode = ((opcode >> 3) & 7) as u8;
	let reg_y = (opcode & 7) as u8;
	let (index, displacement, word, long, data) = (0, 0, 0, 0, 0);
	match mode {
		0b000 => Operand::DataRegisterDirect(reg_y),
		0b001 => Operand::AddressRegisterDirect(reg_y),
		0b010 => Operand::AddressRegisterIndirect(reg_y),
		0b011 => Operand::AddressRegisterIndirectWithPostincrement(reg_y),
		0b100 => Operand::AddressRegisterIndirectWithPredecrement(reg_y),
		0b101 => Operand::AddressRegisterIndirectWithDisplacement(reg_y, mem.read_word(pc+2) as i16),
		0b110 => {
			let (indexinfo, displacement) = parse_extension_word(mem.read_word(pc+2));
			Operand::AddressRegisterIndirectWithIndex(reg_y, indexinfo, displacement)
			},
		0b111 => match reg_y {
			0b010 => Operand::PcWithDisplacement(mem.read_word(pc+2) as i16),
			0b011 => {
				let (indexinfo, displacement) = parse_extension_word(mem.read_word(pc+2));
				Operand::PcWithIndex(indexinfo, displacement)
				},
			0b000 => Operand::AbsoluteWord(word),
			0b001 => Operand::AbsoluteLong(long),
			0b100 => Operand::Immediate(data),
			_ => panic!("Unknown addressing mode {:?} reg {:?}", mode, reg_y),
		},
		_ => panic!("Unknown addressing mode {:?} reg {:?}", mode, reg_y),
	}
}
fn parse_extension_word(extension: u16) -> (u8, i8) {
    // top four bits = (D/A RRR) matches our register array layout
    let xreg_ndx_size = (extension>>11) as u8;
	let displacement = extension as i8;
    (xreg_ndx_size, displacement)
}
fn get_dx(pc: u32, mem: &Memory) -> Operand {
	let opcode = mem.read_word(pc);
	Operand::DataRegisterDirect(((opcode >> 9) & 7) as u8)
}
fn ea_dx(pc: u32, mem: &Memory) -> Vec<Operand> {
	vec![get_ea(pc, mem), get_dx(pc, mem)]
}
fn dx_ea(pc: u32, mem: &Memory) -> Vec<Operand> {
	vec![get_dx(pc, mem), get_ea(pc, mem)]
}
pub const MASK_OUT_X_EA: u32 = 0b1111000111000000; // masks out X and Y register bits, plus mode (????xxx???mmmyyy)

pub trait Memory {
	fn read_word(&self, pc: u32) -> u16;
}

#[derive(Debug)] 
struct MemoryVec {
	mem: Vec<u16>
}

impl Memory for MemoryVec {
	fn read_word(&self, pc: u32) -> u16 {
		if pc % 1 == 1 { panic!("Odd PC!") }
		self.mem[(pc/2) as usize]
	}
}

pub fn disassemble_first(mem: &Memory) -> OpcodeInstance {
    let optable = vec![
        instruction!(MASK_OUT_X_EA, OP_ADD | BYTE_SIZED | DEST_DX, Size::Byte, "ADD", ea_dx),
        instruction!(MASK_OUT_X_EA, OP_ADD | BYTE_SIZED | DEST_EA, Size::Byte, "ADD", dx_ea),
	];
	let opcode = mem.read_word(0) as u32;
	println!("opcode read was {:04x}", opcode);
	for op in optable {
		if (opcode & op.mask) == op.matching {
			let decoder = op.decoder;
			return OpcodeInstance {mnemonic: op.mnemonic, size: op.size, operands: decoder(0, mem)};
		}
	}
	panic!("Could not disassemble {:04x}", opcode);
}

#[cfg(test)]
mod tests {
    use super::{Operand, Size, MemoryVec, disassemble_first};

    #[test]
    fn decodes_add_8_er() {
        let mem = MemoryVec { mem: vec![0xd411]} ;
        let inst = disassemble_first(&mem);
        assert_eq!("ADD", inst.mnemonic);
        assert_eq!(Size::Byte, inst.size);
        assert_eq!(Operand::AddressRegisterIndirect(1), inst.operands[0]);
        assert_eq!(Operand::DataRegisterDirect(2), inst.operands[1]);
        assert_eq!("(A1)", format!("{}", inst.operands[0]));
        assert_eq!("D2", format!("{}", inst.operands[1]));
        assert_eq!("ADD.B\t(A1),D2", format!("{}", inst));
    }
    #[test]
    fn decodes_add_8_re() {
        let mem = MemoryVec { mem: vec![0xd511]} ;
        let inst = disassemble_first(&mem);

        assert_eq!("ADD", inst.mnemonic);
        assert_eq!(Size::Byte, inst.size);
        assert_eq!(Operand::DataRegisterDirect(2), inst.operands[0]);
        assert_eq!(Operand::AddressRegisterIndirect(1), inst.operands[1]);
        assert_eq!("D2", format!("{}", inst.operands[0]));
        assert_eq!("(A1)", format!("{}", inst.operands[1]));
        assert_eq!("ADD.B\tD2,(A1)", format!("{}", inst));
    }
}

// enum Op {
// 	StdOp(&'static str, Operand, Operand)
// }

// 	fn test()
// 	{
// 		let add = StdOp("ADD", D(1), PD(2))
// 		dasm.decode(add.encode())
// 		asm.parse(add,print())
// 	}