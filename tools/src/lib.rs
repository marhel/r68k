#![recursion_limit = "160"] // 150 was too low in rust 1.15
use std::result;
mod operand;
use operand::Operand;
extern crate r68k_common;
use r68k_common::ops::*;
use r68k_common::constants::*;
mod constants;
use constants::*;
#[macro_use]
extern crate pest;

pub mod memory;
pub mod assembler;
pub mod disassembler;
pub mod srecords;

use memory::Memory;

// type alias for exception handling
pub type Result<T> = result::Result<T, Exception>;
type OpcodeValidator = fn(u16) -> bool;
type OperandDecoder = fn(u16, Size, PC, &Memory) -> (Words, Vec<Operand>);
type InstructionEncoder = fn(&OpcodeInstance, u16, PC, &mut Memory) -> PC;
type InstructionSelector = fn(&OpcodeInstance) -> bool;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PC(pub u32);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Words(pub u8);

use std::ops::Sub;
use std::ops::Add;
impl Sub for PC {
    type Output = PC;

    fn sub(self, rhs: PC) -> PC {
        PC(self.0 - rhs.0)
    }
}
impl Add for PC {
    type Output = PC;

    fn add(self, rhs: PC) -> PC {
        PC(self.0 + rhs.0)
    }
}
impl Add<u32> for PC {
    type Output = PC;

    fn add(self, rhs: u32) -> PC {
        PC(self.0 + rhs)
    }
}
impl Add<i32> for PC {
    type Output = PC;

    fn add(self, rhs: i32) -> PC {
        PC((self.0 as i32 + rhs) as u32)
    }
}
impl Add<Words> for PC {
    type Output = PC;

    fn add(self, rhs: Words) -> <Self as Add<Words>>::Output {
        PC(self.0 + (rhs.0 * 2) as u32)
    }
}
impl Add for Words {
    type Output = Words;

    fn add(self, rhs: Words) -> Words {
        Words(self.0 + rhs.0)
    }
}
impl PC {
    fn is_odd(&self) -> bool {
        self.0 % 2 == 1
    }
}
impl From<PC> for usize {
    fn from(pc: PC) -> Self {
        pc.0 as usize
    }
}
impl PartialEq<PC> for u32 {
    fn eq(&self, other: &PC) -> bool {
        *self == other.0
    }
}
impl PartialEq<u32> for PC {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}
impl std::fmt::LowerHex for PC {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:06x}", self.0)
    }
}
impl std::fmt::UpperHex for PC {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:06X}", self.0)
    }
}
#[derive(Debug)]
pub enum Exception {
     IllegalInstruction(u16, PC), // ir, pc
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Size {
	Unsized, Byte, Word, Long
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

// #[derive(Clone, Copy)]
pub struct OpcodeInfo<'a> {
    mask: u32,
    matching: u32,
    size: Size,
    validator: OpcodeValidator,
    decoder: OperandDecoder,
    mnemonic: &'a str,
    synonym: Option<&'a str>,
    encoder: InstructionEncoder,
    selector: InstructionSelector,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OpcodeInstance<'a> {
    pub mnemonic: &'a str,
    pub size: Size,
    pub operands: Vec<Operand>,
}

use std::fmt;
use std::fmt::Formatter;

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
    ($mask:expr, $matching:expr, $size:expr, $mnemonic:expr, $validator:ident, $decoder:ident) =>                                  (OpcodeInfo { mask: $mask, matching: $matching, size: $size, mnemonic: $mnemonic, synonym: None, validator: disassembler::$validator, decoder: disassembler::$decoder, encoder: assembler::nop_encoder, selector: assembler::nop_selector});
    ($mask:expr, $matching:expr, $size:expr, $mnemonic:expr, $validator:ident, $decoder:ident, $selector:ident, $encoder:ident) => (OpcodeInfo { mask: $mask, matching: $matching, size: $size, mnemonic: $mnemonic, synonym: None, validator: disassembler::$validator, decoder: disassembler::$decoder, encoder: assembler::$encoder, selector: assembler::$selector});
    ($mask:expr, $matching:expr, $size:expr, $mnemonic:expr, $synonym:expr, $validator:ident, $decoder:ident, $selector:ident, $encoder:ident) => (OpcodeInfo { mask: $mask, matching: $matching, size: $size, mnemonic: $mnemonic, synonym: Some($synonym), validator: disassembler::$validator, decoder: disassembler::$decoder, encoder: assembler::$encoder, selector: assembler::$selector});
}
fn generate<'a>() -> Vec<OpcodeInfo<'a>> {
    vec![
        instruction!(MASK_OUT_X_Y, OP_ABCD | BYTE_SIZED | RR_MODE, Size::Byte, "ABCD", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_ABCD | BYTE_SIZED | MM_MODE, Size::Byte, "ABCD", always, decode_pdx_pdy, is_pd_pd, encode_pdx_pdy),
        instruction!(MASK_OUT_X_Y, OP_SBCD | BYTE_SIZED | RR_MODE, Size::Byte, "SBCD", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SBCD | BYTE_SIZED | MM_MODE, Size::Byte, "SBCD", always, decode_pdx_pdy, is_pd_pd, encode_pdx_pdy),
        instruction!(MASK_OUT_EA, OP_NBCD, Size::Byte, "NBCD", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),

        instruction!(MASK_OUT_X_EA, OP_ADD | BYTE_SIZED | DEST_DX, Size::Byte, "ADD", ea_all_except_an, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_ADD | BYTE_SIZED | DEST_EA, Size::Byte, "ADD", ea_memory_alterable, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_OUT_X_EA, OP_ADD | WORD_SIZED | DEST_DX, Size::Word, "ADD", ea_all, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_ADD | WORD_SIZED | DEST_EA, Size::Word, "ADD", ea_memory_alterable, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_OUT_X_EA, OP_ADD | LONG_SIZED | DEST_DX, Size::Long, "ADD", ea_all, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_ADD | LONG_SIZED | DEST_EA, Size::Long, "ADD", ea_memory_alterable, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_OUT_X_EA, OP_ADD | DEST_AX_WORD, Size::Word, "ADDA", ea_all, decode_ea_ax, is_ea_an, encode_ea_ax),
        instruction!(MASK_OUT_X_EA, OP_ADD | DEST_AX_LONG, Size::Long, "ADDA", ea_all, decode_ea_ax, is_ea_an, encode_ea_ax),
        instruction!(MASK_OUT_X_EA, OP_ADDQ | BYTE_SIZED, Size::Byte, "ADDQ", ea_alterable_except_an, decode_quick_ea, is_imm_ea, encode_quick_ea),
        instruction!(MASK_OUT_X_EA, OP_ADDQ | WORD_SIZED, Size::Word, "ADDQ", ea_alterable, decode_quick_ea, is_imm_ea, encode_quick_ea),
        instruction!(MASK_OUT_X_EA, OP_ADDQ | LONG_SIZED, Size::Long, "ADDQ", ea_alterable, decode_quick_ea, is_imm_ea, encode_quick_ea),
        instruction!(MASK_OUT_EA, OP_ADDI | BYTE_SIZED, Size::Byte, "ADDI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_EA, OP_ADDI | WORD_SIZED, Size::Word, "ADDI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_EA, OP_ADDI | LONG_SIZED, Size::Long, "ADDI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_X_Y, OP_ADDX | BYTE_SIZED | RR_MODE, Size::Byte, "ADDX", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_ADDX | BYTE_SIZED | MM_MODE, Size::Byte, "ADDX", always, decode_pdx_pdy, is_pd_pd, encode_pdx_pdy),
        instruction!(MASK_OUT_X_Y, OP_ADDX | WORD_SIZED | RR_MODE, Size::Word, "ADDX", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_ADDX | WORD_SIZED | MM_MODE, Size::Word, "ADDX", always, decode_pdx_pdy, is_pd_pd, encode_pdx_pdy),
        instruction!(MASK_OUT_X_Y, OP_ADDX | LONG_SIZED | RR_MODE, Size::Long, "ADDX", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_ADDX | LONG_SIZED | MM_MODE, Size::Long, "ADDX", always, decode_pdx_pdy, is_pd_pd, encode_pdx_pdy),

        instruction!(MASK_OUT_X_EA, OP_AND | BYTE_SIZED | DEST_DX, Size::Byte, "AND", ea_data, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_AND | BYTE_SIZED | DEST_EA, Size::Byte, "AND", ea_memory_alterable, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_OUT_X_EA, OP_AND | WORD_SIZED | DEST_DX, Size::Word, "AND", ea_data, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_AND | WORD_SIZED | DEST_EA, Size::Word, "AND", ea_memory_alterable, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_OUT_X_EA, OP_AND | LONG_SIZED | DEST_DX, Size::Long, "AND", ea_data, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_AND | LONG_SIZED | DEST_EA, Size::Long, "AND", ea_memory_alterable, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_EXACT, OP_ANDI | BYTE_SIZED | DEST_SR, Size::Byte, "ANDI", always, decode_imm_ccr, is_imm_ccr, encode_just_imm),
        instruction!(MASK_EXACT, OP_ANDI | WORD_SIZED | DEST_SR, Size::Word, "ANDI", always, decode_imm_sr, is_imm_sr, encode_just_imm),
        instruction!(MASK_OUT_EA, OP_ANDI | BYTE_SIZED, Size::Byte, "ANDI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_EA, OP_ANDI | WORD_SIZED, Size::Word, "ANDI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_EA, OP_ANDI | LONG_SIZED, Size::Long, "ANDI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),

        instruction!(MASK_OUT_X_EA, OP_OR | BYTE_SIZED | DEST_DX, Size::Byte, "OR", ea_data, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_OR | BYTE_SIZED | DEST_EA, Size::Byte, "OR", ea_memory_alterable, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_OUT_X_EA, OP_OR | WORD_SIZED | DEST_DX, Size::Word, "OR", ea_data, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_OR | WORD_SIZED | DEST_EA, Size::Word, "OR", ea_memory_alterable, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_OUT_X_EA, OP_OR | LONG_SIZED | DEST_DX, Size::Long, "OR", ea_data, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_OR | LONG_SIZED | DEST_EA, Size::Long, "OR", ea_memory_alterable, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_EXACT, OP_ORI | BYTE_SIZED | DEST_SR, Size::Byte, "ORI", always, decode_imm_ccr, is_imm_ccr, encode_just_imm),
        instruction!(MASK_EXACT, OP_ORI | WORD_SIZED | DEST_SR, Size::Word, "ORI", always, decode_imm_sr, is_imm_sr, encode_just_imm),
        instruction!(MASK_OUT_EA, OP_ORI | BYTE_SIZED, Size::Byte, "ORI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_EA, OP_ORI | WORD_SIZED, Size::Word, "ORI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_EA, OP_ORI | LONG_SIZED, Size::Long, "ORI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),

        instruction!(MASK_OUT_X_EA, OP_EOR | BYTE_SIZED | DEST_EA, Size::Byte, "EOR", ea_data_alterable, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_OUT_X_EA, OP_EOR | WORD_SIZED | DEST_EA, Size::Word, "EOR", ea_data_alterable, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_OUT_X_EA, OP_EOR | LONG_SIZED | DEST_EA, Size::Long, "EOR", ea_data_alterable, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_EXACT, OP_EORI | BYTE_SIZED | DEST_SR, Size::Byte, "EORI", always, decode_imm_ccr, is_imm_ccr, encode_just_imm),
        instruction!(MASK_EXACT, OP_EORI | WORD_SIZED | DEST_SR, Size::Word, "EORI", always, decode_imm_sr, is_imm_sr, encode_just_imm),
        instruction!(MASK_OUT_EA, OP_EORI | BYTE_SIZED, Size::Byte, "EORI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_EA, OP_EORI | WORD_SIZED, Size::Word, "EORI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_EA, OP_EORI | LONG_SIZED, Size::Long, "EORI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),

        instruction!(MASK_OUT_X_EA, OP_MOVE | WORD_MOVE | MOVE_TO_AN, Size::Word, "MOVEA", ea_all, decode_ea_ea, is_ea_ea, encode_ea_ea),
        instruction!(MASK_OUT_X_EA, OP_MOVE | LONG_MOVE | MOVE_TO_AN, Size::Long, "MOVEA", ea_all, decode_ea_ea, is_ea_ea, encode_ea_ea),
        instruction!(MASK_OUT_EA, OP_MOVE2 | MOVE_FROM_SR, Size::Word, "MOVE", ea_data_alterable, decode_sr_ea, is_sr_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_MOVE2 | MOVE_TO_SR, Size::Word, "MOVE", ea_data, decode_ea_sr, is_ea_sr, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_MOVE2 | MOVE_TO_CCR, Size::Word, "MOVE", ea_data, decode_ea_ccr, is_ea_ccr, encode_just_ea),
        instruction!(MASK_OUT_Y, OP_MOVE2 | MOVE_USP | TO_AN, Size::Long, "MOVE", always, decode_usp_ay, is_usp_an, encode_just_ay),
        instruction!(MASK_OUT_Y, OP_MOVE2 | MOVE_USP | FROM_AN, Size::Long, "MOVE", always, decode_ay_usp, is_an_usp, encode_just_ay),
        instruction!(MASK_LO3NIB, OP_MOVE | BYTE_MOVE, Size::Byte, "MOVE", ea_all_except_an_to_data_alterable, decode_ea_ea, is_ea_ea, encode_ea_ea),
        instruction!(MASK_LO3NIB, OP_MOVE | WORD_MOVE, Size::Word, "MOVE", ea_all_to_data_alterable, decode_ea_ea, is_ea_ea, encode_ea_ea),
        instruction!(MASK_LO3NIB, OP_MOVE | LONG_MOVE, Size::Long, "MOVE", ea_all_to_data_alterable, decode_ea_ea, is_ea_ea, encode_ea_ea),
        instruction!(MASK_LOBYTX, OP_MOVEQ, Size::Long, "MOVEQ", always, decode_moveq, is_moveq, encode_moveq),
        instruction!(MASK_OUT_EA, OP_MOVEM | REGISTER_TO_MEMORY | WORD_TRANSFER, Size::Word, "MOVEM", ea_control_alterable_or_pd, decode_movem_ea, is_movem_ea, encode_movem_ea),
        instruction!(MASK_OUT_EA, OP_MOVEM | MEMORY_TO_REGISTER | WORD_TRANSFER, Size::Word, "MOVEM", ea_control_or_pi, decode_ea_movem, is_ea_movem, encode_ea_movem),
        instruction!(MASK_OUT_EA, OP_MOVEM | REGISTER_TO_MEMORY | LONG_TRANSFER, Size::Long, "MOVEM", ea_control_alterable_or_pd, decode_movem_ea, is_movem_ea, encode_movem_ea),
        instruction!(MASK_OUT_EA, OP_MOVEM | MEMORY_TO_REGISTER | LONG_TRANSFER, Size::Long, "MOVEM", ea_control_or_pi, decode_ea_movem, is_ea_movem, encode_ea_movem),
        instruction!(MASK_OUT_X_Y, OP_MOVEP | WORD_TRANSFER | MOVEP_MEMORY_TO_REGISTER, Size::Word, "MOVEP", always, decode_diy_dx, is_di_dn, encode_diy_dx),
        instruction!(MASK_OUT_X_Y, OP_MOVEP | WORD_TRANSFER | MOVEP_REGISTER_TO_MEMORY, Size::Word, "MOVEP", always, decode_dx_diy, is_dn_di, encode_dx_diy),
        instruction!(MASK_OUT_X_Y, OP_MOVEP | LONG_TRANSFER | MOVEP_MEMORY_TO_REGISTER, Size::Long, "MOVEP", always, decode_diy_dx, is_di_dn, encode_diy_dx),
        instruction!(MASK_OUT_X_Y, OP_MOVEP | LONG_TRANSFER | MOVEP_REGISTER_TO_MEMORY, Size::Long, "MOVEP", always, decode_dx_diy, is_dn_di, encode_dx_diy),
        instruction!(MASK_OUT_X_EA, OP_LEA, Size::Long, "LEA", ea_control, decode_ea_ax, is_ea_an, encode_ea_ax),
        instruction!(MASK_OUT_EA, OP_PEA, Size::Long, "PEA", ea_control, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_Y, OP_LINK, Size::Word, "LINK", always, decode_ay_imm16, is_an_imm16, encode_ay_imm16),
        instruction!(MASK_OUT_Y, OP_UNLK, Size::Unsized, "UNLK", always, decode_just_ay, is_an, encode_just_ay),

        instruction!(MASK_LOBYTE, OP_BRANCH | IF_HI, Size::Byte, "BHI", valid_byte_displacement, decode_branch, is_branch, encode_branch),
        instruction!(MASK_LOBYTE, OP_BRANCH | IF_LS, Size::Byte, "BLS", valid_byte_displacement, decode_branch, is_branch, encode_branch),
        instruction!(MASK_LOBYTE, OP_BRANCH | IF_CC, Size::Byte, "BCC", "BHS", valid_byte_displacement, decode_branch, is_branch, encode_branch),
        instruction!(MASK_LOBYTE, OP_BRANCH | IF_CS, Size::Byte, "BCS", "BLO", valid_byte_displacement, decode_branch, is_branch, encode_branch),
        instruction!(MASK_LOBYTE, OP_BRANCH | IF_NE, Size::Byte, "BNE", valid_byte_displacement, decode_branch, is_branch, encode_branch),
        instruction!(MASK_LOBYTE, OP_BRANCH | IF_EQ, Size::Byte, "BEQ", valid_byte_displacement, decode_branch, is_branch, encode_branch),
        instruction!(MASK_LOBYTE, OP_BRANCH | IF_VC, Size::Byte, "BVC", valid_byte_displacement, decode_branch, is_branch, encode_branch),
        instruction!(MASK_LOBYTE, OP_BRANCH | IF_VS, Size::Byte, "BVS", valid_byte_displacement, decode_branch, is_branch, encode_branch),
        instruction!(MASK_LOBYTE, OP_BRANCH | IF_PL, Size::Byte, "BPL", valid_byte_displacement, decode_branch, is_branch, encode_branch),
        instruction!(MASK_LOBYTE, OP_BRANCH | IF_MI, Size::Byte, "BMI", valid_byte_displacement, decode_branch, is_branch, encode_branch),
        instruction!(MASK_LOBYTE, OP_BRANCH | IF_GE, Size::Byte, "BGE", valid_byte_displacement, decode_branch, is_branch, encode_branch),
        instruction!(MASK_LOBYTE, OP_BRANCH | IF_LT, Size::Byte, "BLT", valid_byte_displacement, decode_branch, is_branch, encode_branch),
        instruction!(MASK_LOBYTE, OP_BRANCH | IF_GT, Size::Byte, "BGT", valid_byte_displacement, decode_branch, is_branch, encode_branch),
        instruction!(MASK_LOBYTE, OP_BRANCH | IF_LE, Size::Byte, "BLE", valid_byte_displacement, decode_branch, is_branch, encode_branch),
        instruction!(MASK_LOBYTE, OP_BRANCH | IF_T , Size::Byte, "BRA", valid_byte_displacement, decode_branch, is_branch, encode_branch),
        instruction!(MASK_LOBYTE, OP_BRANCH | IF_F , Size::Byte, "BSR", valid_byte_displacement, decode_branch, is_branch, encode_branch),

        instruction!(MASK_EXACT, OP_BRANCH | IF_HI | DISPLACEMENT_16, Size::Word, "BHI", always, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_LS | DISPLACEMENT_16, Size::Word, "BLS", always, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_CC | DISPLACEMENT_16, Size::Word, "BCC", "BHS", always, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_CS | DISPLACEMENT_16, Size::Word, "BCS", "BLO", always, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_NE | DISPLACEMENT_16, Size::Word, "BNE", always, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_EQ | DISPLACEMENT_16, Size::Word, "BEQ", always, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_VC | DISPLACEMENT_16, Size::Word, "BVC", always, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_VS | DISPLACEMENT_16, Size::Word, "BVS", always, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_PL | DISPLACEMENT_16, Size::Word, "BPL", always, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_MI | DISPLACEMENT_16, Size::Word, "BMI", always, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_GE | DISPLACEMENT_16, Size::Word, "BGE", always, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_LT | DISPLACEMENT_16, Size::Word, "BLT", always, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_GT | DISPLACEMENT_16, Size::Word, "BGT", always, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_LE | DISPLACEMENT_16, Size::Word, "BLE", always, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_T  | DISPLACEMENT_16, Size::Word, "BRA", always, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_F  | DISPLACEMENT_16, Size::Word, "BSR", always, decode_branch, is_branch, encode_branch),

        // 'never' means we never accept these instructions (long branch isn't supported on 68000)
        instruction!(MASK_EXACT, OP_BRANCH | IF_HI | DISPLACEMENT_32, Size::Long, "BHI", never, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_LS | DISPLACEMENT_32, Size::Long, "BLS", never, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_CC | DISPLACEMENT_32, Size::Long, "BCC", "BHS", never, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_CS | DISPLACEMENT_32, Size::Long, "BCS", "BLO", never, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_NE | DISPLACEMENT_32, Size::Long, "BNE", never, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_EQ | DISPLACEMENT_32, Size::Long, "BEQ", never, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_VC | DISPLACEMENT_32, Size::Long, "BVC", never, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_VS | DISPLACEMENT_32, Size::Long, "BVS", never, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_PL | DISPLACEMENT_32, Size::Long, "BPL", never, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_MI | DISPLACEMENT_32, Size::Long, "BMI", never, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_GE | DISPLACEMENT_32, Size::Long, "BGE", never, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_LT | DISPLACEMENT_32, Size::Long, "BLT", never, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_GT | DISPLACEMENT_32, Size::Long, "BGT", never, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_LE | DISPLACEMENT_32, Size::Long, "BLE", never, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_T  | DISPLACEMENT_32, Size::Long, "BRA", never, decode_branch, is_branch, encode_branch),
        instruction!(MASK_EXACT, OP_BRANCH | IF_F  | DISPLACEMENT_32, Size::Long, "BSR", never, decode_branch, is_branch, encode_branch),

        instruction!(MASK_OUT_X_EA, OP_CMP | DEST_AX_WORD, Size::Word, "CMPA", ea_all, decode_ea_ax, is_ea_an, encode_ea_ax),
        instruction!(MASK_OUT_X_EA, OP_CMP | DEST_AX_LONG, Size::Long, "CMPA", ea_all, decode_ea_ax, is_ea_an, encode_ea_ax),
        instruction!(MASK_OUT_X_EA, OP_CMP | BYTE_SIZED, Size::Byte, "CMP", ea_all_except_an, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_CMP | WORD_SIZED, Size::Word, "CMP", ea_all, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_CMP | LONG_SIZED, Size::Long, "CMP", ea_all, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_EA, OP_CMPI | BYTE_SIZED, Size::Byte, "CMPI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_EA, OP_CMPI | WORD_SIZED, Size::Word, "CMPI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_EA, OP_CMPI | LONG_SIZED, Size::Long, "CMPI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_X_Y, OP_CMPM | BYTE_SIZED | MM_MODE, Size::Byte, "CMPM", always, decode_pix_piy, is_pi_pi, encode_pix_piy),
        instruction!(MASK_OUT_X_Y, OP_CMPM | WORD_SIZED | MM_MODE, Size::Word, "CMPM", always, decode_pix_piy, is_pi_pi, encode_pix_piy),
        instruction!(MASK_OUT_X_Y, OP_CMPM | LONG_SIZED | MM_MODE, Size::Long, "CMPM", always, decode_pix_piy, is_pi_pi, encode_pix_piy),
        instruction!(MASK_OUT_EA, OP_CLR | BYTE_SIZED, Size::Byte, "CLR", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_CLR | WORD_SIZED, Size::Word, "CLR", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_CLR | LONG_SIZED, Size::Long, "CLR", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_X_EA, OP_CHK | WORD_OP, Size::Word, "CHK", ea_data, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_EA, OP_NOT | BYTE_SIZED, Size::Byte, "NOT", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_NOT | WORD_SIZED, Size::Word, "NOT", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_NOT | LONG_SIZED, Size::Long, "NOT", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_NEG | BYTE_SIZED, Size::Byte, "NEG", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_NEG | WORD_SIZED, Size::Word, "NEG", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_NEG | LONG_SIZED, Size::Long, "NEG", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_NEGX | BYTE_SIZED, Size::Byte, "NEGX", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_NEGX | WORD_SIZED, Size::Word, "NEGX", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_NEGX | LONG_SIZED, Size::Long, "NEGX", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_BITOPS | BIT_TST | SRC_IMM, Size::Byte, "BTST", ea_data_except_dn, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_X_EA, OP_BITOPS | BIT_TST | SRC_REG, Size::Byte, "BTST", ea_data_except_dn, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_OUT_Y, OP_BITOPS | BIT_TST | SRC_IMM | OPER_DN, Size::Long, "BTST", ea_dn, decode_imm8_dy, is_imm8_dn, encode_imm8_dy),
        instruction!(MASK_OUT_X_Y, OP_BITOPS | BIT_TST | SRC_REG | OPER_DN, Size::Long, "BTST", ea_dn, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_EA, OP_BITOPS | BIT_SET | SRC_IMM, Size::Byte, "BSET", ea_data_alterable_except_dn, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_X_EA, OP_BITOPS | BIT_SET | SRC_REG, Size::Byte, "BSET", ea_data_alterable_except_dn, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_OUT_Y, OP_BITOPS | BIT_SET | SRC_IMM | OPER_DN, Size::Long, "BSET", ea_dn, decode_imm8_dy, is_imm8_dn, encode_imm8_dy),
        instruction!(MASK_OUT_X_Y, OP_BITOPS | BIT_SET | SRC_REG | OPER_DN, Size::Long, "BSET", ea_dn, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_EA, OP_BITOPS | BIT_CLR | SRC_IMM, Size::Byte, "BCLR", ea_data_alterable_except_dn, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_X_EA, OP_BITOPS | BIT_CLR | SRC_REG, Size::Byte, "BCLR", ea_data_alterable_except_dn, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_OUT_Y, OP_BITOPS | BIT_CLR | SRC_IMM | OPER_DN, Size::Long, "BCLR", ea_dn, decode_imm8_dy, is_imm8_dn, encode_imm8_dy),
        instruction!(MASK_OUT_X_Y, OP_BITOPS | BIT_CLR | SRC_REG | OPER_DN, Size::Long, "BCLR", ea_dn, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_EA, OP_BITOPS | BIT_CHG | SRC_IMM, Size::Byte, "BCHG", ea_data_alterable_except_dn, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_X_EA, OP_BITOPS | BIT_CHG | SRC_REG, Size::Byte, "BCHG", ea_data_alterable_except_dn, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_OUT_Y, OP_BITOPS | BIT_CHG | SRC_IMM | OPER_DN, Size::Long, "BCHG", ea_dn, decode_imm8_dy, is_imm8_dn, encode_imm8_dy),
        instruction!(MASK_OUT_X_Y, OP_BITOPS | BIT_CHG | SRC_REG | OPER_DN, Size::Long, "BCHG", ea_dn, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_EXACT, OP_RTS, Size::Unsized, "RTS", always, decode_none, is_none, encode_none),
        instruction!(MASK_EXACT, OP_RTR, Size::Unsized, "RTR", always, decode_none, is_none, encode_none),
        instruction!(MASK_EXACT, OP_RTE, Size::Unsized, "RTE", always, decode_none, is_none, encode_none),
        instruction!(MASK_OUT_EA, OP_JSR, Size::Unsized, "JSR", ea_control, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_JMP, Size::Unsized, "JMP", ea_control, decode_just_ea, is_ea, encode_just_ea),

        instruction!(MASK_OUT_Y, OP_DBCC | IF_T, Size::Word, "DBT", always, decode_dy_branch, is_dn_branch, encode_dy_branch),
        instruction!(MASK_OUT_Y, OP_DBCC | IF_F, Size::Word, "DBF", "DBRA", always, decode_dy_branch, is_dn_branch, encode_dy_branch),
        instruction!(MASK_OUT_Y, OP_DBCC | IF_HI, Size::Word, "DBHI", always, decode_dy_branch, is_dn_branch, encode_dy_branch),
        instruction!(MASK_OUT_Y, OP_DBCC | IF_LS, Size::Word, "DBLS", always, decode_dy_branch, is_dn_branch, encode_dy_branch),
        instruction!(MASK_OUT_Y, OP_DBCC | IF_CC, Size::Word, "DBCC", "DBHS", always, decode_dy_branch, is_dn_branch, encode_dy_branch),
        instruction!(MASK_OUT_Y, OP_DBCC | IF_CS, Size::Word, "DBCS", "DBLO", always, decode_dy_branch, is_dn_branch, encode_dy_branch),
        instruction!(MASK_OUT_Y, OP_DBCC | IF_NE, Size::Word, "DBNE", always, decode_dy_branch, is_dn_branch, encode_dy_branch),
        instruction!(MASK_OUT_Y, OP_DBCC | IF_EQ, Size::Word, "DBEQ", always, decode_dy_branch, is_dn_branch, encode_dy_branch),
        instruction!(MASK_OUT_Y, OP_DBCC | IF_VC, Size::Word, "DBVC", always, decode_dy_branch, is_dn_branch, encode_dy_branch),
        instruction!(MASK_OUT_Y, OP_DBCC | IF_VS, Size::Word, "DBVS", always, decode_dy_branch, is_dn_branch, encode_dy_branch),
        instruction!(MASK_OUT_Y, OP_DBCC | IF_PL, Size::Word, "DBPL", always, decode_dy_branch, is_dn_branch, encode_dy_branch),
        instruction!(MASK_OUT_Y, OP_DBCC | IF_MI, Size::Word, "DBMI", always, decode_dy_branch, is_dn_branch, encode_dy_branch),
        instruction!(MASK_OUT_Y, OP_DBCC | IF_GE, Size::Word, "DBGE", always, decode_dy_branch, is_dn_branch, encode_dy_branch),
        instruction!(MASK_OUT_Y, OP_DBCC | IF_LT, Size::Word, "DBLT", always, decode_dy_branch, is_dn_branch, encode_dy_branch),
        instruction!(MASK_OUT_Y, OP_DBCC | IF_GT, Size::Word, "DBGT", always, decode_dy_branch, is_dn_branch, encode_dy_branch),
        instruction!(MASK_OUT_Y, OP_DBCC | IF_LE, Size::Word, "DBLE", always, decode_dy_branch, is_dn_branch, encode_dy_branch),

        instruction!(MASK_OUT_EA, OP_SUBI | BYTE_SIZED, Size::Byte, "SUBI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_EA, OP_SUBI | WORD_SIZED, Size::Word, "SUBI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_EA, OP_SUBI | LONG_SIZED, Size::Long, "SUBI", ea_data_alterable, decode_imm_ea, is_imm_ea, encode_imm_ea),
        instruction!(MASK_OUT_X_EA, OP_SUBQ | BYTE_SIZED, Size::Byte, "SUBQ", ea_alterable_except_an, decode_quick_ea, is_imm_ea, encode_quick_ea),
        instruction!(MASK_OUT_X_EA, OP_SUBQ | WORD_SIZED, Size::Word, "SUBQ", ea_alterable, decode_quick_ea, is_imm_ea, encode_quick_ea),
        instruction!(MASK_OUT_X_EA, OP_SUBQ | LONG_SIZED, Size::Long, "SUBQ", ea_alterable, decode_quick_ea, is_imm_ea, encode_quick_ea),
        instruction!(MASK_OUT_X_EA, OP_SUB | BYTE_SIZED | DEST_DX, Size::Byte, "SUB", ea_all_except_an, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_SUB | BYTE_SIZED | DEST_EA, Size::Byte, "SUB", ea_memory_alterable, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_OUT_X_EA, OP_SUB | WORD_SIZED | DEST_DX, Size::Word, "SUB", ea_all, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_SUB | WORD_SIZED | DEST_EA, Size::Word, "SUB", ea_memory_alterable, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_OUT_X_EA, OP_SUB | LONG_SIZED | DEST_DX, Size::Long, "SUB", ea_all, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_SUB | LONG_SIZED | DEST_EA, Size::Long, "SUB", ea_memory_alterable, decode_dx_ea, is_dn_ea, encode_dx_ea),
        instruction!(MASK_OUT_X_EA, OP_SUB | DEST_AX_WORD, Size::Word, "SUBA", ea_all, decode_ea_ax, is_ea_an, encode_ea_ax),
        instruction!(MASK_OUT_X_EA, OP_SUB | DEST_AX_LONG, Size::Long, "SUBA", ea_all, decode_ea_ax, is_ea_an, encode_ea_ax),
        instruction!(MASK_OUT_X_Y, OP_SUBX | BYTE_SIZED | RR_MODE, Size::Byte, "SUBX", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SUBX | BYTE_SIZED | MM_MODE, Size::Byte, "SUBX", always, decode_pdx_pdy, is_pd_pd, encode_pdx_pdy),
        instruction!(MASK_OUT_X_Y, OP_SUBX | WORD_SIZED | RR_MODE, Size::Word, "SUBX", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SUBX | WORD_SIZED | MM_MODE, Size::Word, "SUBX", always, decode_pdx_pdy, is_pd_pd, encode_pdx_pdy),
        instruction!(MASK_OUT_X_Y, OP_SUBX | LONG_SIZED | RR_MODE, Size::Long, "SUBX", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SUBX | LONG_SIZED | MM_MODE, Size::Long, "SUBX", always, decode_pdx_pdy, is_pd_pd, encode_pdx_pdy),
        instruction!(MASK_OUT_Y, OP_SWAP | WORD_SIZED | OPER_DN, Size::Word, "SWAP", always, decode_just_dy, is_dn, encode_just_dy),

        instruction!(MASK_OUT_EA, OP_SCC | IF_T, Size::Byte, "ST", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_SCC | IF_F, Size::Byte, "SF", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_SCC | IF_HI, Size::Byte, "SHI", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_SCC | IF_LS, Size::Byte, "SLS", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_SCC | IF_CC, Size::Byte, "SCC", "SHS", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_SCC | IF_CS, Size::Byte, "SCS", "SLO", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_SCC | IF_NE, Size::Byte, "SNE", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_SCC | IF_EQ, Size::Byte, "SEQ", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_SCC | IF_VC, Size::Byte, "SVC", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_SCC | IF_VS, Size::Byte, "SVS", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_SCC | IF_PL, Size::Byte, "SPL", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_SCC | IF_MI, Size::Byte, "SMI", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_SCC | IF_GE, Size::Byte, "SGE", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_SCC | IF_LT, Size::Byte, "SLT", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_SCC | IF_GT, Size::Byte, "SGT", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_SCC | IF_LE, Size::Byte, "SLE", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),

        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | BYTE_SIZED | ROTA_REG_SHIFT | IMM_COUNT, Size::Byte, "ROL", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | WORD_SIZED | ROTA_REG_SHIFT | IMM_COUNT, Size::Word, "ROL", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | LONG_SIZED | ROTA_REG_SHIFT | IMM_COUNT, Size::Long, "ROL", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | BYTE_SIZED | ROTA_REG_SHIFT | REG_COUNT, Size::Byte, "ROL", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | WORD_SIZED | ROTA_REG_SHIFT | REG_COUNT, Size::Word, "ROL", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | LONG_SIZED | ROTA_REG_SHIFT | REG_COUNT, Size::Long, "ROL", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_EA, OP_SHIFT | SHIFT_LEFT | WORD_SIZED | ROTA_MEM_SHIFT, Size::Word, "ROL", ea_memory_alterable, decode_just_ea, is_ea, encode_just_ea),

        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | BYTE_SIZED | ROTA_REG_SHIFT | IMM_COUNT, Size::Byte, "ROR", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTA_REG_SHIFT | IMM_COUNT, Size::Word, "ROR", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | LONG_SIZED | ROTA_REG_SHIFT | IMM_COUNT, Size::Long, "ROR", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | BYTE_SIZED | ROTA_REG_SHIFT | REG_COUNT, Size::Byte, "ROR", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTA_REG_SHIFT | REG_COUNT, Size::Word, "ROR", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | LONG_SIZED | ROTA_REG_SHIFT | REG_COUNT, Size::Long, "ROR", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_EA, OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTA_MEM_SHIFT, Size::Word, "ROR", ea_memory_alterable, decode_just_ea, is_ea, encode_just_ea),

        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | BYTE_SIZED | ROTX_REG_SHIFT | IMM_COUNT, Size::Byte, "ROXL", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | WORD_SIZED | ROTX_REG_SHIFT | IMM_COUNT, Size::Word, "ROXL", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | LONG_SIZED | ROTX_REG_SHIFT | IMM_COUNT, Size::Long, "ROXL", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | BYTE_SIZED | ROTX_REG_SHIFT | REG_COUNT, Size::Byte, "ROXL", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | WORD_SIZED | ROTX_REG_SHIFT | REG_COUNT, Size::Word, "ROXL", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | LONG_SIZED | ROTX_REG_SHIFT | REG_COUNT, Size::Long, "ROXL", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_EA, OP_SHIFT | SHIFT_LEFT | WORD_SIZED | ROTX_MEM_SHIFT, Size::Word, "ROXL", ea_memory_alterable, decode_just_ea, is_ea, encode_just_ea),

        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | BYTE_SIZED | ROTX_REG_SHIFT | IMM_COUNT, Size::Byte, "ROXR", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTX_REG_SHIFT | IMM_COUNT, Size::Word, "ROXR", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | LONG_SIZED | ROTX_REG_SHIFT | IMM_COUNT, Size::Long, "ROXR", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | BYTE_SIZED | ROTX_REG_SHIFT | REG_COUNT, Size::Byte, "ROXR", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTX_REG_SHIFT | REG_COUNT, Size::Word, "ROXR", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | LONG_SIZED | ROTX_REG_SHIFT | REG_COUNT, Size::Long, "ROXR", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_EA, OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTX_MEM_SHIFT, Size::Word, "ROXR", ea_memory_alterable, decode_just_ea, is_ea, encode_just_ea),

        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | BYTE_SIZED | LOGI_REG_SHIFT | IMM_COUNT, Size::Byte, "LSL", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | WORD_SIZED | LOGI_REG_SHIFT | IMM_COUNT, Size::Word, "LSL", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | LONG_SIZED | LOGI_REG_SHIFT | IMM_COUNT, Size::Long, "LSL", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | BYTE_SIZED | LOGI_REG_SHIFT | REG_COUNT, Size::Byte, "LSL", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | WORD_SIZED | LOGI_REG_SHIFT | REG_COUNT, Size::Word, "LSL", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | LONG_SIZED | LOGI_REG_SHIFT | REG_COUNT, Size::Long, "LSL", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_EA, OP_SHIFT | SHIFT_LEFT | WORD_SIZED | LOGI_MEM_SHIFT, Size::Word, "LSL", ea_memory_alterable, decode_just_ea, is_ea, encode_just_ea),

        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | BYTE_SIZED | LOGI_REG_SHIFT | IMM_COUNT, Size::Byte, "LSR", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | LOGI_REG_SHIFT | IMM_COUNT, Size::Word, "LSR", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | LONG_SIZED | LOGI_REG_SHIFT | IMM_COUNT, Size::Long, "LSR", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | BYTE_SIZED | LOGI_REG_SHIFT | REG_COUNT, Size::Byte, "LSR", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | LOGI_REG_SHIFT | REG_COUNT, Size::Word, "LSR", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | LONG_SIZED | LOGI_REG_SHIFT | REG_COUNT, Size::Long, "LSR", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_EA, OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | LOGI_MEM_SHIFT, Size::Word, "LSR", ea_memory_alterable, decode_just_ea, is_ea, encode_just_ea),

        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | BYTE_SIZED | ARIT_REG_SHIFT | IMM_COUNT, Size::Byte, "ASL", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | WORD_SIZED | ARIT_REG_SHIFT | IMM_COUNT, Size::Word, "ASL", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | LONG_SIZED | ARIT_REG_SHIFT | IMM_COUNT, Size::Long, "ASL", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | BYTE_SIZED | ARIT_REG_SHIFT | REG_COUNT, Size::Byte, "ASL", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | WORD_SIZED | ARIT_REG_SHIFT | REG_COUNT, Size::Word, "ASL", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_LEFT | LONG_SIZED | ARIT_REG_SHIFT | REG_COUNT, Size::Long, "ASL", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_EA, OP_SHIFT | SHIFT_LEFT | WORD_SIZED | ARIT_MEM_SHIFT, Size::Word, "ASL", ea_memory_alterable, decode_just_ea, is_ea, encode_just_ea),

        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | BYTE_SIZED | ARIT_REG_SHIFT | IMM_COUNT, Size::Byte, "ASR", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ARIT_REG_SHIFT | IMM_COUNT, Size::Word, "ASR", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | LONG_SIZED | ARIT_REG_SHIFT | IMM_COUNT, Size::Long, "ASR", always, decode_quick_dy, is_quick_dn, encode_quick_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | BYTE_SIZED | ARIT_REG_SHIFT | REG_COUNT, Size::Byte, "ASR", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ARIT_REG_SHIFT | REG_COUNT, Size::Word, "ASR", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_SHIFT | SHIFT_RIGHT | LONG_SIZED | ARIT_REG_SHIFT | REG_COUNT, Size::Long, "ASR", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_EA, OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ARIT_MEM_SHIFT, Size::Word, "ASR", ea_memory_alterable, decode_just_ea, is_ea, encode_just_ea),

        instruction!(MASK_OUT_X_EA, OP_MULU, Size::Word, "MULU", ea_data, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_MULS, Size::Word, "MULS", ea_data, decode_ea_dx, is_ea_dn, encode_ea_dx), // PRM says data alterable
        instruction!(MASK_OUT_X_EA, OP_DIVU, Size::Word, "DIVU", ea_data, decode_ea_dx, is_ea_dn, encode_ea_dx),
        instruction!(MASK_OUT_X_EA, OP_DIVS, Size::Word, "DIVS", ea_data, decode_ea_dx, is_ea_dn, encode_ea_dx), // PRM says data alterable
        instruction!(MASK_OUT_EA, OP_TAS | BYTE_SIZED, Size::Byte, "TAS", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_TST | BYTE_SIZED, Size::Byte, "TST", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_TST | WORD_SIZED, Size::Word, "TST", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_OUT_EA, OP_TST | LONG_SIZED, Size::Long, "TST", ea_data_alterable, decode_just_ea, is_ea, encode_just_ea),
        instruction!(MASK_LONIB, OP_TRAP, Size::Unsized, "TRAP", always, decode_just_imm4, is_imm4, encode_just_imm4),
        instruction!(MASK_EXACT, OP_TRAPV, Size::Unsized, "TRAPV", always, decode_none, is_none, encode_none),

        instruction!(MASK_OUT_X_Y, OP_EXG | EXG_DATA_DATA, Size::Long, "EXG", always, decode_dx_dy, is_dn_dn, encode_dx_dy),
        instruction!(MASK_OUT_X_Y, OP_EXG | EXG_ADDR_ADDR, Size::Long, "EXG", always, decode_ax_ay, is_an_an, encode_ax_ay),
        instruction!(MASK_OUT_X_Y, OP_EXG | EXG_DATA_ADDR, Size::Long, "EXG", always, decode_dx_ay, is_dn_an, encode_dx_ay),
        instruction!(MASK_OUT_Y, OP_EXT | BYTE_TO_WORD, Size::Word, "EXT", always, decode_just_dy, is_dn, encode_just_dy),
        instruction!(MASK_OUT_Y, OP_EXT | WORD_TO_LONG, Size::Long, "EXT", always, decode_just_dy, is_dn, encode_just_dy),

        instruction!(MASK_EXACT, OP_STOP, Size::Unsized, "STOP", always, decode_just_imm16, is_imm16, encode_just_imm16),
        instruction!(MASK_EXACT, OP_RESET, Size::Unsized, "RESET", always, decode_none, is_none, encode_none),
        instruction!(MASK_EXACT, OP_ILLEGAL, Size::Unsized, "ILLEGAL", always, decode_none, is_none, encode_none),
        instruction!(MASK_EXACT, OP_NOP, Size::Unsized, "NOP", always, decode_none, is_none, encode_none),
    ]
}

#[cfg(test)]
mod tests {
    use memory::{MemoryVec, Memory};
    use assembler::Assembler;
    use disassembler::{Disassembler, disassemble, disassemble_first};
    use super::Exception;
    use PC;

    #[test]
    fn roundtrips_from_opcode() {
        let opcode = 0xd511;
        let mem = &mut MemoryVec::new16(PC(0), vec![opcode]);
        let asm = {
            let (pc, inst) = disassemble_first(mem);
            format!(" {}", inst)
        };
        let pc = PC(0);
        let a = Assembler::new();
        let inst = a.parse_assembler(asm.as_str());
        let new_pc = a.encode_instruction(asm.as_str(), &inst, pc, mem);
        assert_eq!(PC(2), new_pc);
        assert_eq!(opcode, mem.read_word(pc));
    }
    #[test]
    fn roundtrips_from_asm() {
        let mem = &mut MemoryVec::new();
        let pc = PC(0);
        let asm = " ADD.B\tD2,(A1)";
        let a = Assembler::new();
        let inst = a.parse_assembler(asm);
        a.encode_instruction(asm, &inst, pc, mem);
        let (pc, inst) = disassemble_first(mem);

        assert_eq!(asm, format!(" {}", inst));
    }

    #[test]
    fn synonyms_bcs_blo_byte() {
        synonymous("BCS.B", "BLO.B", "$10")
    }
    #[test]
    fn synonyms_bcc_bhs_byte() {
        synonymous("BCC.B", "BHS.B", "$10")
    }
    #[test]
    fn synonyms_bcs_blo_word() {
        synonymous("BCS.W", "BLO.W", "$10")
    }
    #[test]
    fn synonyms_bcc_bhs_word() {
        synonymous("BCC.W", "BHS.W", "$10")
    }
    #[test]
    fn synonyms_scs_slo_byte() {
        synonymous("SCS.B", "SLO.B", "$10")
    }
    #[test]
    fn synonyms_scc_shs_byte() {
        synonymous("SCC.B", "SHS.B", "$10")
    }
    #[test]
    fn synonyms_dbcs_dblo_word() {
        synonymous("DBCS.W", "DBLO.W", "D0,$10")
    }
    #[test]
    fn synonyms_dbcc_dbhs_word() {
        synonymous("DBCC.W", "DBHS.W", "D0,$10")
    }
    #[test]
    fn synonyms_dbf_dbra_word() {
        synonymous("DBF.W", "DBRA.W", "D0,$10")
    }
    fn synonymous(one: &str, two: &str, ops: &str) {
        let one = assemble_one(&format!(" {}\t{}", one, ops));
        let two = assemble_one(&format!(" {}\t{}", two, ops));

        assert_eq!(one.data(), two.data());
    }
    fn assemble_one(asm: &str) -> MemoryVec {
        let mut mem = MemoryVec::new();
        let pc = PC(0);
        let a = Assembler::new();
        let inst = a.parse_assembler(asm);
        let inst = a.adjust_size(&inst);
        a.encode_instruction(asm, &inst, pc, &mut mem);
        mem
    }

    #[test]
    // #[ignore]
    fn roundtrips() {
        let a = Assembler::new();
        let d = Disassembler::new();
        let mut valid = 0;
        for opcode in 0x0000..0xffff {
            let mut pc = PC(0x1000);
            let extension_word_mask = 0b1111_1000_1111_1111;
            // bits 8-10 should always be zero in the ea extension word
            // as we don't know which word will be seen as the ea extension word
            // (as opposed to immediate operand values) just make sure these aren't set.
            let dasm_mem = &mut MemoryVec::new16(pc, vec![opcode, 0x001f, 0x00a4, 0x1234 & extension_word_mask, 0x5678 & extension_word_mask]);
            // println!("PREDASM {:04x}", opcode);
            match d.disassemble(pc, dasm_mem) {
                Err(Exception::IllegalInstruction(_opcode, _)) => (), //println!("{:04x}:\t\tinvalid", opcode),
                Ok((new_pc, dis_inst)) => {
                    valid += 1;
                    let asm_text = format!("\t{}", dis_inst);
                    // println!("PREPARSE {:04x} disassembled as{}\n\t{:?}", opcode, asm_text, dis_inst);
                    let unsized_inst = a.parse_assembler(asm_text.as_str());
                    // println!("PREADJ {:04x} disassembled as{}\n\t{:?}, parsed as\n\t{:?}", opcode, asm_text, dis_inst, unsized_inst);
                    let sized_inst = a.adjust_size(&unsized_inst);
                    let mut asm_mem = &mut MemoryVec::new();
                    // println!("PREENC {:04x} disassembled as{}\n\t{:?}, parsed as\n\t{:?}, sized to\n\t{:?}", opcode, asm_text, dis_inst, unsized_inst, sized_inst);
                    let asm_pc = a.encode_instruction(asm_text.as_str(), &sized_inst, pc, asm_mem);
                    let new_opcode = asm_mem.read_word(pc);
                    if opcode != new_opcode {
                        panic!("{:04x}: disassembled as{}\n\t{:?}, parsed as\n\t{:?}, sized to\n\t{:?}, assembled to {:04x}", opcode, asm_text, dis_inst, unsized_inst, sized_inst, new_opcode);
                    } else {
                        // println!("{:04x}: disassembled as{}\n\t{:?}, parsed as\n\t{:?}, sized to\n\t{:?}, assembled to {:04x}", opcode, asm_text, dis_inst, unsized_inst, sized_inst, new_opcode);
                        // println!("{:04x}: disassembled as {}", opcode, asm_text);
                    }
                    if new_pc != asm_pc {
                        println!("{:04x}: disassembled as{}\n\t{:?}, parsed as\n\t{:?}, sized to\n\t{:?}, assembled to {:04x}", opcode, asm_text, dis_inst, unsized_inst, sized_inst, new_opcode);
                        println!("disassembled pc {} differ from assembled pc {}", new_pc.0, asm_pc.0);
                    };
                    while pc.0 < new_pc.0 {
                        let read_word = dasm_mem.read_word(pc);
                        let wrote_word = asm_mem.read_word(pc);
                        assert!(read_word == wrote_word, format!("mismatching extension word @{:02x}: read {:04x}, wrote {:04x}", pc.0, read_word, wrote_word));
                        pc = pc + 2;
                    }
                }
            }
        };
        println!("{} opcodes roundtripped ({:.2}% done)", valid, valid as f32 / (540.07f32 - 81.92f32));
    }
}