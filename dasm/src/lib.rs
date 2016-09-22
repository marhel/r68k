// type alias for exception handling
use std::result;
mod operand;
use operand::Operand;
mod memory;
use memory::Memory;
mod assembler;
pub type Result<T> = result::Result<T, Exception>;
type OperandDecoder = fn(u32, &Memory) -> Vec<Operand>;
type InstructionEncoder = fn(&OpcodeInstance, u16, u32, &mut Memory) -> u32;
type InstructionSelector = fn(&OpcodeInstance) -> bool;
extern crate regex;

#[derive(Debug)]
pub enum Exception {
     IllegalInstruction(u16, u32), // ir, pc
}

#[derive(Clone, Copy, Debug, PartialEq)] 
enum Size {
	Unsized, Byte, Word, Long
}

const OP_ADD   : u32 = 0b1101_0000_0000_0000;

const BYTE_SIZED: u32 = 0x00;
#[allow(dead_code)]
const WORD_SIZED: u32 = 0x40;
#[allow(dead_code)]
const LONG_SIZED: u32 = 0x80;

const DEST_DX: u32 = 0x000;
const DEST_EA: u32 = 0x100;

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
pub struct OpcodeInfo<'a> {
    mask: u32,
    matching: u32,
    size: Size,
    decoder: OperandDecoder,
    mnemonic: &'a str,
    encoder: InstructionEncoder,
    selector: InstructionSelector,
}
#[derive(Debug)] 
pub struct OpcodeInstance<'a> {
    mnemonic: &'a str,
    size: Size,
	operands: Vec<Operand>,
}

use std::fmt;
impl<'a> fmt::Debug for OpcodeInfo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ => write!(f, "[some fn]"),
        }
    }
}
impl<'a> fmt::Display for OpcodeInstance<'a> {
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
    ($mask:expr, $matching:expr, $size:expr, $mnemonic:expr, $decoder:ident) => (OpcodeInfo { mask: $mask, matching: $matching, size: $size, mnemonic: $mnemonic, decoder: $decoder, encoder: nop_encoder, selector: nop_selector});
    ($mask:expr, $matching:expr, $size:expr, $mnemonic:expr, $decoder:ident, $selector:ident, $encoder:ident) => (OpcodeInfo { mask: $mask, matching: $matching, size: $size, mnemonic: $mnemonic, decoder: $decoder, encoder: assembler::$encoder, selector: $selector})
}
fn generate<'a>() -> Vec<OpcodeInfo<'a>> {
    vec![
        instruction!(MASK_OUT_X_EA, OP_ADD | BYTE_SIZED | DEST_DX, Size::Byte, "ADD", ea_dx, is_ea_dx, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_ADD | BYTE_SIZED | DEST_EA, Size::Byte, "ADD", dx_ea, is_dx_ea, encode_dx_ea),
    ]
}
fn get_ea(pc: u32, mem: &Memory) -> Operand {
	let opcode = mem.read_word(pc);
	let mode = ((opcode >> 3) & 7) as u8;
	let reg_y = (opcode & 7) as u8;
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
			0b000 => Operand::AbsoluteWord(mem.read_word(pc+2)),
			0b001 => Operand::AbsoluteLong((mem.read_word(pc+2) as u32) << 16 & mem.read_word(pc+4) as u32),
			0b100 => Operand::Immediate(mem.read_word(pc+2)),
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

pub fn disassemble_first(mem: &Memory) -> OpcodeInstance {
    disassemble(0, mem).unwrap()
}

pub fn disassemble(pc: u32, mem: &Memory) -> Result<OpcodeInstance> {
    let optable = generate();
	let opcode = mem.read_word(pc);
	println!("opcode read was {:04x}", opcode);
	for op in optable {
		if ((opcode as u32) & op.mask) == op.matching {
			let decoder = op.decoder;
			return Ok(OpcodeInstance {mnemonic: op.mnemonic, size: op.size, operands: decoder(pc, mem)});
		}
	}
    Err(Exception::IllegalInstruction(opcode, pc))
}

#[allow(unused_variables)]
pub fn nop_encoder(op: &OpcodeInstance, template: u16, pc: u32, mem: &mut Memory) -> u32 {
    pc
}
#[allow(unused_variables)]
pub fn nop_selector(op: &OpcodeInstance) -> bool {
    false
}
pub fn is_ea_dx(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    match op.operands[1] {
        Operand::DataRegisterDirect(_) => true,
        _ => false,
    }
}
pub fn is_dx_ea(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    match op.operands[1] {
        Operand::DataRegisterDirect(_) => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use operand::Operand; 
    use memory::{MemoryVec, Memory};
    use assembler::{parse_assembler, encode_instruction};
    use super::{Size, disassemble, disassemble_first};

    extern crate itertools;
    use self::itertools::assert_equal;

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
    #[test]
    fn roundtrips_from_opcode() {
        let opcode = 0xd511;
        let mut mem = &mut MemoryVec { mem: vec![opcode]} ;
        let asm = {
            let inst = disassemble_first(mem);
            format!("{}", inst)
        };
        let pc = 0;
        let inst = parse_assembler(asm.as_str());
        let new_pc = encode_instruction(&inst, pc, mem);
        assert_eq!(2, new_pc);
        assert_eq!(opcode, mem.read_word(pc));
    }
    #[test]
    fn roundtrips_from_asm() {
        let mut mem = &mut MemoryVec { mem: vec![0x00,0x00,0x00,0x00]} ;
        let pc = 0;
        let asm = "ADD.B\tD2,(A1)";
        let inst = parse_assembler(asm);
        encode_instruction(&inst, pc, mem);
        let inst = disassemble_first(mem);

        assert_eq!(asm, format!("{}", inst));
    }

    fn opcodes(mask: u32, matching: u32) -> Vec<u16> {
        (matching..0x10000u32)
            .filter(|opcode| (opcode & mask) == matching)
            .map(|v|v as u16).collect::<Vec<u16>>()
    }

    #[test]
    fn roundtrips() {
        for opcode in 53248..55000 {
            let pc = 0;
            let dasm_mem = &mut MemoryVec { mem: vec![opcode, 0x0012, 0x0024]} ;
            match disassemble(pc, dasm_mem) {
                Err(err) => println!("{:?}", err),
                Ok(inst) => {
                    let asm = format!("{}", inst);
                    let inst = parse_assembler(asm.as_str());
                    let mut asm_mem = &mut MemoryVec { mem: vec![0x0000, 0x0012, 0x0024]};
                    let new_pc = encode_instruction(&inst, pc, asm_mem);
                    let new_opcode = asm_mem.read_word(pc);
                    if true || opcode != new_opcode {                       
                        println!("{:04x} | {:04x}: {}", opcode, new_opcode, asm);
                    }
                    assert_equal(&dasm_mem.mem, &asm_mem.mem);
                }
            }
        }
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