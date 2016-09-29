// type alias for exception handling
use std::result;
mod operand;
use operand::Operand;
mod memory;
use memory::Memory;
mod assembler;
pub type Result<T> = result::Result<T, Exception>;
type OperandDecoder = fn(u16, Size, u32, &Memory) -> Vec<Operand>;
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
const OP_ADDX  : u32 = 0b1101_0001_0000_0000;
const OP_ADDI  : u32 = 0b0000_0110_0000_0000;
const OP_ADDQ  : u32 = 0b0101_0000_0000_0000;

const BYTE_SIZED: u32 = 0x00;
#[allow(dead_code)]
const WORD_SIZED: u32 = 0x40;
#[allow(dead_code)]
const LONG_SIZED: u32 = 0x80;

const DEST_DX: u32 = 0x000;
const DEST_EA: u32 = 0x100;
// ADDA does not follow the ADD pattern for 'oper' so we cannot use the
// above constants
const DEST_AX_WORD: u32 = 0x0C0;
const DEST_AX_LONG: u32 = 0x1C0;

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
    ea_mask: u16,
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
impl<'a> OpcodeInstance<'a> {
    pub fn length(&self) -> u32 {
        1 + self.operands.iter().map(|op| op.extension_words()).fold(0, |i,j|i+j)
    }
}
macro_rules! instruction {
    ($mask:expr, $matching:expr, $ea_mask:expr, $size:expr, $mnemonic:expr, $decoder:ident) => (OpcodeInfo { mask: $mask, matching: $matching, size: $size, mnemonic: $mnemonic, decoder: $decoder, encoder: assembler::nop_encoder, selector: assembler::nop_selector, ea_mask: $ea_mask});
    ($mask:expr, $matching:expr, $ea_mask:expr, $size:expr, $mnemonic:expr, $decoder:ident, $selector:ident, $encoder:ident) => (OpcodeInfo { mask: $mask, matching: $matching, size: $size, mnemonic: $mnemonic, decoder: $decoder, encoder: assembler::$encoder, selector: assembler::$selector, ea_mask: $ea_mask})
}
fn generate<'a>() -> Vec<OpcodeInfo<'a>> {
    vec![
        instruction!(MASK_OUT_X_EA, OP_ADD | BYTE_SIZED | DEST_EA, EA_MEMORY_ALTERABLE, Size::Byte, "ADD", dx_ea, is_dx_ea, encode_dx_ea),
        instruction!(MASK_OUT_X_EA, OP_ADD | BYTE_SIZED | DEST_DX, EA_ALL_EXCEPT_AN, Size::Byte, "ADD", ea_dx, is_ea_dx, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_ADD | WORD_SIZED | DEST_EA, EA_MEMORY_ALTERABLE, Size::Word, "ADD", dx_ea, is_dx_ea, encode_dx_ea),
        instruction!(MASK_OUT_X_EA, OP_ADD | WORD_SIZED | DEST_DX, EA_ALL, Size::Word, "ADD", ea_dx, is_ea_dx, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_ADD | LONG_SIZED | DEST_EA, EA_MEMORY_ALTERABLE, Size::Long, "ADD", dx_ea, is_dx_ea, encode_dx_ea),
        instruction!(MASK_OUT_X_EA, OP_ADD | LONG_SIZED | DEST_DX, EA_ALL, Size::Long, "ADD", ea_dx, is_ea_dx, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_ADD | DEST_AX_WORD, EA_ALL, Size::Word, "ADDA", ea_ax, is_ea_ax, encode_ea_ax),
        instruction!(MASK_OUT_X_EA, OP_ADD | DEST_AX_LONG, EA_ALL, Size::Long, "ADDA", ea_ax, is_ea_ax, encode_ea_ax),
        instruction!(MASK_OUT_EA, OP_ADDI | BYTE_SIZED, EA_DATA_ALTERABLE, Size::Byte, "ADDI", imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_EA, OP_ADDI | WORD_SIZED, EA_DATA_ALTERABLE, Size::Word, "ADDI", imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_EA, OP_ADDI | LONG_SIZED, EA_DATA_ALTERABLE, Size::Long, "ADDI", imm_ea, is_imm_ea, encode_imm_ea),
    ]
}
fn get_ea(opcode: u16, size: Size, pc: u32, mem: &Memory) -> Operand {
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
			0b001 => Operand::AbsoluteLong((mem.read_word(pc+2) as u32) << 16 | mem.read_word(pc+4) as u32),
			0b100 => 
                match size {
                    Size::Byte => Operand::Immediate(size, (mem.read_word(pc+2) & 0xFF) as u32),
                    Size::Word => Operand::Immediate(size, mem.read_word(pc+2) as u32),
                    Size::Long => Operand::Immediate(size, (mem.read_word(pc+2) as u32) << 16 | mem.read_word(pc+4) as u32),
                    Size::Unsized => panic!("unsized Immediate"),
                },
			_ => panic!("Unknown addressing mode {:03b} reg {:03b}", mode, reg_y),
		},
		_ => panic!("Unknown addressing mode {:03b} reg {:03b}", mode, reg_y),
	}
}
fn parse_extension_word(extension: u16) -> (u8, i8) {
    // top four bits = (D/A RRR) matches our register array layout
    let xreg_ndx_size = (extension>>12) as u8;
	let displacement = extension as i8;
    (xreg_ndx_size, displacement)
}
fn get_dx(opcode: u16, pc: u32, mem: &Memory) -> Operand {
    Operand::DataRegisterDirect(((opcode >> 9) & 7) as u8)
}
fn get_ax(opcode: u16, pc: u32, mem: &Memory) -> Operand {
    Operand::AddressRegisterDirect(((opcode >> 9) & 7) as u8)
}
fn get_imm(size: Size, pc: u32, mem: &Memory) -> Operand {
    match size {
        Size::Byte => Operand::Immediate(size, (mem.read_word(pc+2) & 0xFF) as u32),
        Size::Word => Operand::Immediate(size, mem.read_word(pc+2) as u32),
        Size::Long => Operand::Immediate(size, (mem.read_word(pc+2) as u32) << 16 | mem.read_word(pc+4) as u32),
        Size::Unsized => panic!("unsized Immediate"),
    }
}
fn ea_dx(opcode: u16, size: Size, pc: u32, mem: &Memory) -> Vec<Operand> {
    vec![get_ea(opcode, size, pc, mem), get_dx(opcode, pc, mem)]
}
fn ea_ax(opcode: u16, size: Size, pc: u32, mem: &Memory) -> Vec<Operand> {
    vec![get_ea(opcode, size, pc, mem), get_ax(opcode, pc, mem)]
}
fn dx_ea(opcode: u16, size: Size, pc: u32, mem: &Memory) -> Vec<Operand> {
	vec![get_dx(opcode, pc, mem), get_ea(opcode, size, pc, mem)]
}
fn imm_ea(opcode: u16, size: Size, pc: u32, mem: &Memory) -> Vec<Operand> {
    let imm = get_imm(size, pc, mem);
    vec![imm, get_ea(opcode, size, pc + imm.extension_words()*2, mem)]
}
pub const MASK_OUT_X_EA: u32 = 0b1111000111000000; // masks out X and Y register bits, plus mode (????xxx???mmmyyy)
pub const MASK_OUT_EA: u32 = 0b1111111111000000;   // masks out Y register bits, plus mode (??????????mmmyyy)
pub fn disassemble_first(mem: &Memory) -> OpcodeInstance {
    disassemble(0, mem).unwrap()
}

pub fn disassemble(pc: u32, mem: &Memory) -> Result<OpcodeInstance> {
    let optable = generate();
	let opcode = mem.read_word(pc);
	// println!("opcode read was {:04x}", opcode);
	for op in optable {
		if ((opcode as u32) & op.mask) == op.matching && valid_ea(opcode, op.ea_mask) {
			let decoder = op.decoder;
			return Ok(OpcodeInstance {mnemonic: op.mnemonic, size: op.size, operands: decoder(opcode, op.size, pc, mem)});
		}
	}
    Err(Exception::IllegalInstruction(opcode, pc))
}

const EA_DATA_REGISTER_DIRECT: u16 =      0b1000_0000_0000;
const EA_ADDRESS_REGISTER_DIRECT: u16 =   0b0100_0000_0000;
const EA_ADDRESS_REGISTER_INDIRECT: u16 = 0b0010_0000_0000;
const EA_ARI_POSTINCREMENT: u16 =         0b0001_0000_0000;
const EA_ARI_PREDECREMENT: u16 =          0b0000_1000_0000;
const EA_ARI_DISPLACEMENT: u16 =          0b0000_0100_0000;
const EA_ARI_INDEX: u16 =                 0b0000_0010_0000;
const EA_ABSOLUTE_SHORT: u16 =            0b0000_0001_0000;
const EA_ABSOLUTE_LONG: u16 =             0b0000_0000_1000;
const EA_IMMEDIATE: u16 =                 0b0000_0000_0100;
const EA_PC_DISPLACEMENT: u16 =           0b0000_0000_0010;
const EA_PC_INDEX: u16 =                  0b0000_0000_0001;

const EA_ALL: u16 = 0xfff;
const EA_ALL_EXCEPT_AN: u16 = EA_ALL & !EA_ADDRESS_REGISTER_DIRECT;
const EA_ALTERABLE: u16 = EA_DATA_REGISTER_DIRECT
                        | EA_ADDRESS_REGISTER_DIRECT
                        | EA_ADDRESS_REGISTER_INDIRECT
                        | EA_ARI_POSTINCREMENT
                        | EA_ARI_PREDECREMENT
                        | EA_ARI_DISPLACEMENT
                        | EA_ARI_INDEX
                        | EA_ABSOLUTE_SHORT
                        | EA_ABSOLUTE_LONG;
const EA_CONTROL: u16 = EA_ADDRESS_REGISTER_INDIRECT
                        | EA_ARI_DISPLACEMENT
                        | EA_ARI_INDEX
                        | EA_ABSOLUTE_SHORT
                        | EA_ABSOLUTE_LONG
                        | EA_PC_DISPLACEMENT
                        | EA_PC_INDEX;
const EA_CONTROL_ALTERABLE_OR_PD: u16 = EA_CONTROL & EA_ALTERABLE | EA_ARI_PREDECREMENT;
const EA_CONTROL_OR_PI: u16 = EA_CONTROL | EA_ARI_POSTINCREMENT;
const EA_DATA: u16 = EA_ALL & !(EA_ADDRESS_REGISTER_DIRECT | EA_IMMEDIATE);
const EA_DATA_ALTERABLE: u16 = EA_DATA & EA_ALTERABLE;
const EA_MEMORY_ALTERABLE: u16 = EA_ALTERABLE & !(EA_DATA_REGISTER_DIRECT | EA_ADDRESS_REGISTER_DIRECT);
const EA_NONE: u16 = 0x000;

/* Check if opcode is using a valid ea mode */
fn valid_ea(opcode: u16, mask: u16) -> bool
{
    if mask == 0 {
        true
    } else {
        match opcode & 0x3f {
            0x00 ... 0x07 => (mask & EA_DATA_REGISTER_DIRECT) != 0,
            0x08 ... 0x0f => (mask & EA_ADDRESS_REGISTER_DIRECT) != 0,
            0x10 ... 0x17 => (mask & EA_ADDRESS_REGISTER_INDIRECT) != 0,
            0x18 ... 0x1f => (mask & EA_ARI_POSTINCREMENT) != 0,
            0x20 ... 0x27 => (mask & EA_ARI_PREDECREMENT) != 0,
            0x28 ... 0x2f => (mask & EA_ARI_DISPLACEMENT) != 0,
            0x30 ... 0x37 => (mask & EA_ARI_INDEX) != 0,
            0x38 => (mask & EA_ABSOLUTE_SHORT) != 0,
            0x39 => (mask & EA_ABSOLUTE_LONG) != 0,
            0x3a => (mask & EA_PC_DISPLACEMENT) != 0,
            0x3b => (mask & EA_PC_INDEX) != 0,
            0x3c => (mask & EA_IMMEDIATE) != 0,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use operand::Operand;
    use memory::{MemoryVec, Memory};
    use assembler::{parse_assembler, parse_assembler_re, encode_instruction};
    use super::{Size, disassemble, disassemble_first, Exception};
    use regex::Regex;

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
        let new_pc = encode_instruction(asm.as_str(), &inst, pc, mem);
        assert_eq!(2, new_pc);
        assert_eq!(opcode, mem.read_word(pc));
    }
    #[test]
    fn roundtrips_from_asm() {
        let mut mem = &mut MemoryVec { mem: vec![0x00,0x00,0x00,0x00]} ;
        let pc = 0;
        let asm = "ADD.B\tD2,(A1)";
        let inst = parse_assembler(asm);
        encode_instruction(asm, &inst, pc, mem);
        let inst = disassemble_first(mem);

        assert_eq!(asm, format!("{}", inst));
    }

    fn opcodes(mask: u32, matching: u32) -> Vec<u16> {
        (matching..0x10000u32)
            .filter(|opcode| (opcode & mask) == matching)
            .map(|v|v as u16).collect::<Vec<u16>>()
    }

    #[test]
    #[ignore]
    fn roundtrips() {
        let re = Regex::new(r"^(\w+)(\.\w)?(\s+(\w\d|-?\$?[\dA-F]*\([\w,0-9]+\)\+?|#?\$?[\dA-F]+(?:\.\w)?)(,(\w\d|-?\$?[\dA-F]*\([\w,0-9]+\)\+?|#?-?\$?[\dA-F]+(?:\.\w)?))?)$").unwrap();
        for opcode in 0x0600..0xe000 {
            let pc = 0;
            let extension_word_mask = 0b1111_1000_1111_1111; 
            // bits 8-10 should always be zero in the ea extension word
            // as we don't know which word will be seen as the ea extension word
            // (as opposed to immediate operand values) just make sure these aren't set.
            let dasm_mem = &mut MemoryVec { mem: vec![opcode, 0x001f, 0x00a4, 0x1234 & extension_word_mask, 0x5678 & extension_word_mask]} ;
            match disassemble(pc, dasm_mem) {
                Err(Exception::IllegalInstruction(opcode, _)) => println!("{:04x}:\t\tinvalid", opcode),
                Ok(inst) => {
                    let asm = format!("{}", inst);
                    let inst = parse_assembler_re(&re, asm.as_str());
                    let mut asm_mem = &mut MemoryVec { mem: vec![0x0000, 0x0000, 0x0000, 0x0000, 0x0000]};
                    let new_pc = encode_instruction(asm.as_str(), &inst, pc, asm_mem);
                    assert_eq!(inst.length()*2, new_pc);
                    let new_opcode = asm_mem.read_word(pc);
                    if opcode != new_opcode {
                        panic!("{:04x} | {:04x}: {}", opcode, new_opcode, asm);
                    } else {
                        println!("{:04x}: {}", opcode, asm);
                    }
                    if inst.length() > 1 {
                        let old_ex1 = dasm_mem.read_word(pc+2);
                        let new_ex1 = asm_mem.read_word(pc+2);
                        if old_ex1 != new_ex1 {println!("ew1")};
                        assert_eq!(old_ex1, new_ex1);
                    };
                    if inst.length() > 2 {
                        let old_ex2 = dasm_mem.read_word(pc+4);
                        let new_ex2 = asm_mem.read_word(pc+4);
                        if old_ex2 != new_ex2 {println!("ew2")};
                        assert_eq!(old_ex2, new_ex2);
                    };
                    if inst.length() > 3 {
                        let old_ex3 = dasm_mem.read_word(pc+6);
                        let new_ex3 = asm_mem.read_word(pc+6);
                        if old_ex3 != new_ex3 {println!("ew3")};
                        assert_eq!(old_ex3, new_ex3);
                    };
                }
            }
        }
    }

    #[test]
    fn two_word_imm_ea() {
        // ADDI #$12,$34(A0) is 0x0668 0x0012 0x0034
        let dasm_mem = &mut MemoryVec { mem: vec![0x0668, 0x0012, 0x0034]} ;
        let ops = super::imm_ea(0x0668, Size::Byte, 0, dasm_mem);
        assert_eq!(ops[0], Operand::Immediate(Size::Byte, 0x12));
        assert_eq!(ops[1], Operand::AddressRegisterIndirectWithDisplacement(0, 0x34));
    }
    use super::{EA_ALL_EXCEPT_AN, EA_ALTERABLE, EA_CONTROL ,
    EA_CONTROL_ALTERABLE_OR_PD, EA_CONTROL_OR_PI, EA_DATA ,
    EA_DATA_ALTERABLE , EA_MEMORY_ALTERABLE, EA_ADDRESS_REGISTER_DIRECT,
    EA_IMMEDIATE, EA_PC_DISPLACEMENT, EA_PC_INDEX, EA_ARI_PREDECREMENT,
    EA_ARI_POSTINCREMENT, EA_DATA_REGISTER_DIRECT};

    #[test]
    fn ea_all_except_an() {
        assert_eq!(EA_ALL_EXCEPT_AN & EA_ADDRESS_REGISTER_DIRECT, 0);
    }
    #[test]
    fn ea_alterable() {
        assert_eq!(EA_ALTERABLE & (EA_IMMEDIATE|EA_PC_DISPLACEMENT|EA_PC_INDEX), 0);
    }
    #[test]
    fn ea_control() {
        assert_eq!(EA_CONTROL, 0x27b);
    }
    #[test]
    fn ea_control_alterable_or_pd() {
        assert_eq!(EA_CONTROL_ALTERABLE_OR_PD & EA_ARI_PREDECREMENT, EA_ARI_PREDECREMENT);
    }
    #[test]
    fn ea_control_or_pi() {
        assert_eq!(EA_CONTROL_OR_PI & EA_ARI_POSTINCREMENT, EA_ARI_POSTINCREMENT);
    }
    #[test]
    fn ea_data() {
        assert_eq!(EA_DATA & (EA_ADDRESS_REGISTER_DIRECT | EA_IMMEDIATE), 0);
    }
    #[test]
    fn ea_data_alterable() {
        assert_eq!(EA_DATA_ALTERABLE, EA_DATA & EA_ALTERABLE);
    }
    #[test]
    fn ea_memory_alterable() {
        assert_eq!(EA_MEMORY_ALTERABLE & (EA_DATA_REGISTER_DIRECT | EA_ADDRESS_REGISTER_DIRECT), 0);
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