use ram::{AddressBus, OpsLogging, LoggingMem, SUPERVISOR_PROGRAM};
use cpu::ops::handlers::*;
type OperandDecoder = fn(u32) -> Vec<Operand>;

#[derive(Clone, Copy, Debug, PartialEq)] 
enum Size {
	Unsized, Byte, Word, Long
}
#[derive(Clone, Copy, Debug, PartialEq)] 
enum Operand {
	DataRegisterDirect(u8),
	AddressRegisterDirect(u8),
	AddressRegisterIndirect(u8),
	AddressRegisterIndirectWithPredecrement(u8),
	AddressRegisterIndirectWithPostincrement(u8),
	AddressRegisterIndirectWithDisplacement(u8, u8),
	AddressRegisterIndirectWithIndex(u8, u8, u8),
	PcWithDisplacement(u8),
	PcWithIndex(u8, u8),
	AbsoluteWord(u16),
	AbsoluteLong(u32),
	Immediate(u16),
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
#[derive(Clone, Copy, Debug)] 
pub struct OpcodeInfo {
    mask: u32,
    matching: u32,
    size: Size,
    decoder: OperandDecoder,
    mnemonic: &'static str,
}

macro_rules! instruction {
    ($mask:expr, $matching:expr, $size:expr, $mnemonic:expr, $decoder:ident) => (OpcodeInfo { mask: $mask, matching: $matching, size: $size, mnemonic: $mnemonic, decoder: $decoder})
}
fn get_ea(opcode: u32) -> Operand {
	let mode = ((opcode >> 3) & 7) as u8;
	let reg = (opcode & 7) as u8;
	let (index, displacement, word, long, data) = (0, 0, 0, 0, 0);
	match mode {
		0b000 => Operand::DataRegisterDirect(reg),
		0b001 => Operand::AddressRegisterDirect(reg),
		0b010 => Operand::AddressRegisterIndirect(reg),
		0b011 => Operand::AddressRegisterIndirectWithPostincrement(reg),
		0b100 => Operand::AddressRegisterIndirectWithPredecrement(reg),
		0b101 => Operand::AddressRegisterIndirectWithDisplacement(reg, displacement),
		0b110 => Operand::AddressRegisterIndirectWithIndex(reg, index, displacement),
		0b111 => match reg {
			0b010 => Operand::PcWithDisplacement(displacement),
			0b011 => Operand::PcWithIndex(index, displacement),
			0b000 => Operand::AbsoluteWord(word),
			0b001 => Operand::AbsoluteLong(long),
			0b100 => Operand::Immediate(data),
			_ => panic!("Unknown addressing mode {:?} reg {:?}", mode, reg),
		},
		_ => panic!("Unknown addressing mode {:?} reg {:?}", mode, reg),
	}
}
fn get_dx(opcode: u32) -> Operand {
	Operand::DataRegisterDirect(((opcode >> 9) & 7) as u8)
}
fn ea_dx(opcode: u32) -> Vec<Operand> {
	vec![get_ea(opcode), get_dx(opcode)]
}
pub const MASK_OUT_X_EA: u32 = 0b1111000111000000; // masks out X and Y register bits, plus mode (????xxx???mmmyyy)

pub fn disassemble_first<T:OpsLogging>(mem: &LoggingMem<T>) -> OpcodeInfo {
    let optable = vec![
        instruction!(MASK_OUT_X_EA, OP_ADD | BYTE_SIZED | DEST_DX, Size::Byte, "ADD", ea_dx),
	];
	let opcode = mem.read_word(SUPERVISOR_PROGRAM, 0);
	println!("opcode read was {:04x}", opcode);
	for op in optable {
		if (opcode & op.mask) == op.matching {
			let decoder = op.decoder;
			println!("Operands {:?}", decoder(opcode));
			return op;
		}
	}
	panic!("Could not disassemble {:04x}", opcode);
}

#[cfg(test)]
mod tests {
    use ram::{AddressBus, LoggingMem, NopLogger, Operation, SUPERVISOR_PROGRAM, USER_PROGRAM, USER_DATA};
    use super::Size;

    #[test]
    fn decodes_add_8_er() {
        let mem = LoggingMem::new(0xd000ffff, NopLogger);
        assert_eq!("ADD", super::disassemble_first(&mem).mnemonic);
        assert_eq!(Size::Byte, super::disassemble_first(&mem).size);
    }
}