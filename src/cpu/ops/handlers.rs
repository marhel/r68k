use std::collections::HashMap;
use super::super::Handler;
#[allow(dead_code)]
struct OpcodeHandler {
    mask: u32,
    matching: u32,
    name: &'static str,
    handler: Handler
}

use super::super::InstructionSet;
use super::*;
macro_rules! op_entry {
    ($mask:expr, $matching:expr, $handler:ident) => (OpcodeHandler { mask: $mask, matching: $matching, handler: $handler, name: stringify!($handler) })
}

pub const MASK_OUT_X_Y: u32 = 0b1111000111111000; // masks out X and Y register bits (????xxx??????yyy)
pub const MASK_OUT_X  : u32 = 0b1111000111111111; // masks out X register bits (????xxx?????????)
pub const MASK_OUT_Y  : u32 = 0b1111111111111000; // masks out Y register bits (?????????????yyy)
pub const MASK_EXACT  : u32 = 0b1111111111111111; // masks out no register bits, exact match
pub const MASK_LOBYTE : u32 = 0b1111111100000000; // masks out low byte
pub const MASK_LOBYTX : u32 = 0b1111000100000000; // masks out low byte and X register bits
pub const MASK_LO3NIB : u32 = 0b1111000000000000; // masks out lower three nibbles
pub const MASK_LONIB  : u32 = 0b1111111111110000; // masks out low nibble

const OP_ABCD  : u32 = 0b1100_0001_0000_0000;
const OP_ADD   : u32 = 0b1101_0000_0000_0000;
const OP_ADDX  : u32 = 0b1101_0001_0000_0000;
const OP_ADDI  : u32 = 0b0000_0110_0000_0000;
const OP_ADDQ  : u32 = 0b0101_0000_0000_0000;
const OP_AND   : u32 = 0b1100_0000_0000_0000;
const OP_ANDI  : u32 = 0b0000_0010_0000_0000;
const OP_SHIFT : u32 = 0b1110_0000_0000_0000;
const OP_BRANCH: u32 = 0b0110_0000_0000_0000;
const OP_BITOPS: u32 = 0b0000_0000_0000_0000;
const OP_CHK   : u32 = 0b0100_0000_0000_0000;
const OP_CLR   : u32 = 0b0100_0010_0000_0000;
const OP_CMP   : u32 = 0b1011_0000_0000_0000;
const OP_CMPI  : u32 = 0b0000_1100_0000_0000;
const OP_CMPM  : u32 = 0b1011_0001_0000_0000;
const OP_DBCC  : u32 = 0b0101_0000_1100_1000;
const OP_DIVS  : u32 = 0b1000_0001_1100_0000;
const OP_DIVU  : u32 = 0b1000_0000_1100_0000;
const OP_EOR   : u32 = 0b1011_0000_0000_0000;
const OP_EORI  : u32 = 0b0000_1010_0000_0000;
const OP_EXG   : u32 = 0b1100_0001_0000_0000;
const OP_EXT   : u32 = 0b0100_1000_0000_0000;
const OP_JMP   : u32 = 0b0100_1110_1100_0000;
const OP_JSR   : u32 = 0b0100_1110_1000_0000;
const OP_LEA   : u32 = 0b0100_0001_1100_0000;
const OP_MOVE  : u32 = 0b0000_0000_0000_0000;
const OP_MOVE2 : u32 = 0b0100_0000_0000_0000;
const OP_MOVEM : u32 = 0b0100_1000_1000_0000;
const OP_MOVEP : u32 = 0b0000_0000_0000_1000;
const OP_MULS  : u32 = 0b1100_0001_1100_0000;
const OP_MULU  : u32 = 0b1100_0000_1100_0000;
const OP_NBCD  : u32 = 0b0100_1000_0000_0000;
const OP_NEG   : u32 = 0b0100_0100_0000_0000;
const OP_NEGX  : u32 = 0b0100_0000_0000_0000;
const OP_NOT   : u32 = 0b0100_0110_0000_0000;
const OP_OR    : u32 = 0b1000_0000_0000_0000;
const OP_ORI   : u32 = 0b0000_0000_0000_0000;
const OP_PEA   : u32 = 0b0100_1000_0100_0000;
const OP_SUB   : u32 = 0b1001_0000_0000_0000;
const OP_SUBI  : u32 = 0b0000_0100_0000_0000;
const OP_SUBQ  : u32 = 0b0101_0001_0000_0000;
const OP_SUBX  : u32 = 0b1001_0001_0000_0000;
const OP_SBCD  : u32 = 0b1000_0001_0000_0000;
const OP_SWAP  : u32 = 0b0100_1000_0000_0000;
const OP_SCC   : u32 = 0b0101_0000_1100_0000;
const OP_TAS   : u32 = 0b0100_1010_1100_0000;

const IF_T : u32 = 0b0000_0000_0000; // True            1
const IF_F : u32 = 0b0001_0000_0000; // False           0
const IF_HI: u32 = 0b0010_0000_0000; // High            !C & !Z
const IF_LS: u32 = 0b0011_0000_0000; // LowOrSame       C | Z
const IF_CC: u32 = 0b0100_0000_0000; // CarryClearHI    !C
const IF_CS: u32 = 0b0101_0000_0000; // CarrySetLO      C
const IF_NE: u32 = 0b0110_0000_0000; // NotEqual        !Z
const IF_EQ: u32 = 0b0111_0000_0000; // Equal           Z
const IF_VC: u32 = 0b1000_0000_0000; // OverflowClear   !V
const IF_VS: u32 = 0b1001_0000_0000; // OverflowSet     V
const IF_PL: u32 = 0b1010_0000_0000; // Plus            !N
const IF_MI: u32 = 0b1011_0000_0000; // Minus           N
const IF_GE: u32 = 0b1100_0000_0000; // GreaterOrEqual  N & V | !N & !V
const IF_LT: u32 = 0b1101_0000_0000; // LessThan        N & !V | !N & V
const IF_GT: u32 = 0b1110_0000_0000; // GreaterThan     N & V & !Z | !N & !V & !Z
const IF_LE: u32 = 0b1111_0000_0000; // LessOrEqual     Z | N & !V | !N & V

const OPER_DN  : u32 = 0x00;
const OPER_AN  : u32 = 0x08;
const OPER_AI  : u32 = 0x10;
const OPER_PI  : u32 = 0x18;
const OPER_PD  : u32 = 0x20;
const OPER_DI  : u32 = 0x28;
const OPER_IX  : u32 = 0x30;
const OPER_AW  : u32 = 0x38;
const OPER_AL  : u32 = 0x39;
const OPER_PCDI: u32 = 0x3a;
const OPER_PCIX: u32 = 0x3b;
const OPER_IMM : u32 = 0x3c;

const BYTE_SIZED: u32 = 0x00;
const WORD_SIZED: u32 = 0x40;
const LONG_SIZED: u32 = 0x80;

const DEST_DX: u32 = 0x000;
const DEST_EA: u32 = 0x100;
const DEST_CCR: u32 = 0x3c;
const DEST_SR : u32 = 0x7c;

const RR_MODE: u32 = 0x00;  // Register to Register
const MM_MODE: u32 = 0x08;  // Memory to Memory

const SHIFT_RIGHT: u32 = 0x000;
const SHIFT_LEFT : u32 = 0x100;
const IMM_COUNT  : u32 = 0x00;
const REG_COUNT  : u32 = 0x20;
// Seems that ASL, LSL, ROXL and ROL differs in just two bits;
// However, these bits are placed differently in MEM_SHIFTs and REG_SHIFTs
// so we need 2*4 constants, not 2+4
const ARIT_REG_SHIFT  : u32 = 0b00000;
const LOGI_REG_SHIFT  : u32 = 0b01000;
const ROTX_REG_SHIFT  : u32 = 0b10000;
const ROTA_REG_SHIFT  : u32 = 0b11000;
const ARIT_MEM_SHIFT  : u32 = 0xC0 | (ARIT_REG_SHIFT << 6);
const LOGI_MEM_SHIFT  : u32 = 0xC0 | (LOGI_REG_SHIFT << 6);
const ROTX_MEM_SHIFT  : u32 = 0xC0 | (ROTX_REG_SHIFT << 6);
const ROTA_MEM_SHIFT  : u32 = 0xC0 | (ROTA_REG_SHIFT << 6);

// ADDA does not follow the ADD pattern for 'oper' so we cannot use the
// above constants
const DEST_AX_WORD: u32 = 0x0C0;
const DEST_AX_LONG: u32 = 0x1C0;

pub const OP_UNIMPLEMENTED_1010 : u32 = 0b1010_0000_0000_0000;
pub const OP_UNIMPLEMENTED_1111 : u32 = 0b1111_0000_0000_0000;

// -- OP-constants -------------------------------
pub const OP_ABCD_8_RR: u32 = OP_ABCD | BYTE_SIZED | RR_MODE;
pub const OP_ABCD_8_MM: u32 = OP_ABCD | BYTE_SIZED | MM_MODE;

pub const OP_ADD_8_ER_DN   : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_DN;
pub const OP_ADD_8_ER_AI   : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_AI;
pub const OP_ADD_8_ER_PI   : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_PI;
pub const OP_ADD_8_ER_PD   : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_PD;
pub const OP_ADD_8_ER_DI   : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_DI;
pub const OP_ADD_8_ER_IX   : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_IX;
pub const OP_ADD_8_ER_AW   : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_AW;
pub const OP_ADD_8_ER_AL   : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_AL;
pub const OP_ADD_8_ER_PCDI : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_PCDI;
pub const OP_ADD_8_ER_PCIX : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_PCIX;
pub const OP_ADD_8_ER_IMM  : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_IMM;

pub const OP_ADD_8_RE_AI   : u32 = OP_ADD | BYTE_SIZED | DEST_EA | OPER_AI;
pub const OP_ADD_8_RE_PI   : u32 = OP_ADD | BYTE_SIZED | DEST_EA | OPER_PI;
pub const OP_ADD_8_RE_PD   : u32 = OP_ADD | BYTE_SIZED | DEST_EA | OPER_PD;
pub const OP_ADD_8_RE_DI   : u32 = OP_ADD | BYTE_SIZED | DEST_EA | OPER_DI;
pub const OP_ADD_8_RE_IX   : u32 = OP_ADD | BYTE_SIZED | DEST_EA | OPER_IX;
pub const OP_ADD_8_RE_AW   : u32 = OP_ADD | BYTE_SIZED | DEST_EA | OPER_AW;
pub const OP_ADD_8_RE_AL   : u32 = OP_ADD | BYTE_SIZED | DEST_EA | OPER_AL;

pub const OP_ADD_16_ER_DN  : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_DN;
pub const OP_ADD_16_ER_AN  : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_AN;
pub const OP_ADD_16_ER_AI  : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_AI;
pub const OP_ADD_16_ER_PI  : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_PI;
pub const OP_ADD_16_ER_PD  : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_PD;
pub const OP_ADD_16_ER_DI  : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_DI;
pub const OP_ADD_16_ER_IX  : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_IX;
pub const OP_ADD_16_ER_AW  : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_AW;
pub const OP_ADD_16_ER_AL  : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_AL;
pub const OP_ADD_16_ER_PCDI: u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_PCDI;
pub const OP_ADD_16_ER_PCIX: u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_PCIX;
pub const OP_ADD_16_ER_IMM : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_IMM;

pub const OP_ADD_16_RE_AI  : u32 = OP_ADD | WORD_SIZED | DEST_EA | OPER_AI;
pub const OP_ADD_16_RE_PI  : u32 = OP_ADD | WORD_SIZED | DEST_EA | OPER_PI;
pub const OP_ADD_16_RE_PD  : u32 = OP_ADD | WORD_SIZED | DEST_EA | OPER_PD;
pub const OP_ADD_16_RE_DI  : u32 = OP_ADD | WORD_SIZED | DEST_EA | OPER_DI;
pub const OP_ADD_16_RE_IX  : u32 = OP_ADD | WORD_SIZED | DEST_EA | OPER_IX;
pub const OP_ADD_16_RE_AW  : u32 = OP_ADD | WORD_SIZED | DEST_EA | OPER_AW;
pub const OP_ADD_16_RE_AL  : u32 = OP_ADD | WORD_SIZED | DEST_EA | OPER_AL;

pub const OP_ADD_32_ER_DN  : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_DN;
pub const OP_ADD_32_ER_AN  : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_AN;
pub const OP_ADD_32_ER_AI  : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_AI;
pub const OP_ADD_32_ER_PI  : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_PI;
pub const OP_ADD_32_ER_PD  : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_PD;
pub const OP_ADD_32_ER_DI  : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_DI;
pub const OP_ADD_32_ER_IX  : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_IX;
pub const OP_ADD_32_ER_AW  : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_AW;
pub const OP_ADD_32_ER_AL  : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_AL;
pub const OP_ADD_32_ER_PCDI: u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_PCDI;
pub const OP_ADD_32_ER_PCIX: u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_PCIX;
pub const OP_ADD_32_ER_IMM : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_IMM;

pub const OP_ADD_32_RE_AI  : u32 = OP_ADD | LONG_SIZED | DEST_EA | OPER_AI;
pub const OP_ADD_32_RE_PI  : u32 = OP_ADD | LONG_SIZED | DEST_EA | OPER_PI;
pub const OP_ADD_32_RE_PD  : u32 = OP_ADD | LONG_SIZED | DEST_EA | OPER_PD;
pub const OP_ADD_32_RE_DI  : u32 = OP_ADD | LONG_SIZED | DEST_EA | OPER_DI;
pub const OP_ADD_32_RE_IX  : u32 = OP_ADD | LONG_SIZED | DEST_EA | OPER_IX;
pub const OP_ADD_32_RE_AW  : u32 = OP_ADD | LONG_SIZED | DEST_EA | OPER_AW;
pub const OP_ADD_32_RE_AL  : u32 = OP_ADD | LONG_SIZED | DEST_EA | OPER_AL;

pub const OP_ADDA_16_DN    : u32 = OP_ADD | DEST_AX_WORD | OPER_DN;
pub const OP_ADDA_16_AN    : u32 = OP_ADD | DEST_AX_WORD | OPER_AN;
pub const OP_ADDA_16_AI    : u32 = OP_ADD | DEST_AX_WORD | OPER_AI;
pub const OP_ADDA_16_PI    : u32 = OP_ADD | DEST_AX_WORD | OPER_PI;
pub const OP_ADDA_16_PD    : u32 = OP_ADD | DEST_AX_WORD | OPER_PD;
pub const OP_ADDA_16_DI    : u32 = OP_ADD | DEST_AX_WORD | OPER_DI;
pub const OP_ADDA_16_IX    : u32 = OP_ADD | DEST_AX_WORD | OPER_IX;
pub const OP_ADDA_16_AW    : u32 = OP_ADD | DEST_AX_WORD | OPER_AW;
pub const OP_ADDA_16_AL    : u32 = OP_ADD | DEST_AX_WORD | OPER_AL;
pub const OP_ADDA_16_PCDI  : u32 = OP_ADD | DEST_AX_WORD | OPER_PCDI;
pub const OP_ADDA_16_PCIX  : u32 = OP_ADD | DEST_AX_WORD | OPER_PCIX;
pub const OP_ADDA_16_IMM   : u32 = OP_ADD | DEST_AX_WORD | OPER_IMM;

pub const OP_ADDA_32_DN    : u32 = OP_ADD | DEST_AX_LONG | OPER_DN;
pub const OP_ADDA_32_AN    : u32 = OP_ADD | DEST_AX_LONG | OPER_AN;
pub const OP_ADDA_32_AI    : u32 = OP_ADD | DEST_AX_LONG | OPER_AI;
pub const OP_ADDA_32_PI    : u32 = OP_ADD | DEST_AX_LONG | OPER_PI;
pub const OP_ADDA_32_PD    : u32 = OP_ADD | DEST_AX_LONG | OPER_PD;
pub const OP_ADDA_32_DI    : u32 = OP_ADD | DEST_AX_LONG | OPER_DI;
pub const OP_ADDA_32_IX    : u32 = OP_ADD | DEST_AX_LONG | OPER_IX;
pub const OP_ADDA_32_AW    : u32 = OP_ADD | DEST_AX_LONG | OPER_AW;
pub const OP_ADDA_32_AL    : u32 = OP_ADD | DEST_AX_LONG | OPER_AL;
pub const OP_ADDA_32_PCDI  : u32 = OP_ADD | DEST_AX_LONG | OPER_PCDI;
pub const OP_ADDA_32_PCIX  : u32 = OP_ADD | DEST_AX_LONG | OPER_PCIX;
pub const OP_ADDA_32_IMM   : u32 = OP_ADD | DEST_AX_LONG | OPER_IMM;

pub const OP_ADDI_8_DN     : u32 = OP_ADDI | BYTE_SIZED | OPER_DN;
pub const OP_ADDI_8_AI     : u32 = OP_ADDI | BYTE_SIZED | OPER_AI;
pub const OP_ADDI_8_PI     : u32 = OP_ADDI | BYTE_SIZED | OPER_PI;
pub const OP_ADDI_8_PD     : u32 = OP_ADDI | BYTE_SIZED | OPER_PD;
pub const OP_ADDI_8_DI     : u32 = OP_ADDI | BYTE_SIZED | OPER_DI;
pub const OP_ADDI_8_IX     : u32 = OP_ADDI | BYTE_SIZED | OPER_IX;
pub const OP_ADDI_8_AW     : u32 = OP_ADDI | BYTE_SIZED | OPER_AW;
pub const OP_ADDI_8_AL     : u32 = OP_ADDI | BYTE_SIZED | OPER_AL;

pub const OP_ADDI_16_DN    : u32 = OP_ADDI | WORD_SIZED | OPER_DN;
pub const OP_ADDI_16_AI    : u32 = OP_ADDI | WORD_SIZED | OPER_AI;
pub const OP_ADDI_16_PI    : u32 = OP_ADDI | WORD_SIZED | OPER_PI;
pub const OP_ADDI_16_PD    : u32 = OP_ADDI | WORD_SIZED | OPER_PD;
pub const OP_ADDI_16_DI    : u32 = OP_ADDI | WORD_SIZED | OPER_DI;
pub const OP_ADDI_16_IX    : u32 = OP_ADDI | WORD_SIZED | OPER_IX;
pub const OP_ADDI_16_AW    : u32 = OP_ADDI | WORD_SIZED | OPER_AW;
pub const OP_ADDI_16_AL    : u32 = OP_ADDI | WORD_SIZED | OPER_AL;

pub const OP_ADDI_32_DN    : u32 = OP_ADDI | LONG_SIZED | OPER_DN;
pub const OP_ADDI_32_AI    : u32 = OP_ADDI | LONG_SIZED | OPER_AI;
pub const OP_ADDI_32_PI    : u32 = OP_ADDI | LONG_SIZED | OPER_PI;
pub const OP_ADDI_32_PD    : u32 = OP_ADDI | LONG_SIZED | OPER_PD;
pub const OP_ADDI_32_DI    : u32 = OP_ADDI | LONG_SIZED | OPER_DI;
pub const OP_ADDI_32_IX    : u32 = OP_ADDI | LONG_SIZED | OPER_IX;
pub const OP_ADDI_32_AW    : u32 = OP_ADDI | LONG_SIZED | OPER_AW;
pub const OP_ADDI_32_AL    : u32 = OP_ADDI | LONG_SIZED | OPER_AL;

pub const OP_ADDQ_8_DN     : u32 = OP_ADDQ | BYTE_SIZED | OPER_DN;
pub const OP_ADDQ_8_AI     : u32 = OP_ADDQ | BYTE_SIZED | OPER_AI;
pub const OP_ADDQ_8_PI     : u32 = OP_ADDQ | BYTE_SIZED | OPER_PI;
pub const OP_ADDQ_8_PD     : u32 = OP_ADDQ | BYTE_SIZED | OPER_PD;
pub const OP_ADDQ_8_DI     : u32 = OP_ADDQ | BYTE_SIZED | OPER_DI;
pub const OP_ADDQ_8_IX     : u32 = OP_ADDQ | BYTE_SIZED | OPER_IX;
pub const OP_ADDQ_8_AW     : u32 = OP_ADDQ | BYTE_SIZED | OPER_AW;
pub const OP_ADDQ_8_AL     : u32 = OP_ADDQ | BYTE_SIZED | OPER_AL;

pub const OP_ADDQ_16_DN    : u32 = OP_ADDQ | WORD_SIZED | OPER_DN;
pub const OP_ADDQ_16_AN    : u32 = OP_ADDQ | WORD_SIZED | OPER_AN;
pub const OP_ADDQ_16_AI    : u32 = OP_ADDQ | WORD_SIZED | OPER_AI;
pub const OP_ADDQ_16_PI    : u32 = OP_ADDQ | WORD_SIZED | OPER_PI;
pub const OP_ADDQ_16_PD    : u32 = OP_ADDQ | WORD_SIZED | OPER_PD;
pub const OP_ADDQ_16_DI    : u32 = OP_ADDQ | WORD_SIZED | OPER_DI;
pub const OP_ADDQ_16_IX    : u32 = OP_ADDQ | WORD_SIZED | OPER_IX;
pub const OP_ADDQ_16_AW    : u32 = OP_ADDQ | WORD_SIZED | OPER_AW;
pub const OP_ADDQ_16_AL    : u32 = OP_ADDQ | WORD_SIZED | OPER_AL;

pub const OP_ADDQ_32_DN    : u32 = OP_ADDQ | LONG_SIZED | OPER_DN;
pub const OP_ADDQ_32_AN    : u32 = OP_ADDQ | LONG_SIZED | OPER_AN;
pub const OP_ADDQ_32_AI    : u32 = OP_ADDQ | LONG_SIZED | OPER_AI;
pub const OP_ADDQ_32_PI    : u32 = OP_ADDQ | LONG_SIZED | OPER_PI;
pub const OP_ADDQ_32_PD    : u32 = OP_ADDQ | LONG_SIZED | OPER_PD;
pub const OP_ADDQ_32_DI    : u32 = OP_ADDQ | LONG_SIZED | OPER_DI;
pub const OP_ADDQ_32_IX    : u32 = OP_ADDQ | LONG_SIZED | OPER_IX;
pub const OP_ADDQ_32_AW    : u32 = OP_ADDQ | LONG_SIZED | OPER_AW;
pub const OP_ADDQ_32_AL    : u32 = OP_ADDQ | LONG_SIZED | OPER_AL;

pub const OP_ADDX_8_RR     : u32 = OP_ADDX | BYTE_SIZED | RR_MODE;
pub const OP_ADDX_8_MM     : u32 = OP_ADDX | BYTE_SIZED | MM_MODE;
pub const OP_ADDX_16_RR    : u32 = OP_ADDX | WORD_SIZED | RR_MODE;
pub const OP_ADDX_16_MM    : u32 = OP_ADDX | WORD_SIZED | MM_MODE;
pub const OP_ADDX_32_RR    : u32 = OP_ADDX | LONG_SIZED | RR_MODE;
pub const OP_ADDX_32_MM    : u32 = OP_ADDX | LONG_SIZED | MM_MODE;

pub const OP_AND_8_ER_DN   : u32 = OP_AND | BYTE_SIZED | DEST_DX | OPER_DN;
pub const OP_AND_8_ER_AI   : u32 = OP_AND | BYTE_SIZED | DEST_DX | OPER_AI;
pub const OP_AND_8_ER_PI   : u32 = OP_AND | BYTE_SIZED | DEST_DX | OPER_PI;
pub const OP_AND_8_ER_PD   : u32 = OP_AND | BYTE_SIZED | DEST_DX | OPER_PD;
pub const OP_AND_8_ER_DI   : u32 = OP_AND | BYTE_SIZED | DEST_DX | OPER_DI;
pub const OP_AND_8_ER_IX   : u32 = OP_AND | BYTE_SIZED | DEST_DX | OPER_IX;
pub const OP_AND_8_ER_AW   : u32 = OP_AND | BYTE_SIZED | DEST_DX | OPER_AW;
pub const OP_AND_8_ER_AL   : u32 = OP_AND | BYTE_SIZED | DEST_DX | OPER_AL;
pub const OP_AND_8_ER_PCDI : u32 = OP_AND | BYTE_SIZED | DEST_DX | OPER_PCDI;
pub const OP_AND_8_ER_PCIX : u32 = OP_AND | BYTE_SIZED | DEST_DX | OPER_PCIX;
pub const OP_AND_8_ER_IMM  : u32 = OP_AND | BYTE_SIZED | DEST_DX | OPER_IMM;

pub const OP_AND_8_RE_AI   : u32 = OP_AND | BYTE_SIZED | DEST_EA | OPER_AI;
pub const OP_AND_8_RE_PI   : u32 = OP_AND | BYTE_SIZED | DEST_EA | OPER_PI;
pub const OP_AND_8_RE_PD   : u32 = OP_AND | BYTE_SIZED | DEST_EA | OPER_PD;
pub const OP_AND_8_RE_DI   : u32 = OP_AND | BYTE_SIZED | DEST_EA | OPER_DI;
pub const OP_AND_8_RE_IX   : u32 = OP_AND | BYTE_SIZED | DEST_EA | OPER_IX;
pub const OP_AND_8_RE_AW   : u32 = OP_AND | BYTE_SIZED | DEST_EA | OPER_AW;
pub const OP_AND_8_RE_AL   : u32 = OP_AND | BYTE_SIZED | DEST_EA | OPER_AL;

pub const OP_AND_16_ER_DN  : u32 = OP_AND | WORD_SIZED | DEST_DX | OPER_DN;
pub const OP_AND_16_ER_AI  : u32 = OP_AND | WORD_SIZED | DEST_DX | OPER_AI;
pub const OP_AND_16_ER_PI  : u32 = OP_AND | WORD_SIZED | DEST_DX | OPER_PI;
pub const OP_AND_16_ER_PD  : u32 = OP_AND | WORD_SIZED | DEST_DX | OPER_PD;
pub const OP_AND_16_ER_DI  : u32 = OP_AND | WORD_SIZED | DEST_DX | OPER_DI;
pub const OP_AND_16_ER_IX  : u32 = OP_AND | WORD_SIZED | DEST_DX | OPER_IX;
pub const OP_AND_16_ER_AW  : u32 = OP_AND | WORD_SIZED | DEST_DX | OPER_AW;
pub const OP_AND_16_ER_AL  : u32 = OP_AND | WORD_SIZED | DEST_DX | OPER_AL;
pub const OP_AND_16_ER_PCDI: u32 = OP_AND | WORD_SIZED | DEST_DX | OPER_PCDI;
pub const OP_AND_16_ER_PCIX: u32 = OP_AND | WORD_SIZED | DEST_DX | OPER_PCIX;
pub const OP_AND_16_ER_IMM : u32 = OP_AND | WORD_SIZED | DEST_DX | OPER_IMM;

pub const OP_AND_16_RE_AI  : u32 = OP_AND | WORD_SIZED | DEST_EA | OPER_AI;
pub const OP_AND_16_RE_PI  : u32 = OP_AND | WORD_SIZED | DEST_EA | OPER_PI;
pub const OP_AND_16_RE_PD  : u32 = OP_AND | WORD_SIZED | DEST_EA | OPER_PD;
pub const OP_AND_16_RE_DI  : u32 = OP_AND | WORD_SIZED | DEST_EA | OPER_DI;
pub const OP_AND_16_RE_IX  : u32 = OP_AND | WORD_SIZED | DEST_EA | OPER_IX;
pub const OP_AND_16_RE_AW  : u32 = OP_AND | WORD_SIZED | DEST_EA | OPER_AW;
pub const OP_AND_16_RE_AL  : u32 = OP_AND | WORD_SIZED | DEST_EA | OPER_AL;

pub const OP_AND_32_ER_DN  : u32 = OP_AND | LONG_SIZED | DEST_DX | OPER_DN;
pub const OP_AND_32_ER_AI  : u32 = OP_AND | LONG_SIZED | DEST_DX | OPER_AI;
pub const OP_AND_32_ER_PI  : u32 = OP_AND | LONG_SIZED | DEST_DX | OPER_PI;
pub const OP_AND_32_ER_PD  : u32 = OP_AND | LONG_SIZED | DEST_DX | OPER_PD;
pub const OP_AND_32_ER_DI  : u32 = OP_AND | LONG_SIZED | DEST_DX | OPER_DI;
pub const OP_AND_32_ER_IX  : u32 = OP_AND | LONG_SIZED | DEST_DX | OPER_IX;
pub const OP_AND_32_ER_AW  : u32 = OP_AND | LONG_SIZED | DEST_DX | OPER_AW;
pub const OP_AND_32_ER_AL  : u32 = OP_AND | LONG_SIZED | DEST_DX | OPER_AL;
pub const OP_AND_32_ER_PCDI: u32 = OP_AND | LONG_SIZED | DEST_DX | OPER_PCDI;
pub const OP_AND_32_ER_PCIX: u32 = OP_AND | LONG_SIZED | DEST_DX | OPER_PCIX;
pub const OP_AND_32_ER_IMM : u32 = OP_AND | LONG_SIZED | DEST_DX | OPER_IMM;

pub const OP_AND_32_RE_AI  : u32 = OP_AND | LONG_SIZED | DEST_EA | OPER_AI;
pub const OP_AND_32_RE_PI  : u32 = OP_AND | LONG_SIZED | DEST_EA | OPER_PI;
pub const OP_AND_32_RE_PD  : u32 = OP_AND | LONG_SIZED | DEST_EA | OPER_PD;
pub const OP_AND_32_RE_DI  : u32 = OP_AND | LONG_SIZED | DEST_EA | OPER_DI;
pub const OP_AND_32_RE_IX  : u32 = OP_AND | LONG_SIZED | DEST_EA | OPER_IX;
pub const OP_AND_32_RE_AW  : u32 = OP_AND | LONG_SIZED | DEST_EA | OPER_AW;
pub const OP_AND_32_RE_AL  : u32 = OP_AND | LONG_SIZED | DEST_EA | OPER_AL;

pub const OP_ANDI_8_DN     : u32 = OP_ANDI | BYTE_SIZED | OPER_DN;
pub const OP_ANDI_8_AI     : u32 = OP_ANDI | BYTE_SIZED | OPER_AI;
pub const OP_ANDI_8_PI     : u32 = OP_ANDI | BYTE_SIZED | OPER_PI;
pub const OP_ANDI_8_PD     : u32 = OP_ANDI | BYTE_SIZED | OPER_PD;
pub const OP_ANDI_8_DI     : u32 = OP_ANDI | BYTE_SIZED | OPER_DI;
pub const OP_ANDI_8_IX     : u32 = OP_ANDI | BYTE_SIZED | OPER_IX;
pub const OP_ANDI_8_AW     : u32 = OP_ANDI | BYTE_SIZED | OPER_AW;
pub const OP_ANDI_8_AL     : u32 = OP_ANDI | BYTE_SIZED | OPER_AL;

pub const OP_ANDI_16_DN    : u32 = OP_ANDI | WORD_SIZED | OPER_DN;
pub const OP_ANDI_16_AI    : u32 = OP_ANDI | WORD_SIZED | OPER_AI;
pub const OP_ANDI_16_PI    : u32 = OP_ANDI | WORD_SIZED | OPER_PI;
pub const OP_ANDI_16_PD    : u32 = OP_ANDI | WORD_SIZED | OPER_PD;
pub const OP_ANDI_16_DI    : u32 = OP_ANDI | WORD_SIZED | OPER_DI;
pub const OP_ANDI_16_IX    : u32 = OP_ANDI | WORD_SIZED | OPER_IX;
pub const OP_ANDI_16_AW    : u32 = OP_ANDI | WORD_SIZED | OPER_AW;
pub const OP_ANDI_16_AL    : u32 = OP_ANDI | WORD_SIZED | OPER_AL;

pub const OP_ANDI_32_DN    : u32 = OP_ANDI | LONG_SIZED | OPER_DN;
pub const OP_ANDI_32_AI    : u32 = OP_ANDI | LONG_SIZED | OPER_AI;
pub const OP_ANDI_32_PI    : u32 = OP_ANDI | LONG_SIZED | OPER_PI;
pub const OP_ANDI_32_PD    : u32 = OP_ANDI | LONG_SIZED | OPER_PD;
pub const OP_ANDI_32_DI    : u32 = OP_ANDI | LONG_SIZED | OPER_DI;
pub const OP_ANDI_32_IX    : u32 = OP_ANDI | LONG_SIZED | OPER_IX;
pub const OP_ANDI_32_AW    : u32 = OP_ANDI | LONG_SIZED | OPER_AW;
pub const OP_ANDI_32_AL    : u32 = OP_ANDI | LONG_SIZED | OPER_AL;

pub const OP_ANDI_16_TOC   : u32 = OP_ANDI | DEST_CCR;
pub const OP_ANDI_16_TOS   : u32 = OP_ANDI | DEST_SR;

pub const OP_ASL_8_R        : u32 = OP_SHIFT | SHIFT_LEFT  | BYTE_SIZED | ARIT_REG_SHIFT | REG_COUNT;
pub const OP_ASL_8_S        : u32 = OP_SHIFT | SHIFT_LEFT  | BYTE_SIZED | ARIT_REG_SHIFT | IMM_COUNT;
pub const OP_ASL_16_R       : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ARIT_REG_SHIFT | REG_COUNT;
pub const OP_ASL_16_S       : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ARIT_REG_SHIFT | IMM_COUNT;
pub const OP_ASL_32_R       : u32 = OP_SHIFT | SHIFT_LEFT  | LONG_SIZED | ARIT_REG_SHIFT | REG_COUNT;
pub const OP_ASL_32_S       : u32 = OP_SHIFT | SHIFT_LEFT  | LONG_SIZED | ARIT_REG_SHIFT | IMM_COUNT;

pub const OP_ASL_16_AI      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ARIT_MEM_SHIFT | OPER_AI;
pub const OP_ASL_16_PI      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ARIT_MEM_SHIFT | OPER_PI;
pub const OP_ASL_16_PD      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ARIT_MEM_SHIFT | OPER_PD;
pub const OP_ASL_16_DI      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ARIT_MEM_SHIFT | OPER_DI;
pub const OP_ASL_16_IX      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ARIT_MEM_SHIFT | OPER_IX;
pub const OP_ASL_16_AW      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ARIT_MEM_SHIFT | OPER_AW;
pub const OP_ASL_16_AL      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ARIT_MEM_SHIFT | OPER_AL;

pub const OP_ASR_8_R        : u32 = OP_SHIFT | SHIFT_RIGHT | BYTE_SIZED | ARIT_REG_SHIFT | REG_COUNT;
pub const OP_ASR_8_S        : u32 = OP_SHIFT | SHIFT_RIGHT | BYTE_SIZED | ARIT_REG_SHIFT | IMM_COUNT;
pub const OP_ASR_16_R       : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ARIT_REG_SHIFT | REG_COUNT;
pub const OP_ASR_16_S       : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ARIT_REG_SHIFT | IMM_COUNT;
pub const OP_ASR_32_R       : u32 = OP_SHIFT | SHIFT_RIGHT | LONG_SIZED | ARIT_REG_SHIFT | REG_COUNT;
pub const OP_ASR_32_S       : u32 = OP_SHIFT | SHIFT_RIGHT | LONG_SIZED | ARIT_REG_SHIFT | IMM_COUNT;

pub const OP_ASR_16_AI      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ARIT_MEM_SHIFT | OPER_AI;
pub const OP_ASR_16_PI      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ARIT_MEM_SHIFT | OPER_PI;
pub const OP_ASR_16_PD      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ARIT_MEM_SHIFT | OPER_PD;
pub const OP_ASR_16_DI      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ARIT_MEM_SHIFT | OPER_DI;
pub const OP_ASR_16_IX      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ARIT_MEM_SHIFT | OPER_IX;
pub const OP_ASR_16_AW      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ARIT_MEM_SHIFT | OPER_AW;
pub const OP_ASR_16_AL      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ARIT_MEM_SHIFT | OPER_AL;

pub const OP_BHI_8            : u32 = OP_BRANCH | IF_HI;
pub const OP_BLS_8            : u32 = OP_BRANCH | IF_LS;
pub const OP_BCC_8            : u32 = OP_BRANCH | IF_CC;
pub const OP_BCS_8            : u32 = OP_BRANCH | IF_CS;
pub const OP_BNE_8            : u32 = OP_BRANCH | IF_NE;
pub const OP_BEQ_8            : u32 = OP_BRANCH | IF_EQ;
pub const OP_BVC_8            : u32 = OP_BRANCH | IF_VC;
pub const OP_BVS_8            : u32 = OP_BRANCH | IF_VS;
pub const OP_BPL_8            : u32 = OP_BRANCH | IF_PL;
pub const OP_BMI_8            : u32 = OP_BRANCH | IF_MI;
pub const OP_BGE_8            : u32 = OP_BRANCH | IF_GE;
pub const OP_BLT_8            : u32 = OP_BRANCH | IF_LT;
pub const OP_BGT_8            : u32 = OP_BRANCH | IF_GT;
pub const OP_BLE_8            : u32 = OP_BRANCH | IF_LE;
pub const OP_BRA_8            : u32 = OP_BRANCH | IF_T;
pub const OP_BSR_8            : u32 = OP_BRANCH | IF_F;

pub const OP_BHI_16            : u32 = OP_BRANCH | IF_HI;
pub const OP_BLS_16            : u32 = OP_BRANCH | IF_LS;
pub const OP_BCC_16            : u32 = OP_BRANCH | IF_CC;
pub const OP_BCS_16            : u32 = OP_BRANCH | IF_CS;
pub const OP_BNE_16            : u32 = OP_BRANCH | IF_NE;
pub const OP_BEQ_16            : u32 = OP_BRANCH | IF_EQ;
pub const OP_BVC_16            : u32 = OP_BRANCH | IF_VC;
pub const OP_BVS_16            : u32 = OP_BRANCH | IF_VS;
pub const OP_BPL_16            : u32 = OP_BRANCH | IF_PL;
pub const OP_BMI_16            : u32 = OP_BRANCH | IF_MI;
pub const OP_BGE_16            : u32 = OP_BRANCH | IF_GE;
pub const OP_BLT_16            : u32 = OP_BRANCH | IF_LT;
pub const OP_BGT_16            : u32 = OP_BRANCH | IF_GT;
pub const OP_BLE_16            : u32 = OP_BRANCH | IF_LE;
pub const OP_BRA_16            : u32 = OP_BRANCH | IF_T;
pub const OP_BSR_16            : u32 = OP_BRANCH | IF_F;

const SRC_REG: u32 = 0x100;
const SRC_IMM: u32 = 0x800;
const BIT_TST: u32 = 0x00;
const BIT_CHG: u32 = 0x40;
const BIT_CLR: u32 = 0x80;
const BIT_SET: u32 = 0xC0;

pub const OP_BCHG_32_R_DN   : u32 = OP_BITOPS | BIT_CHG | SRC_REG | OPER_DN;
pub const OP_BCHG_32_S_DN   : u32 = OP_BITOPS | BIT_CHG | SRC_IMM | OPER_DN;
pub const OP_BCHG_8_R_AI    : u32 = OP_BITOPS | BIT_CHG | SRC_REG | OPER_AI;
pub const OP_BCHG_8_R_PI    : u32 = OP_BITOPS | BIT_CHG | SRC_REG | OPER_PI;
pub const OP_BCHG_8_R_PD    : u32 = OP_BITOPS | BIT_CHG | SRC_REG | OPER_PD;
pub const OP_BCHG_8_R_DI    : u32 = OP_BITOPS | BIT_CHG | SRC_REG | OPER_DI;
pub const OP_BCHG_8_R_IX    : u32 = OP_BITOPS | BIT_CHG | SRC_REG | OPER_IX;
pub const OP_BCHG_8_R_AW    : u32 = OP_BITOPS | BIT_CHG | SRC_REG | OPER_AW;
pub const OP_BCHG_8_R_AL    : u32 = OP_BITOPS | BIT_CHG | SRC_REG | OPER_AL;
pub const OP_BCHG_8_S_AI    : u32 = OP_BITOPS | BIT_CHG | SRC_IMM | OPER_AI;
pub const OP_BCHG_8_S_PI    : u32 = OP_BITOPS | BIT_CHG | SRC_IMM | OPER_PI;
pub const OP_BCHG_8_S_PD    : u32 = OP_BITOPS | BIT_CHG | SRC_IMM | OPER_PD;
pub const OP_BCHG_8_S_DI    : u32 = OP_BITOPS | BIT_CHG | SRC_IMM | OPER_DI;
pub const OP_BCHG_8_S_IX    : u32 = OP_BITOPS | BIT_CHG | SRC_IMM | OPER_IX;
pub const OP_BCHG_8_S_AW    : u32 = OP_BITOPS | BIT_CHG | SRC_IMM | OPER_AW;
pub const OP_BCHG_8_S_AL    : u32 = OP_BITOPS | BIT_CHG | SRC_IMM | OPER_AL;

pub const OP_BCLR_32_R_DN   : u32 = OP_BITOPS | BIT_CLR | SRC_REG | OPER_DN;
pub const OP_BCLR_32_S_DN   : u32 = OP_BITOPS | BIT_CLR | SRC_IMM | OPER_DN;
pub const OP_BCLR_8_R_AI    : u32 = OP_BITOPS | BIT_CLR | SRC_REG | OPER_AI;
pub const OP_BCLR_8_R_PI    : u32 = OP_BITOPS | BIT_CLR | SRC_REG | OPER_PI;
pub const OP_BCLR_8_R_PD    : u32 = OP_BITOPS | BIT_CLR | SRC_REG | OPER_PD;
pub const OP_BCLR_8_R_DI    : u32 = OP_BITOPS | BIT_CLR | SRC_REG | OPER_DI;
pub const OP_BCLR_8_R_IX    : u32 = OP_BITOPS | BIT_CLR | SRC_REG | OPER_IX;
pub const OP_BCLR_8_R_AW    : u32 = OP_BITOPS | BIT_CLR | SRC_REG | OPER_AW;
pub const OP_BCLR_8_R_AL    : u32 = OP_BITOPS | BIT_CLR | SRC_REG | OPER_AL;
pub const OP_BCLR_8_S_AI    : u32 = OP_BITOPS | BIT_CLR | SRC_IMM | OPER_AI;
pub const OP_BCLR_8_S_PI    : u32 = OP_BITOPS | BIT_CLR | SRC_IMM | OPER_PI;
pub const OP_BCLR_8_S_PD    : u32 = OP_BITOPS | BIT_CLR | SRC_IMM | OPER_PD;
pub const OP_BCLR_8_S_DI    : u32 = OP_BITOPS | BIT_CLR | SRC_IMM | OPER_DI;
pub const OP_BCLR_8_S_IX    : u32 = OP_BITOPS | BIT_CLR | SRC_IMM | OPER_IX;
pub const OP_BCLR_8_S_AW    : u32 = OP_BITOPS | BIT_CLR | SRC_IMM | OPER_AW;
pub const OP_BCLR_8_S_AL    : u32 = OP_BITOPS | BIT_CLR | SRC_IMM | OPER_AL;

pub const OP_BSET_32_R_DN   : u32 = OP_BITOPS | BIT_SET | SRC_REG | OPER_DN;
pub const OP_BSET_32_S_DN   : u32 = OP_BITOPS | BIT_SET | SRC_IMM | OPER_DN;
pub const OP_BSET_8_R_AI    : u32 = OP_BITOPS | BIT_SET | SRC_REG | OPER_AI;
pub const OP_BSET_8_R_PI    : u32 = OP_BITOPS | BIT_SET | SRC_REG | OPER_PI;
pub const OP_BSET_8_R_PD    : u32 = OP_BITOPS | BIT_SET | SRC_REG | OPER_PD;
pub const OP_BSET_8_R_DI    : u32 = OP_BITOPS | BIT_SET | SRC_REG | OPER_DI;
pub const OP_BSET_8_R_IX    : u32 = OP_BITOPS | BIT_SET | SRC_REG | OPER_IX;
pub const OP_BSET_8_R_AW    : u32 = OP_BITOPS | BIT_SET | SRC_REG | OPER_AW;
pub const OP_BSET_8_R_AL    : u32 = OP_BITOPS | BIT_SET | SRC_REG | OPER_AL;
pub const OP_BSET_8_S_AI    : u32 = OP_BITOPS | BIT_SET | SRC_IMM | OPER_AI;
pub const OP_BSET_8_S_PI    : u32 = OP_BITOPS | BIT_SET | SRC_IMM | OPER_PI;
pub const OP_BSET_8_S_PD    : u32 = OP_BITOPS | BIT_SET | SRC_IMM | OPER_PD;
pub const OP_BSET_8_S_DI    : u32 = OP_BITOPS | BIT_SET | SRC_IMM | OPER_DI;
pub const OP_BSET_8_S_IX    : u32 = OP_BITOPS | BIT_SET | SRC_IMM | OPER_IX;
pub const OP_BSET_8_S_AW    : u32 = OP_BITOPS | BIT_SET | SRC_IMM | OPER_AW;
pub const OP_BSET_8_S_AL    : u32 = OP_BITOPS | BIT_SET | SRC_IMM | OPER_AL;

pub const OP_BTST_32_R_DN   : u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_DN;
pub const OP_BTST_32_S_DN   : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_DN;
pub const OP_BTST_8_R_AI    : u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_AI;
pub const OP_BTST_8_R_PI    : u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_PI;
pub const OP_BTST_8_R_PD    : u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_PD;
pub const OP_BTST_8_R_DI    : u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_DI;
pub const OP_BTST_8_R_IX    : u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_IX;
pub const OP_BTST_8_R_AW    : u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_AW;
pub const OP_BTST_8_R_AL    : u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_AL;
pub const OP_BTST_8_S_AI    : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_AI;
pub const OP_BTST_8_S_PI    : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_PI;
pub const OP_BTST_8_S_PD    : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_PD;
pub const OP_BTST_8_S_DI    : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_DI;
pub const OP_BTST_8_S_IX    : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_IX;
pub const OP_BTST_8_S_AW    : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_AW;
pub const OP_BTST_8_S_AL    : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_AL;

const WORD_OP: u32 = 0x180;
// const LONG_OP: u32 = 0x100;  only implemented by MC68020+
pub const OP_CHK_16_DN      : u32 = OP_CHK | WORD_OP | OPER_DN;
pub const OP_CHK_16_AI      : u32 = OP_CHK | WORD_OP | OPER_AI;
pub const OP_CHK_16_PI      : u32 = OP_CHK | WORD_OP | OPER_PI;
pub const OP_CHK_16_PD      : u32 = OP_CHK | WORD_OP | OPER_PD;
pub const OP_CHK_16_DI      : u32 = OP_CHK | WORD_OP | OPER_DI;
pub const OP_CHK_16_IX      : u32 = OP_CHK | WORD_OP | OPER_IX;
pub const OP_CHK_16_AW      : u32 = OP_CHK | WORD_OP | OPER_AW;
pub const OP_CHK_16_AL      : u32 = OP_CHK | WORD_OP | OPER_AL;
pub const OP_CHK_16_PCDI    : u32 = OP_CHK | WORD_OP | OPER_PCDI;
pub const OP_CHK_16_PCIX    : u32 = OP_CHK | WORD_OP | OPER_PCIX;
pub const OP_CHK_16_IMM     : u32 = OP_CHK | WORD_OP | OPER_IMM;

pub const OP_CLR_8_DN      : u32 = OP_CLR | BYTE_SIZED | OPER_DN;
pub const OP_CLR_8_AI      : u32 = OP_CLR | BYTE_SIZED | OPER_AI;
pub const OP_CLR_8_PI      : u32 = OP_CLR | BYTE_SIZED | OPER_PI;
pub const OP_CLR_8_PD      : u32 = OP_CLR | BYTE_SIZED | OPER_PD;
pub const OP_CLR_8_DI      : u32 = OP_CLR | BYTE_SIZED | OPER_DI;
pub const OP_CLR_8_IX      : u32 = OP_CLR | BYTE_SIZED | OPER_IX;
pub const OP_CLR_8_AW      : u32 = OP_CLR | BYTE_SIZED | OPER_AW;
pub const OP_CLR_8_AL      : u32 = OP_CLR | BYTE_SIZED | OPER_AL;

pub const OP_CLR_16_DN      : u32 = OP_CLR | WORD_SIZED | OPER_DN;
pub const OP_CLR_16_AI      : u32 = OP_CLR | WORD_SIZED | OPER_AI;
pub const OP_CLR_16_PI      : u32 = OP_CLR | WORD_SIZED | OPER_PI;
pub const OP_CLR_16_PD      : u32 = OP_CLR | WORD_SIZED | OPER_PD;
pub const OP_CLR_16_DI      : u32 = OP_CLR | WORD_SIZED | OPER_DI;
pub const OP_CLR_16_IX      : u32 = OP_CLR | WORD_SIZED | OPER_IX;
pub const OP_CLR_16_AW      : u32 = OP_CLR | WORD_SIZED | OPER_AW;
pub const OP_CLR_16_AL      : u32 = OP_CLR | WORD_SIZED | OPER_AL;

pub const OP_CLR_32_DN      : u32 = OP_CLR | LONG_SIZED | OPER_DN;
pub const OP_CLR_32_AI      : u32 = OP_CLR | LONG_SIZED | OPER_AI;
pub const OP_CLR_32_PI      : u32 = OP_CLR | LONG_SIZED | OPER_PI;
pub const OP_CLR_32_PD      : u32 = OP_CLR | LONG_SIZED | OPER_PD;
pub const OP_CLR_32_DI      : u32 = OP_CLR | LONG_SIZED | OPER_DI;
pub const OP_CLR_32_IX      : u32 = OP_CLR | LONG_SIZED | OPER_IX;
pub const OP_CLR_32_AW      : u32 = OP_CLR | LONG_SIZED | OPER_AW;
pub const OP_CLR_32_AL      : u32 = OP_CLR | LONG_SIZED | OPER_AL;

pub const OP_CMP_8_DN       : u32 = OP_CMP | BYTE_SIZED | OPER_DN;
pub const OP_CMP_8_AI       : u32 = OP_CMP | BYTE_SIZED | OPER_AI;
pub const OP_CMP_8_PI       : u32 = OP_CMP | BYTE_SIZED | OPER_PI;
pub const OP_CMP_8_PD       : u32 = OP_CMP | BYTE_SIZED | OPER_PD;
pub const OP_CMP_8_DI       : u32 = OP_CMP | BYTE_SIZED | OPER_DI;
pub const OP_CMP_8_IX       : u32 = OP_CMP | BYTE_SIZED | OPER_IX;
pub const OP_CMP_8_AW       : u32 = OP_CMP | BYTE_SIZED | OPER_AW;
pub const OP_CMP_8_AL       : u32 = OP_CMP | BYTE_SIZED | OPER_AL;
pub const OP_CMP_8_PCDI     : u32 = OP_CMP | BYTE_SIZED | OPER_PCDI;
pub const OP_CMP_8_PCIX     : u32 = OP_CMP | BYTE_SIZED | OPER_PCIX;
pub const OP_CMP_8_IMM      : u32 = OP_CMP | BYTE_SIZED | OPER_IMM;

pub const OP_CMP_16_DN       : u32 = OP_CMP | WORD_SIZED | OPER_DN;
pub const OP_CMP_16_AN       : u32 = OP_CMP | WORD_SIZED | OPER_AN;
pub const OP_CMP_16_AI       : u32 = OP_CMP | WORD_SIZED | OPER_AI;
pub const OP_CMP_16_PI       : u32 = OP_CMP | WORD_SIZED | OPER_PI;
pub const OP_CMP_16_PD       : u32 = OP_CMP | WORD_SIZED | OPER_PD;
pub const OP_CMP_16_DI       : u32 = OP_CMP | WORD_SIZED | OPER_DI;
pub const OP_CMP_16_IX       : u32 = OP_CMP | WORD_SIZED | OPER_IX;
pub const OP_CMP_16_AW       : u32 = OP_CMP | WORD_SIZED | OPER_AW;
pub const OP_CMP_16_AL       : u32 = OP_CMP | WORD_SIZED | OPER_AL;
pub const OP_CMP_16_PCDI     : u32 = OP_CMP | WORD_SIZED | OPER_PCDI;
pub const OP_CMP_16_PCIX     : u32 = OP_CMP | WORD_SIZED | OPER_PCIX;
pub const OP_CMP_16_IMM      : u32 = OP_CMP | WORD_SIZED | OPER_IMM;

pub const OP_CMP_32_DN       : u32 = OP_CMP | LONG_SIZED | OPER_DN;
pub const OP_CMP_32_AN       : u32 = OP_CMP | LONG_SIZED | OPER_AN;
pub const OP_CMP_32_AI       : u32 = OP_CMP | LONG_SIZED | OPER_AI;
pub const OP_CMP_32_PI       : u32 = OP_CMP | LONG_SIZED | OPER_PI;
pub const OP_CMP_32_PD       : u32 = OP_CMP | LONG_SIZED | OPER_PD;
pub const OP_CMP_32_DI       : u32 = OP_CMP | LONG_SIZED | OPER_DI;
pub const OP_CMP_32_IX       : u32 = OP_CMP | LONG_SIZED | OPER_IX;
pub const OP_CMP_32_AW       : u32 = OP_CMP | LONG_SIZED | OPER_AW;
pub const OP_CMP_32_AL       : u32 = OP_CMP | LONG_SIZED | OPER_AL;
pub const OP_CMP_32_PCDI     : u32 = OP_CMP | LONG_SIZED | OPER_PCDI;
pub const OP_CMP_32_PCIX     : u32 = OP_CMP | LONG_SIZED | OPER_PCIX;
pub const OP_CMP_32_IMM      : u32 = OP_CMP | LONG_SIZED | OPER_IMM;

pub const OP_CMPA_16_DN    : u32 = OP_CMP | DEST_AX_WORD | OPER_DN;
pub const OP_CMPA_16_AN    : u32 = OP_CMP | DEST_AX_WORD | OPER_AN;
pub const OP_CMPA_16_AI    : u32 = OP_CMP | DEST_AX_WORD | OPER_AI;
pub const OP_CMPA_16_PI    : u32 = OP_CMP | DEST_AX_WORD | OPER_PI;
pub const OP_CMPA_16_PD    : u32 = OP_CMP | DEST_AX_WORD | OPER_PD;
pub const OP_CMPA_16_DI    : u32 = OP_CMP | DEST_AX_WORD | OPER_DI;
pub const OP_CMPA_16_IX    : u32 = OP_CMP | DEST_AX_WORD | OPER_IX;
pub const OP_CMPA_16_AW    : u32 = OP_CMP | DEST_AX_WORD | OPER_AW;
pub const OP_CMPA_16_AL    : u32 = OP_CMP | DEST_AX_WORD | OPER_AL;
pub const OP_CMPA_16_PCDI  : u32 = OP_CMP | DEST_AX_WORD | OPER_PCDI;
pub const OP_CMPA_16_PCIX  : u32 = OP_CMP | DEST_AX_WORD | OPER_PCIX;
pub const OP_CMPA_16_IMM   : u32 = OP_CMP | DEST_AX_WORD | OPER_IMM;

pub const OP_CMPA_32_DN    : u32 = OP_CMP | DEST_AX_LONG | OPER_DN;
pub const OP_CMPA_32_AN    : u32 = OP_CMP | DEST_AX_LONG | OPER_AN;
pub const OP_CMPA_32_AI    : u32 = OP_CMP | DEST_AX_LONG | OPER_AI;
pub const OP_CMPA_32_PI    : u32 = OP_CMP | DEST_AX_LONG | OPER_PI;
pub const OP_CMPA_32_PD    : u32 = OP_CMP | DEST_AX_LONG | OPER_PD;
pub const OP_CMPA_32_DI    : u32 = OP_CMP | DEST_AX_LONG | OPER_DI;
pub const OP_CMPA_32_IX    : u32 = OP_CMP | DEST_AX_LONG | OPER_IX;
pub const OP_CMPA_32_AW    : u32 = OP_CMP | DEST_AX_LONG | OPER_AW;
pub const OP_CMPA_32_AL    : u32 = OP_CMP | DEST_AX_LONG | OPER_AL;
pub const OP_CMPA_32_PCDI  : u32 = OP_CMP | DEST_AX_LONG | OPER_PCDI;
pub const OP_CMPA_32_PCIX  : u32 = OP_CMP | DEST_AX_LONG | OPER_PCIX;
pub const OP_CMPA_32_IMM   : u32 = OP_CMP | DEST_AX_LONG | OPER_IMM;

pub const OP_CMPI_8_DN     : u32 = OP_CMPI | BYTE_SIZED | OPER_DN;
pub const OP_CMPI_8_AI     : u32 = OP_CMPI | BYTE_SIZED | OPER_AI;
pub const OP_CMPI_8_PI     : u32 = OP_CMPI | BYTE_SIZED | OPER_PI;
pub const OP_CMPI_8_PD     : u32 = OP_CMPI | BYTE_SIZED | OPER_PD;
pub const OP_CMPI_8_DI     : u32 = OP_CMPI | BYTE_SIZED | OPER_DI;
pub const OP_CMPI_8_IX     : u32 = OP_CMPI | BYTE_SIZED | OPER_IX;
pub const OP_CMPI_8_AW     : u32 = OP_CMPI | BYTE_SIZED | OPER_AW;
pub const OP_CMPI_8_AL     : u32 = OP_CMPI | BYTE_SIZED | OPER_AL;

pub const OP_CMPI_16_DN    : u32 = OP_CMPI | WORD_SIZED | OPER_DN;
pub const OP_CMPI_16_AI    : u32 = OP_CMPI | WORD_SIZED | OPER_AI;
pub const OP_CMPI_16_PI    : u32 = OP_CMPI | WORD_SIZED | OPER_PI;
pub const OP_CMPI_16_PD    : u32 = OP_CMPI | WORD_SIZED | OPER_PD;
pub const OP_CMPI_16_DI    : u32 = OP_CMPI | WORD_SIZED | OPER_DI;
pub const OP_CMPI_16_IX    : u32 = OP_CMPI | WORD_SIZED | OPER_IX;
pub const OP_CMPI_16_AW    : u32 = OP_CMPI | WORD_SIZED | OPER_AW;
pub const OP_CMPI_16_AL    : u32 = OP_CMPI | WORD_SIZED | OPER_AL;

pub const OP_CMPI_32_DN    : u32 = OP_CMPI | LONG_SIZED | OPER_DN;
pub const OP_CMPI_32_AI    : u32 = OP_CMPI | LONG_SIZED | OPER_AI;
pub const OP_CMPI_32_PI    : u32 = OP_CMPI | LONG_SIZED | OPER_PI;
pub const OP_CMPI_32_PD    : u32 = OP_CMPI | LONG_SIZED | OPER_PD;
pub const OP_CMPI_32_DI    : u32 = OP_CMPI | LONG_SIZED | OPER_DI;
pub const OP_CMPI_32_IX    : u32 = OP_CMPI | LONG_SIZED | OPER_IX;
pub const OP_CMPI_32_AW    : u32 = OP_CMPI | LONG_SIZED | OPER_AW;
pub const OP_CMPI_32_AL    : u32 = OP_CMPI | LONG_SIZED | OPER_AL;

pub const OP_CMPM_8        : u32 = OP_CMPM | BYTE_SIZED | MM_MODE;
pub const OP_CMPM_16       : u32 = OP_CMPM | WORD_SIZED | MM_MODE;
pub const OP_CMPM_32       : u32 = OP_CMPM | LONG_SIZED | MM_MODE;

// Put constants for DBcc here
pub const OP_DBT_16        : u32 = OP_DBCC | IF_T;
pub const OP_DBF_16        : u32 = OP_DBCC | IF_F;
pub const OP_DBHI_16       : u32 = OP_DBCC | IF_HI;
pub const OP_DBLS_16       : u32 = OP_DBCC | IF_LS;
pub const OP_DBCC_16       : u32 = OP_DBCC | IF_CC;
pub const OP_DBCS_16       : u32 = OP_DBCC | IF_CS;
pub const OP_DBNE_16       : u32 = OP_DBCC | IF_NE;
pub const OP_DBEQ_16       : u32 = OP_DBCC | IF_EQ;
pub const OP_DBVC_16       : u32 = OP_DBCC | IF_VC;
pub const OP_DBVS_16       : u32 = OP_DBCC | IF_VS;
pub const OP_DBPL_16       : u32 = OP_DBCC | IF_PL;
pub const OP_DBMI_16       : u32 = OP_DBCC | IF_MI;
pub const OP_DBGE_16       : u32 = OP_DBCC | IF_GE;
pub const OP_DBLT_16       : u32 = OP_DBCC | IF_LT;
pub const OP_DBGT_16       : u32 = OP_DBCC | IF_GT;
pub const OP_DBLE_16       : u32 = OP_DBCC | IF_LE;

// Put constants for DIVS here
pub const OP_DIVS_16_AI    : u32 = OP_DIVS | OPER_AI;
pub const OP_DIVS_16_AL    : u32 = OP_DIVS | OPER_AL;
pub const OP_DIVS_16_AW    : u32 = OP_DIVS | OPER_AW;
pub const OP_DIVS_16_DN    : u32 = OP_DIVS | OPER_DN;
pub const OP_DIVS_16_DI    : u32 = OP_DIVS | OPER_DI;
pub const OP_DIVS_16_IMM   : u32 = OP_DIVS | OPER_IMM;
pub const OP_DIVS_16_IX    : u32 = OP_DIVS | OPER_IX;
pub const OP_DIVS_16_PCDI  : u32 = OP_DIVS | OPER_PCDI;
pub const OP_DIVS_16_PCIX  : u32 = OP_DIVS | OPER_PCIX;
pub const OP_DIVS_16_PD    : u32 = OP_DIVS | OPER_PD;
pub const OP_DIVS_16_PI    : u32 = OP_DIVS | OPER_PI;

// Put constants for DIVU here
pub const OP_DIVU_16_AI    : u32 = OP_DIVU | OPER_AI;
pub const OP_DIVU_16_AL    : u32 = OP_DIVU | OPER_AL;
pub const OP_DIVU_16_AW    : u32 = OP_DIVU | OPER_AW;
pub const OP_DIVU_16_DN    : u32 = OP_DIVU | OPER_DN;
pub const OP_DIVU_16_DI    : u32 = OP_DIVU | OPER_DI;
pub const OP_DIVU_16_IMM   : u32 = OP_DIVU | OPER_IMM;
pub const OP_DIVU_16_IX    : u32 = OP_DIVU | OPER_IX;
pub const OP_DIVU_16_PCDI  : u32 = OP_DIVU | OPER_PCDI;
pub const OP_DIVU_16_PCIX  : u32 = OP_DIVU | OPER_PCIX;
pub const OP_DIVU_16_PD    : u32 = OP_DIVU | OPER_PD;
pub const OP_DIVU_16_PI    : u32 = OP_DIVU | OPER_PI;

// Put constants for EOR, EORI, EORI to CCR and EORI to SR here
pub const OP_EOR_8_DN   : u32 = OP_EOR | BYTE_SIZED | DEST_EA | OPER_DN;
pub const OP_EOR_8_AI   : u32 = OP_EOR | BYTE_SIZED | DEST_EA | OPER_AI;
pub const OP_EOR_8_PI   : u32 = OP_EOR | BYTE_SIZED | DEST_EA | OPER_PI;
pub const OP_EOR_8_PD   : u32 = OP_EOR | BYTE_SIZED | DEST_EA | OPER_PD;
pub const OP_EOR_8_DI   : u32 = OP_EOR | BYTE_SIZED | DEST_EA | OPER_DI;
pub const OP_EOR_8_IX   : u32 = OP_EOR | BYTE_SIZED | DEST_EA | OPER_IX;
pub const OP_EOR_8_AW   : u32 = OP_EOR | BYTE_SIZED | DEST_EA | OPER_AW;
pub const OP_EOR_8_AL   : u32 = OP_EOR | BYTE_SIZED | DEST_EA | OPER_AL;

pub const OP_EOR_16_DN  : u32 = OP_EOR | WORD_SIZED | DEST_EA | OPER_DN;
pub const OP_EOR_16_AI  : u32 = OP_EOR | WORD_SIZED | DEST_EA | OPER_AI;
pub const OP_EOR_16_PI  : u32 = OP_EOR | WORD_SIZED | DEST_EA | OPER_PI;
pub const OP_EOR_16_PD  : u32 = OP_EOR | WORD_SIZED | DEST_EA | OPER_PD;
pub const OP_EOR_16_DI  : u32 = OP_EOR | WORD_SIZED | DEST_EA | OPER_DI;
pub const OP_EOR_16_IX  : u32 = OP_EOR | WORD_SIZED | DEST_EA | OPER_IX;
pub const OP_EOR_16_AW  : u32 = OP_EOR | WORD_SIZED | DEST_EA | OPER_AW;
pub const OP_EOR_16_AL  : u32 = OP_EOR | WORD_SIZED | DEST_EA | OPER_AL;

pub const OP_EOR_32_DN  : u32 = OP_EOR | LONG_SIZED | DEST_EA | OPER_DN;
pub const OP_EOR_32_AI  : u32 = OP_EOR | LONG_SIZED | DEST_EA | OPER_AI;
pub const OP_EOR_32_PI  : u32 = OP_EOR | LONG_SIZED | DEST_EA | OPER_PI;
pub const OP_EOR_32_PD  : u32 = OP_EOR | LONG_SIZED | DEST_EA | OPER_PD;
pub const OP_EOR_32_DI  : u32 = OP_EOR | LONG_SIZED | DEST_EA | OPER_DI;
pub const OP_EOR_32_IX  : u32 = OP_EOR | LONG_SIZED | DEST_EA | OPER_IX;
pub const OP_EOR_32_AW  : u32 = OP_EOR | LONG_SIZED | DEST_EA | OPER_AW;
pub const OP_EOR_32_AL  : u32 = OP_EOR | LONG_SIZED | DEST_EA | OPER_AL;

pub const OP_EORI_8_DN     : u32 = OP_EORI | BYTE_SIZED | OPER_DN;
pub const OP_EORI_8_AI     : u32 = OP_EORI | BYTE_SIZED | OPER_AI;
pub const OP_EORI_8_PI     : u32 = OP_EORI | BYTE_SIZED | OPER_PI;
pub const OP_EORI_8_PD     : u32 = OP_EORI | BYTE_SIZED | OPER_PD;
pub const OP_EORI_8_DI     : u32 = OP_EORI | BYTE_SIZED | OPER_DI;
pub const OP_EORI_8_IX     : u32 = OP_EORI | BYTE_SIZED | OPER_IX;
pub const OP_EORI_8_AW     : u32 = OP_EORI | BYTE_SIZED | OPER_AW;
pub const OP_EORI_8_AL     : u32 = OP_EORI | BYTE_SIZED | OPER_AL;

pub const OP_EORI_16_DN    : u32 = OP_EORI | WORD_SIZED | OPER_DN;
pub const OP_EORI_16_AI    : u32 = OP_EORI | WORD_SIZED | OPER_AI;
pub const OP_EORI_16_PI    : u32 = OP_EORI | WORD_SIZED | OPER_PI;
pub const OP_EORI_16_PD    : u32 = OP_EORI | WORD_SIZED | OPER_PD;
pub const OP_EORI_16_DI    : u32 = OP_EORI | WORD_SIZED | OPER_DI;
pub const OP_EORI_16_IX    : u32 = OP_EORI | WORD_SIZED | OPER_IX;
pub const OP_EORI_16_AW    : u32 = OP_EORI | WORD_SIZED | OPER_AW;
pub const OP_EORI_16_AL    : u32 = OP_EORI | WORD_SIZED | OPER_AL;

pub const OP_EORI_32_DN    : u32 = OP_EORI | LONG_SIZED | OPER_DN;
pub const OP_EORI_32_AI    : u32 = OP_EORI | LONG_SIZED | OPER_AI;
pub const OP_EORI_32_PI    : u32 = OP_EORI | LONG_SIZED | OPER_PI;
pub const OP_EORI_32_PD    : u32 = OP_EORI | LONG_SIZED | OPER_PD;
pub const OP_EORI_32_DI    : u32 = OP_EORI | LONG_SIZED | OPER_DI;
pub const OP_EORI_32_IX    : u32 = OP_EORI | LONG_SIZED | OPER_IX;
pub const OP_EORI_32_AW    : u32 = OP_EORI | LONG_SIZED | OPER_AW;
pub const OP_EORI_32_AL    : u32 = OP_EORI | LONG_SIZED | OPER_AL;

pub const OP_EORI_16_TOC   : u32 = OP_EORI | DEST_CCR;
pub const OP_EORI_16_TOS   : u32 = OP_EORI | DEST_SR;

// Put constants for EXG here
const EXG_DATA_DATA: u32 = 0x40; // Exchange two data registers
const EXG_ADDR_ADDR: u32 = 0x48; // Exchange two address registers
const EXG_DATA_ADDR: u32 = 0x88; // Exchange a data register and an address register

pub const OP_EXG_32_DD: u32 = OP_EXG | EXG_DATA_DATA;
pub const OP_EXG_32_AA: u32 = OP_EXG | EXG_ADDR_ADDR;
pub const OP_EXG_32_DA: u32 = OP_EXG | EXG_DATA_ADDR;

// Put constants for EXT here (these are the same as DEST_AX_WORD,
// DEST_AX_LONG, perhaps there's a better common name somewhere)
const BYTE_TO_WORD: u32 = 0x080;
const WORD_TO_LONG: u32 = 0x0C0;
// const BYTE_TO_LONG: u32 = 0x1C0; // 020+

pub const OP_EXT_BW: u32 = OP_EXT | BYTE_TO_WORD;
pub const OP_EXT_WL: u32 = OP_EXT | WORD_TO_LONG;
// pub const OP_EXT_BL: u32 = OP_EXT | BYTE_TO_LONG; // 020+

// Put constants for ILLEGAL here
pub const OP_ILLEGAL : u32 = 0b0100_1010_1111_1100;

// Put constants for JMP here
pub const OP_JMP_32_AI   : u32 = OP_JMP | OPER_AI;
pub const OP_JMP_32_AL   : u32 = OP_JMP | OPER_AL;
pub const OP_JMP_32_AW   : u32 = OP_JMP | OPER_AW;
pub const OP_JMP_32_DI   : u32 = OP_JMP | OPER_DI;
pub const OP_JMP_32_IX   : u32 = OP_JMP | OPER_IX;
pub const OP_JMP_32_PCDI : u32 = OP_JMP | OPER_PCDI;
pub const OP_JMP_32_PCIX : u32 = OP_JMP | OPER_PCIX;

// Put constants for JSR here
pub const OP_JSR_32_AI   : u32 = OP_JSR | OPER_AI;
pub const OP_JSR_32_AL   : u32 = OP_JSR | OPER_AL;
pub const OP_JSR_32_AW   : u32 = OP_JSR | OPER_AW;
pub const OP_JSR_32_DI   : u32 = OP_JSR | OPER_DI;
pub const OP_JSR_32_IX   : u32 = OP_JSR | OPER_IX;
pub const OP_JSR_32_PCDI : u32 = OP_JSR | OPER_PCDI;
pub const OP_JSR_32_PCIX : u32 = OP_JSR | OPER_PCIX;

// Put constants for LEA here
pub const OP_LEA_32_AI   : u32 = OP_LEA | OPER_AI;
pub const OP_LEA_32_AL   : u32 = OP_LEA | OPER_AL;
pub const OP_LEA_32_AW   : u32 = OP_LEA | OPER_AW;
pub const OP_LEA_32_DI   : u32 = OP_LEA | OPER_DI;
pub const OP_LEA_32_IX   : u32 = OP_LEA | OPER_IX;
pub const OP_LEA_32_PCDI : u32 = OP_LEA | OPER_PCDI;
pub const OP_LEA_32_PCIX : u32 = OP_LEA | OPER_PCIX;

// Put constants for LINK here
pub const OP_LINK_16     : u32 = 0b0100_1110_0101_0000;

// Put constants for LSL, LSR here
pub const OP_LSL_8_R        : u32 = OP_SHIFT | SHIFT_LEFT  | BYTE_SIZED | LOGI_REG_SHIFT | REG_COUNT;
pub const OP_LSL_8_S        : u32 = OP_SHIFT | SHIFT_LEFT  | BYTE_SIZED | LOGI_REG_SHIFT | IMM_COUNT;
pub const OP_LSL_16_R       : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | LOGI_REG_SHIFT | REG_COUNT;
pub const OP_LSL_16_S       : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | LOGI_REG_SHIFT | IMM_COUNT;
pub const OP_LSL_32_R       : u32 = OP_SHIFT | SHIFT_LEFT  | LONG_SIZED | LOGI_REG_SHIFT | REG_COUNT;
pub const OP_LSL_32_S       : u32 = OP_SHIFT | SHIFT_LEFT  | LONG_SIZED | LOGI_REG_SHIFT | IMM_COUNT;

pub const OP_LSL_16_AI      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | LOGI_MEM_SHIFT | OPER_AI;
pub const OP_LSL_16_PI      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | LOGI_MEM_SHIFT | OPER_PI;
pub const OP_LSL_16_PD      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | LOGI_MEM_SHIFT | OPER_PD;
pub const OP_LSL_16_DI      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | LOGI_MEM_SHIFT | OPER_DI;
pub const OP_LSL_16_IX      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | LOGI_MEM_SHIFT | OPER_IX;
pub const OP_LSL_16_AW      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | LOGI_MEM_SHIFT | OPER_AW;
pub const OP_LSL_16_AL      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | LOGI_MEM_SHIFT | OPER_AL;

pub const OP_LSR_8_R        : u32 = OP_SHIFT | SHIFT_RIGHT | BYTE_SIZED | LOGI_REG_SHIFT | REG_COUNT;
pub const OP_LSR_8_S        : u32 = OP_SHIFT | SHIFT_RIGHT | BYTE_SIZED | LOGI_REG_SHIFT | IMM_COUNT;
pub const OP_LSR_16_R       : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | LOGI_REG_SHIFT | REG_COUNT;
pub const OP_LSR_16_S       : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | LOGI_REG_SHIFT | IMM_COUNT;
pub const OP_LSR_32_R       : u32 = OP_SHIFT | SHIFT_RIGHT | LONG_SIZED | LOGI_REG_SHIFT | REG_COUNT;
pub const OP_LSR_32_S       : u32 = OP_SHIFT | SHIFT_RIGHT | LONG_SIZED | LOGI_REG_SHIFT | IMM_COUNT;

pub const OP_LSR_16_AI      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | LOGI_MEM_SHIFT | OPER_AI;
pub const OP_LSR_16_PI      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | LOGI_MEM_SHIFT | OPER_PI;
pub const OP_LSR_16_PD      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | LOGI_MEM_SHIFT | OPER_PD;
pub const OP_LSR_16_DI      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | LOGI_MEM_SHIFT | OPER_DI;
pub const OP_LSR_16_IX      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | LOGI_MEM_SHIFT | OPER_IX;
pub const OP_LSR_16_AW      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | LOGI_MEM_SHIFT | OPER_AW;
pub const OP_LSR_16_AL      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | LOGI_MEM_SHIFT | OPER_AL;

// Put constants for MOVE here
const BYTE_MOVE: u32 = 0x1000;
const WORD_MOVE: u32 = 0x3000;
const LONG_MOVE: u32 = 0x2000;

// OPER_XX:s are the 6 least significant bits structured as mmmrrr and
// we need to swap and shift that into place as rrrmmm000000
// to generate the MOVE_TO_XX:s
const MOVE_TO_DN  : u32 = (OPER_DN & 0b111000) << 3; // rrr == 0
const MOVE_TO_AN  : u32 = (OPER_AN & 0b111000) << 3; // rrr == 0
const MOVE_TO_AI  : u32 = (OPER_AI & 0b111000) << 3; // rrr == 0
const MOVE_TO_PI  : u32 = (OPER_PI & 0b111000) << 3; // rrr == 0
const MOVE_TO_PD  : u32 = (OPER_PD & 0b111000) << 3; // rrr == 0
const MOVE_TO_DI  : u32 = (OPER_DI & 0b111000) << 3; // rrr == 0
const MOVE_TO_IX  : u32 = (OPER_IX & 0b111000) << 3; // rrr == 0
const MOVE_TO_AW  : u32 = (OPER_AW & 0b111000) << 3; // rrr == 0
const MOVE_TO_AL  : u32 = (OPER_AL & 0b111000) << 3 | (OPER_AL & 0b111) << 9;
// const MOVE_IMM : u32 = (OPER_IMM & 0b111000) << 3 | (OPER_IMM & 0b111) << 9;

pub const OP_MOVE_8_DN_DN   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DN | OPER_DN;
pub const OP_MOVE_8_AI_DN   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AI | OPER_DN;
pub const OP_MOVE_8_PI_DN   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PI | OPER_DN;
pub const OP_MOVE_8_PD_DN   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PD | OPER_DN;
pub const OP_MOVE_8_DI_DN   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DI | OPER_DN;
pub const OP_MOVE_8_IX_DN   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_IX | OPER_DN;
pub const OP_MOVE_8_AW_DN   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AW | OPER_DN;
pub const OP_MOVE_8_AL_DN   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AL | OPER_DN;

pub const OP_MOVE_8_DN_AI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DN | OPER_AI;
pub const OP_MOVE_8_AI_AI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AI | OPER_AI;
pub const OP_MOVE_8_PI_AI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PI | OPER_AI;
pub const OP_MOVE_8_PD_AI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PD | OPER_AI;
pub const OP_MOVE_8_DI_AI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DI | OPER_AI;
pub const OP_MOVE_8_IX_AI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_IX | OPER_AI;
pub const OP_MOVE_8_AW_AI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AW | OPER_AI;
pub const OP_MOVE_8_AL_AI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AL | OPER_AI;

pub const OP_MOVE_8_DN_PI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DN | OPER_PI;
pub const OP_MOVE_8_AI_PI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AI | OPER_PI;
pub const OP_MOVE_8_PI_PI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PI | OPER_PI;
pub const OP_MOVE_8_PD_PI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PD | OPER_PI;
pub const OP_MOVE_8_DI_PI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DI | OPER_PI;
pub const OP_MOVE_8_IX_PI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_IX | OPER_PI;
pub const OP_MOVE_8_AW_PI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AW | OPER_PI;
pub const OP_MOVE_8_AL_PI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AL | OPER_PI;

pub const OP_MOVE_8_DN_PD   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DN | OPER_PD;
pub const OP_MOVE_8_AI_PD   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AI | OPER_PD;
pub const OP_MOVE_8_PI_PD   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PI | OPER_PD;
pub const OP_MOVE_8_PD_PD   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PD | OPER_PD;
pub const OP_MOVE_8_DI_PD   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DI | OPER_PD;
pub const OP_MOVE_8_IX_PD   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_IX | OPER_PD;
pub const OP_MOVE_8_AW_PD   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AW | OPER_PD;
pub const OP_MOVE_8_AL_PD   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AL | OPER_PD;

pub const OP_MOVE_8_DN_DI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DN | OPER_DI;
pub const OP_MOVE_8_AI_DI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AI | OPER_DI;
pub const OP_MOVE_8_PI_DI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PI | OPER_DI;
pub const OP_MOVE_8_PD_DI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PD | OPER_DI;
pub const OP_MOVE_8_DI_DI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DI | OPER_DI;
pub const OP_MOVE_8_IX_DI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_IX | OPER_DI;
pub const OP_MOVE_8_AW_DI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AW | OPER_DI;
pub const OP_MOVE_8_AL_DI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AL | OPER_DI;

pub const OP_MOVE_8_DN_IX   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DN | OPER_IX;
pub const OP_MOVE_8_AI_IX   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AI | OPER_IX;
pub const OP_MOVE_8_PI_IX   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PI | OPER_IX;
pub const OP_MOVE_8_PD_IX   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PD | OPER_IX;
pub const OP_MOVE_8_DI_IX   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DI | OPER_IX;
pub const OP_MOVE_8_IX_IX   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_IX | OPER_IX;
pub const OP_MOVE_8_AW_IX   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AW | OPER_IX;
pub const OP_MOVE_8_AL_IX   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AL | OPER_IX;

pub const OP_MOVE_8_DN_AW   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DN | OPER_AW;
pub const OP_MOVE_8_AI_AW   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AI | OPER_AW;
pub const OP_MOVE_8_PI_AW   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PI | OPER_AW;
pub const OP_MOVE_8_PD_AW   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PD | OPER_AW;
pub const OP_MOVE_8_DI_AW   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DI | OPER_AW;
pub const OP_MOVE_8_IX_AW   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_IX | OPER_AW;
pub const OP_MOVE_8_AW_AW   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AW | OPER_AW;
pub const OP_MOVE_8_AL_AW   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AL | OPER_AW;

pub const OP_MOVE_8_DN_AL   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DN | OPER_AL;
pub const OP_MOVE_8_AI_AL   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AI | OPER_AL;
pub const OP_MOVE_8_PI_AL   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PI | OPER_AL;
pub const OP_MOVE_8_PD_AL   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PD | OPER_AL;
pub const OP_MOVE_8_DI_AL   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DI | OPER_AL;
pub const OP_MOVE_8_IX_AL   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_IX | OPER_AL;
pub const OP_MOVE_8_AW_AL   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AW | OPER_AL;
pub const OP_MOVE_8_AL_AL   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AL | OPER_AL;

pub const OP_MOVE_8_DN_PCDI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DN | OPER_PCDI;
pub const OP_MOVE_8_AI_PCDI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AI | OPER_PCDI;
pub const OP_MOVE_8_PI_PCDI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PI | OPER_PCDI;
pub const OP_MOVE_8_PD_PCDI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PD | OPER_PCDI;
pub const OP_MOVE_8_DI_PCDI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DI | OPER_PCDI;
pub const OP_MOVE_8_IX_PCDI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_IX | OPER_PCDI;
pub const OP_MOVE_8_AW_PCDI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AW | OPER_PCDI;
pub const OP_MOVE_8_AL_PCDI   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AL | OPER_PCDI;

pub const OP_MOVE_8_DN_PCIX   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DN | OPER_PCIX;
pub const OP_MOVE_8_AI_PCIX   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AI | OPER_PCIX;
pub const OP_MOVE_8_PI_PCIX   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PI | OPER_PCIX;
pub const OP_MOVE_8_PD_PCIX   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PD | OPER_PCIX;
pub const OP_MOVE_8_DI_PCIX   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DI | OPER_PCIX;
pub const OP_MOVE_8_IX_PCIX   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_IX | OPER_PCIX;
pub const OP_MOVE_8_AW_PCIX   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AW | OPER_PCIX;
pub const OP_MOVE_8_AL_PCIX   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AL | OPER_PCIX;

pub const OP_MOVE_8_DN_IMM   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DN | OPER_IMM;
pub const OP_MOVE_8_AI_IMM   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AI | OPER_IMM;
pub const OP_MOVE_8_PI_IMM   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PI | OPER_IMM;
pub const OP_MOVE_8_PD_IMM   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_PD | OPER_IMM;
pub const OP_MOVE_8_DI_IMM   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_DI | OPER_IMM;
pub const OP_MOVE_8_IX_IMM   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_IX | OPER_IMM;
pub const OP_MOVE_8_AW_IMM   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AW | OPER_IMM;
pub const OP_MOVE_8_AL_IMM   : u32 = OP_MOVE | BYTE_MOVE | MOVE_TO_AL | OPER_IMM;

pub const OP_MOVE_16_DN_DN   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DN | OPER_DN;
pub const OP_MOVE_16_AI_DN   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AI | OPER_DN;
pub const OP_MOVE_16_PI_DN   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PI | OPER_DN;
pub const OP_MOVE_16_PD_DN   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PD | OPER_DN;
pub const OP_MOVE_16_DI_DN   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DI | OPER_DN;
pub const OP_MOVE_16_IX_DN   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_IX | OPER_DN;
pub const OP_MOVE_16_AW_DN   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AW | OPER_DN;
pub const OP_MOVE_16_AL_DN   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AL | OPER_DN;

pub const OP_MOVE_16_DN_AI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DN | OPER_AI;
pub const OP_MOVE_16_AI_AI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AI | OPER_AI;
pub const OP_MOVE_16_PI_AI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PI | OPER_AI;
pub const OP_MOVE_16_PD_AI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PD | OPER_AI;
pub const OP_MOVE_16_DI_AI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DI | OPER_AI;
pub const OP_MOVE_16_IX_AI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_IX | OPER_AI;
pub const OP_MOVE_16_AW_AI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AW | OPER_AI;
pub const OP_MOVE_16_AL_AI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AL | OPER_AI;

pub const OP_MOVE_16_DN_PI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DN | OPER_PI;
pub const OP_MOVE_16_AI_PI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AI | OPER_PI;
pub const OP_MOVE_16_PI_PI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PI | OPER_PI;
pub const OP_MOVE_16_PD_PI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PD | OPER_PI;
pub const OP_MOVE_16_DI_PI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DI | OPER_PI;
pub const OP_MOVE_16_IX_PI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_IX | OPER_PI;
pub const OP_MOVE_16_AW_PI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AW | OPER_PI;
pub const OP_MOVE_16_AL_PI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AL | OPER_PI;

pub const OP_MOVE_16_DN_PD   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DN | OPER_PD;
pub const OP_MOVE_16_AI_PD   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AI | OPER_PD;
pub const OP_MOVE_16_PI_PD   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PI | OPER_PD;
pub const OP_MOVE_16_PD_PD   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PD | OPER_PD;
pub const OP_MOVE_16_DI_PD   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DI | OPER_PD;
pub const OP_MOVE_16_IX_PD   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_IX | OPER_PD;
pub const OP_MOVE_16_AW_PD   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AW | OPER_PD;
pub const OP_MOVE_16_AL_PD   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AL | OPER_PD;

pub const OP_MOVE_16_DN_DI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DN | OPER_DI;
pub const OP_MOVE_16_AI_DI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AI | OPER_DI;
pub const OP_MOVE_16_PI_DI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PI | OPER_DI;
pub const OP_MOVE_16_PD_DI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PD | OPER_DI;
pub const OP_MOVE_16_DI_DI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DI | OPER_DI;
pub const OP_MOVE_16_IX_DI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_IX | OPER_DI;
pub const OP_MOVE_16_AW_DI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AW | OPER_DI;
pub const OP_MOVE_16_AL_DI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AL | OPER_DI;

pub const OP_MOVE_16_DN_IX   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DN | OPER_IX;
pub const OP_MOVE_16_AI_IX   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AI | OPER_IX;
pub const OP_MOVE_16_PI_IX   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PI | OPER_IX;
pub const OP_MOVE_16_PD_IX   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PD | OPER_IX;
pub const OP_MOVE_16_DI_IX   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DI | OPER_IX;
pub const OP_MOVE_16_IX_IX   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_IX | OPER_IX;
pub const OP_MOVE_16_AW_IX   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AW | OPER_IX;
pub const OP_MOVE_16_AL_IX   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AL | OPER_IX;

pub const OP_MOVE_16_DN_AW   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DN | OPER_AW;
pub const OP_MOVE_16_AI_AW   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AI | OPER_AW;
pub const OP_MOVE_16_PI_AW   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PI | OPER_AW;
pub const OP_MOVE_16_PD_AW   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PD | OPER_AW;
pub const OP_MOVE_16_DI_AW   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DI | OPER_AW;
pub const OP_MOVE_16_IX_AW   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_IX | OPER_AW;
pub const OP_MOVE_16_AW_AW   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AW | OPER_AW;
pub const OP_MOVE_16_AL_AW   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AL | OPER_AW;

pub const OP_MOVE_16_DN_AL   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DN | OPER_AL;
pub const OP_MOVE_16_AI_AL   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AI | OPER_AL;
pub const OP_MOVE_16_PI_AL   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PI | OPER_AL;
pub const OP_MOVE_16_PD_AL   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PD | OPER_AL;
pub const OP_MOVE_16_DI_AL   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DI | OPER_AL;
pub const OP_MOVE_16_IX_AL   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_IX | OPER_AL;
pub const OP_MOVE_16_AW_AL   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AW | OPER_AL;
pub const OP_MOVE_16_AL_AL   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AL | OPER_AL;

pub const OP_MOVE_16_DN_PCDI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DN | OPER_PCDI;
pub const OP_MOVE_16_AI_PCDI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AI | OPER_PCDI;
pub const OP_MOVE_16_PI_PCDI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PI | OPER_PCDI;
pub const OP_MOVE_16_PD_PCDI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PD | OPER_PCDI;
pub const OP_MOVE_16_DI_PCDI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DI | OPER_PCDI;
pub const OP_MOVE_16_IX_PCDI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_IX | OPER_PCDI;
pub const OP_MOVE_16_AW_PCDI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AW | OPER_PCDI;
pub const OP_MOVE_16_AL_PCDI   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AL | OPER_PCDI;

pub const OP_MOVE_16_DN_PCIX   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DN | OPER_PCIX;
pub const OP_MOVE_16_AI_PCIX   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AI | OPER_PCIX;
pub const OP_MOVE_16_PI_PCIX   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PI | OPER_PCIX;
pub const OP_MOVE_16_PD_PCIX   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PD | OPER_PCIX;
pub const OP_MOVE_16_DI_PCIX   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DI | OPER_PCIX;
pub const OP_MOVE_16_IX_PCIX   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_IX | OPER_PCIX;
pub const OP_MOVE_16_AW_PCIX   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AW | OPER_PCIX;
pub const OP_MOVE_16_AL_PCIX   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AL | OPER_PCIX;

pub const OP_MOVE_16_DN_IMM   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DN | OPER_IMM;
pub const OP_MOVE_16_AI_IMM   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AI | OPER_IMM;
pub const OP_MOVE_16_PI_IMM   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PI | OPER_IMM;
pub const OP_MOVE_16_PD_IMM   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PD | OPER_IMM;
pub const OP_MOVE_16_DI_IMM   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DI | OPER_IMM;
pub const OP_MOVE_16_IX_IMM   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_IX | OPER_IMM;
pub const OP_MOVE_16_AW_IMM   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AW | OPER_IMM;
pub const OP_MOVE_16_AL_IMM   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AL | OPER_IMM;

pub const OP_MOVE_32_DN_DN   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DN | OPER_DN;
pub const OP_MOVE_32_AI_DN   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AI | OPER_DN;
pub const OP_MOVE_32_PI_DN   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PI | OPER_DN;
pub const OP_MOVE_32_PD_DN   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PD | OPER_DN;
pub const OP_MOVE_32_DI_DN   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DI | OPER_DN;
pub const OP_MOVE_32_IX_DN   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_IX | OPER_DN;
pub const OP_MOVE_32_AW_DN   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AW | OPER_DN;
pub const OP_MOVE_32_AL_DN   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AL | OPER_DN;

pub const OP_MOVE_32_DN_AI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DN | OPER_AI;
pub const OP_MOVE_32_AI_AI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AI | OPER_AI;
pub const OP_MOVE_32_PI_AI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PI | OPER_AI;
pub const OP_MOVE_32_PD_AI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PD | OPER_AI;
pub const OP_MOVE_32_DI_AI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DI | OPER_AI;
pub const OP_MOVE_32_IX_AI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_IX | OPER_AI;
pub const OP_MOVE_32_AW_AI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AW | OPER_AI;
pub const OP_MOVE_32_AL_AI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AL | OPER_AI;

pub const OP_MOVE_32_DN_PI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DN | OPER_PI;
pub const OP_MOVE_32_AI_PI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AI | OPER_PI;
pub const OP_MOVE_32_PI_PI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PI | OPER_PI;
pub const OP_MOVE_32_PD_PI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PD | OPER_PI;
pub const OP_MOVE_32_DI_PI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DI | OPER_PI;
pub const OP_MOVE_32_IX_PI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_IX | OPER_PI;
pub const OP_MOVE_32_AW_PI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AW | OPER_PI;
pub const OP_MOVE_32_AL_PI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AL | OPER_PI;

pub const OP_MOVE_32_DN_PD   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DN | OPER_PD;
pub const OP_MOVE_32_AI_PD   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AI | OPER_PD;
pub const OP_MOVE_32_PI_PD   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PI | OPER_PD;
pub const OP_MOVE_32_PD_PD   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PD | OPER_PD;
pub const OP_MOVE_32_DI_PD   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DI | OPER_PD;
pub const OP_MOVE_32_IX_PD   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_IX | OPER_PD;
pub const OP_MOVE_32_AW_PD   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AW | OPER_PD;
pub const OP_MOVE_32_AL_PD   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AL | OPER_PD;

pub const OP_MOVE_32_DN_DI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DN | OPER_DI;
pub const OP_MOVE_32_AI_DI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AI | OPER_DI;
pub const OP_MOVE_32_PI_DI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PI | OPER_DI;
pub const OP_MOVE_32_PD_DI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PD | OPER_DI;
pub const OP_MOVE_32_DI_DI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DI | OPER_DI;
pub const OP_MOVE_32_IX_DI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_IX | OPER_DI;
pub const OP_MOVE_32_AW_DI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AW | OPER_DI;
pub const OP_MOVE_32_AL_DI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AL | OPER_DI;

pub const OP_MOVE_32_DN_IX   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DN | OPER_IX;
pub const OP_MOVE_32_AI_IX   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AI | OPER_IX;
pub const OP_MOVE_32_PI_IX   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PI | OPER_IX;
pub const OP_MOVE_32_PD_IX   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PD | OPER_IX;
pub const OP_MOVE_32_DI_IX   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DI | OPER_IX;
pub const OP_MOVE_32_IX_IX   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_IX | OPER_IX;
pub const OP_MOVE_32_AW_IX   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AW | OPER_IX;
pub const OP_MOVE_32_AL_IX   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AL | OPER_IX;

pub const OP_MOVE_32_DN_AW   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DN | OPER_AW;
pub const OP_MOVE_32_AI_AW   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AI | OPER_AW;
pub const OP_MOVE_32_PI_AW   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PI | OPER_AW;
pub const OP_MOVE_32_PD_AW   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PD | OPER_AW;
pub const OP_MOVE_32_DI_AW   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DI | OPER_AW;
pub const OP_MOVE_32_IX_AW   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_IX | OPER_AW;
pub const OP_MOVE_32_AW_AW   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AW | OPER_AW;
pub const OP_MOVE_32_AL_AW   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AL | OPER_AW;

pub const OP_MOVE_32_DN_AL   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DN | OPER_AL;
pub const OP_MOVE_32_AI_AL   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AI | OPER_AL;
pub const OP_MOVE_32_PI_AL   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PI | OPER_AL;
pub const OP_MOVE_32_PD_AL   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PD | OPER_AL;
pub const OP_MOVE_32_DI_AL   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DI | OPER_AL;
pub const OP_MOVE_32_IX_AL   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_IX | OPER_AL;
pub const OP_MOVE_32_AW_AL   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AW | OPER_AL;
pub const OP_MOVE_32_AL_AL   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AL | OPER_AL;

pub const OP_MOVE_32_DN_PCDI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DN | OPER_PCDI;
pub const OP_MOVE_32_AI_PCDI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AI | OPER_PCDI;
pub const OP_MOVE_32_PI_PCDI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PI | OPER_PCDI;
pub const OP_MOVE_32_PD_PCDI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PD | OPER_PCDI;
pub const OP_MOVE_32_DI_PCDI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DI | OPER_PCDI;
pub const OP_MOVE_32_IX_PCDI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_IX | OPER_PCDI;
pub const OP_MOVE_32_AW_PCDI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AW | OPER_PCDI;
pub const OP_MOVE_32_AL_PCDI   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AL | OPER_PCDI;

pub const OP_MOVE_32_DN_PCIX   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DN | OPER_PCIX;
pub const OP_MOVE_32_AI_PCIX   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AI | OPER_PCIX;
pub const OP_MOVE_32_PI_PCIX   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PI | OPER_PCIX;
pub const OP_MOVE_32_PD_PCIX   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PD | OPER_PCIX;
pub const OP_MOVE_32_DI_PCIX   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DI | OPER_PCIX;
pub const OP_MOVE_32_IX_PCIX   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_IX | OPER_PCIX;
pub const OP_MOVE_32_AW_PCIX   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AW | OPER_PCIX;
pub const OP_MOVE_32_AL_PCIX   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AL | OPER_PCIX;

pub const OP_MOVE_32_DN_IMM   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DN | OPER_IMM;
pub const OP_MOVE_32_AI_IMM   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AI | OPER_IMM;
pub const OP_MOVE_32_PI_IMM   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PI | OPER_IMM;
pub const OP_MOVE_32_PD_IMM   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PD | OPER_IMM;
pub const OP_MOVE_32_DI_IMM   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DI | OPER_IMM;
pub const OP_MOVE_32_IX_IMM   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_IX | OPER_IMM;
pub const OP_MOVE_32_AW_IMM   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AW | OPER_IMM;
pub const OP_MOVE_32_AL_IMM   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AL | OPER_IMM;

// Put constants for MOVEA here
pub const OP_MOVEA_16_DN      : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AN | OPER_DN;
pub const OP_MOVEA_16_AN      : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AN | OPER_AN;
pub const OP_MOVEA_16_AI      : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AN | OPER_AI;
pub const OP_MOVEA_16_PI      : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AN | OPER_PI;
pub const OP_MOVEA_16_PD      : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AN | OPER_PD;
pub const OP_MOVEA_16_DI      : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AN | OPER_DI;
pub const OP_MOVEA_16_IX      : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AN | OPER_IX;
pub const OP_MOVEA_16_AW      : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AN | OPER_AW;
pub const OP_MOVEA_16_AL      : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AN | OPER_AL;
pub const OP_MOVEA_16_PCDI    : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AN | OPER_PCDI;
pub const OP_MOVEA_16_PCIX    : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AN | OPER_PCIX;
pub const OP_MOVEA_16_IMM     : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AN | OPER_IMM;

pub const OP_MOVEA_32_DN      : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AN | OPER_DN;
pub const OP_MOVEA_32_AN      : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AN | OPER_AN;
pub const OP_MOVEA_32_AI      : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AN | OPER_AI;
pub const OP_MOVEA_32_PI      : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AN | OPER_PI;
pub const OP_MOVEA_32_PD      : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AN | OPER_PD;
pub const OP_MOVEA_32_DI      : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AN | OPER_DI;
pub const OP_MOVEA_32_IX      : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AN | OPER_IX;
pub const OP_MOVEA_32_AW      : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AN | OPER_AW;
pub const OP_MOVEA_32_AL      : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AN | OPER_AL;
pub const OP_MOVEA_32_PCDI    : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AN | OPER_PCDI;
pub const OP_MOVEA_32_PCIX    : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AN | OPER_PCIX;
pub const OP_MOVEA_32_IMM     : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AN | OPER_IMM;

// Put constants for MOVE to CCR here
const MOVE_FROM_SR : u32 = 0x0c0;
// const MOVE_FROM_CCR : u32 = 0x2c0; // Only 010+
const MOVE_TO_CCR  : u32 = 0x4c0;
const MOVE_TO_SR   : u32 = 0x6c0;

pub const OP_MOVE_16_TOC_DN   : u32 = OP_MOVE2 | MOVE_TO_CCR | OPER_DN;
pub const OP_MOVE_16_TOC_AI   : u32 = OP_MOVE2 | MOVE_TO_CCR | OPER_AI;
pub const OP_MOVE_16_TOC_PI   : u32 = OP_MOVE2 | MOVE_TO_CCR | OPER_PI;
pub const OP_MOVE_16_TOC_PD   : u32 = OP_MOVE2 | MOVE_TO_CCR | OPER_PD;
pub const OP_MOVE_16_TOC_DI   : u32 = OP_MOVE2 | MOVE_TO_CCR | OPER_DI;
pub const OP_MOVE_16_TOC_IX   : u32 = OP_MOVE2 | MOVE_TO_CCR | OPER_IX;
pub const OP_MOVE_16_TOC_AW   : u32 = OP_MOVE2 | MOVE_TO_CCR | OPER_AW;
pub const OP_MOVE_16_TOC_AL   : u32 = OP_MOVE2 | MOVE_TO_CCR | OPER_AL;
pub const OP_MOVE_16_TOC_PCDI : u32 = OP_MOVE2 | MOVE_TO_CCR | OPER_PCDI;
pub const OP_MOVE_16_TOC_PCIX : u32 = OP_MOVE2 | MOVE_TO_CCR | OPER_PCIX;
pub const OP_MOVE_16_TOC_IMM  : u32 = OP_MOVE2 | MOVE_TO_CCR | OPER_IMM;

// Put constants for MOVE from SR here
pub const OP_MOVE_16_FRS_DN   : u32 = OP_MOVE2 | MOVE_FROM_SR | OPER_DN;
pub const OP_MOVE_16_FRS_AI   : u32 = OP_MOVE2 | MOVE_FROM_SR | OPER_AI;
pub const OP_MOVE_16_FRS_PI   : u32 = OP_MOVE2 | MOVE_FROM_SR | OPER_PI;
pub const OP_MOVE_16_FRS_PD   : u32 = OP_MOVE2 | MOVE_FROM_SR | OPER_PD;
pub const OP_MOVE_16_FRS_DI   : u32 = OP_MOVE2 | MOVE_FROM_SR | OPER_DI;
pub const OP_MOVE_16_FRS_IX   : u32 = OP_MOVE2 | MOVE_FROM_SR | OPER_IX;
pub const OP_MOVE_16_FRS_AW   : u32 = OP_MOVE2 | MOVE_FROM_SR | OPER_AW;
pub const OP_MOVE_16_FRS_AL   : u32 = OP_MOVE2 | MOVE_FROM_SR | OPER_AL;

// Put constants for MOVE to SR here
pub const OP_MOVE_16_TOS_DN   : u32 = OP_MOVE2 | MOVE_TO_SR | OPER_DN;
pub const OP_MOVE_16_TOS_AI   : u32 = OP_MOVE2 | MOVE_TO_SR | OPER_AI;
pub const OP_MOVE_16_TOS_PI   : u32 = OP_MOVE2 | MOVE_TO_SR | OPER_PI;
pub const OP_MOVE_16_TOS_PD   : u32 = OP_MOVE2 | MOVE_TO_SR | OPER_PD;
pub const OP_MOVE_16_TOS_DI   : u32 = OP_MOVE2 | MOVE_TO_SR | OPER_DI;
pub const OP_MOVE_16_TOS_IX   : u32 = OP_MOVE2 | MOVE_TO_SR | OPER_IX;
pub const OP_MOVE_16_TOS_AW   : u32 = OP_MOVE2 | MOVE_TO_SR | OPER_AW;
pub const OP_MOVE_16_TOS_AL   : u32 = OP_MOVE2 | MOVE_TO_SR | OPER_AL;
pub const OP_MOVE_16_TOS_PCDI : u32 = OP_MOVE2 | MOVE_TO_SR | OPER_PCDI;
pub const OP_MOVE_16_TOS_PCIX : u32 = OP_MOVE2 | MOVE_TO_SR | OPER_PCIX;
pub const OP_MOVE_16_TOS_IMM  : u32 = OP_MOVE2 | MOVE_TO_SR | OPER_IMM;

// Put constants for MOVE USP here
const MOVE_USP: u32 = 0xe60;
const TO_AN: u32 = 0x0;
const FROM_AN: u32 = 0x8;
pub const OP_MOVE_32_TOU : u32 = OP_MOVE2 | MOVE_USP | TO_AN;
pub const OP_MOVE_32_FRU : u32 = OP_MOVE2 | MOVE_USP | FROM_AN;

// Put constants for MOVEM here
const WORD_TRANSFER: u32 = 0x00;
const LONG_TRANSFER: u32 = 0x40;
const REGISTER_TO_MEMORY: u32 = 0x000;
const MEMORY_TO_REGISTER: u32 = 0x400;

pub const OP_MOVEM_16_RE_AI: u32 = OP_MOVEM | REGISTER_TO_MEMORY | WORD_TRANSFER | OPER_AI;
pub const OP_MOVEM_16_RE_PD: u32 = OP_MOVEM | REGISTER_TO_MEMORY | WORD_TRANSFER | OPER_PD;
pub const OP_MOVEM_16_RE_DI: u32 = OP_MOVEM | REGISTER_TO_MEMORY | WORD_TRANSFER | OPER_DI;
pub const OP_MOVEM_16_RE_IX: u32 = OP_MOVEM | REGISTER_TO_MEMORY | WORD_TRANSFER | OPER_IX;
pub const OP_MOVEM_16_RE_AW: u32 = OP_MOVEM | REGISTER_TO_MEMORY | WORD_TRANSFER | OPER_AW;
pub const OP_MOVEM_16_RE_AL: u32 = OP_MOVEM | REGISTER_TO_MEMORY | WORD_TRANSFER | OPER_AL;

pub const OP_MOVEM_16_ER_AI:   u32 = OP_MOVEM | MEMORY_TO_REGISTER | WORD_TRANSFER | OPER_AI;
pub const OP_MOVEM_16_ER_PI:   u32 = OP_MOVEM | MEMORY_TO_REGISTER | WORD_TRANSFER | OPER_PI;
pub const OP_MOVEM_16_ER_DI:   u32 = OP_MOVEM | MEMORY_TO_REGISTER | WORD_TRANSFER | OPER_DI;
pub const OP_MOVEM_16_ER_IX:   u32 = OP_MOVEM | MEMORY_TO_REGISTER | WORD_TRANSFER | OPER_IX;
pub const OP_MOVEM_16_ER_AW:   u32 = OP_MOVEM | MEMORY_TO_REGISTER | WORD_TRANSFER | OPER_AW;
pub const OP_MOVEM_16_ER_AL:   u32 = OP_MOVEM | MEMORY_TO_REGISTER | WORD_TRANSFER | OPER_AL;
pub const OP_MOVEM_16_ER_PCDI: u32 = OP_MOVEM | MEMORY_TO_REGISTER | WORD_TRANSFER | OPER_PCDI;
pub const OP_MOVEM_16_ER_PCIX: u32 = OP_MOVEM | MEMORY_TO_REGISTER | WORD_TRANSFER | OPER_PCIX;

pub const OP_MOVEM_32_RE_AI: u32 = OP_MOVEM | REGISTER_TO_MEMORY | LONG_TRANSFER | OPER_AI;
pub const OP_MOVEM_32_RE_PD: u32 = OP_MOVEM | REGISTER_TO_MEMORY | LONG_TRANSFER | OPER_PD;
pub const OP_MOVEM_32_RE_DI: u32 = OP_MOVEM | REGISTER_TO_MEMORY | LONG_TRANSFER | OPER_DI;
pub const OP_MOVEM_32_RE_IX: u32 = OP_MOVEM | REGISTER_TO_MEMORY | LONG_TRANSFER | OPER_IX;
pub const OP_MOVEM_32_RE_AW: u32 = OP_MOVEM | REGISTER_TO_MEMORY | LONG_TRANSFER | OPER_AW;
pub const OP_MOVEM_32_RE_AL: u32 = OP_MOVEM | REGISTER_TO_MEMORY | LONG_TRANSFER | OPER_AL;

pub const OP_MOVEM_32_ER_AI:   u32 = OP_MOVEM | MEMORY_TO_REGISTER | LONG_TRANSFER | OPER_AI;
pub const OP_MOVEM_32_ER_PI:   u32 = OP_MOVEM | MEMORY_TO_REGISTER | LONG_TRANSFER | OPER_PI;
pub const OP_MOVEM_32_ER_DI:   u32 = OP_MOVEM | MEMORY_TO_REGISTER | LONG_TRANSFER | OPER_DI;
pub const OP_MOVEM_32_ER_IX:   u32 = OP_MOVEM | MEMORY_TO_REGISTER | LONG_TRANSFER | OPER_IX;
pub const OP_MOVEM_32_ER_AW:   u32 = OP_MOVEM | MEMORY_TO_REGISTER | LONG_TRANSFER | OPER_AW;
pub const OP_MOVEM_32_ER_AL:   u32 = OP_MOVEM | MEMORY_TO_REGISTER | LONG_TRANSFER | OPER_AL;
pub const OP_MOVEM_32_ER_PCDI: u32 = OP_MOVEM | MEMORY_TO_REGISTER | LONG_TRANSFER | OPER_PCDI;
pub const OP_MOVEM_32_ER_PCIX: u32 = OP_MOVEM | MEMORY_TO_REGISTER | LONG_TRANSFER | OPER_PCIX;

// Put constants for MOVEP here
const MOVEP_MEMORY_TO_REGISTER: u32 = 0x100;
const MOVEP_REGISTER_TO_MEMORY: u32 = 0x180;
pub const OP_MOVEP_16_ER: u32 = OP_MOVEP | WORD_TRANSFER | MOVEP_MEMORY_TO_REGISTER;
pub const OP_MOVEP_16_RE: u32 = OP_MOVEP | WORD_TRANSFER | MOVEP_REGISTER_TO_MEMORY;
pub const OP_MOVEP_32_ER: u32 = OP_MOVEP | LONG_TRANSFER | MOVEP_MEMORY_TO_REGISTER;
pub const OP_MOVEP_32_RE: u32 = OP_MOVEP | LONG_TRANSFER | MOVEP_REGISTER_TO_MEMORY;

// Put constants for MOVEQ here
pub const OP_MOVEQ_32: u32 = 0b0111_0000_0000_0000;

// Put constants for MULS here
pub const OP_MULS_16_DN:   u32 = OP_MULS | OPER_DN;
pub const OP_MULS_16_AI:   u32 = OP_MULS | OPER_AI;
pub const OP_MULS_16_PI:   u32 = OP_MULS | OPER_PI;
pub const OP_MULS_16_PD:   u32 = OP_MULS | OPER_PD;
pub const OP_MULS_16_DI:   u32 = OP_MULS | OPER_DI;
pub const OP_MULS_16_IX:   u32 = OP_MULS | OPER_IX;
pub const OP_MULS_16_AW:   u32 = OP_MULS | OPER_AW;
pub const OP_MULS_16_AL:   u32 = OP_MULS | OPER_AL;
pub const OP_MULS_16_PCDI: u32 = OP_MULS | OPER_PCDI;
pub const OP_MULS_16_PCIX: u32 = OP_MULS | OPER_PCIX;
pub const OP_MULS_16_IMM:  u32 = OP_MULS | OPER_IMM;

// Put constants for MULU here
pub const OP_MULU_16_DN:   u32 = OP_MULU | OPER_DN;
pub const OP_MULU_16_AI:   u32 = OP_MULU | OPER_AI;
pub const OP_MULU_16_PI:   u32 = OP_MULU | OPER_PI;
pub const OP_MULU_16_PD:   u32 = OP_MULU | OPER_PD;
pub const OP_MULU_16_DI:   u32 = OP_MULU | OPER_DI;
pub const OP_MULU_16_IX:   u32 = OP_MULU | OPER_IX;
pub const OP_MULU_16_AW:   u32 = OP_MULU | OPER_AW;
pub const OP_MULU_16_AL:   u32 = OP_MULU | OPER_AL;
pub const OP_MULU_16_PCDI: u32 = OP_MULU | OPER_PCDI;
pub const OP_MULU_16_PCIX: u32 = OP_MULU | OPER_PCIX;
pub const OP_MULU_16_IMM:  u32 = OP_MULU | OPER_IMM;

// Put constants for NBCD here
pub const OP_NBCD_8_DN:   u32 = OP_NBCD | OPER_DN;
pub const OP_NBCD_8_AI:   u32 = OP_NBCD | OPER_AI;
pub const OP_NBCD_8_PI:   u32 = OP_NBCD | OPER_PI;
pub const OP_NBCD_8_PD:   u32 = OP_NBCD | OPER_PD;
pub const OP_NBCD_8_DI:   u32 = OP_NBCD | OPER_DI;
pub const OP_NBCD_8_IX:   u32 = OP_NBCD | OPER_IX;
pub const OP_NBCD_8_AW:   u32 = OP_NBCD | OPER_AW;
pub const OP_NBCD_8_AL:   u32 = OP_NBCD | OPER_AL;

// Put constants for NEG here
pub const OP_NEG_8_DN:   u32 = OP_NEG | BYTE_SIZED | OPER_DN;
pub const OP_NEG_8_AI:   u32 = OP_NEG | BYTE_SIZED | OPER_AI;
pub const OP_NEG_8_PI:   u32 = OP_NEG | BYTE_SIZED | OPER_PI;
pub const OP_NEG_8_PD:   u32 = OP_NEG | BYTE_SIZED | OPER_PD;
pub const OP_NEG_8_DI:   u32 = OP_NEG | BYTE_SIZED | OPER_DI;
pub const OP_NEG_8_IX:   u32 = OP_NEG | BYTE_SIZED | OPER_IX;
pub const OP_NEG_8_AW:   u32 = OP_NEG | BYTE_SIZED | OPER_AW;
pub const OP_NEG_8_AL:   u32 = OP_NEG | BYTE_SIZED | OPER_AL;

pub const OP_NEG_16_DN:   u32 = OP_NEG | WORD_SIZED | OPER_DN;
pub const OP_NEG_16_AI:   u32 = OP_NEG | WORD_SIZED | OPER_AI;
pub const OP_NEG_16_PI:   u32 = OP_NEG | WORD_SIZED | OPER_PI;
pub const OP_NEG_16_PD:   u32 = OP_NEG | WORD_SIZED | OPER_PD;
pub const OP_NEG_16_DI:   u32 = OP_NEG | WORD_SIZED | OPER_DI;
pub const OP_NEG_16_IX:   u32 = OP_NEG | WORD_SIZED | OPER_IX;
pub const OP_NEG_16_AW:   u32 = OP_NEG | WORD_SIZED | OPER_AW;
pub const OP_NEG_16_AL:   u32 = OP_NEG | WORD_SIZED | OPER_AL;

pub const OP_NEG_32_DN:   u32 = OP_NEG | LONG_SIZED | OPER_DN;
pub const OP_NEG_32_AI:   u32 = OP_NEG | LONG_SIZED | OPER_AI;
pub const OP_NEG_32_PI:   u32 = OP_NEG | LONG_SIZED | OPER_PI;
pub const OP_NEG_32_PD:   u32 = OP_NEG | LONG_SIZED | OPER_PD;
pub const OP_NEG_32_DI:   u32 = OP_NEG | LONG_SIZED | OPER_DI;
pub const OP_NEG_32_IX:   u32 = OP_NEG | LONG_SIZED | OPER_IX;
pub const OP_NEG_32_AW:   u32 = OP_NEG | LONG_SIZED | OPER_AW;
pub const OP_NEG_32_AL:   u32 = OP_NEG | LONG_SIZED | OPER_AL;

// Put constants for NEGX here
pub const OP_NEGX_8_DN:   u32 = OP_NEGX | BYTE_SIZED | OPER_DN;
pub const OP_NEGX_8_AI:   u32 = OP_NEGX | BYTE_SIZED | OPER_AI;
pub const OP_NEGX_8_PI:   u32 = OP_NEGX | BYTE_SIZED | OPER_PI;
pub const OP_NEGX_8_PD:   u32 = OP_NEGX | BYTE_SIZED | OPER_PD;
pub const OP_NEGX_8_DI:   u32 = OP_NEGX | BYTE_SIZED | OPER_DI;
pub const OP_NEGX_8_IX:   u32 = OP_NEGX | BYTE_SIZED | OPER_IX;
pub const OP_NEGX_8_AW:   u32 = OP_NEGX | BYTE_SIZED | OPER_AW;
pub const OP_NEGX_8_AL:   u32 = OP_NEGX | BYTE_SIZED | OPER_AL;

pub const OP_NEGX_16_DN:   u32 = OP_NEGX | WORD_SIZED | OPER_DN;
pub const OP_NEGX_16_AI:   u32 = OP_NEGX | WORD_SIZED | OPER_AI;
pub const OP_NEGX_16_PI:   u32 = OP_NEGX | WORD_SIZED | OPER_PI;
pub const OP_NEGX_16_PD:   u32 = OP_NEGX | WORD_SIZED | OPER_PD;
pub const OP_NEGX_16_DI:   u32 = OP_NEGX | WORD_SIZED | OPER_DI;
pub const OP_NEGX_16_IX:   u32 = OP_NEGX | WORD_SIZED | OPER_IX;
pub const OP_NEGX_16_AW:   u32 = OP_NEGX | WORD_SIZED | OPER_AW;
pub const OP_NEGX_16_AL:   u32 = OP_NEGX | WORD_SIZED | OPER_AL;

pub const OP_NEGX_32_DN:   u32 = OP_NEGX | LONG_SIZED | OPER_DN;
pub const OP_NEGX_32_AI:   u32 = OP_NEGX | LONG_SIZED | OPER_AI;
pub const OP_NEGX_32_PI:   u32 = OP_NEGX | LONG_SIZED | OPER_PI;
pub const OP_NEGX_32_PD:   u32 = OP_NEGX | LONG_SIZED | OPER_PD;
pub const OP_NEGX_32_DI:   u32 = OP_NEGX | LONG_SIZED | OPER_DI;
pub const OP_NEGX_32_IX:   u32 = OP_NEGX | LONG_SIZED | OPER_IX;
pub const OP_NEGX_32_AW:   u32 = OP_NEGX | LONG_SIZED | OPER_AW;
pub const OP_NEGX_32_AL:   u32 = OP_NEGX | LONG_SIZED | OPER_AL;

// Put constants for NOP here
pub const OP_NOP:   u32 = 0b0100_1110_0111_0001;

// Put constants for NOT here
pub const OP_NOT_8_DN:   u32 = OP_NOT | BYTE_SIZED | OPER_DN;
pub const OP_NOT_8_AI:   u32 = OP_NOT | BYTE_SIZED | OPER_AI;
pub const OP_NOT_8_PI:   u32 = OP_NOT | BYTE_SIZED | OPER_PI;
pub const OP_NOT_8_PD:   u32 = OP_NOT | BYTE_SIZED | OPER_PD;
pub const OP_NOT_8_DI:   u32 = OP_NOT | BYTE_SIZED | OPER_DI;
pub const OP_NOT_8_IX:   u32 = OP_NOT | BYTE_SIZED | OPER_IX;
pub const OP_NOT_8_AW:   u32 = OP_NOT | BYTE_SIZED | OPER_AW;
pub const OP_NOT_8_AL:   u32 = OP_NOT | BYTE_SIZED | OPER_AL;

pub const OP_NOT_16_DN:   u32 = OP_NOT | WORD_SIZED | OPER_DN;
pub const OP_NOT_16_AI:   u32 = OP_NOT | WORD_SIZED | OPER_AI;
pub const OP_NOT_16_PI:   u32 = OP_NOT | WORD_SIZED | OPER_PI;
pub const OP_NOT_16_PD:   u32 = OP_NOT | WORD_SIZED | OPER_PD;
pub const OP_NOT_16_DI:   u32 = OP_NOT | WORD_SIZED | OPER_DI;
pub const OP_NOT_16_IX:   u32 = OP_NOT | WORD_SIZED | OPER_IX;
pub const OP_NOT_16_AW:   u32 = OP_NOT | WORD_SIZED | OPER_AW;
pub const OP_NOT_16_AL:   u32 = OP_NOT | WORD_SIZED | OPER_AL;

pub const OP_NOT_32_DN:   u32 = OP_NOT | LONG_SIZED | OPER_DN;
pub const OP_NOT_32_AI:   u32 = OP_NOT | LONG_SIZED | OPER_AI;
pub const OP_NOT_32_PI:   u32 = OP_NOT | LONG_SIZED | OPER_PI;
pub const OP_NOT_32_PD:   u32 = OP_NOT | LONG_SIZED | OPER_PD;
pub const OP_NOT_32_DI:   u32 = OP_NOT | LONG_SIZED | OPER_DI;
pub const OP_NOT_32_IX:   u32 = OP_NOT | LONG_SIZED | OPER_IX;
pub const OP_NOT_32_AW:   u32 = OP_NOT | LONG_SIZED | OPER_AW;
pub const OP_NOT_32_AL:   u32 = OP_NOT | LONG_SIZED | OPER_AL;

// Put constants for OR here

pub const OP_OR_8_ER_DN   : u32 = OP_OR | BYTE_SIZED | DEST_DX | OPER_DN;
pub const OP_OR_8_ER_AI   : u32 = OP_OR | BYTE_SIZED | DEST_DX | OPER_AI;
pub const OP_OR_8_ER_PI   : u32 = OP_OR | BYTE_SIZED | DEST_DX | OPER_PI;
pub const OP_OR_8_ER_PD   : u32 = OP_OR | BYTE_SIZED | DEST_DX | OPER_PD;
pub const OP_OR_8_ER_DI   : u32 = OP_OR | BYTE_SIZED | DEST_DX | OPER_DI;
pub const OP_OR_8_ER_IX   : u32 = OP_OR | BYTE_SIZED | DEST_DX | OPER_IX;
pub const OP_OR_8_ER_AW   : u32 = OP_OR | BYTE_SIZED | DEST_DX | OPER_AW;
pub const OP_OR_8_ER_AL   : u32 = OP_OR | BYTE_SIZED | DEST_DX | OPER_AL;
pub const OP_OR_8_ER_PCDI : u32 = OP_OR | BYTE_SIZED | DEST_DX | OPER_PCDI;
pub const OP_OR_8_ER_PCIX : u32 = OP_OR | BYTE_SIZED | DEST_DX | OPER_PCIX;
pub const OP_OR_8_ER_IMM  : u32 = OP_OR | BYTE_SIZED | DEST_DX | OPER_IMM;

pub const OP_OR_8_RE_AI   : u32 = OP_OR | BYTE_SIZED | DEST_EA | OPER_AI;
pub const OP_OR_8_RE_PI   : u32 = OP_OR | BYTE_SIZED | DEST_EA | OPER_PI;
pub const OP_OR_8_RE_PD   : u32 = OP_OR | BYTE_SIZED | DEST_EA | OPER_PD;
pub const OP_OR_8_RE_DI   : u32 = OP_OR | BYTE_SIZED | DEST_EA | OPER_DI;
pub const OP_OR_8_RE_IX   : u32 = OP_OR | BYTE_SIZED | DEST_EA | OPER_IX;
pub const OP_OR_8_RE_AW   : u32 = OP_OR | BYTE_SIZED | DEST_EA | OPER_AW;
pub const OP_OR_8_RE_AL   : u32 = OP_OR | BYTE_SIZED | DEST_EA | OPER_AL;

pub const OP_OR_16_ER_DN  : u32 = OP_OR | WORD_SIZED | DEST_DX | OPER_DN;
pub const OP_OR_16_ER_AI  : u32 = OP_OR | WORD_SIZED | DEST_DX | OPER_AI;
pub const OP_OR_16_ER_PI  : u32 = OP_OR | WORD_SIZED | DEST_DX | OPER_PI;
pub const OP_OR_16_ER_PD  : u32 = OP_OR | WORD_SIZED | DEST_DX | OPER_PD;
pub const OP_OR_16_ER_DI  : u32 = OP_OR | WORD_SIZED | DEST_DX | OPER_DI;
pub const OP_OR_16_ER_IX  : u32 = OP_OR | WORD_SIZED | DEST_DX | OPER_IX;
pub const OP_OR_16_ER_AW  : u32 = OP_OR | WORD_SIZED | DEST_DX | OPER_AW;
pub const OP_OR_16_ER_AL  : u32 = OP_OR | WORD_SIZED | DEST_DX | OPER_AL;
pub const OP_OR_16_ER_PCDI: u32 = OP_OR | WORD_SIZED | DEST_DX | OPER_PCDI;
pub const OP_OR_16_ER_PCIX: u32 = OP_OR | WORD_SIZED | DEST_DX | OPER_PCIX;
pub const OP_OR_16_ER_IMM : u32 = OP_OR | WORD_SIZED | DEST_DX | OPER_IMM;

pub const OP_OR_16_RE_AI  : u32 = OP_OR | WORD_SIZED | DEST_EA | OPER_AI;
pub const OP_OR_16_RE_PI  : u32 = OP_OR | WORD_SIZED | DEST_EA | OPER_PI;
pub const OP_OR_16_RE_PD  : u32 = OP_OR | WORD_SIZED | DEST_EA | OPER_PD;
pub const OP_OR_16_RE_DI  : u32 = OP_OR | WORD_SIZED | DEST_EA | OPER_DI;
pub const OP_OR_16_RE_IX  : u32 = OP_OR | WORD_SIZED | DEST_EA | OPER_IX;
pub const OP_OR_16_RE_AW  : u32 = OP_OR | WORD_SIZED | DEST_EA | OPER_AW;
pub const OP_OR_16_RE_AL  : u32 = OP_OR | WORD_SIZED | DEST_EA | OPER_AL;

pub const OP_OR_32_ER_DN  : u32 = OP_OR | LONG_SIZED | DEST_DX | OPER_DN;
pub const OP_OR_32_ER_AI  : u32 = OP_OR | LONG_SIZED | DEST_DX | OPER_AI;
pub const OP_OR_32_ER_PI  : u32 = OP_OR | LONG_SIZED | DEST_DX | OPER_PI;
pub const OP_OR_32_ER_PD  : u32 = OP_OR | LONG_SIZED | DEST_DX | OPER_PD;
pub const OP_OR_32_ER_DI  : u32 = OP_OR | LONG_SIZED | DEST_DX | OPER_DI;
pub const OP_OR_32_ER_IX  : u32 = OP_OR | LONG_SIZED | DEST_DX | OPER_IX;
pub const OP_OR_32_ER_AW  : u32 = OP_OR | LONG_SIZED | DEST_DX | OPER_AW;
pub const OP_OR_32_ER_AL  : u32 = OP_OR | LONG_SIZED | DEST_DX | OPER_AL;
pub const OP_OR_32_ER_PCDI: u32 = OP_OR | LONG_SIZED | DEST_DX | OPER_PCDI;
pub const OP_OR_32_ER_PCIX: u32 = OP_OR | LONG_SIZED | DEST_DX | OPER_PCIX;
pub const OP_OR_32_ER_IMM : u32 = OP_OR | LONG_SIZED | DEST_DX | OPER_IMM;

pub const OP_OR_32_RE_AI  : u32 = OP_OR | LONG_SIZED | DEST_EA | OPER_AI;
pub const OP_OR_32_RE_PI  : u32 = OP_OR | LONG_SIZED | DEST_EA | OPER_PI;
pub const OP_OR_32_RE_PD  : u32 = OP_OR | LONG_SIZED | DEST_EA | OPER_PD;
pub const OP_OR_32_RE_DI  : u32 = OP_OR | LONG_SIZED | DEST_EA | OPER_DI;
pub const OP_OR_32_RE_IX  : u32 = OP_OR | LONG_SIZED | DEST_EA | OPER_IX;
pub const OP_OR_32_RE_AW  : u32 = OP_OR | LONG_SIZED | DEST_EA | OPER_AW;
pub const OP_OR_32_RE_AL  : u32 = OP_OR | LONG_SIZED | DEST_EA | OPER_AL;

// Put constants for ORI here
pub const OP_ORI_8_DN     : u32 = OP_ORI | BYTE_SIZED | OPER_DN;
pub const OP_ORI_8_AI     : u32 = OP_ORI | BYTE_SIZED | OPER_AI;
pub const OP_ORI_8_PI     : u32 = OP_ORI | BYTE_SIZED | OPER_PI;
pub const OP_ORI_8_PD     : u32 = OP_ORI | BYTE_SIZED | OPER_PD;
pub const OP_ORI_8_DI     : u32 = OP_ORI | BYTE_SIZED | OPER_DI;
pub const OP_ORI_8_IX     : u32 = OP_ORI | BYTE_SIZED | OPER_IX;
pub const OP_ORI_8_AW     : u32 = OP_ORI | BYTE_SIZED | OPER_AW;
pub const OP_ORI_8_AL     : u32 = OP_ORI | BYTE_SIZED | OPER_AL;

pub const OP_ORI_16_DN    : u32 = OP_ORI | WORD_SIZED | OPER_DN;
pub const OP_ORI_16_AI    : u32 = OP_ORI | WORD_SIZED | OPER_AI;
pub const OP_ORI_16_PI    : u32 = OP_ORI | WORD_SIZED | OPER_PI;
pub const OP_ORI_16_PD    : u32 = OP_ORI | WORD_SIZED | OPER_PD;
pub const OP_ORI_16_DI    : u32 = OP_ORI | WORD_SIZED | OPER_DI;
pub const OP_ORI_16_IX    : u32 = OP_ORI | WORD_SIZED | OPER_IX;
pub const OP_ORI_16_AW    : u32 = OP_ORI | WORD_SIZED | OPER_AW;
pub const OP_ORI_16_AL    : u32 = OP_ORI | WORD_SIZED | OPER_AL;

pub const OP_ORI_32_DN    : u32 = OP_ORI | LONG_SIZED | OPER_DN;
pub const OP_ORI_32_AI    : u32 = OP_ORI | LONG_SIZED | OPER_AI;
pub const OP_ORI_32_PI    : u32 = OP_ORI | LONG_SIZED | OPER_PI;
pub const OP_ORI_32_PD    : u32 = OP_ORI | LONG_SIZED | OPER_PD;
pub const OP_ORI_32_DI    : u32 = OP_ORI | LONG_SIZED | OPER_DI;
pub const OP_ORI_32_IX    : u32 = OP_ORI | LONG_SIZED | OPER_IX;
pub const OP_ORI_32_AW    : u32 = OP_ORI | LONG_SIZED | OPER_AW;
pub const OP_ORI_32_AL    : u32 = OP_ORI | LONG_SIZED | OPER_AL;

// Put constants for ORI to CCR here
pub const OP_ORI_16_TOC   : u32 = OP_ORI | DEST_CCR;

// Put constants for ORI to SR here
pub const OP_ORI_16_TOS   : u32 = OP_ORI | DEST_SR;

// Put constants for PEA here
pub const OP_PEA_32_AI   : u32 = OP_PEA | OPER_AI;
pub const OP_PEA_32_DI   : u32 = OP_PEA | OPER_DI;
pub const OP_PEA_32_IX   : u32 = OP_PEA | OPER_IX;
pub const OP_PEA_32_AW   : u32 = OP_PEA | OPER_AW;
pub const OP_PEA_32_AL   : u32 = OP_PEA | OPER_AL;
pub const OP_PEA_32_PCDI : u32 = OP_PEA | OPER_PCDI;
pub const OP_PEA_32_PCIX : u32 = OP_PEA | OPER_PCIX;

// Put constants for RESET here
pub const OP_RESET : u32 = 0b0100_1110_0111_0000;

// Put constants for ROL, ROR here
pub const OP_ROL_8_R        : u32 = OP_SHIFT | SHIFT_LEFT  | BYTE_SIZED | ROTA_REG_SHIFT | REG_COUNT;
pub const OP_ROL_8_S        : u32 = OP_SHIFT | SHIFT_LEFT  | BYTE_SIZED | ROTA_REG_SHIFT | IMM_COUNT;
pub const OP_ROL_16_R       : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTA_REG_SHIFT | REG_COUNT;
pub const OP_ROL_16_S       : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTA_REG_SHIFT | IMM_COUNT;
pub const OP_ROL_32_R       : u32 = OP_SHIFT | SHIFT_LEFT  | LONG_SIZED | ROTA_REG_SHIFT | REG_COUNT;
pub const OP_ROL_32_S       : u32 = OP_SHIFT | SHIFT_LEFT  | LONG_SIZED | ROTA_REG_SHIFT | IMM_COUNT;

pub const OP_ROL_16_AI      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTA_MEM_SHIFT | OPER_AI;
pub const OP_ROL_16_PI      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTA_MEM_SHIFT | OPER_PI;
pub const OP_ROL_16_PD      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTA_MEM_SHIFT | OPER_PD;
pub const OP_ROL_16_DI      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTA_MEM_SHIFT | OPER_DI;
pub const OP_ROL_16_IX      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTA_MEM_SHIFT | OPER_IX;
pub const OP_ROL_16_AW      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTA_MEM_SHIFT | OPER_AW;
pub const OP_ROL_16_AL      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTA_MEM_SHIFT | OPER_AL;

pub const OP_ROR_8_R        : u32 = OP_SHIFT | SHIFT_RIGHT | BYTE_SIZED | ROTA_REG_SHIFT | REG_COUNT;
pub const OP_ROR_8_S        : u32 = OP_SHIFT | SHIFT_RIGHT | BYTE_SIZED | ROTA_REG_SHIFT | IMM_COUNT;
pub const OP_ROR_16_R       : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTA_REG_SHIFT | REG_COUNT;
pub const OP_ROR_16_S       : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTA_REG_SHIFT | IMM_COUNT;
pub const OP_ROR_32_R       : u32 = OP_SHIFT | SHIFT_RIGHT | LONG_SIZED | ROTA_REG_SHIFT | REG_COUNT;
pub const OP_ROR_32_S       : u32 = OP_SHIFT | SHIFT_RIGHT | LONG_SIZED | ROTA_REG_SHIFT | IMM_COUNT;

pub const OP_ROR_16_AI      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTA_MEM_SHIFT | OPER_AI;
pub const OP_ROR_16_PI      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTA_MEM_SHIFT | OPER_PI;
pub const OP_ROR_16_PD      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTA_MEM_SHIFT | OPER_PD;
pub const OP_ROR_16_DI      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTA_MEM_SHIFT | OPER_DI;
pub const OP_ROR_16_IX      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTA_MEM_SHIFT | OPER_IX;
pub const OP_ROR_16_AW      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTA_MEM_SHIFT | OPER_AW;
pub const OP_ROR_16_AL      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTA_MEM_SHIFT | OPER_AL;

// Put constants for ROXL, ROXR here
pub const OP_ROXL_8_R        : u32 = OP_SHIFT | SHIFT_LEFT  | BYTE_SIZED | ROTX_REG_SHIFT | REG_COUNT;
pub const OP_ROXL_8_S        : u32 = OP_SHIFT | SHIFT_LEFT  | BYTE_SIZED | ROTX_REG_SHIFT | IMM_COUNT;
pub const OP_ROXL_16_R       : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTX_REG_SHIFT | REG_COUNT;
pub const OP_ROXL_16_S       : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTX_REG_SHIFT | IMM_COUNT;
pub const OP_ROXL_32_R       : u32 = OP_SHIFT | SHIFT_LEFT  | LONG_SIZED | ROTX_REG_SHIFT | REG_COUNT;
pub const OP_ROXL_32_S       : u32 = OP_SHIFT | SHIFT_LEFT  | LONG_SIZED | ROTX_REG_SHIFT | IMM_COUNT;

pub const OP_ROXL_16_AI      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTX_MEM_SHIFT | OPER_AI;
pub const OP_ROXL_16_PI      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTX_MEM_SHIFT | OPER_PI;
pub const OP_ROXL_16_PD      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTX_MEM_SHIFT | OPER_PD;
pub const OP_ROXL_16_DI      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTX_MEM_SHIFT | OPER_DI;
pub const OP_ROXL_16_IX      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTX_MEM_SHIFT | OPER_IX;
pub const OP_ROXL_16_AW      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTX_MEM_SHIFT | OPER_AW;
pub const OP_ROXL_16_AL      : u32 = OP_SHIFT | SHIFT_LEFT  | WORD_SIZED | ROTX_MEM_SHIFT | OPER_AL;

pub const OP_ROXR_8_R        : u32 = OP_SHIFT | SHIFT_RIGHT | BYTE_SIZED | ROTX_REG_SHIFT | REG_COUNT;
pub const OP_ROXR_8_S        : u32 = OP_SHIFT | SHIFT_RIGHT | BYTE_SIZED | ROTX_REG_SHIFT | IMM_COUNT;
pub const OP_ROXR_16_R       : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTX_REG_SHIFT | REG_COUNT;
pub const OP_ROXR_16_S       : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTX_REG_SHIFT | IMM_COUNT;
pub const OP_ROXR_32_R       : u32 = OP_SHIFT | SHIFT_RIGHT | LONG_SIZED | ROTX_REG_SHIFT | REG_COUNT;
pub const OP_ROXR_32_S       : u32 = OP_SHIFT | SHIFT_RIGHT | LONG_SIZED | ROTX_REG_SHIFT | IMM_COUNT;

pub const OP_ROXR_16_AI      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTX_MEM_SHIFT | OPER_AI;
pub const OP_ROXR_16_PI      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTX_MEM_SHIFT | OPER_PI;
pub const OP_ROXR_16_PD      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTX_MEM_SHIFT | OPER_PD;
pub const OP_ROXR_16_DI      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTX_MEM_SHIFT | OPER_DI;
pub const OP_ROXR_16_IX      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTX_MEM_SHIFT | OPER_IX;
pub const OP_ROXR_16_AW      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTX_MEM_SHIFT | OPER_AW;
pub const OP_ROXR_16_AL      : u32 = OP_SHIFT | SHIFT_RIGHT | WORD_SIZED | ROTX_MEM_SHIFT | OPER_AL;

// Put constants for RTE here
pub const OP_RTE_32 : u32 = 0b0100111001110011;

// Put constants for RTR here
pub const OP_RTR_32 : u32 = 0b0100111001110111;

// Put constants for RTS here
pub const OP_RTS_32 : u32 = 0b0100111001110101;

pub const OP_SBCD_8_RR: u32 = OP_SBCD | BYTE_SIZED | RR_MODE;
pub const OP_SBCD_8_MM: u32 = OP_SBCD | BYTE_SIZED | MM_MODE;

pub const OP_SCC_8_AI      : u32 = OP_SCC | IF_CC | OPER_AI;
pub const OP_SCC_8_AL      : u32 = OP_SCC | IF_CC | OPER_AL;
pub const OP_SCC_8_AW      : u32 = OP_SCC | IF_CC | OPER_AW;
pub const OP_SCC_8_DN      : u32 = OP_SCC | IF_CC | OPER_DN;
pub const OP_SCC_8_DI      : u32 = OP_SCC | IF_CC | OPER_DI;
pub const OP_SCC_8_IX      : u32 = OP_SCC | IF_CC | OPER_IX;
pub const OP_SCC_8_PD      : u32 = OP_SCC | IF_CC | OPER_PD;
pub const OP_SCC_8_PI      : u32 = OP_SCC | IF_CC | OPER_PI;

pub const OP_SCS_8_AI      : u32 = OP_SCC | IF_CS | OPER_AI;
pub const OP_SCS_8_AL      : u32 = OP_SCC | IF_CS | OPER_AL;
pub const OP_SCS_8_AW      : u32 = OP_SCC | IF_CS | OPER_AW;
pub const OP_SCS_8_DN      : u32 = OP_SCC | IF_CS | OPER_DN;
pub const OP_SCS_8_DI      : u32 = OP_SCC | IF_CS | OPER_DI;
pub const OP_SCS_8_IX      : u32 = OP_SCC | IF_CS | OPER_IX;
pub const OP_SCS_8_PD      : u32 = OP_SCC | IF_CS | OPER_PD;
pub const OP_SCS_8_PI      : u32 = OP_SCC | IF_CS | OPER_PI;

pub const OP_SEQ_8_AI      : u32 = OP_SCC | IF_EQ | OPER_AI;
pub const OP_SEQ_8_AL      : u32 = OP_SCC | IF_EQ | OPER_AL;
pub const OP_SEQ_8_AW      : u32 = OP_SCC | IF_EQ | OPER_AW;
pub const OP_SEQ_8_DN      : u32 = OP_SCC | IF_EQ | OPER_DN;
pub const OP_SEQ_8_DI      : u32 = OP_SCC | IF_EQ | OPER_DI;
pub const OP_SEQ_8_IX      : u32 = OP_SCC | IF_EQ | OPER_IX;
pub const OP_SEQ_8_PD      : u32 = OP_SCC | IF_EQ | OPER_PD;
pub const OP_SEQ_8_PI      : u32 = OP_SCC | IF_EQ | OPER_PI;

pub const OP_SF_8_AI       : u32 = OP_SCC | IF_F | OPER_AI;
pub const OP_SF_8_AL       : u32 = OP_SCC | IF_F | OPER_AL;
pub const OP_SF_8_AW       : u32 = OP_SCC | IF_F | OPER_AW;
pub const OP_SF_8_DN       : u32 = OP_SCC | IF_F | OPER_DN;
pub const OP_SF_8_DI       : u32 = OP_SCC | IF_F | OPER_DI;
pub const OP_SF_8_IX       : u32 = OP_SCC | IF_F | OPER_IX;
pub const OP_SF_8_PD       : u32 = OP_SCC | IF_F | OPER_PD;
pub const OP_SF_8_PI       : u32 = OP_SCC | IF_F | OPER_PI;

pub const OP_SGE_8_AI      : u32 = OP_SCC | IF_GE | OPER_AI;
pub const OP_SGE_8_AL      : u32 = OP_SCC | IF_GE | OPER_AL;
pub const OP_SGE_8_AW      : u32 = OP_SCC | IF_GE | OPER_AW;
pub const OP_SGE_8_DN      : u32 = OP_SCC | IF_GE | OPER_DN;
pub const OP_SGE_8_DI      : u32 = OP_SCC | IF_GE | OPER_DI;
pub const OP_SGE_8_IX      : u32 = OP_SCC | IF_GE | OPER_IX;
pub const OP_SGE_8_PD      : u32 = OP_SCC | IF_GE | OPER_PD;
pub const OP_SGE_8_PI      : u32 = OP_SCC | IF_GE | OPER_PI;

pub const OP_SGT_8_AI      : u32 = OP_SCC | IF_GT | OPER_AI;
pub const OP_SGT_8_AL      : u32 = OP_SCC | IF_GT | OPER_AL;
pub const OP_SGT_8_AW      : u32 = OP_SCC | IF_GT | OPER_AW;
pub const OP_SGT_8_DN      : u32 = OP_SCC | IF_GT | OPER_DN;
pub const OP_SGT_8_DI      : u32 = OP_SCC | IF_GT | OPER_DI;
pub const OP_SGT_8_IX      : u32 = OP_SCC | IF_GT | OPER_IX;
pub const OP_SGT_8_PD      : u32 = OP_SCC | IF_GT | OPER_PD;
pub const OP_SGT_8_PI      : u32 = OP_SCC | IF_GT | OPER_PI;

pub const OP_SHI_8_AI      : u32 = OP_SCC | IF_HI | OPER_AI;
pub const OP_SHI_8_AL      : u32 = OP_SCC | IF_HI | OPER_AL;
pub const OP_SHI_8_AW      : u32 = OP_SCC | IF_HI | OPER_AW;
pub const OP_SHI_8_DN      : u32 = OP_SCC | IF_HI | OPER_DN;
pub const OP_SHI_8_DI      : u32 = OP_SCC | IF_HI | OPER_DI;
pub const OP_SHI_8_IX      : u32 = OP_SCC | IF_HI | OPER_IX;
pub const OP_SHI_8_PD      : u32 = OP_SCC | IF_HI | OPER_PD;
pub const OP_SHI_8_PI      : u32 = OP_SCC | IF_HI | OPER_PI;

pub const OP_SLE_8_AI      : u32 = OP_SCC | IF_LE | OPER_AI;
pub const OP_SLE_8_AL      : u32 = OP_SCC | IF_LE | OPER_AL;
pub const OP_SLE_8_AW      : u32 = OP_SCC | IF_LE | OPER_AW;
pub const OP_SLE_8_DN      : u32 = OP_SCC | IF_LE | OPER_DN;
pub const OP_SLE_8_DI      : u32 = OP_SCC | IF_LE | OPER_DI;
pub const OP_SLE_8_IX      : u32 = OP_SCC | IF_LE | OPER_IX;
pub const OP_SLE_8_PD      : u32 = OP_SCC | IF_LE | OPER_PD;
pub const OP_SLE_8_PI      : u32 = OP_SCC | IF_LE | OPER_PI;

pub const OP_SLS_8_AI      : u32 = OP_SCC | IF_LS | OPER_AI;
pub const OP_SLS_8_AL      : u32 = OP_SCC | IF_LS | OPER_AL;
pub const OP_SLS_8_AW      : u32 = OP_SCC | IF_LS | OPER_AW;
pub const OP_SLS_8_DN      : u32 = OP_SCC | IF_LS | OPER_DN;
pub const OP_SLS_8_DI      : u32 = OP_SCC | IF_LS | OPER_DI;
pub const OP_SLS_8_IX      : u32 = OP_SCC | IF_LS | OPER_IX;
pub const OP_SLS_8_PD      : u32 = OP_SCC | IF_LS | OPER_PD;
pub const OP_SLS_8_PI      : u32 = OP_SCC | IF_LS | OPER_PI;

pub const OP_SLT_8_AI      : u32 = OP_SCC | IF_LT | OPER_AI;
pub const OP_SLT_8_AL      : u32 = OP_SCC | IF_LT | OPER_AL;
pub const OP_SLT_8_AW      : u32 = OP_SCC | IF_LT | OPER_AW;
pub const OP_SLT_8_DN      : u32 = OP_SCC | IF_LT | OPER_DN;
pub const OP_SLT_8_DI      : u32 = OP_SCC | IF_LT | OPER_DI;
pub const OP_SLT_8_IX      : u32 = OP_SCC | IF_LT | OPER_IX;
pub const OP_SLT_8_PD      : u32 = OP_SCC | IF_LT | OPER_PD;
pub const OP_SLT_8_PI      : u32 = OP_SCC | IF_LT | OPER_PI;

pub const OP_SMI_8_AI      : u32 = OP_SCC | IF_MI | OPER_AI;
pub const OP_SMI_8_AL      : u32 = OP_SCC | IF_MI | OPER_AL;
pub const OP_SMI_8_AW      : u32 = OP_SCC | IF_MI | OPER_AW;
pub const OP_SMI_8_DN      : u32 = OP_SCC | IF_MI | OPER_DN;
pub const OP_SMI_8_DI      : u32 = OP_SCC | IF_MI | OPER_DI;
pub const OP_SMI_8_IX      : u32 = OP_SCC | IF_MI | OPER_IX;
pub const OP_SMI_8_PD      : u32 = OP_SCC | IF_MI | OPER_PD;
pub const OP_SMI_8_PI      : u32 = OP_SCC | IF_MI | OPER_PI;

pub const OP_SNE_8_AI      : u32 = OP_SCC | IF_NE | OPER_AI;
pub const OP_SNE_8_AL      : u32 = OP_SCC | IF_NE | OPER_AL;
pub const OP_SNE_8_AW      : u32 = OP_SCC | IF_NE | OPER_AW;
pub const OP_SNE_8_DN      : u32 = OP_SCC | IF_NE | OPER_DN;
pub const OP_SNE_8_DI      : u32 = OP_SCC | IF_NE | OPER_DI;
pub const OP_SNE_8_IX      : u32 = OP_SCC | IF_NE | OPER_IX;
pub const OP_SNE_8_PD      : u32 = OP_SCC | IF_NE | OPER_PD;
pub const OP_SNE_8_PI      : u32 = OP_SCC | IF_NE | OPER_PI;

pub const OP_SPL_8_AI      : u32 = OP_SCC | IF_PL | OPER_AI;
pub const OP_SPL_8_AL      : u32 = OP_SCC | IF_PL | OPER_AL;
pub const OP_SPL_8_AW      : u32 = OP_SCC | IF_PL | OPER_AW;
pub const OP_SPL_8_DN      : u32 = OP_SCC | IF_PL | OPER_DN;
pub const OP_SPL_8_DI      : u32 = OP_SCC | IF_PL | OPER_DI;
pub const OP_SPL_8_IX      : u32 = OP_SCC | IF_PL | OPER_IX;
pub const OP_SPL_8_PD      : u32 = OP_SCC | IF_PL | OPER_PD;
pub const OP_SPL_8_PI      : u32 = OP_SCC | IF_PL | OPER_PI;

pub const OP_ST_8_AI       : u32 = OP_SCC | IF_T | OPER_AI;
pub const OP_ST_8_AL       : u32 = OP_SCC | IF_T | OPER_AL;
pub const OP_ST_8_AW       : u32 = OP_SCC | IF_T | OPER_AW;
pub const OP_ST_8_DN       : u32 = OP_SCC | IF_T | OPER_DN;
pub const OP_ST_8_DI       : u32 = OP_SCC | IF_T | OPER_DI;
pub const OP_ST_8_IX       : u32 = OP_SCC | IF_T | OPER_IX;
pub const OP_ST_8_PD       : u32 = OP_SCC | IF_T | OPER_PD;
pub const OP_ST_8_PI       : u32 = OP_SCC | IF_T | OPER_PI;

pub const OP_SVC_8_AI      : u32 = OP_SCC | IF_VC | OPER_AI;
pub const OP_SVC_8_AL      : u32 = OP_SCC | IF_VC | OPER_AL;
pub const OP_SVC_8_AW      : u32 = OP_SCC | IF_VC | OPER_AW;
pub const OP_SVC_8_DN      : u32 = OP_SCC | IF_VC | OPER_DN;
pub const OP_SVC_8_DI      : u32 = OP_SCC | IF_VC | OPER_DI;
pub const OP_SVC_8_IX      : u32 = OP_SCC | IF_VC | OPER_IX;
pub const OP_SVC_8_PD      : u32 = OP_SCC | IF_VC | OPER_PD;
pub const OP_SVC_8_PI      : u32 = OP_SCC | IF_VC | OPER_PI;

pub const OP_SVS_8_AI      : u32 = OP_SCC | IF_VS | OPER_AI;
pub const OP_SVS_8_AL      : u32 = OP_SCC | IF_VS | OPER_AL;
pub const OP_SVS_8_AW      : u32 = OP_SCC | IF_VS | OPER_AW;
pub const OP_SVS_8_DN      : u32 = OP_SCC | IF_VS | OPER_DN;
pub const OP_SVS_8_DI      : u32 = OP_SCC | IF_VS | OPER_DI;
pub const OP_SVS_8_IX      : u32 = OP_SCC | IF_VS | OPER_IX;
pub const OP_SVS_8_PD      : u32 = OP_SCC | IF_VS | OPER_PD;
pub const OP_SVS_8_PI      : u32 = OP_SCC | IF_VS | OPER_PI;

// Put constants for Scc here
// Put constants for STOP here
pub const OP_STOP          : u32 = 0b0100111001110010;

// Put constants for SUB here
pub const OP_SUB_8_ER_DN   : u32 = OP_SUB | BYTE_SIZED | DEST_DX | OPER_DN;
pub const OP_SUB_8_ER_AI   : u32 = OP_SUB | BYTE_SIZED | DEST_DX | OPER_AI;
pub const OP_SUB_8_ER_PI   : u32 = OP_SUB | BYTE_SIZED | DEST_DX | OPER_PI;
pub const OP_SUB_8_ER_PD   : u32 = OP_SUB | BYTE_SIZED | DEST_DX | OPER_PD;
pub const OP_SUB_8_ER_DI   : u32 = OP_SUB | BYTE_SIZED | DEST_DX | OPER_DI;
pub const OP_SUB_8_ER_IX   : u32 = OP_SUB | BYTE_SIZED | DEST_DX | OPER_IX;
pub const OP_SUB_8_ER_AW   : u32 = OP_SUB | BYTE_SIZED | DEST_DX | OPER_AW;
pub const OP_SUB_8_ER_AL   : u32 = OP_SUB | BYTE_SIZED | DEST_DX | OPER_AL;
pub const OP_SUB_8_ER_PCDI : u32 = OP_SUB | BYTE_SIZED | DEST_DX | OPER_PCDI;
pub const OP_SUB_8_ER_PCIX : u32 = OP_SUB | BYTE_SIZED | DEST_DX | OPER_PCIX;
pub const OP_SUB_8_ER_IMM  : u32 = OP_SUB | BYTE_SIZED | DEST_DX | OPER_IMM;

pub const OP_SUB_8_RE_AI   : u32 = OP_SUB | BYTE_SIZED | DEST_EA | OPER_AI;
pub const OP_SUB_8_RE_PI   : u32 = OP_SUB | BYTE_SIZED | DEST_EA | OPER_PI;
pub const OP_SUB_8_RE_PD   : u32 = OP_SUB | BYTE_SIZED | DEST_EA | OPER_PD;
pub const OP_SUB_8_RE_DI   : u32 = OP_SUB | BYTE_SIZED | DEST_EA | OPER_DI;
pub const OP_SUB_8_RE_IX   : u32 = OP_SUB | BYTE_SIZED | DEST_EA | OPER_IX;
pub const OP_SUB_8_RE_AW   : u32 = OP_SUB | BYTE_SIZED | DEST_EA | OPER_AW;
pub const OP_SUB_8_RE_AL   : u32 = OP_SUB | BYTE_SIZED | DEST_EA | OPER_AL;

pub const OP_SUB_16_ER_DN  : u32 = OP_SUB | WORD_SIZED | DEST_DX | OPER_DN;
pub const OP_SUB_16_ER_AN  : u32 = OP_SUB | WORD_SIZED | DEST_DX | OPER_AN;
pub const OP_SUB_16_ER_AI  : u32 = OP_SUB | WORD_SIZED | DEST_DX | OPER_AI;
pub const OP_SUB_16_ER_PI  : u32 = OP_SUB | WORD_SIZED | DEST_DX | OPER_PI;
pub const OP_SUB_16_ER_PD  : u32 = OP_SUB | WORD_SIZED | DEST_DX | OPER_PD;
pub const OP_SUB_16_ER_DI  : u32 = OP_SUB | WORD_SIZED | DEST_DX | OPER_DI;
pub const OP_SUB_16_ER_IX  : u32 = OP_SUB | WORD_SIZED | DEST_DX | OPER_IX;
pub const OP_SUB_16_ER_AW  : u32 = OP_SUB | WORD_SIZED | DEST_DX | OPER_AW;
pub const OP_SUB_16_ER_AL  : u32 = OP_SUB | WORD_SIZED | DEST_DX | OPER_AL;
pub const OP_SUB_16_ER_PCDI: u32 = OP_SUB | WORD_SIZED | DEST_DX | OPER_PCDI;
pub const OP_SUB_16_ER_PCIX: u32 = OP_SUB | WORD_SIZED | DEST_DX | OPER_PCIX;
pub const OP_SUB_16_ER_IMM : u32 = OP_SUB | WORD_SIZED | DEST_DX | OPER_IMM;

pub const OP_SUB_16_RE_AI  : u32 = OP_SUB | WORD_SIZED | DEST_EA | OPER_AI;
pub const OP_SUB_16_RE_PI  : u32 = OP_SUB | WORD_SIZED | DEST_EA | OPER_PI;
pub const OP_SUB_16_RE_PD  : u32 = OP_SUB | WORD_SIZED | DEST_EA | OPER_PD;
pub const OP_SUB_16_RE_DI  : u32 = OP_SUB | WORD_SIZED | DEST_EA | OPER_DI;
pub const OP_SUB_16_RE_IX  : u32 = OP_SUB | WORD_SIZED | DEST_EA | OPER_IX;
pub const OP_SUB_16_RE_AW  : u32 = OP_SUB | WORD_SIZED | DEST_EA | OPER_AW;
pub const OP_SUB_16_RE_AL  : u32 = OP_SUB | WORD_SIZED | DEST_EA | OPER_AL;

pub const OP_SUB_32_ER_DN  : u32 = OP_SUB | LONG_SIZED | DEST_DX | OPER_DN;
pub const OP_SUB_32_ER_AN  : u32 = OP_SUB | LONG_SIZED | DEST_DX | OPER_AN;
pub const OP_SUB_32_ER_AI  : u32 = OP_SUB | LONG_SIZED | DEST_DX | OPER_AI;
pub const OP_SUB_32_ER_PI  : u32 = OP_SUB | LONG_SIZED | DEST_DX | OPER_PI;
pub const OP_SUB_32_ER_PD  : u32 = OP_SUB | LONG_SIZED | DEST_DX | OPER_PD;
pub const OP_SUB_32_ER_DI  : u32 = OP_SUB | LONG_SIZED | DEST_DX | OPER_DI;
pub const OP_SUB_32_ER_IX  : u32 = OP_SUB | LONG_SIZED | DEST_DX | OPER_IX;
pub const OP_SUB_32_ER_AW  : u32 = OP_SUB | LONG_SIZED | DEST_DX | OPER_AW;
pub const OP_SUB_32_ER_AL  : u32 = OP_SUB | LONG_SIZED | DEST_DX | OPER_AL;
pub const OP_SUB_32_ER_PCDI: u32 = OP_SUB | LONG_SIZED | DEST_DX | OPER_PCDI;
pub const OP_SUB_32_ER_PCIX: u32 = OP_SUB | LONG_SIZED | DEST_DX | OPER_PCIX;
pub const OP_SUB_32_ER_IMM : u32 = OP_SUB | LONG_SIZED | DEST_DX | OPER_IMM;

pub const OP_SUB_32_RE_AI  : u32 = OP_SUB | LONG_SIZED | DEST_EA | OPER_AI;
pub const OP_SUB_32_RE_PI  : u32 = OP_SUB | LONG_SIZED | DEST_EA | OPER_PI;
pub const OP_SUB_32_RE_PD  : u32 = OP_SUB | LONG_SIZED | DEST_EA | OPER_PD;
pub const OP_SUB_32_RE_DI  : u32 = OP_SUB | LONG_SIZED | DEST_EA | OPER_DI;
pub const OP_SUB_32_RE_IX  : u32 = OP_SUB | LONG_SIZED | DEST_EA | OPER_IX;
pub const OP_SUB_32_RE_AW  : u32 = OP_SUB | LONG_SIZED | DEST_EA | OPER_AW;
pub const OP_SUB_32_RE_AL  : u32 = OP_SUB | LONG_SIZED | DEST_EA | OPER_AL;

pub const OP_SUBA_16_DN    : u32 = OP_SUB | DEST_AX_WORD | OPER_DN;
pub const OP_SUBA_16_AN    : u32 = OP_SUB | DEST_AX_WORD | OPER_AN;
pub const OP_SUBA_16_AI    : u32 = OP_SUB | DEST_AX_WORD | OPER_AI;
pub const OP_SUBA_16_PI    : u32 = OP_SUB | DEST_AX_WORD | OPER_PI;
pub const OP_SUBA_16_PD    : u32 = OP_SUB | DEST_AX_WORD | OPER_PD;
pub const OP_SUBA_16_DI    : u32 = OP_SUB | DEST_AX_WORD | OPER_DI;
pub const OP_SUBA_16_IX    : u32 = OP_SUB | DEST_AX_WORD | OPER_IX;
pub const OP_SUBA_16_AW    : u32 = OP_SUB | DEST_AX_WORD | OPER_AW;
pub const OP_SUBA_16_AL    : u32 = OP_SUB | DEST_AX_WORD | OPER_AL;
pub const OP_SUBA_16_PCDI  : u32 = OP_SUB | DEST_AX_WORD | OPER_PCDI;
pub const OP_SUBA_16_PCIX  : u32 = OP_SUB | DEST_AX_WORD | OPER_PCIX;
pub const OP_SUBA_16_IMM   : u32 = OP_SUB | DEST_AX_WORD | OPER_IMM;

pub const OP_SUBA_32_DN    : u32 = OP_SUB | DEST_AX_LONG | OPER_DN;
pub const OP_SUBA_32_AN    : u32 = OP_SUB | DEST_AX_LONG | OPER_AN;
pub const OP_SUBA_32_AI    : u32 = OP_SUB | DEST_AX_LONG | OPER_AI;
pub const OP_SUBA_32_PI    : u32 = OP_SUB | DEST_AX_LONG | OPER_PI;
pub const OP_SUBA_32_PD    : u32 = OP_SUB | DEST_AX_LONG | OPER_PD;
pub const OP_SUBA_32_DI    : u32 = OP_SUB | DEST_AX_LONG | OPER_DI;
pub const OP_SUBA_32_IX    : u32 = OP_SUB | DEST_AX_LONG | OPER_IX;
pub const OP_SUBA_32_AW    : u32 = OP_SUB | DEST_AX_LONG | OPER_AW;
pub const OP_SUBA_32_AL    : u32 = OP_SUB | DEST_AX_LONG | OPER_AL;
pub const OP_SUBA_32_PCDI  : u32 = OP_SUB | DEST_AX_LONG | OPER_PCDI;
pub const OP_SUBA_32_PCIX  : u32 = OP_SUB | DEST_AX_LONG | OPER_PCIX;
pub const OP_SUBA_32_IMM   : u32 = OP_SUB | DEST_AX_LONG | OPER_IMM;

pub const OP_SUBI_8_DN     : u32 = OP_SUBI | BYTE_SIZED | OPER_DN;
pub const OP_SUBI_8_AI     : u32 = OP_SUBI | BYTE_SIZED | OPER_AI;
pub const OP_SUBI_8_PI     : u32 = OP_SUBI | BYTE_SIZED | OPER_PI;
pub const OP_SUBI_8_PD     : u32 = OP_SUBI | BYTE_SIZED | OPER_PD;
pub const OP_SUBI_8_DI     : u32 = OP_SUBI | BYTE_SIZED | OPER_DI;
pub const OP_SUBI_8_IX     : u32 = OP_SUBI | BYTE_SIZED | OPER_IX;
pub const OP_SUBI_8_AW     : u32 = OP_SUBI | BYTE_SIZED | OPER_AW;
pub const OP_SUBI_8_AL     : u32 = OP_SUBI | BYTE_SIZED | OPER_AL;

pub const OP_SUBI_16_DN    : u32 = OP_SUBI | WORD_SIZED | OPER_DN;
pub const OP_SUBI_16_AI    : u32 = OP_SUBI | WORD_SIZED | OPER_AI;
pub const OP_SUBI_16_PI    : u32 = OP_SUBI | WORD_SIZED | OPER_PI;
pub const OP_SUBI_16_PD    : u32 = OP_SUBI | WORD_SIZED | OPER_PD;
pub const OP_SUBI_16_DI    : u32 = OP_SUBI | WORD_SIZED | OPER_DI;
pub const OP_SUBI_16_IX    : u32 = OP_SUBI | WORD_SIZED | OPER_IX;
pub const OP_SUBI_16_AW    : u32 = OP_SUBI | WORD_SIZED | OPER_AW;
pub const OP_SUBI_16_AL    : u32 = OP_SUBI | WORD_SIZED | OPER_AL;

pub const OP_SUBI_32_DN    : u32 = OP_SUBI | LONG_SIZED | OPER_DN;
pub const OP_SUBI_32_AI    : u32 = OP_SUBI | LONG_SIZED | OPER_AI;
pub const OP_SUBI_32_PI    : u32 = OP_SUBI | LONG_SIZED | OPER_PI;
pub const OP_SUBI_32_PD    : u32 = OP_SUBI | LONG_SIZED | OPER_PD;
pub const OP_SUBI_32_DI    : u32 = OP_SUBI | LONG_SIZED | OPER_DI;
pub const OP_SUBI_32_IX    : u32 = OP_SUBI | LONG_SIZED | OPER_IX;
pub const OP_SUBI_32_AW    : u32 = OP_SUBI | LONG_SIZED | OPER_AW;
pub const OP_SUBI_32_AL    : u32 = OP_SUBI | LONG_SIZED | OPER_AL;

pub const OP_SUBQ_8_DN     : u32 = OP_SUBQ | BYTE_SIZED | OPER_DN;
pub const OP_SUBQ_8_AI     : u32 = OP_SUBQ | BYTE_SIZED | OPER_AI;
pub const OP_SUBQ_8_PI     : u32 = OP_SUBQ | BYTE_SIZED | OPER_PI;
pub const OP_SUBQ_8_PD     : u32 = OP_SUBQ | BYTE_SIZED | OPER_PD;
pub const OP_SUBQ_8_DI     : u32 = OP_SUBQ | BYTE_SIZED | OPER_DI;
pub const OP_SUBQ_8_IX     : u32 = OP_SUBQ | BYTE_SIZED | OPER_IX;
pub const OP_SUBQ_8_AW     : u32 = OP_SUBQ | BYTE_SIZED | OPER_AW;
pub const OP_SUBQ_8_AL     : u32 = OP_SUBQ | BYTE_SIZED | OPER_AL;

pub const OP_SUBQ_16_DN    : u32 = OP_SUBQ | WORD_SIZED | OPER_DN;
pub const OP_SUBQ_16_AN    : u32 = OP_SUBQ | WORD_SIZED | OPER_AN;
pub const OP_SUBQ_16_AI    : u32 = OP_SUBQ | WORD_SIZED | OPER_AI;
pub const OP_SUBQ_16_PI    : u32 = OP_SUBQ | WORD_SIZED | OPER_PI;
pub const OP_SUBQ_16_PD    : u32 = OP_SUBQ | WORD_SIZED | OPER_PD;
pub const OP_SUBQ_16_DI    : u32 = OP_SUBQ | WORD_SIZED | OPER_DI;
pub const OP_SUBQ_16_IX    : u32 = OP_SUBQ | WORD_SIZED | OPER_IX;
pub const OP_SUBQ_16_AW    : u32 = OP_SUBQ | WORD_SIZED | OPER_AW;
pub const OP_SUBQ_16_AL    : u32 = OP_SUBQ | WORD_SIZED | OPER_AL;

pub const OP_SUBQ_32_DN    : u32 = OP_SUBQ | LONG_SIZED | OPER_DN;
pub const OP_SUBQ_32_AN    : u32 = OP_SUBQ | LONG_SIZED | OPER_AN;
pub const OP_SUBQ_32_AI    : u32 = OP_SUBQ | LONG_SIZED | OPER_AI;
pub const OP_SUBQ_32_PI    : u32 = OP_SUBQ | LONG_SIZED | OPER_PI;
pub const OP_SUBQ_32_PD    : u32 = OP_SUBQ | LONG_SIZED | OPER_PD;
pub const OP_SUBQ_32_DI    : u32 = OP_SUBQ | LONG_SIZED | OPER_DI;
pub const OP_SUBQ_32_IX    : u32 = OP_SUBQ | LONG_SIZED | OPER_IX;
pub const OP_SUBQ_32_AW    : u32 = OP_SUBQ | LONG_SIZED | OPER_AW;
pub const OP_SUBQ_32_AL    : u32 = OP_SUBQ | LONG_SIZED | OPER_AL;

pub const OP_SUBX_8_RR     : u32 = OP_SUBX | BYTE_SIZED | RR_MODE;
pub const OP_SUBX_8_MM     : u32 = OP_SUBX | BYTE_SIZED | MM_MODE;
pub const OP_SUBX_16_RR    : u32 = OP_SUBX | WORD_SIZED | RR_MODE;
pub const OP_SUBX_16_MM    : u32 = OP_SUBX | WORD_SIZED | MM_MODE;
pub const OP_SUBX_32_RR    : u32 = OP_SUBX | LONG_SIZED | RR_MODE;
pub const OP_SUBX_32_MM    : u32 = OP_SUBX | LONG_SIZED | MM_MODE;

// Put constants for SWAP here
pub const OP_SWAP_32_DN    : u32 = OP_SWAP | WORD_SIZED | OPER_DN;

// Put constants for TAS here
pub const OP_TAS_8_DN    : u32 = OP_TAS | OPER_DN;
pub const OP_TAS_8_AI    : u32 = OP_TAS | OPER_AI;
pub const OP_TAS_8_PI    : u32 = OP_TAS | OPER_PI;
pub const OP_TAS_8_PD    : u32 = OP_TAS | OPER_PD;
pub const OP_TAS_8_DI    : u32 = OP_TAS | OPER_DI;
pub const OP_TAS_8_IX    : u32 = OP_TAS | OPER_IX;
pub const OP_TAS_8_AW    : u32 = OP_TAS | OPER_AW;
pub const OP_TAS_8_AL    : u32 = OP_TAS | OPER_AL;

// Put constants for TRAP here
pub const OP_TRAP  : u32 = 0b0100_1110_0100_0000;

// Put constants for TRAPV here
pub const OP_TRAPV : u32 = 0b0100_1110_0111_0110;

// Put constants for TST here
// Put constants for UNLK here
fn generate_optable() -> Vec<OpcodeHandler> {
    // the optable contains opcode mask, matching mask and the corresponding handler + name
    let optable = vec![
        op_entry!(MASK_LO3NIB, OP_UNIMPLEMENTED_1010, unimplemented_1010),
        op_entry!(MASK_LO3NIB, OP_UNIMPLEMENTED_1111, unimplemented_1111),

        op_entry!(MASK_OUT_X_Y, OP_ABCD_8_RR, abcd_8_rr),
        op_entry!(MASK_OUT_X_Y, OP_ABCD_8_MM, abcd_8_mm),

        op_entry!(MASK_OUT_X_Y, OP_ADD_8_ER_DN,   add_8_er_dn),
        op_entry!(MASK_OUT_X_Y, OP_ADD_8_ER_AI,   add_8_er_ai),
        op_entry!(MASK_OUT_X_Y, OP_ADD_8_ER_PI,   add_8_er_pi),
        op_entry!(MASK_OUT_X_Y, OP_ADD_8_ER_PD,   add_8_er_pd),
        op_entry!(MASK_OUT_X_Y, OP_ADD_8_ER_DI,   add_8_er_di),
        op_entry!(MASK_OUT_X_Y, OP_ADD_8_ER_IX,   add_8_er_ix),
        op_entry!(MASK_OUT_X,   OP_ADD_8_ER_AW,   add_8_er_aw),
        op_entry!(MASK_OUT_X,   OP_ADD_8_ER_AL,   add_8_er_al),
        op_entry!(MASK_OUT_X,   OP_ADD_8_ER_PCDI, add_8_er_pcdi),
        op_entry!(MASK_OUT_X,   OP_ADD_8_ER_PCIX, add_8_er_pcix),
        op_entry!(MASK_OUT_X,   OP_ADD_8_ER_IMM,  add_8_er_imm),

        op_entry!(MASK_OUT_X_Y, OP_ADD_8_RE_AI,   add_8_re_ai),
        op_entry!(MASK_OUT_X_Y, OP_ADD_8_RE_PI,   add_8_re_pi),
        op_entry!(MASK_OUT_X_Y, OP_ADD_8_RE_PD,   add_8_re_pd),
        op_entry!(MASK_OUT_X_Y, OP_ADD_8_RE_DI,   add_8_re_di),
        op_entry!(MASK_OUT_X_Y, OP_ADD_8_RE_IX,   add_8_re_ix),
        op_entry!(MASK_OUT_X,   OP_ADD_8_RE_AW,   add_8_re_aw),
        op_entry!(MASK_OUT_X,   OP_ADD_8_RE_AL,   add_8_re_al),

        op_entry!(MASK_OUT_X_Y, OP_ADD_16_ER_DN,   add_16_er_dn),
        op_entry!(MASK_OUT_X_Y, OP_ADD_16_ER_AN,   add_16_er_an),
        op_entry!(MASK_OUT_X_Y, OP_ADD_16_ER_AI,   add_16_er_ai),
        op_entry!(MASK_OUT_X_Y, OP_ADD_16_ER_PI,   add_16_er_pi),
        op_entry!(MASK_OUT_X_Y, OP_ADD_16_ER_PD,   add_16_er_pd),
        op_entry!(MASK_OUT_X_Y, OP_ADD_16_ER_DI,   add_16_er_di),
        op_entry!(MASK_OUT_X_Y, OP_ADD_16_ER_IX,   add_16_er_ix),
        op_entry!(MASK_OUT_X,   OP_ADD_16_ER_AW,   add_16_er_aw),
        op_entry!(MASK_OUT_X,   OP_ADD_16_ER_AL,   add_16_er_al),
        op_entry!(MASK_OUT_X,   OP_ADD_16_ER_PCDI, add_16_er_pcdi),
        op_entry!(MASK_OUT_X,   OP_ADD_16_ER_PCIX, add_16_er_pcix),
        op_entry!(MASK_OUT_X,   OP_ADD_16_ER_IMM,  add_16_er_imm),

        op_entry!(MASK_OUT_X_Y, OP_ADD_16_RE_AI,   add_16_re_ai),
        op_entry!(MASK_OUT_X_Y, OP_ADD_16_RE_PI,   add_16_re_pi),
        op_entry!(MASK_OUT_X_Y, OP_ADD_16_RE_PD,   add_16_re_pd),
        op_entry!(MASK_OUT_X_Y, OP_ADD_16_RE_DI,   add_16_re_di),
        op_entry!(MASK_OUT_X_Y, OP_ADD_16_RE_IX,   add_16_re_ix),
        op_entry!(MASK_OUT_X,   OP_ADD_16_RE_AW,   add_16_re_aw),
        op_entry!(MASK_OUT_X,   OP_ADD_16_RE_AL,   add_16_re_al),

        op_entry!(MASK_OUT_X_Y, OP_ADD_32_ER_DN,   add_32_er_dn),
        op_entry!(MASK_OUT_X_Y, OP_ADD_32_ER_AN,   add_32_er_an),
        op_entry!(MASK_OUT_X_Y, OP_ADD_32_ER_AI,   add_32_er_ai),
        op_entry!(MASK_OUT_X_Y, OP_ADD_32_ER_PI,   add_32_er_pi),
        op_entry!(MASK_OUT_X_Y, OP_ADD_32_ER_PD,   add_32_er_pd),
        op_entry!(MASK_OUT_X_Y, OP_ADD_32_ER_DI,   add_32_er_di),
        op_entry!(MASK_OUT_X_Y, OP_ADD_32_ER_IX,   add_32_er_ix),
        op_entry!(MASK_OUT_X,   OP_ADD_32_ER_AW,   add_32_er_aw),
        op_entry!(MASK_OUT_X,   OP_ADD_32_ER_AL,   add_32_er_al),
        op_entry!(MASK_OUT_X,   OP_ADD_32_ER_PCDI, add_32_er_pcdi),
        op_entry!(MASK_OUT_X,   OP_ADD_32_ER_PCIX, add_32_er_pcix),
        op_entry!(MASK_OUT_X,   OP_ADD_32_ER_IMM,  add_32_er_imm),

        op_entry!(MASK_OUT_X_Y, OP_ADD_32_RE_AI,   add_32_re_ai),
        op_entry!(MASK_OUT_X_Y, OP_ADD_32_RE_PI,   add_32_re_pi),
        op_entry!(MASK_OUT_X_Y, OP_ADD_32_RE_PD,   add_32_re_pd),
        op_entry!(MASK_OUT_X_Y, OP_ADD_32_RE_DI,   add_32_re_di),
        op_entry!(MASK_OUT_X_Y, OP_ADD_32_RE_IX,   add_32_re_ix),
        op_entry!(MASK_OUT_X,   OP_ADD_32_RE_AW,   add_32_re_aw),
        op_entry!(MASK_OUT_X,   OP_ADD_32_RE_AL,   add_32_re_al),

        op_entry!(MASK_OUT_X_Y, OP_ADDA_16_DN,   adda_16_dn),
        op_entry!(MASK_OUT_X_Y, OP_ADDA_16_AN,   adda_16_an),
        op_entry!(MASK_OUT_X_Y, OP_ADDA_16_AI,   adda_16_ai),
        op_entry!(MASK_OUT_X_Y, OP_ADDA_16_PI,   adda_16_pi),
        op_entry!(MASK_OUT_X_Y, OP_ADDA_16_PD,   adda_16_pd),
        op_entry!(MASK_OUT_X_Y, OP_ADDA_16_DI,   adda_16_di),
        op_entry!(MASK_OUT_X_Y, OP_ADDA_16_IX,   adda_16_ix),
        op_entry!(MASK_OUT_X,   OP_ADDA_16_AW,   adda_16_aw),
        op_entry!(MASK_OUT_X,   OP_ADDA_16_AL,   adda_16_al),
        op_entry!(MASK_OUT_X,   OP_ADDA_16_PCDI, adda_16_pcdi),
        op_entry!(MASK_OUT_X,   OP_ADDA_16_PCIX, adda_16_pcix),
        op_entry!(MASK_OUT_X,   OP_ADDA_16_IMM,  adda_16_imm),

        op_entry!(MASK_OUT_X_Y, OP_ADDA_32_DN,   adda_32_dn),
        op_entry!(MASK_OUT_X_Y, OP_ADDA_32_AN,   adda_32_an),
        op_entry!(MASK_OUT_X_Y, OP_ADDA_32_AI,   adda_32_ai),
        op_entry!(MASK_OUT_X_Y, OP_ADDA_32_PI,   adda_32_pi),
        op_entry!(MASK_OUT_X_Y, OP_ADDA_32_PD,   adda_32_pd),
        op_entry!(MASK_OUT_X_Y, OP_ADDA_32_DI,   adda_32_di),
        op_entry!(MASK_OUT_X_Y, OP_ADDA_32_IX,   adda_32_ix),
        op_entry!(MASK_OUT_X,   OP_ADDA_32_AW,   adda_32_aw),
        op_entry!(MASK_OUT_X,   OP_ADDA_32_AL,   adda_32_al),
        op_entry!(MASK_OUT_X,   OP_ADDA_32_PCDI, adda_32_pcdi),
        op_entry!(MASK_OUT_X,   OP_ADDA_32_PCIX, adda_32_pcix),
        op_entry!(MASK_OUT_X,   OP_ADDA_32_IMM,  adda_32_imm),

        op_entry!(MASK_OUT_Y, OP_ADDI_8_DN,   addi_8_dn),
        op_entry!(MASK_OUT_Y, OP_ADDI_8_AI,   addi_8_ai),
        op_entry!(MASK_OUT_Y, OP_ADDI_8_PI,   addi_8_pi),
        op_entry!(MASK_OUT_Y, OP_ADDI_8_PD,   addi_8_pd),
        op_entry!(MASK_OUT_Y, OP_ADDI_8_DI,   addi_8_di),
        op_entry!(MASK_OUT_Y, OP_ADDI_8_IX,   addi_8_ix),
        op_entry!(MASK_EXACT, OP_ADDI_8_AW,   addi_8_aw),
        op_entry!(MASK_EXACT, OP_ADDI_8_AL,   addi_8_al),

        op_entry!(MASK_OUT_Y, OP_ADDI_16_DN,   addi_16_dn),
        op_entry!(MASK_OUT_Y, OP_ADDI_16_AI,   addi_16_ai),
        op_entry!(MASK_OUT_Y, OP_ADDI_16_PI,   addi_16_pi),
        op_entry!(MASK_OUT_Y, OP_ADDI_16_PD,   addi_16_pd),
        op_entry!(MASK_OUT_Y, OP_ADDI_16_DI,   addi_16_di),
        op_entry!(MASK_OUT_Y, OP_ADDI_16_IX,   addi_16_ix),
        op_entry!(MASK_EXACT, OP_ADDI_16_AW,   addi_16_aw),
        op_entry!(MASK_EXACT, OP_ADDI_16_AL,   addi_16_al),

        op_entry!(MASK_OUT_Y, OP_ADDI_32_DN,   addi_32_dn),
        op_entry!(MASK_OUT_Y, OP_ADDI_32_AI,   addi_32_ai),
        op_entry!(MASK_OUT_Y, OP_ADDI_32_PI,   addi_32_pi),
        op_entry!(MASK_OUT_Y, OP_ADDI_32_PD,   addi_32_pd),
        op_entry!(MASK_OUT_Y, OP_ADDI_32_DI,   addi_32_di),
        op_entry!(MASK_OUT_Y, OP_ADDI_32_IX,   addi_32_ix),
        op_entry!(MASK_EXACT, OP_ADDI_32_AW,   addi_32_aw),
        op_entry!(MASK_EXACT, OP_ADDI_32_AL,   addi_32_al),

        op_entry!(MASK_OUT_X_Y, OP_ADDQ_8_DN, addq_8_dn),
        op_entry!(MASK_OUT_X_Y, OP_ADDQ_8_AI, addq_8_ai),
        op_entry!(MASK_OUT_X_Y, OP_ADDQ_8_PI, addq_8_pi),
        op_entry!(MASK_OUT_X_Y, OP_ADDQ_8_PD, addq_8_pd),
        op_entry!(MASK_OUT_X_Y, OP_ADDQ_8_DI, addq_8_di),
        op_entry!(MASK_OUT_X_Y, OP_ADDQ_8_IX, addq_8_ix),
        op_entry!(MASK_OUT_X,   OP_ADDQ_8_AW, addq_8_aw),
        op_entry!(MASK_OUT_X,   OP_ADDQ_8_AL, addq_8_al),

        op_entry!(MASK_OUT_X_Y, OP_ADDQ_16_DN, addq_16_dn),
        op_entry!(MASK_OUT_X_Y, OP_ADDQ_16_AN, addq_16_an),
        op_entry!(MASK_OUT_X_Y, OP_ADDQ_16_AI, addq_16_ai),
        op_entry!(MASK_OUT_X_Y, OP_ADDQ_16_PI, addq_16_pi),
        op_entry!(MASK_OUT_X_Y, OP_ADDQ_16_PD, addq_16_pd),
        op_entry!(MASK_OUT_X_Y, OP_ADDQ_16_DI, addq_16_di),
        op_entry!(MASK_OUT_X_Y, OP_ADDQ_16_IX, addq_16_ix),
        op_entry!(MASK_OUT_X,   OP_ADDQ_16_AW, addq_16_aw),
        op_entry!(MASK_OUT_X,   OP_ADDQ_16_AL, addq_16_al),

        op_entry!(MASK_OUT_X_Y, OP_ADDQ_32_DN, addq_32_dn),
        op_entry!(MASK_OUT_X_Y, OP_ADDQ_32_AN, addq_32_an),
        op_entry!(MASK_OUT_X_Y, OP_ADDQ_32_AI, addq_32_ai),
        op_entry!(MASK_OUT_X_Y, OP_ADDQ_32_PI, addq_32_pi),
        op_entry!(MASK_OUT_X_Y, OP_ADDQ_32_PD, addq_32_pd),
        op_entry!(MASK_OUT_X_Y, OP_ADDQ_32_DI, addq_32_di),
        op_entry!(MASK_OUT_X_Y, OP_ADDQ_32_IX, addq_32_ix),
        op_entry!(MASK_OUT_X,   OP_ADDQ_32_AW, addq_32_aw),
        op_entry!(MASK_OUT_X,   OP_ADDQ_32_AL, addq_32_al),

        op_entry!(MASK_OUT_X_Y, OP_ADDX_8_RR,  addx_8_rr),
        op_entry!(MASK_OUT_X_Y, OP_ADDX_8_MM,  addx_8_mm),
        op_entry!(MASK_OUT_X_Y, OP_ADDX_16_RR, addx_16_rr),
        op_entry!(MASK_OUT_X_Y, OP_ADDX_16_MM, addx_16_mm),
        op_entry!(MASK_OUT_X_Y, OP_ADDX_32_RR, addx_32_rr),
        op_entry!(MASK_OUT_X_Y, OP_ADDX_32_MM, addx_32_mm),

        op_entry!(MASK_OUT_X_Y, OP_AND_8_ER_DN,   and_8_er_dn),
        op_entry!(MASK_OUT_X_Y, OP_AND_8_ER_AI,   and_8_er_ai),
        op_entry!(MASK_OUT_X_Y, OP_AND_8_ER_PI,   and_8_er_pi),
        op_entry!(MASK_OUT_X_Y, OP_AND_8_ER_PD,   and_8_er_pd),
        op_entry!(MASK_OUT_X_Y, OP_AND_8_ER_DI,   and_8_er_di),
        op_entry!(MASK_OUT_X_Y, OP_AND_8_ER_IX,   and_8_er_ix),
        op_entry!(MASK_OUT_X,   OP_AND_8_ER_AW,   and_8_er_aw),
        op_entry!(MASK_OUT_X,   OP_AND_8_ER_AL,   and_8_er_al),
        op_entry!(MASK_OUT_X,   OP_AND_8_ER_PCDI, and_8_er_pcdi),
        op_entry!(MASK_OUT_X,   OP_AND_8_ER_PCIX, and_8_er_pcix),
        op_entry!(MASK_OUT_X,   OP_AND_8_ER_IMM,  and_8_er_imm),

        op_entry!(MASK_OUT_X_Y, OP_AND_8_RE_AI,   and_8_re_ai),
        op_entry!(MASK_OUT_X_Y, OP_AND_8_RE_PI,   and_8_re_pi),
        op_entry!(MASK_OUT_X_Y, OP_AND_8_RE_PD,   and_8_re_pd),
        op_entry!(MASK_OUT_X_Y, OP_AND_8_RE_DI,   and_8_re_di),
        op_entry!(MASK_OUT_X_Y, OP_AND_8_RE_IX,   and_8_re_ix),
        op_entry!(MASK_OUT_X,   OP_AND_8_RE_AW,   and_8_re_aw),
        op_entry!(MASK_OUT_X,   OP_AND_8_RE_AL,   and_8_re_al),

        op_entry!(MASK_OUT_X_Y, OP_AND_16_ER_DN,   and_16_er_dn),
        op_entry!(MASK_OUT_X_Y, OP_AND_16_ER_AI,   and_16_er_ai),
        op_entry!(MASK_OUT_X_Y, OP_AND_16_ER_PI,   and_16_er_pi),
        op_entry!(MASK_OUT_X_Y, OP_AND_16_ER_PD,   and_16_er_pd),
        op_entry!(MASK_OUT_X_Y, OP_AND_16_ER_DI,   and_16_er_di),
        op_entry!(MASK_OUT_X_Y, OP_AND_16_ER_IX,   and_16_er_ix),
        op_entry!(MASK_OUT_X,   OP_AND_16_ER_AW,   and_16_er_aw),
        op_entry!(MASK_OUT_X,   OP_AND_16_ER_AL,   and_16_er_al),
        op_entry!(MASK_OUT_X,   OP_AND_16_ER_PCDI, and_16_er_pcdi),
        op_entry!(MASK_OUT_X,   OP_AND_16_ER_PCIX, and_16_er_pcix),
        op_entry!(MASK_OUT_X,   OP_AND_16_ER_IMM,  and_16_er_imm),

        op_entry!(MASK_OUT_X_Y, OP_AND_16_RE_AI,   and_16_re_ai),
        op_entry!(MASK_OUT_X_Y, OP_AND_16_RE_PI,   and_16_re_pi),
        op_entry!(MASK_OUT_X_Y, OP_AND_16_RE_PD,   and_16_re_pd),
        op_entry!(MASK_OUT_X_Y, OP_AND_16_RE_DI,   and_16_re_di),
        op_entry!(MASK_OUT_X_Y, OP_AND_16_RE_IX,   and_16_re_ix),
        op_entry!(MASK_OUT_X,   OP_AND_16_RE_AW,   and_16_re_aw),
        op_entry!(MASK_OUT_X,   OP_AND_16_RE_AL,   and_16_re_al),

        op_entry!(MASK_OUT_X_Y, OP_AND_32_ER_DN,   and_32_er_dn),
        op_entry!(MASK_OUT_X_Y, OP_AND_32_ER_AI,   and_32_er_ai),
        op_entry!(MASK_OUT_X_Y, OP_AND_32_ER_PI,   and_32_er_pi),
        op_entry!(MASK_OUT_X_Y, OP_AND_32_ER_PD,   and_32_er_pd),
        op_entry!(MASK_OUT_X_Y, OP_AND_32_ER_DI,   and_32_er_di),
        op_entry!(MASK_OUT_X_Y, OP_AND_32_ER_IX,   and_32_er_ix),
        op_entry!(MASK_OUT_X,   OP_AND_32_ER_AW,   and_32_er_aw),
        op_entry!(MASK_OUT_X,   OP_AND_32_ER_AL,   and_32_er_al),
        op_entry!(MASK_OUT_X,   OP_AND_32_ER_PCDI, and_32_er_pcdi),
        op_entry!(MASK_OUT_X,   OP_AND_32_ER_PCIX, and_32_er_pcix),
        op_entry!(MASK_OUT_X,   OP_AND_32_ER_IMM,  and_32_er_imm),

        op_entry!(MASK_OUT_X_Y, OP_AND_32_RE_AI,   and_32_re_ai),
        op_entry!(MASK_OUT_X_Y, OP_AND_32_RE_PI,   and_32_re_pi),
        op_entry!(MASK_OUT_X_Y, OP_AND_32_RE_PD,   and_32_re_pd),
        op_entry!(MASK_OUT_X_Y, OP_AND_32_RE_DI,   and_32_re_di),
        op_entry!(MASK_OUT_X_Y, OP_AND_32_RE_IX,   and_32_re_ix),
        op_entry!(MASK_OUT_X,   OP_AND_32_RE_AW,   and_32_re_aw),
        op_entry!(MASK_OUT_X,   OP_AND_32_RE_AL,   and_32_re_al),

        op_entry!(MASK_OUT_Y, OP_ANDI_8_DN,   andi_8_dn),
        op_entry!(MASK_OUT_Y, OP_ANDI_8_AI,   andi_8_ai),
        op_entry!(MASK_OUT_Y, OP_ANDI_8_PI,   andi_8_pi),
        op_entry!(MASK_OUT_Y, OP_ANDI_8_PD,   andi_8_pd),
        op_entry!(MASK_OUT_Y, OP_ANDI_8_DI,   andi_8_di),
        op_entry!(MASK_OUT_Y, OP_ANDI_8_IX,   andi_8_ix),
        op_entry!(MASK_EXACT, OP_ANDI_8_AW,   andi_8_aw),
        op_entry!(MASK_EXACT, OP_ANDI_8_AL,   andi_8_al),

        op_entry!(MASK_OUT_Y, OP_ANDI_16_DN,   andi_16_dn),
        op_entry!(MASK_OUT_Y, OP_ANDI_16_AI,   andi_16_ai),
        op_entry!(MASK_OUT_Y, OP_ANDI_16_PI,   andi_16_pi),
        op_entry!(MASK_OUT_Y, OP_ANDI_16_PD,   andi_16_pd),
        op_entry!(MASK_OUT_Y, OP_ANDI_16_DI,   andi_16_di),
        op_entry!(MASK_OUT_Y, OP_ANDI_16_IX,   andi_16_ix),
        op_entry!(MASK_EXACT, OP_ANDI_16_AW,   andi_16_aw),
        op_entry!(MASK_EXACT, OP_ANDI_16_AL,   andi_16_al),

        op_entry!(MASK_OUT_Y, OP_ANDI_32_DN,   andi_32_dn),
        op_entry!(MASK_OUT_Y, OP_ANDI_32_AI,   andi_32_ai),
        op_entry!(MASK_OUT_Y, OP_ANDI_32_PI,   andi_32_pi),
        op_entry!(MASK_OUT_Y, OP_ANDI_32_PD,   andi_32_pd),
        op_entry!(MASK_OUT_Y, OP_ANDI_32_DI,   andi_32_di),
        op_entry!(MASK_OUT_Y, OP_ANDI_32_IX,   andi_32_ix),
        op_entry!(MASK_EXACT, OP_ANDI_32_AW,   andi_32_aw),
        op_entry!(MASK_EXACT, OP_ANDI_32_AL,   andi_32_al),

        op_entry!(MASK_EXACT, OP_ANDI_16_TOC,   andi_16_toc),
        op_entry!(MASK_EXACT, OP_ANDI_16_TOS,   andi_16_tos),

        op_entry!(MASK_OUT_X_Y, OP_ASL_8_R  , asl_8_r),
        op_entry!(MASK_OUT_X_Y, OP_ASL_8_S  , asl_8_s),
        op_entry!(MASK_OUT_X_Y, OP_ASL_16_R , asl_16_r),
        op_entry!(MASK_OUT_X_Y, OP_ASL_16_S , asl_16_s),
        op_entry!(MASK_OUT_X_Y, OP_ASL_32_R , asl_32_r),
        op_entry!(MASK_OUT_X_Y, OP_ASL_32_S , asl_32_s),

        op_entry!(MASK_OUT_X_Y, OP_ASR_8_R  , asr_8_r),
        op_entry!(MASK_OUT_X_Y, OP_ASR_8_S  , asr_8_s),
        op_entry!(MASK_OUT_X_Y, OP_ASR_16_R , asr_16_r),
        op_entry!(MASK_OUT_X_Y, OP_ASR_16_S , asr_16_s),
        op_entry!(MASK_OUT_X_Y, OP_ASR_32_R , asr_32_r),
        op_entry!(MASK_OUT_X_Y, OP_ASR_32_S , asr_32_s),

        op_entry!(MASK_OUT_Y, OP_ASL_16_AI, asl_16_ai),
        op_entry!(MASK_OUT_Y, OP_ASL_16_PI, asl_16_pi),
        op_entry!(MASK_OUT_Y, OP_ASL_16_PD, asl_16_pd),
        op_entry!(MASK_OUT_Y, OP_ASL_16_DI, asl_16_di),
        op_entry!(MASK_OUT_Y, OP_ASL_16_IX, asl_16_ix),
        op_entry!(MASK_EXACT, OP_ASL_16_AW, asl_16_aw),
        op_entry!(MASK_EXACT, OP_ASL_16_AL, asl_16_al),

        op_entry!(MASK_OUT_Y, OP_ASR_16_AI, asr_16_ai),
        op_entry!(MASK_OUT_Y, OP_ASR_16_PI, asr_16_pi),
        op_entry!(MASK_OUT_Y, OP_ASR_16_PD, asr_16_pd),
        op_entry!(MASK_OUT_Y, OP_ASR_16_DI, asr_16_di),
        op_entry!(MASK_OUT_Y, OP_ASR_16_IX, asr_16_ix),
        op_entry!(MASK_EXACT, OP_ASR_16_AW, asr_16_aw),
        op_entry!(MASK_EXACT, OP_ASR_16_AL, asr_16_al),

        op_entry!(MASK_LOBYTE, OP_BHI_8, bhi_8),
        op_entry!(MASK_LOBYTE, OP_BLS_8, bls_8),
        op_entry!(MASK_LOBYTE, OP_BCC_8, bcc_8),
        op_entry!(MASK_LOBYTE, OP_BCS_8, bcs_8),
        op_entry!(MASK_LOBYTE, OP_BNE_8, bne_8),
        op_entry!(MASK_LOBYTE, OP_BEQ_8, beq_8),
        op_entry!(MASK_LOBYTE, OP_BVC_8, bvc_8),
        op_entry!(MASK_LOBYTE, OP_BVS_8, bvs_8),
        op_entry!(MASK_LOBYTE, OP_BPL_8, bpl_8),
        op_entry!(MASK_LOBYTE, OP_BMI_8, bmi_8),
        op_entry!(MASK_LOBYTE, OP_BGE_8, bge_8),
        op_entry!(MASK_LOBYTE, OP_BLT_8, blt_8),
        op_entry!(MASK_LOBYTE, OP_BGT_8, bgt_8),
        op_entry!(MASK_LOBYTE, OP_BLE_8, ble_8),
        op_entry!(MASK_LOBYTE, OP_BRA_8, bra_8),
        op_entry!(MASK_LOBYTE, OP_BSR_8, bsr_8),

        op_entry!(MASK_EXACT, OP_BHI_16, bhi_16),
        op_entry!(MASK_EXACT, OP_BLS_16, bls_16),
        op_entry!(MASK_EXACT, OP_BCC_16, bcc_16),
        op_entry!(MASK_EXACT, OP_BCS_16, bcs_16),
        op_entry!(MASK_EXACT, OP_BNE_16, bne_16),
        op_entry!(MASK_EXACT, OP_BEQ_16, beq_16),
        op_entry!(MASK_EXACT, OP_BVC_16, bvc_16),
        op_entry!(MASK_EXACT, OP_BVS_16, bvs_16),
        op_entry!(MASK_EXACT, OP_BPL_16, bpl_16),
        op_entry!(MASK_EXACT, OP_BMI_16, bmi_16),
        op_entry!(MASK_EXACT, OP_BGE_16, bge_16),
        op_entry!(MASK_EXACT, OP_BLT_16, blt_16),
        op_entry!(MASK_EXACT, OP_BGT_16, bgt_16),
        op_entry!(MASK_EXACT, OP_BLE_16, ble_16),
        op_entry!(MASK_EXACT, OP_BRA_16, bra_16),
        op_entry!(MASK_EXACT, OP_BSR_16, bsr_16),

        op_entry!(MASK_OUT_X_Y, OP_BCHG_32_R_DN,bchg_32_r_dn),
        op_entry!(MASK_OUT_Y,   OP_BCHG_32_S_DN,bchg_32_s_dn),
        op_entry!(MASK_OUT_X_Y, OP_BCHG_8_R_AI, bchg_8_r_ai),
        op_entry!(MASK_OUT_X_Y, OP_BCHG_8_R_PI, bchg_8_r_pi),
        op_entry!(MASK_OUT_X_Y, OP_BCHG_8_R_PD, bchg_8_r_pd),
        op_entry!(MASK_OUT_X_Y, OP_BCHG_8_R_DI, bchg_8_r_di),
        op_entry!(MASK_OUT_X_Y, OP_BCHG_8_R_IX, bchg_8_r_ix),
        op_entry!(MASK_OUT_X,   OP_BCHG_8_R_AW, bchg_8_r_aw),
        op_entry!(MASK_OUT_X,   OP_BCHG_8_R_AL, bchg_8_r_al),
        op_entry!(MASK_OUT_Y,   OP_BCHG_8_S_AI, bchg_8_s_ai),
        op_entry!(MASK_OUT_Y,   OP_BCHG_8_S_PI, bchg_8_s_pi),
        op_entry!(MASK_OUT_Y,   OP_BCHG_8_S_PD, bchg_8_s_pd),
        op_entry!(MASK_OUT_Y,   OP_BCHG_8_S_DI, bchg_8_s_di),
        op_entry!(MASK_OUT_Y,   OP_BCHG_8_S_IX, bchg_8_s_ix),
        op_entry!(MASK_EXACT,   OP_BCHG_8_S_AW, bchg_8_s_aw),
        op_entry!(MASK_EXACT,   OP_BCHG_8_S_AL, bchg_8_s_al),

        op_entry!(MASK_OUT_X_Y, OP_BCLR_32_R_DN,bclr_32_r_dn),
        op_entry!(MASK_OUT_Y,   OP_BCLR_32_S_DN,bclr_32_s_dn),
        op_entry!(MASK_OUT_X_Y, OP_BCLR_8_R_AI, bclr_8_r_ai),
        op_entry!(MASK_OUT_X_Y, OP_BCLR_8_R_PI, bclr_8_r_pi),
        op_entry!(MASK_OUT_X_Y, OP_BCLR_8_R_PD, bclr_8_r_pd),
        op_entry!(MASK_OUT_X_Y, OP_BCLR_8_R_DI, bclr_8_r_di),
        op_entry!(MASK_OUT_X_Y, OP_BCLR_8_R_IX, bclr_8_r_ix),
        op_entry!(MASK_OUT_X,   OP_BCLR_8_R_AW, bclr_8_r_aw),
        op_entry!(MASK_OUT_X,   OP_BCLR_8_R_AL, bclr_8_r_al),
        op_entry!(MASK_OUT_Y,   OP_BCLR_8_S_AI, bclr_8_s_ai),
        op_entry!(MASK_OUT_Y,   OP_BCLR_8_S_PI, bclr_8_s_pi),
        op_entry!(MASK_OUT_Y,   OP_BCLR_8_S_PD, bclr_8_s_pd),
        op_entry!(MASK_OUT_Y,   OP_BCLR_8_S_DI, bclr_8_s_di),
        op_entry!(MASK_OUT_Y,   OP_BCLR_8_S_IX, bclr_8_s_ix),
        op_entry!(MASK_EXACT,   OP_BCLR_8_S_AW, bclr_8_s_aw),
        op_entry!(MASK_EXACT,   OP_BCLR_8_S_AL, bclr_8_s_al),

        op_entry!(MASK_OUT_X_Y, OP_BSET_32_R_DN,bset_32_r_dn),
        op_entry!(MASK_OUT_Y,   OP_BSET_32_S_DN,bset_32_s_dn),
        op_entry!(MASK_OUT_X_Y, OP_BSET_8_R_AI, bset_8_r_ai),
        op_entry!(MASK_OUT_X_Y, OP_BSET_8_R_PI, bset_8_r_pi),
        op_entry!(MASK_OUT_X_Y, OP_BSET_8_R_PD, bset_8_r_pd),
        op_entry!(MASK_OUT_X_Y, OP_BSET_8_R_DI, bset_8_r_di),
        op_entry!(MASK_OUT_X_Y, OP_BSET_8_R_IX, bset_8_r_ix),
        op_entry!(MASK_OUT_X,   OP_BSET_8_R_AW, bset_8_r_aw),
        op_entry!(MASK_OUT_X,   OP_BSET_8_R_AL, bset_8_r_al),
        op_entry!(MASK_OUT_Y,   OP_BSET_8_S_AI, bset_8_s_ai),
        op_entry!(MASK_OUT_Y,   OP_BSET_8_S_PI, bset_8_s_pi),
        op_entry!(MASK_OUT_Y,   OP_BSET_8_S_PD, bset_8_s_pd),
        op_entry!(MASK_OUT_Y,   OP_BSET_8_S_DI, bset_8_s_di),
        op_entry!(MASK_OUT_Y,   OP_BSET_8_S_IX, bset_8_s_ix),
        op_entry!(MASK_EXACT,   OP_BSET_8_S_AW, bset_8_s_aw),
        op_entry!(MASK_EXACT,   OP_BSET_8_S_AL, bset_8_s_al),

        op_entry!(MASK_OUT_X_Y, OP_BTST_32_R_DN,btst_32_r_dn),
        op_entry!(MASK_OUT_Y,   OP_BTST_32_S_DN,btst_32_s_dn),
        op_entry!(MASK_OUT_X_Y, OP_BTST_8_R_AI, btst_8_r_ai),
        op_entry!(MASK_OUT_X_Y, OP_BTST_8_R_PI, btst_8_r_pi),
        op_entry!(MASK_OUT_X_Y, OP_BTST_8_R_PD, btst_8_r_pd),
        op_entry!(MASK_OUT_X_Y, OP_BTST_8_R_DI, btst_8_r_di),
        op_entry!(MASK_OUT_X_Y, OP_BTST_8_R_IX, btst_8_r_ix),
        op_entry!(MASK_OUT_X,   OP_BTST_8_R_AW, btst_8_r_aw),
        op_entry!(MASK_OUT_X,   OP_BTST_8_R_AL, btst_8_r_al),
        op_entry!(MASK_OUT_Y,   OP_BTST_8_S_AI, btst_8_s_ai),
        op_entry!(MASK_OUT_Y,   OP_BTST_8_S_PI, btst_8_s_pi),
        op_entry!(MASK_OUT_Y,   OP_BTST_8_S_PD, btst_8_s_pd),
        op_entry!(MASK_OUT_Y,   OP_BTST_8_S_DI, btst_8_s_di),
        op_entry!(MASK_OUT_Y,   OP_BTST_8_S_IX, btst_8_s_ix),
        op_entry!(MASK_EXACT,   OP_BTST_8_S_AW, btst_8_s_aw),
        op_entry!(MASK_EXACT,   OP_BTST_8_S_AL, btst_8_s_al),

        op_entry!(MASK_OUT_X_Y, OP_CHK_16_AI,   chk_16_ai),
        op_entry!(MASK_OUT_X,   OP_CHK_16_AL,   chk_16_al),
        op_entry!(MASK_OUT_X,   OP_CHK_16_AW,   chk_16_aw),
        op_entry!(MASK_OUT_X_Y, OP_CHK_16_DN,   chk_16_dn),
        op_entry!(MASK_OUT_X_Y, OP_CHK_16_DI,   chk_16_di),
        op_entry!(MASK_OUT_X,   OP_CHK_16_IMM,  chk_16_imm),
        op_entry!(MASK_OUT_X_Y, OP_CHK_16_IX,   chk_16_ix),
        op_entry!(MASK_OUT_X,   OP_CHK_16_PCDI, chk_16_pcdi),
        op_entry!(MASK_OUT_X,   OP_CHK_16_PCIX, chk_16_pcix),
        op_entry!(MASK_OUT_X_Y, OP_CHK_16_PD,   chk_16_pd),
        op_entry!(MASK_OUT_X_Y, OP_CHK_16_PI,   chk_16_pi),

        op_entry!(MASK_OUT_Y, OP_CLR_8_DN, clr_8_dn),
        op_entry!(MASK_OUT_Y, OP_CLR_8_AI, clr_8_ai),
        op_entry!(MASK_OUT_Y, OP_CLR_8_PI, clr_8_pi),
        op_entry!(MASK_OUT_Y, OP_CLR_8_PD, clr_8_pd),
        op_entry!(MASK_OUT_Y, OP_CLR_8_DI, clr_8_di),
        op_entry!(MASK_OUT_Y, OP_CLR_8_IX, clr_8_ix),
        op_entry!(MASK_EXACT, OP_CLR_8_AW, clr_8_aw),
        op_entry!(MASK_EXACT, OP_CLR_8_AL, clr_8_al),

        op_entry!(MASK_OUT_Y, OP_CLR_16_DN, clr_16_dn),
        op_entry!(MASK_OUT_Y, OP_CLR_16_AI, clr_16_ai),
        op_entry!(MASK_OUT_Y, OP_CLR_16_PI, clr_16_pi),
        op_entry!(MASK_OUT_Y, OP_CLR_16_PD, clr_16_pd),
        op_entry!(MASK_OUT_Y, OP_CLR_16_DI, clr_16_di),
        op_entry!(MASK_OUT_Y, OP_CLR_16_IX, clr_16_ix),
        op_entry!(MASK_EXACT, OP_CLR_16_AW, clr_16_aw),
        op_entry!(MASK_EXACT, OP_CLR_16_AL, clr_16_al),

        op_entry!(MASK_OUT_Y, OP_CLR_32_DN, clr_32_dn),
        op_entry!(MASK_OUT_Y, OP_CLR_32_AI, clr_32_ai),
        op_entry!(MASK_OUT_Y, OP_CLR_32_PI, clr_32_pi),
        op_entry!(MASK_OUT_Y, OP_CLR_32_PD, clr_32_pd),
        op_entry!(MASK_OUT_Y, OP_CLR_32_DI, clr_32_di),
        op_entry!(MASK_OUT_Y, OP_CLR_32_IX, clr_32_ix),
        op_entry!(MASK_EXACT, OP_CLR_32_AW, clr_32_aw),
        op_entry!(MASK_EXACT, OP_CLR_32_AL, clr_32_al),

        op_entry!(MASK_OUT_X_Y, OP_CMP_8_DN,   cmp_8_dn),
        op_entry!(MASK_OUT_X_Y, OP_CMP_8_AI,   cmp_8_ai),
        op_entry!(MASK_OUT_X_Y, OP_CMP_8_PI,   cmp_8_pi),
        op_entry!(MASK_OUT_X_Y, OP_CMP_8_PD,   cmp_8_pd),
        op_entry!(MASK_OUT_X_Y, OP_CMP_8_DI,   cmp_8_di),
        op_entry!(MASK_OUT_X_Y, OP_CMP_8_IX,   cmp_8_ix),
        op_entry!(MASK_OUT_X,   OP_CMP_8_AW,   cmp_8_aw),
        op_entry!(MASK_OUT_X,   OP_CMP_8_AL,   cmp_8_al),
        op_entry!(MASK_OUT_X,   OP_CMP_8_PCDI, cmp_8_pcdi),
        op_entry!(MASK_OUT_X,   OP_CMP_8_PCIX, cmp_8_pcix),
        op_entry!(MASK_OUT_X,   OP_CMP_8_IMM,  cmp_8_imm),

        op_entry!(MASK_OUT_X_Y, OP_CMP_16_DN,   cmp_16_dn),
        op_entry!(MASK_OUT_X_Y, OP_CMP_16_AN,   cmp_16_an),
        op_entry!(MASK_OUT_X_Y, OP_CMP_16_AI,   cmp_16_ai),
        op_entry!(MASK_OUT_X_Y, OP_CMP_16_PI,   cmp_16_pi),
        op_entry!(MASK_OUT_X_Y, OP_CMP_16_PD,   cmp_16_pd),
        op_entry!(MASK_OUT_X_Y, OP_CMP_16_DI,   cmp_16_di),
        op_entry!(MASK_OUT_X_Y, OP_CMP_16_IX,   cmp_16_ix),
        op_entry!(MASK_OUT_X,   OP_CMP_16_AW,   cmp_16_aw),
        op_entry!(MASK_OUT_X,   OP_CMP_16_AL,   cmp_16_al),
        op_entry!(MASK_OUT_X,   OP_CMP_16_PCDI, cmp_16_pcdi),
        op_entry!(MASK_OUT_X,   OP_CMP_16_PCIX, cmp_16_pcix),
        op_entry!(MASK_OUT_X,   OP_CMP_16_IMM,  cmp_16_imm),

        op_entry!(MASK_OUT_X_Y, OP_CMP_32_DN,   cmp_32_dn),
        op_entry!(MASK_OUT_X_Y, OP_CMP_32_AN,   cmp_32_an),
        op_entry!(MASK_OUT_X_Y, OP_CMP_32_AI,   cmp_32_ai),
        op_entry!(MASK_OUT_X_Y, OP_CMP_32_PI,   cmp_32_pi),
        op_entry!(MASK_OUT_X_Y, OP_CMP_32_PD,   cmp_32_pd),
        op_entry!(MASK_OUT_X_Y, OP_CMP_32_DI,   cmp_32_di),
        op_entry!(MASK_OUT_X_Y, OP_CMP_32_IX,   cmp_32_ix),
        op_entry!(MASK_OUT_X,   OP_CMP_32_AW,   cmp_32_aw),
        op_entry!(MASK_OUT_X,   OP_CMP_32_AL,   cmp_32_al),
        op_entry!(MASK_OUT_X,   OP_CMP_32_PCDI, cmp_32_pcdi),
        op_entry!(MASK_OUT_X,   OP_CMP_32_PCIX, cmp_32_pcix),
        op_entry!(MASK_OUT_X,   OP_CMP_32_IMM,  cmp_32_imm),

        op_entry!(MASK_OUT_X_Y, OP_CMPA_16_DN,   cmpa_16_dn),
        op_entry!(MASK_OUT_X_Y, OP_CMPA_16_AN,   cmpa_16_an),
        op_entry!(MASK_OUT_X_Y, OP_CMPA_16_AI,   cmpa_16_ai),
        op_entry!(MASK_OUT_X_Y, OP_CMPA_16_PI,   cmpa_16_pi),
        op_entry!(MASK_OUT_X_Y, OP_CMPA_16_PD,   cmpa_16_pd),
        op_entry!(MASK_OUT_X_Y, OP_CMPA_16_DI,   cmpa_16_di),
        op_entry!(MASK_OUT_X_Y, OP_CMPA_16_IX,   cmpa_16_ix),
        op_entry!(MASK_OUT_X,   OP_CMPA_16_AW,   cmpa_16_aw),
        op_entry!(MASK_OUT_X,   OP_CMPA_16_AL,   cmpa_16_al),
        op_entry!(MASK_OUT_X,   OP_CMPA_16_PCDI, cmpa_16_pcdi),
        op_entry!(MASK_OUT_X,   OP_CMPA_16_PCIX, cmpa_16_pcix),
        op_entry!(MASK_OUT_X,   OP_CMPA_16_IMM,  cmpa_16_imm),

        op_entry!(MASK_OUT_X_Y, OP_CMPA_32_DN,   cmpa_32_dn),
        op_entry!(MASK_OUT_X_Y, OP_CMPA_32_AN,   cmpa_32_an),
        op_entry!(MASK_OUT_X_Y, OP_CMPA_32_AI,   cmpa_32_ai),
        op_entry!(MASK_OUT_X_Y, OP_CMPA_32_PI,   cmpa_32_pi),
        op_entry!(MASK_OUT_X_Y, OP_CMPA_32_PD,   cmpa_32_pd),
        op_entry!(MASK_OUT_X_Y, OP_CMPA_32_DI,   cmpa_32_di),
        op_entry!(MASK_OUT_X_Y, OP_CMPA_32_IX,   cmpa_32_ix),
        op_entry!(MASK_OUT_X,   OP_CMPA_32_AW,   cmpa_32_aw),
        op_entry!(MASK_OUT_X,   OP_CMPA_32_AL,   cmpa_32_al),
        op_entry!(MASK_OUT_X,   OP_CMPA_32_PCDI, cmpa_32_pcdi),
        op_entry!(MASK_OUT_X,   OP_CMPA_32_PCIX, cmpa_32_pcix),
        op_entry!(MASK_OUT_X,   OP_CMPA_32_IMM,  cmpa_32_imm),

        op_entry!(MASK_OUT_Y, OP_CMPI_8_DN,   cmpi_8_dn),
        op_entry!(MASK_OUT_Y, OP_CMPI_8_AI,   cmpi_8_ai),
        op_entry!(MASK_OUT_Y, OP_CMPI_8_PI,   cmpi_8_pi),
        op_entry!(MASK_OUT_Y, OP_CMPI_8_PD,   cmpi_8_pd),
        op_entry!(MASK_OUT_Y, OP_CMPI_8_DI,   cmpi_8_di),
        op_entry!(MASK_OUT_Y, OP_CMPI_8_IX,   cmpi_8_ix),
        op_entry!(MASK_EXACT, OP_CMPI_8_AW,   cmpi_8_aw),
        op_entry!(MASK_EXACT, OP_CMPI_8_AL,   cmpi_8_al),

        op_entry!(MASK_OUT_Y, OP_CMPI_16_DN,   cmpi_16_dn),
        op_entry!(MASK_OUT_Y, OP_CMPI_16_AI,   cmpi_16_ai),
        op_entry!(MASK_OUT_Y, OP_CMPI_16_PI,   cmpi_16_pi),
        op_entry!(MASK_OUT_Y, OP_CMPI_16_PD,   cmpi_16_pd),
        op_entry!(MASK_OUT_Y, OP_CMPI_16_DI,   cmpi_16_di),
        op_entry!(MASK_OUT_Y, OP_CMPI_16_IX,   cmpi_16_ix),
        op_entry!(MASK_EXACT, OP_CMPI_16_AW,   cmpi_16_aw),
        op_entry!(MASK_EXACT, OP_CMPI_16_AL,   cmpi_16_al),

        op_entry!(MASK_OUT_Y, OP_CMPI_32_DN,   cmpi_32_dn),
        op_entry!(MASK_OUT_Y, OP_CMPI_32_AI,   cmpi_32_ai),
        op_entry!(MASK_OUT_Y, OP_CMPI_32_PI,   cmpi_32_pi),
        op_entry!(MASK_OUT_Y, OP_CMPI_32_PD,   cmpi_32_pd),
        op_entry!(MASK_OUT_Y, OP_CMPI_32_DI,   cmpi_32_di),
        op_entry!(MASK_OUT_Y, OP_CMPI_32_IX,   cmpi_32_ix),
        op_entry!(MASK_EXACT, OP_CMPI_32_AW,   cmpi_32_aw),
        op_entry!(MASK_EXACT, OP_CMPI_32_AL,   cmpi_32_al),

        op_entry!(MASK_OUT_X_Y, OP_CMPM_8,  cmpm_8),
        op_entry!(MASK_OUT_X_Y, OP_CMPM_16, cmpm_16),
        op_entry!(MASK_OUT_X_Y, OP_CMPM_32, cmpm_32),

        // Put op-entries for DBcc here
        op_entry!(MASK_OUT_Y, OP_DBT_16,  dbt_16),
        op_entry!(MASK_OUT_Y, OP_DBF_16,  dbf_16),
        op_entry!(MASK_OUT_Y, OP_DBHI_16, dbhi_16),
        op_entry!(MASK_OUT_Y, OP_DBLS_16, dbls_16),
        op_entry!(MASK_OUT_Y, OP_DBCC_16, dbcc_16),
        op_entry!(MASK_OUT_Y, OP_DBCS_16, dbcs_16),
        op_entry!(MASK_OUT_Y, OP_DBNE_16, dbne_16),
        op_entry!(MASK_OUT_Y, OP_DBEQ_16, dbeq_16),
        op_entry!(MASK_OUT_Y, OP_DBVC_16, dbvc_16),
        op_entry!(MASK_OUT_Y, OP_DBVS_16, dbvs_16),
        op_entry!(MASK_OUT_Y, OP_DBPL_16, dbpl_16),
        op_entry!(MASK_OUT_Y, OP_DBMI_16, dbmi_16),
        op_entry!(MASK_OUT_Y, OP_DBGE_16, dbge_16),
        op_entry!(MASK_OUT_Y, OP_DBLT_16, dblt_16),
        op_entry!(MASK_OUT_Y, OP_DBGT_16, dbgt_16),
        op_entry!(MASK_OUT_Y, OP_DBLE_16, dble_16),

        // Put op-entries for DIVS here
        op_entry!(MASK_OUT_X_Y, OP_DIVS_16_AI,   divs_16_ai),
        op_entry!(MASK_OUT_X,   OP_DIVS_16_AL,   divs_16_al),
        op_entry!(MASK_OUT_X,   OP_DIVS_16_AW,   divs_16_aw),
        op_entry!(MASK_OUT_X_Y, OP_DIVS_16_DN,   divs_16_dn),
        op_entry!(MASK_OUT_X_Y, OP_DIVS_16_DI,   divs_16_di),
        op_entry!(MASK_OUT_X,   OP_DIVS_16_IMM,  divs_16_imm),
        op_entry!(MASK_OUT_X_Y, OP_DIVS_16_IX,   divs_16_ix),
        op_entry!(MASK_OUT_X,   OP_DIVS_16_PCDI, divs_16_pcdi),
        op_entry!(MASK_OUT_X,   OP_DIVS_16_PCIX, divs_16_pcix),
        op_entry!(MASK_OUT_X_Y, OP_DIVS_16_PD,   divs_16_pd),
        op_entry!(MASK_OUT_X_Y, OP_DIVS_16_PI,   divs_16_pi),

        // Put op-entries for DIVU here
        op_entry!(MASK_OUT_X_Y, OP_DIVU_16_AI,   divu_16_ai),
        op_entry!(MASK_OUT_X,   OP_DIVU_16_AL,   divu_16_al),
        op_entry!(MASK_OUT_X,   OP_DIVU_16_AW,   divu_16_aw),
        op_entry!(MASK_OUT_X_Y, OP_DIVU_16_DN,   divu_16_dn),
        op_entry!(MASK_OUT_X_Y, OP_DIVU_16_DI,   divu_16_di),
        op_entry!(MASK_OUT_X,   OP_DIVU_16_IMM,  divu_16_imm),
        op_entry!(MASK_OUT_X_Y, OP_DIVU_16_IX,   divu_16_ix),
        op_entry!(MASK_OUT_X,   OP_DIVU_16_PCDI, divu_16_pcdi),
        op_entry!(MASK_OUT_X,   OP_DIVU_16_PCIX, divu_16_pcix),
        op_entry!(MASK_OUT_X_Y, OP_DIVU_16_PD,   divu_16_pd),
        op_entry!(MASK_OUT_X_Y, OP_DIVU_16_PI,   divu_16_pi),

        // Put op-entries for EOR, EORI, EORI to CCR and EORI to SR here
        op_entry!(MASK_OUT_X_Y, OP_EOR_8_DN,   eor_8_dn),
        op_entry!(MASK_OUT_X_Y, OP_EOR_8_AI,   eor_8_ai),
        op_entry!(MASK_OUT_X_Y, OP_EOR_8_PI,   eor_8_pi),
        op_entry!(MASK_OUT_X_Y, OP_EOR_8_PD,   eor_8_pd),
        op_entry!(MASK_OUT_X_Y, OP_EOR_8_DI,   eor_8_di),
        op_entry!(MASK_OUT_X_Y, OP_EOR_8_IX,   eor_8_ix),
        op_entry!(MASK_OUT_X,   OP_EOR_8_AW,   eor_8_aw),
        op_entry!(MASK_OUT_X,   OP_EOR_8_AL,   eor_8_al),

        op_entry!(MASK_OUT_X_Y, OP_EOR_16_DN,   eor_16_dn),
        op_entry!(MASK_OUT_X_Y, OP_EOR_16_AI,   eor_16_ai),
        op_entry!(MASK_OUT_X_Y, OP_EOR_16_PI,   eor_16_pi),
        op_entry!(MASK_OUT_X_Y, OP_EOR_16_PD,   eor_16_pd),
        op_entry!(MASK_OUT_X_Y, OP_EOR_16_DI,   eor_16_di),
        op_entry!(MASK_OUT_X_Y, OP_EOR_16_IX,   eor_16_ix),
        op_entry!(MASK_OUT_X,   OP_EOR_16_AW,   eor_16_aw),
        op_entry!(MASK_OUT_X,   OP_EOR_16_AL,   eor_16_al),

        op_entry!(MASK_OUT_X_Y, OP_EOR_32_DN,   eor_32_dn),
        op_entry!(MASK_OUT_X_Y, OP_EOR_32_AI,   eor_32_ai),
        op_entry!(MASK_OUT_X_Y, OP_EOR_32_PI,   eor_32_pi),
        op_entry!(MASK_OUT_X_Y, OP_EOR_32_PD,   eor_32_pd),
        op_entry!(MASK_OUT_X_Y, OP_EOR_32_DI,   eor_32_di),
        op_entry!(MASK_OUT_X_Y, OP_EOR_32_IX,   eor_32_ix),
        op_entry!(MASK_OUT_X,   OP_EOR_32_AW,   eor_32_aw),
        op_entry!(MASK_OUT_X,   OP_EOR_32_AL,   eor_32_al),

        op_entry!(MASK_OUT_Y, OP_EORI_8_DN,   eori_8_dn),
        op_entry!(MASK_OUT_Y, OP_EORI_8_AI,   eori_8_ai),
        op_entry!(MASK_OUT_Y, OP_EORI_8_PI,   eori_8_pi),
        op_entry!(MASK_OUT_Y, OP_EORI_8_PD,   eori_8_pd),
        op_entry!(MASK_OUT_Y, OP_EORI_8_DI,   eori_8_di),
        op_entry!(MASK_OUT_Y, OP_EORI_8_IX,   eori_8_ix),
        op_entry!(MASK_EXACT, OP_EORI_8_AW,   eori_8_aw),
        op_entry!(MASK_EXACT, OP_EORI_8_AL,   eori_8_al),

        op_entry!(MASK_OUT_Y, OP_EORI_16_DN,   eori_16_dn),
        op_entry!(MASK_OUT_Y, OP_EORI_16_AI,   eori_16_ai),
        op_entry!(MASK_OUT_Y, OP_EORI_16_PI,   eori_16_pi),
        op_entry!(MASK_OUT_Y, OP_EORI_16_PD,   eori_16_pd),
        op_entry!(MASK_OUT_Y, OP_EORI_16_DI,   eori_16_di),
        op_entry!(MASK_OUT_Y, OP_EORI_16_IX,   eori_16_ix),
        op_entry!(MASK_EXACT, OP_EORI_16_AW,   eori_16_aw),
        op_entry!(MASK_EXACT, OP_EORI_16_AL,   eori_16_al),

        op_entry!(MASK_OUT_Y, OP_EORI_32_DN,   eori_32_dn),
        op_entry!(MASK_OUT_Y, OP_EORI_32_AI,   eori_32_ai),
        op_entry!(MASK_OUT_Y, OP_EORI_32_PI,   eori_32_pi),
        op_entry!(MASK_OUT_Y, OP_EORI_32_PD,   eori_32_pd),
        op_entry!(MASK_OUT_Y, OP_EORI_32_DI,   eori_32_di),
        op_entry!(MASK_OUT_Y, OP_EORI_32_IX,   eori_32_ix),
        op_entry!(MASK_EXACT, OP_EORI_32_AW,   eori_32_aw),
        op_entry!(MASK_EXACT, OP_EORI_32_AL,   eori_32_al),

        op_entry!(MASK_EXACT, OP_EORI_16_TOC,   eori_16_toc),
        op_entry!(MASK_EXACT, OP_EORI_16_TOS,   eori_16_tos),

        // Put op-entries for EXG here
        op_entry!(MASK_OUT_X_Y, OP_EXG_32_DD, exg_32_dd),
        op_entry!(MASK_OUT_X_Y, OP_EXG_32_AA, exg_32_aa),
        op_entry!(MASK_OUT_X_Y, OP_EXG_32_DA, exg_32_da),

        // Put op-entries for EXT here
        op_entry!(MASK_OUT_Y, OP_EXT_BW, ext_bw),
        op_entry!(MASK_OUT_Y, OP_EXT_WL, ext_wl),

        // Put op-entries for ILLEGAL here
        op_entry!(MASK_EXACT, OP_ILLEGAL, real_illegal),

        // Put op-entries for JMP here
        op_entry!(MASK_OUT_Y, OP_JMP_32_AI,   jmp_32_ai),
        op_entry!(MASK_EXACT, OP_JMP_32_AL,   jmp_32_al),
        op_entry!(MASK_EXACT, OP_JMP_32_AW,   jmp_32_aw),
        op_entry!(MASK_OUT_Y, OP_JMP_32_DI,   jmp_32_di),
        op_entry!(MASK_OUT_Y, OP_JMP_32_IX,   jmp_32_ix),
        op_entry!(MASK_EXACT, OP_JMP_32_PCDI, jmp_32_pcdi),
        op_entry!(MASK_EXACT, OP_JMP_32_PCIX, jmp_32_pcix),

        // Put op-entries for JSR here
        op_entry!(MASK_OUT_Y, OP_JSR_32_AI,   jsr_32_ai),
        op_entry!(MASK_EXACT, OP_JSR_32_AL,   jsr_32_al),
        op_entry!(MASK_EXACT, OP_JSR_32_AW,   jsr_32_aw),
        op_entry!(MASK_OUT_Y, OP_JSR_32_DI,   jsr_32_di),
        op_entry!(MASK_OUT_Y, OP_JSR_32_IX,   jsr_32_ix),
        op_entry!(MASK_EXACT, OP_JSR_32_PCDI, jsr_32_pcdi),
        op_entry!(MASK_EXACT, OP_JSR_32_PCIX, jsr_32_pcix),

        // Put op-entries for LEA here
        op_entry!(MASK_OUT_Y, OP_LEA_32_AI, lea_32_ai),
        op_entry!(MASK_EXACT, OP_LEA_32_AL, lea_32_al),
        op_entry!(MASK_EXACT, OP_LEA_32_AW, lea_32_aw),
        op_entry!(MASK_OUT_Y, OP_LEA_32_DI, lea_32_di),
        op_entry!(MASK_OUT_Y, OP_LEA_32_IX, lea_32_ix),
        op_entry!(MASK_EXACT, OP_LEA_32_PCDI, lea_32_pcdi),
        op_entry!(MASK_EXACT, OP_LEA_32_PCIX, lea_32_pcix),

        // Put op-entries for LINK here
        op_entry!(MASK_OUT_Y, OP_LINK_16, link_16),

        // Put op-entries for LSL, LSR here
        op_entry!(MASK_OUT_X_Y, OP_LSR_8_S,  lsr_8_s),
        op_entry!(MASK_OUT_X_Y, OP_LSR_16_S, lsr_16_s),
        op_entry!(MASK_OUT_X_Y, OP_LSR_32_S, lsr_32_s),
        op_entry!(MASK_OUT_X_Y, OP_LSR_8_R,  lsr_8_r),
        op_entry!(MASK_OUT_X_Y, OP_LSR_16_R, lsr_16_r),
        op_entry!(MASK_OUT_X_Y, OP_LSR_32_R, lsr_32_r),

        op_entry!(MASK_OUT_X_Y, OP_LSL_8_S,  lsl_8_s),
        op_entry!(MASK_OUT_X_Y, OP_LSL_16_S, lsl_16_s),
        op_entry!(MASK_OUT_X_Y, OP_LSL_32_S, lsl_32_s),
        op_entry!(MASK_OUT_X_Y, OP_LSL_8_R,  lsl_8_r),
        op_entry!(MASK_OUT_X_Y, OP_LSL_16_R, lsl_16_r),
        op_entry!(MASK_OUT_X_Y, OP_LSL_32_R, lsl_32_r),

        op_entry!(MASK_OUT_Y, OP_LSL_16_AI, lsl_16_ai),
        op_entry!(MASK_OUT_Y, OP_LSL_16_PI, lsl_16_pi),
        op_entry!(MASK_OUT_Y, OP_LSL_16_PD, lsl_16_pd),
        op_entry!(MASK_OUT_Y, OP_LSL_16_DI, lsl_16_di),
        op_entry!(MASK_OUT_Y, OP_LSL_16_IX, lsl_16_ix),
        op_entry!(MASK_EXACT, OP_LSL_16_AW, lsl_16_aw),
        op_entry!(MASK_EXACT, OP_LSL_16_AL, lsl_16_al),

        op_entry!(MASK_OUT_Y, OP_LSR_16_AI, lsr_16_ai),
        op_entry!(MASK_OUT_Y, OP_LSR_16_PI, lsr_16_pi),
        op_entry!(MASK_OUT_Y, OP_LSR_16_PD, lsr_16_pd),
        op_entry!(MASK_OUT_Y, OP_LSR_16_DI, lsr_16_di),
        op_entry!(MASK_OUT_Y, OP_LSR_16_IX, lsr_16_ix),
        op_entry!(MASK_EXACT, OP_LSR_16_AW, lsr_16_aw),
        op_entry!(MASK_EXACT, OP_LSR_16_AL, lsr_16_al),

        // Put op-entries for MOVE here
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_DN_DN, move_8_dn_dn),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_AI_DN, move_8_ai_dn),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_PI_DN, move_8_pi_dn),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_PD_DN, move_8_pd_dn),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_DI_DN, move_8_di_dn),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_IX_DN, move_8_ix_dn),
        op_entry!(MASK_OUT_Y,   OP_MOVE_8_AW_DN, move_8_aw_dn),
        op_entry!(MASK_OUT_Y,   OP_MOVE_8_AL_DN, move_8_al_dn),

        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_DN_AI, move_8_dn_ai),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_AI_AI, move_8_ai_ai),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_PI_AI, move_8_pi_ai),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_PD_AI, move_8_pd_ai),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_DI_AI, move_8_di_ai),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_IX_AI, move_8_ix_ai),
        op_entry!(MASK_OUT_Y,   OP_MOVE_8_AW_AI, move_8_aw_ai),
        op_entry!(MASK_OUT_Y,   OP_MOVE_8_AL_AI, move_8_al_ai),

        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_DN_PI, move_8_dn_pi),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_AI_PI, move_8_ai_pi),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_PI_PI, move_8_pi_pi),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_PD_PI, move_8_pd_pi),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_DI_PI, move_8_di_pi),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_IX_PI, move_8_ix_pi),
        op_entry!(MASK_OUT_Y,   OP_MOVE_8_AW_PI, move_8_aw_pi),
        op_entry!(MASK_OUT_Y,   OP_MOVE_8_AL_PI, move_8_al_pi),

        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_DN_PD, move_8_dn_pd),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_AI_PD, move_8_ai_pd),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_PI_PD, move_8_pi_pd),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_PD_PD, move_8_pd_pd),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_DI_PD, move_8_di_pd),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_IX_PD, move_8_ix_pd),
        op_entry!(MASK_OUT_Y,   OP_MOVE_8_AW_PD, move_8_aw_pd),
        op_entry!(MASK_OUT_Y,   OP_MOVE_8_AL_PD, move_8_al_pd),

        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_DN_DI, move_8_dn_di),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_AI_DI, move_8_ai_di),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_PI_DI, move_8_pi_di),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_PD_DI, move_8_pd_di),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_DI_DI, move_8_di_di),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_IX_DI, move_8_ix_di),
        op_entry!(MASK_OUT_Y,   OP_MOVE_8_AW_DI, move_8_aw_di),
        op_entry!(MASK_OUT_Y,   OP_MOVE_8_AL_DI, move_8_al_di),

        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_DN_IX, move_8_dn_ix),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_AI_IX, move_8_ai_ix),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_PI_IX, move_8_pi_ix),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_PD_IX, move_8_pd_ix),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_DI_IX, move_8_di_ix),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_8_IX_IX, move_8_ix_ix),
        op_entry!(MASK_OUT_Y,   OP_MOVE_8_AW_IX, move_8_aw_ix),
        op_entry!(MASK_OUT_Y,   OP_MOVE_8_AL_IX, move_8_al_ix),

        op_entry!(MASK_OUT_X, OP_MOVE_8_DN_AW, move_8_dn_aw),
        op_entry!(MASK_OUT_X, OP_MOVE_8_AI_AW, move_8_ai_aw),
        op_entry!(MASK_OUT_X, OP_MOVE_8_PI_AW, move_8_pi_aw),
        op_entry!(MASK_OUT_X, OP_MOVE_8_PD_AW, move_8_pd_aw),
        op_entry!(MASK_OUT_X, OP_MOVE_8_DI_AW, move_8_di_aw),
        op_entry!(MASK_OUT_X, OP_MOVE_8_IX_AW, move_8_ix_aw),
        op_entry!(MASK_EXACT, OP_MOVE_8_AW_AW, move_8_aw_aw),
        op_entry!(MASK_EXACT, OP_MOVE_8_AL_AW, move_8_al_aw),

        op_entry!(MASK_OUT_X, OP_MOVE_8_DN_AL, move_8_dn_al),
        op_entry!(MASK_OUT_X, OP_MOVE_8_AI_AL, move_8_ai_al),
        op_entry!(MASK_OUT_X, OP_MOVE_8_PI_AL, move_8_pi_al),
        op_entry!(MASK_OUT_X, OP_MOVE_8_PD_AL, move_8_pd_al),
        op_entry!(MASK_OUT_X, OP_MOVE_8_DI_AL, move_8_di_al),
        op_entry!(MASK_OUT_X, OP_MOVE_8_IX_AL, move_8_ix_al),
        op_entry!(MASK_EXACT, OP_MOVE_8_AW_AL, move_8_aw_al),
        op_entry!(MASK_EXACT, OP_MOVE_8_AL_AL, move_8_al_al),

        op_entry!(MASK_OUT_X, OP_MOVE_8_DN_PCDI, move_8_dn_pcdi),
        op_entry!(MASK_OUT_X, OP_MOVE_8_AI_PCDI, move_8_ai_pcdi),
        op_entry!(MASK_OUT_X, OP_MOVE_8_PI_PCDI, move_8_pi_pcdi),
        op_entry!(MASK_OUT_X, OP_MOVE_8_PD_PCDI, move_8_pd_pcdi),
        op_entry!(MASK_OUT_X, OP_MOVE_8_DI_PCDI, move_8_di_pcdi),
        op_entry!(MASK_OUT_X, OP_MOVE_8_IX_PCDI, move_8_ix_pcdi),
        op_entry!(MASK_EXACT, OP_MOVE_8_AW_PCDI, move_8_aw_pcdi),
        op_entry!(MASK_EXACT, OP_MOVE_8_AL_PCDI, move_8_al_pcdi),

        op_entry!(MASK_OUT_X, OP_MOVE_8_DN_PCIX, move_8_dn_pcix),
        op_entry!(MASK_OUT_X, OP_MOVE_8_AI_PCIX, move_8_ai_pcix),
        op_entry!(MASK_OUT_X, OP_MOVE_8_PI_PCIX, move_8_pi_pcix),
        op_entry!(MASK_OUT_X, OP_MOVE_8_PD_PCIX, move_8_pd_pcix),
        op_entry!(MASK_OUT_X, OP_MOVE_8_DI_PCIX, move_8_di_pcix),
        op_entry!(MASK_OUT_X, OP_MOVE_8_IX_PCIX, move_8_ix_pcix),
        op_entry!(MASK_EXACT, OP_MOVE_8_AW_PCIX, move_8_aw_pcix),
        op_entry!(MASK_EXACT, OP_MOVE_8_AL_PCIX, move_8_al_pcix),

        op_entry!(MASK_OUT_X, OP_MOVE_8_DN_IMM, move_8_dn_imm),
        op_entry!(MASK_OUT_X, OP_MOVE_8_AI_IMM, move_8_ai_imm),
        op_entry!(MASK_OUT_X, OP_MOVE_8_PI_IMM, move_8_pi_imm),
        op_entry!(MASK_OUT_X, OP_MOVE_8_PD_IMM, move_8_pd_imm),
        op_entry!(MASK_OUT_X, OP_MOVE_8_DI_IMM, move_8_di_imm),
        op_entry!(MASK_OUT_X, OP_MOVE_8_IX_IMM, move_8_ix_imm),
        op_entry!(MASK_EXACT, OP_MOVE_8_AW_IMM, move_8_aw_imm),
        op_entry!(MASK_EXACT, OP_MOVE_8_AL_IMM, move_8_al_imm),

        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_DN_DN, move_16_dn_dn),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_AI_DN, move_16_ai_dn),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_PI_DN, move_16_pi_dn),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_PD_DN, move_16_pd_dn),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_DI_DN, move_16_di_dn),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_IX_DN, move_16_ix_dn),
        op_entry!(MASK_OUT_Y,   OP_MOVE_16_AW_DN, move_16_aw_dn),
        op_entry!(MASK_OUT_Y,   OP_MOVE_16_AL_DN, move_16_al_dn),

        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_DN_AI, move_16_dn_ai),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_AI_AI, move_16_ai_ai),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_PI_AI, move_16_pi_ai),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_PD_AI, move_16_pd_ai),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_DI_AI, move_16_di_ai),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_IX_AI, move_16_ix_ai),
        op_entry!(MASK_OUT_Y,   OP_MOVE_16_AW_AI, move_16_aw_ai),
        op_entry!(MASK_OUT_Y,   OP_MOVE_16_AL_AI, move_16_al_ai),

        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_DN_PI, move_16_dn_pi),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_AI_PI, move_16_ai_pi),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_PI_PI, move_16_pi_pi),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_PD_PI, move_16_pd_pi),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_DI_PI, move_16_di_pi),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_IX_PI, move_16_ix_pi),
        op_entry!(MASK_OUT_Y,   OP_MOVE_16_AW_PI, move_16_aw_pi),
        op_entry!(MASK_OUT_Y,   OP_MOVE_16_AL_PI, move_16_al_pi),

        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_DN_PD, move_16_dn_pd),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_AI_PD, move_16_ai_pd),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_PI_PD, move_16_pi_pd),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_PD_PD, move_16_pd_pd),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_DI_PD, move_16_di_pd),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_IX_PD, move_16_ix_pd),
        op_entry!(MASK_OUT_Y,   OP_MOVE_16_AW_PD, move_16_aw_pd),
        op_entry!(MASK_OUT_Y,   OP_MOVE_16_AL_PD, move_16_al_pd),

        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_DN_DI, move_16_dn_di),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_AI_DI, move_16_ai_di),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_PI_DI, move_16_pi_di),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_PD_DI, move_16_pd_di),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_DI_DI, move_16_di_di),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_IX_DI, move_16_ix_di),
        op_entry!(MASK_OUT_Y,   OP_MOVE_16_AW_DI, move_16_aw_di),
        op_entry!(MASK_OUT_Y,   OP_MOVE_16_AL_DI, move_16_al_di),

        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_DN_IX, move_16_dn_ix),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_AI_IX, move_16_ai_ix),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_PI_IX, move_16_pi_ix),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_PD_IX, move_16_pd_ix),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_DI_IX, move_16_di_ix),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_16_IX_IX, move_16_ix_ix),
        op_entry!(MASK_OUT_Y,   OP_MOVE_16_AW_IX, move_16_aw_ix),
        op_entry!(MASK_OUT_Y,   OP_MOVE_16_AL_IX, move_16_al_ix),

        op_entry!(MASK_OUT_X, OP_MOVE_16_DN_AW, move_16_dn_aw),
        op_entry!(MASK_OUT_X, OP_MOVE_16_AI_AW, move_16_ai_aw),
        op_entry!(MASK_OUT_X, OP_MOVE_16_PI_AW, move_16_pi_aw),
        op_entry!(MASK_OUT_X, OP_MOVE_16_PD_AW, move_16_pd_aw),
        op_entry!(MASK_OUT_X, OP_MOVE_16_DI_AW, move_16_di_aw),
        op_entry!(MASK_OUT_X, OP_MOVE_16_IX_AW, move_16_ix_aw),
        op_entry!(MASK_EXACT, OP_MOVE_16_AW_AW, move_16_aw_aw),
        op_entry!(MASK_EXACT, OP_MOVE_16_AL_AW, move_16_al_aw),

        op_entry!(MASK_OUT_X, OP_MOVE_16_DN_AL, move_16_dn_al),
        op_entry!(MASK_OUT_X, OP_MOVE_16_AI_AL, move_16_ai_al),
        op_entry!(MASK_OUT_X, OP_MOVE_16_PI_AL, move_16_pi_al),
        op_entry!(MASK_OUT_X, OP_MOVE_16_PD_AL, move_16_pd_al),
        op_entry!(MASK_OUT_X, OP_MOVE_16_DI_AL, move_16_di_al),
        op_entry!(MASK_OUT_X, OP_MOVE_16_IX_AL, move_16_ix_al),
        op_entry!(MASK_EXACT, OP_MOVE_16_AW_AL, move_16_aw_al),
        op_entry!(MASK_EXACT, OP_MOVE_16_AL_AL, move_16_al_al),

        op_entry!(MASK_OUT_X, OP_MOVE_16_DN_PCDI, move_16_dn_pcdi),
        op_entry!(MASK_OUT_X, OP_MOVE_16_AI_PCDI, move_16_ai_pcdi),
        op_entry!(MASK_OUT_X, OP_MOVE_16_PI_PCDI, move_16_pi_pcdi),
        op_entry!(MASK_OUT_X, OP_MOVE_16_PD_PCDI, move_16_pd_pcdi),
        op_entry!(MASK_OUT_X, OP_MOVE_16_DI_PCDI, move_16_di_pcdi),
        op_entry!(MASK_OUT_X, OP_MOVE_16_IX_PCDI, move_16_ix_pcdi),
        op_entry!(MASK_EXACT, OP_MOVE_16_AW_PCDI, move_16_aw_pcdi),
        op_entry!(MASK_EXACT, OP_MOVE_16_AL_PCDI, move_16_al_pcdi),

        op_entry!(MASK_OUT_X, OP_MOVE_16_DN_PCIX, move_16_dn_pcix),
        op_entry!(MASK_OUT_X, OP_MOVE_16_AI_PCIX, move_16_ai_pcix),
        op_entry!(MASK_OUT_X, OP_MOVE_16_PI_PCIX, move_16_pi_pcix),
        op_entry!(MASK_OUT_X, OP_MOVE_16_PD_PCIX, move_16_pd_pcix),
        op_entry!(MASK_OUT_X, OP_MOVE_16_DI_PCIX, move_16_di_pcix),
        op_entry!(MASK_OUT_X, OP_MOVE_16_IX_PCIX, move_16_ix_pcix),
        op_entry!(MASK_EXACT, OP_MOVE_16_AW_PCIX, move_16_aw_pcix),
        op_entry!(MASK_EXACT, OP_MOVE_16_AL_PCIX, move_16_al_pcix),

        op_entry!(MASK_OUT_X, OP_MOVE_16_DN_IMM, move_16_dn_imm),
        op_entry!(MASK_OUT_X, OP_MOVE_16_AI_IMM, move_16_ai_imm),
        op_entry!(MASK_OUT_X, OP_MOVE_16_PI_IMM, move_16_pi_imm),
        op_entry!(MASK_OUT_X, OP_MOVE_16_PD_IMM, move_16_pd_imm),
        op_entry!(MASK_OUT_X, OP_MOVE_16_DI_IMM, move_16_di_imm),
        op_entry!(MASK_OUT_X, OP_MOVE_16_IX_IMM, move_16_ix_imm),
        op_entry!(MASK_EXACT, OP_MOVE_16_AW_IMM, move_16_aw_imm),
        op_entry!(MASK_EXACT, OP_MOVE_16_AL_IMM, move_16_al_imm),

        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_DN_DN, move_32_dn_dn),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_AI_DN, move_32_ai_dn),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_PI_DN, move_32_pi_dn),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_PD_DN, move_32_pd_dn),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_DI_DN, move_32_di_dn),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_IX_DN, move_32_ix_dn),
        op_entry!(MASK_OUT_Y,   OP_MOVE_32_AW_DN, move_32_aw_dn),
        op_entry!(MASK_OUT_Y,   OP_MOVE_32_AL_DN, move_32_al_dn),

        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_DN_AI, move_32_dn_ai),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_AI_AI, move_32_ai_ai),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_PI_AI, move_32_pi_ai),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_PD_AI, move_32_pd_ai),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_DI_AI, move_32_di_ai),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_IX_AI, move_32_ix_ai),
        op_entry!(MASK_OUT_Y,   OP_MOVE_32_AW_AI, move_32_aw_ai),
        op_entry!(MASK_OUT_Y,   OP_MOVE_32_AL_AI, move_32_al_ai),

        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_DN_PI, move_32_dn_pi),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_AI_PI, move_32_ai_pi),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_PI_PI, move_32_pi_pi),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_PD_PI, move_32_pd_pi),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_DI_PI, move_32_di_pi),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_IX_PI, move_32_ix_pi),
        op_entry!(MASK_OUT_Y,   OP_MOVE_32_AW_PI, move_32_aw_pi),
        op_entry!(MASK_OUT_Y,   OP_MOVE_32_AL_PI, move_32_al_pi),

        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_DN_PD, move_32_dn_pd),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_AI_PD, move_32_ai_pd),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_PI_PD, move_32_pi_pd),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_PD_PD, move_32_pd_pd),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_DI_PD, move_32_di_pd),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_IX_PD, move_32_ix_pd),
        op_entry!(MASK_OUT_Y,   OP_MOVE_32_AW_PD, move_32_aw_pd),
        op_entry!(MASK_OUT_Y,   OP_MOVE_32_AL_PD, move_32_al_pd),

        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_DN_DI, move_32_dn_di),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_AI_DI, move_32_ai_di),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_PI_DI, move_32_pi_di),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_PD_DI, move_32_pd_di),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_DI_DI, move_32_di_di),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_IX_DI, move_32_ix_di),
        op_entry!(MASK_OUT_Y,   OP_MOVE_32_AW_DI, move_32_aw_di),
        op_entry!(MASK_OUT_Y,   OP_MOVE_32_AL_DI, move_32_al_di),

        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_DN_IX, move_32_dn_ix),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_AI_IX, move_32_ai_ix),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_PI_IX, move_32_pi_ix),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_PD_IX, move_32_pd_ix),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_DI_IX, move_32_di_ix),
        op_entry!(MASK_OUT_X_Y, OP_MOVE_32_IX_IX, move_32_ix_ix),
        op_entry!(MASK_OUT_Y,   OP_MOVE_32_AW_IX, move_32_aw_ix),
        op_entry!(MASK_OUT_Y,   OP_MOVE_32_AL_IX, move_32_al_ix),

        op_entry!(MASK_OUT_X, OP_MOVE_32_DN_AW, move_32_dn_aw),
        op_entry!(MASK_OUT_X, OP_MOVE_32_AI_AW, move_32_ai_aw),
        op_entry!(MASK_OUT_X, OP_MOVE_32_PI_AW, move_32_pi_aw),
        op_entry!(MASK_OUT_X, OP_MOVE_32_PD_AW, move_32_pd_aw),
        op_entry!(MASK_OUT_X, OP_MOVE_32_DI_AW, move_32_di_aw),
        op_entry!(MASK_OUT_X, OP_MOVE_32_IX_AW, move_32_ix_aw),
        op_entry!(MASK_EXACT, OP_MOVE_32_AW_AW, move_32_aw_aw),
        op_entry!(MASK_EXACT, OP_MOVE_32_AL_AW, move_32_al_aw),

        op_entry!(MASK_OUT_X, OP_MOVE_32_DN_AL, move_32_dn_al),
        op_entry!(MASK_OUT_X, OP_MOVE_32_AI_AL, move_32_ai_al),
        op_entry!(MASK_OUT_X, OP_MOVE_32_PI_AL, move_32_pi_al),
        op_entry!(MASK_OUT_X, OP_MOVE_32_PD_AL, move_32_pd_al),
        op_entry!(MASK_OUT_X, OP_MOVE_32_DI_AL, move_32_di_al),
        op_entry!(MASK_OUT_X, OP_MOVE_32_IX_AL, move_32_ix_al),
        op_entry!(MASK_EXACT, OP_MOVE_32_AW_AL, move_32_aw_al),
        op_entry!(MASK_EXACT, OP_MOVE_32_AL_AL, move_32_al_al),

        op_entry!(MASK_OUT_X, OP_MOVE_32_DN_PCDI, move_32_dn_pcdi),
        op_entry!(MASK_OUT_X, OP_MOVE_32_AI_PCDI, move_32_ai_pcdi),
        op_entry!(MASK_OUT_X, OP_MOVE_32_PI_PCDI, move_32_pi_pcdi),
        op_entry!(MASK_OUT_X, OP_MOVE_32_PD_PCDI, move_32_pd_pcdi),
        op_entry!(MASK_OUT_X, OP_MOVE_32_DI_PCDI, move_32_di_pcdi),
        op_entry!(MASK_OUT_X, OP_MOVE_32_IX_PCDI, move_32_ix_pcdi),
        op_entry!(MASK_EXACT, OP_MOVE_32_AW_PCDI, move_32_aw_pcdi),
        op_entry!(MASK_EXACT, OP_MOVE_32_AL_PCDI, move_32_al_pcdi),

        op_entry!(MASK_OUT_X, OP_MOVE_32_DN_PCIX, move_32_dn_pcix),
        op_entry!(MASK_OUT_X, OP_MOVE_32_AI_PCIX, move_32_ai_pcix),
        op_entry!(MASK_OUT_X, OP_MOVE_32_PI_PCIX, move_32_pi_pcix),
        op_entry!(MASK_OUT_X, OP_MOVE_32_PD_PCIX, move_32_pd_pcix),
        op_entry!(MASK_OUT_X, OP_MOVE_32_DI_PCIX, move_32_di_pcix),
        op_entry!(MASK_OUT_X, OP_MOVE_32_IX_PCIX, move_32_ix_pcix),
        op_entry!(MASK_EXACT, OP_MOVE_32_AW_PCIX, move_32_aw_pcix),
        op_entry!(MASK_EXACT, OP_MOVE_32_AL_PCIX, move_32_al_pcix),

        op_entry!(MASK_OUT_X, OP_MOVE_32_DN_IMM, move_32_dn_imm),
        op_entry!(MASK_OUT_X, OP_MOVE_32_AI_IMM, move_32_ai_imm),
        op_entry!(MASK_OUT_X, OP_MOVE_32_PI_IMM, move_32_pi_imm),
        op_entry!(MASK_OUT_X, OP_MOVE_32_PD_IMM, move_32_pd_imm),
        op_entry!(MASK_OUT_X, OP_MOVE_32_DI_IMM, move_32_di_imm),
        op_entry!(MASK_OUT_X, OP_MOVE_32_IX_IMM, move_32_ix_imm),
        op_entry!(MASK_EXACT, OP_MOVE_32_AW_IMM, move_32_aw_imm),
        op_entry!(MASK_EXACT, OP_MOVE_32_AL_IMM, move_32_al_imm),

        // Put op-entries for MOVEA here
        op_entry!(MASK_OUT_X_Y, OP_MOVEA_16_DN,   movea_16_dn),
        op_entry!(MASK_OUT_X_Y, OP_MOVEA_16_AN,   movea_16_an),
        op_entry!(MASK_OUT_X_Y, OP_MOVEA_16_AI,   movea_16_ai),
        op_entry!(MASK_OUT_X_Y, OP_MOVEA_16_PI,   movea_16_pi),
        op_entry!(MASK_OUT_X_Y, OP_MOVEA_16_PD,   movea_16_pd),
        op_entry!(MASK_OUT_X_Y, OP_MOVEA_16_DI,   movea_16_di),
        op_entry!(MASK_OUT_X_Y, OP_MOVEA_16_IX,   movea_16_ix),
        op_entry!(MASK_OUT_X,   OP_MOVEA_16_AW,   movea_16_aw),
        op_entry!(MASK_OUT_X,   OP_MOVEA_16_AL,   movea_16_al),
        op_entry!(MASK_OUT_X,   OP_MOVEA_16_PCDI, movea_16_pcdi),
        op_entry!(MASK_OUT_X,   OP_MOVEA_16_PCIX, movea_16_pcix),
        op_entry!(MASK_OUT_X,   OP_MOVEA_16_IMM,  movea_16_imm),

        op_entry!(MASK_OUT_X_Y, OP_MOVEA_32_DN,   movea_32_dn),
        op_entry!(MASK_OUT_X_Y, OP_MOVEA_32_AN,   movea_32_an),
        op_entry!(MASK_OUT_X_Y, OP_MOVEA_32_AI,   movea_32_ai),
        op_entry!(MASK_OUT_X_Y, OP_MOVEA_32_PI,   movea_32_pi),
        op_entry!(MASK_OUT_X_Y, OP_MOVEA_32_PD,   movea_32_pd),
        op_entry!(MASK_OUT_X_Y, OP_MOVEA_32_DI,   movea_32_di),
        op_entry!(MASK_OUT_X_Y, OP_MOVEA_32_IX,   movea_32_ix),
        op_entry!(MASK_OUT_X,   OP_MOVEA_32_AW,   movea_32_aw),
        op_entry!(MASK_OUT_X,   OP_MOVEA_32_AL,   movea_32_al),
        op_entry!(MASK_OUT_X,   OP_MOVEA_32_PCDI, movea_32_pcdi),
        op_entry!(MASK_OUT_X,   OP_MOVEA_32_PCIX, movea_32_pcix),
        op_entry!(MASK_OUT_X,   OP_MOVEA_32_IMM,  movea_32_imm),

        // Put op-entries for MOVE to CCR here
        op_entry!(MASK_OUT_Y, OP_MOVE_16_TOC_DN,   move_16_toc_dn),
        op_entry!(MASK_OUT_Y, OP_MOVE_16_TOC_AI,   move_16_toc_ai),
        op_entry!(MASK_OUT_Y, OP_MOVE_16_TOC_PI,   move_16_toc_pi),
        op_entry!(MASK_OUT_Y, OP_MOVE_16_TOC_PD,   move_16_toc_pd),
        op_entry!(MASK_OUT_Y, OP_MOVE_16_TOC_DI,   move_16_toc_di),
        op_entry!(MASK_OUT_Y, OP_MOVE_16_TOC_IX,   move_16_toc_ix),
        op_entry!(MASK_EXACT, OP_MOVE_16_TOC_AW,   move_16_toc_aw),
        op_entry!(MASK_EXACT, OP_MOVE_16_TOC_AL,   move_16_toc_al),
        op_entry!(MASK_EXACT, OP_MOVE_16_TOC_PCDI, move_16_toc_pcdi),
        op_entry!(MASK_EXACT, OP_MOVE_16_TOC_PCIX, move_16_toc_pcix),
        op_entry!(MASK_EXACT, OP_MOVE_16_TOC_IMM,  move_16_toc_imm),

        // Put op-entries for MOVE from SR here
        op_entry!(MASK_OUT_Y, OP_MOVE_16_FRS_DN, move_16_frs_dn),
        op_entry!(MASK_OUT_Y, OP_MOVE_16_FRS_AI, move_16_frs_ai),
        op_entry!(MASK_OUT_Y, OP_MOVE_16_FRS_PI, move_16_frs_pi),
        op_entry!(MASK_OUT_Y, OP_MOVE_16_FRS_PD, move_16_frs_pd),
        op_entry!(MASK_OUT_Y, OP_MOVE_16_FRS_DI, move_16_frs_di),
        op_entry!(MASK_OUT_Y, OP_MOVE_16_FRS_IX, move_16_frs_ix),
        op_entry!(MASK_EXACT, OP_MOVE_16_FRS_AW, move_16_frs_aw),
        op_entry!(MASK_EXACT, OP_MOVE_16_FRS_AL, move_16_frs_al),

        // Put op-entries for MOVE to SR here
        op_entry!(MASK_OUT_Y, OP_MOVE_16_TOS_DN, move_16_tos_dn),
        op_entry!(MASK_OUT_Y, OP_MOVE_16_TOS_AI, move_16_tos_ai),
        op_entry!(MASK_OUT_Y, OP_MOVE_16_TOS_PI, move_16_tos_pi),
        op_entry!(MASK_OUT_Y, OP_MOVE_16_TOS_PD, move_16_tos_pd),
        op_entry!(MASK_OUT_Y, OP_MOVE_16_TOS_DI, move_16_tos_di),
        op_entry!(MASK_OUT_Y, OP_MOVE_16_TOS_IX, move_16_tos_ix),
        op_entry!(MASK_EXACT, OP_MOVE_16_TOS_AW, move_16_tos_aw),
        op_entry!(MASK_EXACT, OP_MOVE_16_TOS_AL, move_16_tos_al),
        op_entry!(MASK_EXACT, OP_MOVE_16_TOS_PCDI, move_16_tos_pcdi),
        op_entry!(MASK_EXACT, OP_MOVE_16_TOS_PCIX, move_16_tos_pcix),
        op_entry!(MASK_EXACT, OP_MOVE_16_TOS_IMM, move_16_tos_imm),

        // Put op-entries for MOVE USP here
        op_entry!(MASK_OUT_Y, OP_MOVE_32_TOU, move_32_tou),
        op_entry!(MASK_OUT_Y, OP_MOVE_32_FRU, move_32_fru),

        // Put op-entries for MOVEM here
        op_entry!(MASK_OUT_Y, OP_MOVEM_16_RE_AI,   movem_16_re_ai),
        op_entry!(MASK_OUT_Y, OP_MOVEM_16_RE_PD,   movem_16_re_pd),
        op_entry!(MASK_OUT_Y, OP_MOVEM_16_RE_DI,   movem_16_re_di),
        op_entry!(MASK_OUT_Y, OP_MOVEM_16_RE_IX,   movem_16_re_ix),
        op_entry!(MASK_EXACT, OP_MOVEM_16_RE_AW,   movem_16_re_aw),
        op_entry!(MASK_EXACT, OP_MOVEM_16_RE_AL,   movem_16_re_al),

        op_entry!(MASK_OUT_Y, OP_MOVEM_16_ER_AI,   movem_16_er_ai),
        op_entry!(MASK_OUT_Y, OP_MOVEM_16_ER_PI,   movem_16_er_pi),
        op_entry!(MASK_OUT_Y, OP_MOVEM_16_ER_DI,   movem_16_er_di),
        op_entry!(MASK_OUT_Y, OP_MOVEM_16_ER_IX,   movem_16_er_ix),
        op_entry!(MASK_EXACT, OP_MOVEM_16_ER_AW,   movem_16_er_aw),
        op_entry!(MASK_EXACT, OP_MOVEM_16_ER_AL,   movem_16_er_al),
        op_entry!(MASK_EXACT, OP_MOVEM_16_ER_PCDI, movem_16_er_pcdi),
        op_entry!(MASK_EXACT, OP_MOVEM_16_ER_PCIX, movem_16_er_pcix),

        op_entry!(MASK_OUT_Y, OP_MOVEM_32_RE_AI,   movem_32_re_ai),
        op_entry!(MASK_OUT_Y, OP_MOVEM_32_RE_PD,   movem_32_re_pd),
        op_entry!(MASK_OUT_Y, OP_MOVEM_32_RE_DI,   movem_32_re_di),
        op_entry!(MASK_OUT_Y, OP_MOVEM_32_RE_IX,   movem_32_re_ix),
        op_entry!(MASK_EXACT, OP_MOVEM_32_RE_AW,   movem_32_re_aw),
        op_entry!(MASK_EXACT, OP_MOVEM_32_RE_AL,   movem_32_re_al),

        op_entry!(MASK_OUT_Y, OP_MOVEM_32_ER_AI,   movem_32_er_ai),
        op_entry!(MASK_OUT_Y, OP_MOVEM_32_ER_PI,   movem_32_er_pi),
        op_entry!(MASK_OUT_Y, OP_MOVEM_32_ER_DI,   movem_32_er_di),
        op_entry!(MASK_OUT_Y, OP_MOVEM_32_ER_IX,   movem_32_er_ix),
        op_entry!(MASK_EXACT, OP_MOVEM_32_ER_AW,   movem_32_er_aw),
        op_entry!(MASK_EXACT, OP_MOVEM_32_ER_AL,   movem_32_er_al),
        op_entry!(MASK_EXACT, OP_MOVEM_32_ER_PCDI, movem_32_er_pcdi),
        op_entry!(MASK_EXACT, OP_MOVEM_32_ER_PCIX, movem_32_er_pcix),

        // Put op-entries for MOVEP here
        op_entry!(MASK_OUT_X_Y, OP_MOVEP_16_ER, movep_16_er),
        op_entry!(MASK_OUT_X_Y, OP_MOVEP_16_RE, movep_16_re),
        op_entry!(MASK_OUT_X_Y, OP_MOVEP_32_ER, movep_32_er),
        op_entry!(MASK_OUT_X_Y, OP_MOVEP_32_RE, movep_32_re),

        // Put op-entries for MOVEQ here
        op_entry!(MASK_LOBYTX, OP_MOVEQ_32, moveq_32),

        // Put op-entries for MULS here
        op_entry!(MASK_OUT_X_Y, OP_MULS_16_DN, muls_16_dn),
        op_entry!(MASK_OUT_X_Y, OP_MULS_16_AI, muls_16_ai),
        op_entry!(MASK_OUT_X_Y, OP_MULS_16_PI, muls_16_pi),
        op_entry!(MASK_OUT_X_Y, OP_MULS_16_PD, muls_16_pd),
        op_entry!(MASK_OUT_X_Y, OP_MULS_16_DI, muls_16_di),
        op_entry!(MASK_OUT_X_Y, OP_MULS_16_IX, muls_16_ix),
        op_entry!(MASK_OUT_X,   OP_MULS_16_AW, muls_16_aw),
        op_entry!(MASK_OUT_X,   OP_MULS_16_AL, muls_16_al),
        op_entry!(MASK_OUT_X,   OP_MULS_16_PCDI, muls_16_pcdi),
        op_entry!(MASK_OUT_X,   OP_MULS_16_PCIX, muls_16_pcix),
        op_entry!(MASK_OUT_X,   OP_MULS_16_IMM, muls_16_imm),

        // Put op-entries for MULU here
        op_entry!(MASK_OUT_X_Y, OP_MULU_16_DN, mulu_16_dn),
        op_entry!(MASK_OUT_X_Y, OP_MULU_16_AI, mulu_16_ai),
        op_entry!(MASK_OUT_X_Y, OP_MULU_16_PI, mulu_16_pi),
        op_entry!(MASK_OUT_X_Y, OP_MULU_16_PD, mulu_16_pd),
        op_entry!(MASK_OUT_X_Y, OP_MULU_16_DI, mulu_16_di),
        op_entry!(MASK_OUT_X_Y, OP_MULU_16_IX, mulu_16_ix),
        op_entry!(MASK_OUT_X,   OP_MULU_16_AW, mulu_16_aw),
        op_entry!(MASK_OUT_X,   OP_MULU_16_AL, mulu_16_al),
        op_entry!(MASK_OUT_X,   OP_MULU_16_PCDI, mulu_16_pcdi),
        op_entry!(MASK_OUT_X,   OP_MULU_16_PCIX, mulu_16_pcix),
        op_entry!(MASK_OUT_X,   OP_MULU_16_IMM, mulu_16_imm),

        // Put op-entries for NBCD here
        op_entry!(MASK_OUT_Y, OP_NBCD_8_DN, nbcd_8_dn),
        op_entry!(MASK_OUT_Y, OP_NBCD_8_AI, nbcd_8_ai),
        op_entry!(MASK_OUT_Y, OP_NBCD_8_PI, nbcd_8_pi),
        op_entry!(MASK_OUT_Y, OP_NBCD_8_PD, nbcd_8_pd),
        op_entry!(MASK_OUT_Y, OP_NBCD_8_DI, nbcd_8_di),
        op_entry!(MASK_OUT_Y, OP_NBCD_8_IX, nbcd_8_ix),
        op_entry!(MASK_EXACT, OP_NBCD_8_AW, nbcd_8_aw),
        op_entry!(MASK_EXACT, OP_NBCD_8_AL, nbcd_8_al),

        // Put op-entries for NEG here
        op_entry!(MASK_OUT_Y, OP_NEG_8_DN, neg_8_dn),
        op_entry!(MASK_OUT_Y, OP_NEG_8_AI, neg_8_ai),
        op_entry!(MASK_OUT_Y, OP_NEG_8_PI, neg_8_pi),
        op_entry!(MASK_OUT_Y, OP_NEG_8_PD, neg_8_pd),
        op_entry!(MASK_OUT_Y, OP_NEG_8_DI, neg_8_di),
        op_entry!(MASK_OUT_Y, OP_NEG_8_IX, neg_8_ix),
        op_entry!(MASK_EXACT, OP_NEG_8_AW, neg_8_aw),
        op_entry!(MASK_EXACT, OP_NEG_8_AL, neg_8_al),

        op_entry!(MASK_OUT_Y, OP_NEG_16_DN, neg_16_dn),
        op_entry!(MASK_OUT_Y, OP_NEG_16_AI, neg_16_ai),
        op_entry!(MASK_OUT_Y, OP_NEG_16_PI, neg_16_pi),
        op_entry!(MASK_OUT_Y, OP_NEG_16_PD, neg_16_pd),
        op_entry!(MASK_OUT_Y, OP_NEG_16_DI, neg_16_di),
        op_entry!(MASK_OUT_Y, OP_NEG_16_IX, neg_16_ix),
        op_entry!(MASK_EXACT, OP_NEG_16_AW, neg_16_aw),
        op_entry!(MASK_EXACT, OP_NEG_16_AL, neg_16_al),

        op_entry!(MASK_OUT_Y, OP_NEG_32_DN, neg_32_dn),
        op_entry!(MASK_OUT_Y, OP_NEG_32_AI, neg_32_ai),
        op_entry!(MASK_OUT_Y, OP_NEG_32_PI, neg_32_pi),
        op_entry!(MASK_OUT_Y, OP_NEG_32_PD, neg_32_pd),
        op_entry!(MASK_OUT_Y, OP_NEG_32_DI, neg_32_di),
        op_entry!(MASK_OUT_Y, OP_NEG_32_IX, neg_32_ix),
        op_entry!(MASK_EXACT, OP_NEG_32_AW, neg_32_aw),
        op_entry!(MASK_EXACT, OP_NEG_32_AL, neg_32_al),

        // Put op-entries for NEGX here
        op_entry!(MASK_OUT_Y, OP_NEGX_8_DN, negx_8_dn),
        op_entry!(MASK_OUT_Y, OP_NEGX_8_AI, negx_8_ai),
        op_entry!(MASK_OUT_Y, OP_NEGX_8_PI, negx_8_pi),
        op_entry!(MASK_OUT_Y, OP_NEGX_8_PD, negx_8_pd),
        op_entry!(MASK_OUT_Y, OP_NEGX_8_DI, negx_8_di),
        op_entry!(MASK_OUT_Y, OP_NEGX_8_IX, negx_8_ix),
        op_entry!(MASK_EXACT, OP_NEGX_8_AW, negx_8_aw),
        op_entry!(MASK_EXACT, OP_NEGX_8_AL, negx_8_al),

        op_entry!(MASK_OUT_Y, OP_NEGX_16_DN, negx_16_dn),
        op_entry!(MASK_OUT_Y, OP_NEGX_16_AI, negx_16_ai),
        op_entry!(MASK_OUT_Y, OP_NEGX_16_PI, negx_16_pi),
        op_entry!(MASK_OUT_Y, OP_NEGX_16_PD, negx_16_pd),
        op_entry!(MASK_OUT_Y, OP_NEGX_16_DI, negx_16_di),
        op_entry!(MASK_OUT_Y, OP_NEGX_16_IX, negx_16_ix),
        op_entry!(MASK_EXACT, OP_NEGX_16_AW, negx_16_aw),
        op_entry!(MASK_EXACT, OP_NEGX_16_AL, negx_16_al),

        op_entry!(MASK_OUT_Y, OP_NEGX_32_DN, negx_32_dn),
        op_entry!(MASK_OUT_Y, OP_NEGX_32_AI, negx_32_ai),
        op_entry!(MASK_OUT_Y, OP_NEGX_32_PI, negx_32_pi),
        op_entry!(MASK_OUT_Y, OP_NEGX_32_PD, negx_32_pd),
        op_entry!(MASK_OUT_Y, OP_NEGX_32_DI, negx_32_di),
        op_entry!(MASK_OUT_Y, OP_NEGX_32_IX, negx_32_ix),
        op_entry!(MASK_EXACT, OP_NEGX_32_AW, negx_32_aw),
        op_entry!(MASK_EXACT, OP_NEGX_32_AL, negx_32_al),

        // Put op-entries for NOP here
        op_entry!(MASK_EXACT, OP_NOP, nop),

        // Put op-entries for NOT here
        op_entry!(MASK_OUT_Y, OP_NOT_8_DN, not_8_dn),
        op_entry!(MASK_OUT_Y, OP_NOT_8_AI, not_8_ai),
        op_entry!(MASK_OUT_Y, OP_NOT_8_PI, not_8_pi),
        op_entry!(MASK_OUT_Y, OP_NOT_8_PD, not_8_pd),
        op_entry!(MASK_OUT_Y, OP_NOT_8_DI, not_8_di),
        op_entry!(MASK_OUT_Y, OP_NOT_8_IX, not_8_ix),
        op_entry!(MASK_EXACT, OP_NOT_8_AW, not_8_aw),
        op_entry!(MASK_EXACT, OP_NOT_8_AL, not_8_al),

        op_entry!(MASK_OUT_Y, OP_NOT_16_DN, not_16_dn),
        op_entry!(MASK_OUT_Y, OP_NOT_16_AI, not_16_ai),
        op_entry!(MASK_OUT_Y, OP_NOT_16_PI, not_16_pi),
        op_entry!(MASK_OUT_Y, OP_NOT_16_PD, not_16_pd),
        op_entry!(MASK_OUT_Y, OP_NOT_16_DI, not_16_di),
        op_entry!(MASK_OUT_Y, OP_NOT_16_IX, not_16_ix),
        op_entry!(MASK_EXACT, OP_NOT_16_AW, not_16_aw),
        op_entry!(MASK_EXACT, OP_NOT_16_AL, not_16_al),

        op_entry!(MASK_OUT_Y, OP_NOT_32_DN, not_32_dn),
        op_entry!(MASK_OUT_Y, OP_NOT_32_AI, not_32_ai),
        op_entry!(MASK_OUT_Y, OP_NOT_32_PI, not_32_pi),
        op_entry!(MASK_OUT_Y, OP_NOT_32_PD, not_32_pd),
        op_entry!(MASK_OUT_Y, OP_NOT_32_DI, not_32_di),
        op_entry!(MASK_OUT_Y, OP_NOT_32_IX, not_32_ix),
        op_entry!(MASK_EXACT, OP_NOT_32_AW, not_32_aw),
        op_entry!(MASK_EXACT, OP_NOT_32_AL, not_32_al),

        // Put op-entries for OR here
        op_entry!(MASK_OUT_X_Y, OP_OR_8_ER_DN,   or_8_er_dn),
        op_entry!(MASK_OUT_X_Y, OP_OR_8_ER_AI,   or_8_er_ai),
        op_entry!(MASK_OUT_X_Y, OP_OR_8_ER_PI,   or_8_er_pi),
        op_entry!(MASK_OUT_X_Y, OP_OR_8_ER_PD,   or_8_er_pd),
        op_entry!(MASK_OUT_X_Y, OP_OR_8_ER_DI,   or_8_er_di),
        op_entry!(MASK_OUT_X_Y, OP_OR_8_ER_IX,   or_8_er_ix),
        op_entry!(MASK_OUT_X,   OP_OR_8_ER_AW,   or_8_er_aw),
        op_entry!(MASK_OUT_X,   OP_OR_8_ER_AL,   or_8_er_al),
        op_entry!(MASK_OUT_X,   OP_OR_8_ER_PCDI, or_8_er_pcdi),
        op_entry!(MASK_OUT_X,   OP_OR_8_ER_PCIX, or_8_er_pcix),
        op_entry!(MASK_OUT_X,   OP_OR_8_ER_IMM,  or_8_er_imm),

        op_entry!(MASK_OUT_X_Y, OP_OR_8_RE_AI,   or_8_re_ai),
        op_entry!(MASK_OUT_X_Y, OP_OR_8_RE_PI,   or_8_re_pi),
        op_entry!(MASK_OUT_X_Y, OP_OR_8_RE_PD,   or_8_re_pd),
        op_entry!(MASK_OUT_X_Y, OP_OR_8_RE_DI,   or_8_re_di),
        op_entry!(MASK_OUT_X_Y, OP_OR_8_RE_IX,   or_8_re_ix),
        op_entry!(MASK_OUT_X,   OP_OR_8_RE_AW,   or_8_re_aw),
        op_entry!(MASK_OUT_X,   OP_OR_8_RE_AL,   or_8_re_al),

        op_entry!(MASK_OUT_X_Y, OP_OR_16_ER_DN,   or_16_er_dn),
        op_entry!(MASK_OUT_X_Y, OP_OR_16_ER_AI,   or_16_er_ai),
        op_entry!(MASK_OUT_X_Y, OP_OR_16_ER_PI,   or_16_er_pi),
        op_entry!(MASK_OUT_X_Y, OP_OR_16_ER_PD,   or_16_er_pd),
        op_entry!(MASK_OUT_X_Y, OP_OR_16_ER_DI,   or_16_er_di),
        op_entry!(MASK_OUT_X_Y, OP_OR_16_ER_IX,   or_16_er_ix),
        op_entry!(MASK_OUT_X,   OP_OR_16_ER_AW,   or_16_er_aw),
        op_entry!(MASK_OUT_X,   OP_OR_16_ER_AL,   or_16_er_al),
        op_entry!(MASK_OUT_X,   OP_OR_16_ER_PCDI, or_16_er_pcdi),
        op_entry!(MASK_OUT_X,   OP_OR_16_ER_PCIX, or_16_er_pcix),
        op_entry!(MASK_OUT_X,   OP_OR_16_ER_IMM,  or_16_er_imm),

        op_entry!(MASK_OUT_X_Y, OP_OR_16_RE_AI,   or_16_re_ai),
        op_entry!(MASK_OUT_X_Y, OP_OR_16_RE_PI,   or_16_re_pi),
        op_entry!(MASK_OUT_X_Y, OP_OR_16_RE_PD,   or_16_re_pd),
        op_entry!(MASK_OUT_X_Y, OP_OR_16_RE_DI,   or_16_re_di),
        op_entry!(MASK_OUT_X_Y, OP_OR_16_RE_IX,   or_16_re_ix),
        op_entry!(MASK_OUT_X,   OP_OR_16_RE_AW,   or_16_re_aw),
        op_entry!(MASK_OUT_X,   OP_OR_16_RE_AL,   or_16_re_al),

        op_entry!(MASK_OUT_X_Y, OP_OR_32_ER_DN,   or_32_er_dn),
        op_entry!(MASK_OUT_X_Y, OP_OR_32_ER_AI,   or_32_er_ai),
        op_entry!(MASK_OUT_X_Y, OP_OR_32_ER_PI,   or_32_er_pi),
        op_entry!(MASK_OUT_X_Y, OP_OR_32_ER_PD,   or_32_er_pd),
        op_entry!(MASK_OUT_X_Y, OP_OR_32_ER_DI,   or_32_er_di),
        op_entry!(MASK_OUT_X_Y, OP_OR_32_ER_IX,   or_32_er_ix),
        op_entry!(MASK_OUT_X,   OP_OR_32_ER_AW,   or_32_er_aw),
        op_entry!(MASK_OUT_X,   OP_OR_32_ER_AL,   or_32_er_al),
        op_entry!(MASK_OUT_X,   OP_OR_32_ER_PCDI, or_32_er_pcdi),
        op_entry!(MASK_OUT_X,   OP_OR_32_ER_PCIX, or_32_er_pcix),
        op_entry!(MASK_OUT_X,   OP_OR_32_ER_IMM,  or_32_er_imm),

        op_entry!(MASK_OUT_X_Y, OP_OR_32_RE_AI,   or_32_re_ai),
        op_entry!(MASK_OUT_X_Y, OP_OR_32_RE_PI,   or_32_re_pi),
        op_entry!(MASK_OUT_X_Y, OP_OR_32_RE_PD,   or_32_re_pd),
        op_entry!(MASK_OUT_X_Y, OP_OR_32_RE_DI,   or_32_re_di),
        op_entry!(MASK_OUT_X_Y, OP_OR_32_RE_IX,   or_32_re_ix),
        op_entry!(MASK_OUT_X,   OP_OR_32_RE_AW,   or_32_re_aw),
        op_entry!(MASK_OUT_X,   OP_OR_32_RE_AL,   or_32_re_al),

        // Put op-entries for ORI here
        op_entry!(MASK_OUT_Y, OP_ORI_8_DN,   ori_8_dn),
        op_entry!(MASK_OUT_Y, OP_ORI_8_AI,   ori_8_ai),
        op_entry!(MASK_OUT_Y, OP_ORI_8_PI,   ori_8_pi),
        op_entry!(MASK_OUT_Y, OP_ORI_8_PD,   ori_8_pd),
        op_entry!(MASK_OUT_Y, OP_ORI_8_DI,   ori_8_di),
        op_entry!(MASK_OUT_Y, OP_ORI_8_IX,   ori_8_ix),
        op_entry!(MASK_EXACT, OP_ORI_8_AW,   ori_8_aw),
        op_entry!(MASK_EXACT, OP_ORI_8_AL,   ori_8_al),

        op_entry!(MASK_OUT_Y, OP_ORI_16_DN,   ori_16_dn),
        op_entry!(MASK_OUT_Y, OP_ORI_16_AI,   ori_16_ai),
        op_entry!(MASK_OUT_Y, OP_ORI_16_PI,   ori_16_pi),
        op_entry!(MASK_OUT_Y, OP_ORI_16_PD,   ori_16_pd),
        op_entry!(MASK_OUT_Y, OP_ORI_16_DI,   ori_16_di),
        op_entry!(MASK_OUT_Y, OP_ORI_16_IX,   ori_16_ix),
        op_entry!(MASK_EXACT, OP_ORI_16_AW,   ori_16_aw),
        op_entry!(MASK_EXACT, OP_ORI_16_AL,   ori_16_al),

        op_entry!(MASK_OUT_Y, OP_ORI_32_DN,   ori_32_dn),
        op_entry!(MASK_OUT_Y, OP_ORI_32_AI,   ori_32_ai),
        op_entry!(MASK_OUT_Y, OP_ORI_32_PI,   ori_32_pi),
        op_entry!(MASK_OUT_Y, OP_ORI_32_PD,   ori_32_pd),
        op_entry!(MASK_OUT_Y, OP_ORI_32_DI,   ori_32_di),
        op_entry!(MASK_OUT_Y, OP_ORI_32_IX,   ori_32_ix),
        op_entry!(MASK_EXACT, OP_ORI_32_AW,   ori_32_aw),
        op_entry!(MASK_EXACT, OP_ORI_32_AL,   ori_32_al),

        // Put op-entries for ORI to CCR here
        op_entry!(MASK_EXACT, OP_ORI_16_TOC,  ori_16_toc),

        // Put op-entries for ORI to SR here
        op_entry!(MASK_EXACT, OP_ORI_16_TOS,  ori_16_tos),

        // Put op-entries for PEA here
        op_entry!(MASK_OUT_Y, OP_PEA_32_AI,   pea_32_ai),
        op_entry!(MASK_OUT_Y, OP_PEA_32_DI,   pea_32_di),
        op_entry!(MASK_OUT_Y, OP_PEA_32_IX,   pea_32_ix),
        op_entry!(MASK_EXACT, OP_PEA_32_AW,   pea_32_aw),
        op_entry!(MASK_EXACT, OP_PEA_32_AL,   pea_32_al),
        op_entry!(MASK_EXACT, OP_PEA_32_PCDI, pea_32_pcdi),
        op_entry!(MASK_EXACT, OP_PEA_32_PCIX, pea_32_pcix),

        // Put op-entries for RESET here
        op_entry!(MASK_EXACT, OP_RESET, reset),

        // Put op-entries for ROL, ROR here
        op_entry!(MASK_OUT_X_Y, OP_ROR_8_S,  ror_8_s),
        op_entry!(MASK_OUT_X_Y, OP_ROR_16_S, ror_16_s),
        op_entry!(MASK_OUT_X_Y, OP_ROR_32_S, ror_32_s),
        op_entry!(MASK_OUT_X_Y, OP_ROR_8_R,  ror_8_r),
        op_entry!(MASK_OUT_X_Y, OP_ROR_16_R, ror_16_r),
        op_entry!(MASK_OUT_X_Y, OP_ROR_32_R, ror_32_r),

        op_entry!(MASK_OUT_X_Y, OP_ROL_8_S,  rol_8_s),
        op_entry!(MASK_OUT_X_Y, OP_ROL_16_S, rol_16_s),
        op_entry!(MASK_OUT_X_Y, OP_ROL_32_S, rol_32_s),
        op_entry!(MASK_OUT_X_Y, OP_ROL_8_R,  rol_8_r),
        op_entry!(MASK_OUT_X_Y, OP_ROL_16_R, rol_16_r),
        op_entry!(MASK_OUT_X_Y, OP_ROL_32_R, rol_32_r),

        op_entry!(MASK_OUT_Y, OP_ROL_16_AI, rol_16_ai),
        op_entry!(MASK_OUT_Y, OP_ROL_16_PI, rol_16_pi),
        op_entry!(MASK_OUT_Y, OP_ROL_16_PD, rol_16_pd),
        op_entry!(MASK_OUT_Y, OP_ROL_16_DI, rol_16_di),
        op_entry!(MASK_OUT_Y, OP_ROL_16_IX, rol_16_ix),
        op_entry!(MASK_EXACT, OP_ROL_16_AW, rol_16_aw),
        op_entry!(MASK_EXACT, OP_ROL_16_AL, rol_16_al),

        op_entry!(MASK_OUT_Y, OP_ROR_16_AI, ror_16_ai),
        op_entry!(MASK_OUT_Y, OP_ROR_16_PI, ror_16_pi),
        op_entry!(MASK_OUT_Y, OP_ROR_16_PD, ror_16_pd),
        op_entry!(MASK_OUT_Y, OP_ROR_16_DI, ror_16_di),
        op_entry!(MASK_OUT_Y, OP_ROR_16_IX, ror_16_ix),
        op_entry!(MASK_EXACT, OP_ROR_16_AW, ror_16_aw),
        op_entry!(MASK_EXACT, OP_ROR_16_AL, ror_16_al),

        // Put op-entries for ROXL, ROXR here
        op_entry!(MASK_OUT_X_Y, OP_ROXR_8_S,  roxr_8_s),
        op_entry!(MASK_OUT_X_Y, OP_ROXR_16_S, roxr_16_s),
        op_entry!(MASK_OUT_X_Y, OP_ROXR_32_S, roxr_32_s),
        op_entry!(MASK_OUT_X_Y, OP_ROXR_8_R,  roxr_8_r),
        op_entry!(MASK_OUT_X_Y, OP_ROXR_16_R, roxr_16_r),
        op_entry!(MASK_OUT_X_Y, OP_ROXR_32_R, roxr_32_r),

        op_entry!(MASK_OUT_X_Y, OP_ROXL_8_S,  roxl_8_s),
        op_entry!(MASK_OUT_X_Y, OP_ROXL_16_S, roxl_16_s),
        op_entry!(MASK_OUT_X_Y, OP_ROXL_32_S, roxl_32_s),
        op_entry!(MASK_OUT_X_Y, OP_ROXL_8_R,  roxl_8_r),
        op_entry!(MASK_OUT_X_Y, OP_ROXL_16_R, roxl_16_r),
        op_entry!(MASK_OUT_X_Y, OP_ROXL_32_R, roxl_32_r),

        op_entry!(MASK_OUT_Y, OP_ROXL_16_AI, roxl_16_ai),
        op_entry!(MASK_OUT_Y, OP_ROXL_16_PI, roxl_16_pi),
        op_entry!(MASK_OUT_Y, OP_ROXL_16_PD, roxl_16_pd),
        op_entry!(MASK_OUT_Y, OP_ROXL_16_DI, roxl_16_di),
        op_entry!(MASK_OUT_Y, OP_ROXL_16_IX, roxl_16_ix),
        op_entry!(MASK_EXACT, OP_ROXL_16_AW, roxl_16_aw),
        op_entry!(MASK_EXACT, OP_ROXL_16_AL, roxl_16_al),

        op_entry!(MASK_OUT_Y, OP_ROXR_16_AI, roxr_16_ai),
        op_entry!(MASK_OUT_Y, OP_ROXR_16_PI, roxr_16_pi),
        op_entry!(MASK_OUT_Y, OP_ROXR_16_PD, roxr_16_pd),
        op_entry!(MASK_OUT_Y, OP_ROXR_16_DI, roxr_16_di),
        op_entry!(MASK_OUT_Y, OP_ROXR_16_IX, roxr_16_ix),
        op_entry!(MASK_EXACT, OP_ROXR_16_AW, roxr_16_aw),
        op_entry!(MASK_EXACT, OP_ROXR_16_AL, roxr_16_al),

        // Put op-entries for RTE here
        op_entry!(MASK_EXACT, OP_RTE_32, rte_32),

        // Put op-entries for RTR here
        op_entry!(MASK_EXACT, OP_RTR_32, rtr_32),

        // Put op-entries for RTS here
        op_entry!(MASK_EXACT, OP_RTS_32, rts_32),

        // Put op-entries for SBCD here
        op_entry!(MASK_OUT_X_Y, OP_SBCD_8_RR, sbcd_8_rr),
        op_entry!(MASK_OUT_X_Y, OP_SBCD_8_MM, sbcd_8_mm),

        // Put op-entries for Scc here
        op_entry!(MASK_OUT_Y, OP_SCC_8_AI, scc_8_ai),
        op_entry!(MASK_EXACT, OP_SCC_8_AL, scc_8_al),
        op_entry!(MASK_EXACT, OP_SCC_8_AW, scc_8_aw),
        op_entry!(MASK_OUT_Y, OP_SCC_8_DN, scc_8_dn),
        op_entry!(MASK_OUT_Y, OP_SCC_8_DI, scc_8_di),
        op_entry!(MASK_OUT_Y, OP_SCC_8_IX, scc_8_ix),
        op_entry!(MASK_OUT_Y, OP_SCC_8_PD, scc_8_pd),
        op_entry!(MASK_OUT_Y, OP_SCC_8_PI, scc_8_pi),

        op_entry!(MASK_OUT_Y, OP_SCS_8_AI, scs_8_ai),
        op_entry!(MASK_EXACT, OP_SCS_8_AL, scs_8_al),
        op_entry!(MASK_EXACT, OP_SCS_8_AW, scs_8_aw),
        op_entry!(MASK_OUT_Y, OP_SCS_8_DN, scs_8_dn),
        op_entry!(MASK_OUT_Y, OP_SCS_8_DI, scs_8_di),
        op_entry!(MASK_OUT_Y, OP_SCS_8_IX, scs_8_ix),
        op_entry!(MASK_OUT_Y, OP_SCS_8_PD, scs_8_pd),
        op_entry!(MASK_OUT_Y, OP_SCS_8_PI, scs_8_pi),

        op_entry!(MASK_OUT_Y, OP_SEQ_8_AI, seq_8_ai),
        op_entry!(MASK_EXACT, OP_SEQ_8_AL, seq_8_al),
        op_entry!(MASK_EXACT, OP_SEQ_8_AW, seq_8_aw),
        op_entry!(MASK_OUT_Y, OP_SEQ_8_DN, seq_8_dn),
        op_entry!(MASK_OUT_Y, OP_SEQ_8_DI, seq_8_di),
        op_entry!(MASK_OUT_Y, OP_SEQ_8_IX, seq_8_ix),
        op_entry!(MASK_OUT_Y, OP_SEQ_8_PD, seq_8_pd),
        op_entry!(MASK_OUT_Y, OP_SEQ_8_PI, seq_8_pi),

        op_entry!(MASK_OUT_Y, OP_SF_8_AI, sf_8_ai),
        op_entry!(MASK_EXACT, OP_SF_8_AL, sf_8_al),
        op_entry!(MASK_EXACT, OP_SF_8_AW, sf_8_aw),
        op_entry!(MASK_OUT_Y, OP_SF_8_DN, sf_8_dn),
        op_entry!(MASK_OUT_Y, OP_SF_8_DI, sf_8_di),
        op_entry!(MASK_OUT_Y, OP_SF_8_IX, sf_8_ix),
        op_entry!(MASK_OUT_Y, OP_SF_8_PD, sf_8_pd),
        op_entry!(MASK_OUT_Y, OP_SF_8_PI, sf_8_pi),

        op_entry!(MASK_OUT_Y, OP_SGE_8_AI, sge_8_ai),
        op_entry!(MASK_EXACT, OP_SGE_8_AL, sge_8_al),
        op_entry!(MASK_EXACT, OP_SGE_8_AW, sge_8_aw),
        op_entry!(MASK_OUT_Y, OP_SGE_8_DN, sge_8_dn),
        op_entry!(MASK_OUT_Y, OP_SGE_8_DI, sge_8_di),
        op_entry!(MASK_OUT_Y, OP_SGE_8_IX, sge_8_ix),
        op_entry!(MASK_OUT_Y, OP_SGE_8_PD, sge_8_pd),
        op_entry!(MASK_OUT_Y, OP_SGE_8_PI, sge_8_pi),

        op_entry!(MASK_OUT_Y, OP_SGT_8_AI, sgt_8_ai),
        op_entry!(MASK_EXACT, OP_SGT_8_AL, sgt_8_al),
        op_entry!(MASK_EXACT, OP_SGT_8_AW, sgt_8_aw),
        op_entry!(MASK_OUT_Y, OP_SGT_8_DN, sgt_8_dn),
        op_entry!(MASK_OUT_Y, OP_SGT_8_DI, sgt_8_di),
        op_entry!(MASK_OUT_Y, OP_SGT_8_IX, sgt_8_ix),
        op_entry!(MASK_OUT_Y, OP_SGT_8_PD, sgt_8_pd),
        op_entry!(MASK_OUT_Y, OP_SGT_8_PI, sgt_8_pi),

        op_entry!(MASK_OUT_Y, OP_SHI_8_AI, shi_8_ai),
        op_entry!(MASK_EXACT, OP_SHI_8_AL, shi_8_al),
        op_entry!(MASK_EXACT, OP_SHI_8_AW, shi_8_aw),
        op_entry!(MASK_OUT_Y, OP_SHI_8_DN, shi_8_dn),
        op_entry!(MASK_OUT_Y, OP_SHI_8_DI, shi_8_di),
        op_entry!(MASK_OUT_Y, OP_SHI_8_IX, shi_8_ix),
        op_entry!(MASK_OUT_Y, OP_SHI_8_PD, shi_8_pd),
        op_entry!(MASK_OUT_Y, OP_SHI_8_PI, shi_8_pi),

        op_entry!(MASK_OUT_Y, OP_SLE_8_AI, sle_8_ai),
        op_entry!(MASK_EXACT, OP_SLE_8_AL, sle_8_al),
        op_entry!(MASK_EXACT, OP_SLE_8_AW, sle_8_aw),
        op_entry!(MASK_OUT_Y, OP_SLE_8_DN, sle_8_dn),
        op_entry!(MASK_OUT_Y, OP_SLE_8_DI, sle_8_di),
        op_entry!(MASK_OUT_Y, OP_SLE_8_IX, sle_8_ix),
        op_entry!(MASK_OUT_Y, OP_SLE_8_PD, sle_8_pd),
        op_entry!(MASK_OUT_Y, OP_SLE_8_PI, sle_8_pi),

        op_entry!(MASK_OUT_Y, OP_SLS_8_AI, sls_8_ai),
        op_entry!(MASK_EXACT, OP_SLS_8_AL, sls_8_al),
        op_entry!(MASK_EXACT, OP_SLS_8_AW, sls_8_aw),
        op_entry!(MASK_OUT_Y, OP_SLS_8_DN, sls_8_dn),
        op_entry!(MASK_OUT_Y, OP_SLS_8_DI, sls_8_di),
        op_entry!(MASK_OUT_Y, OP_SLS_8_IX, sls_8_ix),
        op_entry!(MASK_OUT_Y, OP_SLS_8_PD, sls_8_pd),
        op_entry!(MASK_OUT_Y, OP_SLS_8_PI, sls_8_pi),

        op_entry!(MASK_OUT_Y, OP_SLT_8_AI, slt_8_ai),
        op_entry!(MASK_EXACT, OP_SLT_8_AL, slt_8_al),
        op_entry!(MASK_EXACT, OP_SLT_8_AW, slt_8_aw),
        op_entry!(MASK_OUT_Y, OP_SLT_8_DN, slt_8_dn),
        op_entry!(MASK_OUT_Y, OP_SLT_8_DI, slt_8_di),
        op_entry!(MASK_OUT_Y, OP_SLT_8_IX, slt_8_ix),
        op_entry!(MASK_OUT_Y, OP_SLT_8_PD, slt_8_pd),
        op_entry!(MASK_OUT_Y, OP_SLT_8_PI, slt_8_pi),

        op_entry!(MASK_OUT_Y, OP_SMI_8_AI, smi_8_ai),
        op_entry!(MASK_EXACT, OP_SMI_8_AL, smi_8_al),
        op_entry!(MASK_EXACT, OP_SMI_8_AW, smi_8_aw),
        op_entry!(MASK_OUT_Y, OP_SMI_8_DN, smi_8_dn),
        op_entry!(MASK_OUT_Y, OP_SMI_8_DI, smi_8_di),
        op_entry!(MASK_OUT_Y, OP_SMI_8_IX, smi_8_ix),
        op_entry!(MASK_OUT_Y, OP_SMI_8_PD, smi_8_pd),
        op_entry!(MASK_OUT_Y, OP_SMI_8_PI, smi_8_pi),

        op_entry!(MASK_OUT_Y, OP_SNE_8_AI, sne_8_ai),
        op_entry!(MASK_EXACT, OP_SNE_8_AL, sne_8_al),
        op_entry!(MASK_EXACT, OP_SNE_8_AW, sne_8_aw),
        op_entry!(MASK_OUT_Y, OP_SNE_8_DN, sne_8_dn),
        op_entry!(MASK_OUT_Y, OP_SNE_8_DI, sne_8_di),
        op_entry!(MASK_OUT_Y, OP_SNE_8_IX, sne_8_ix),
        op_entry!(MASK_OUT_Y, OP_SNE_8_PD, sne_8_pd),
        op_entry!(MASK_OUT_Y, OP_SNE_8_PI, sne_8_pi),

        op_entry!(MASK_OUT_Y, OP_SPL_8_AI, spl_8_ai),
        op_entry!(MASK_EXACT, OP_SPL_8_AL, spl_8_al),
        op_entry!(MASK_EXACT, OP_SPL_8_AW, spl_8_aw),
        op_entry!(MASK_OUT_Y, OP_SPL_8_DN, spl_8_dn),
        op_entry!(MASK_OUT_Y, OP_SPL_8_DI, spl_8_di),
        op_entry!(MASK_OUT_Y, OP_SPL_8_IX, spl_8_ix),
        op_entry!(MASK_OUT_Y, OP_SPL_8_PD, spl_8_pd),
        op_entry!(MASK_OUT_Y, OP_SPL_8_PI, spl_8_pi),

        op_entry!(MASK_OUT_Y, OP_ST_8_AI, st_8_ai),
        op_entry!(MASK_EXACT, OP_ST_8_AL, st_8_al),
        op_entry!(MASK_EXACT, OP_ST_8_AW, st_8_aw),
        op_entry!(MASK_OUT_Y, OP_ST_8_DN, st_8_dn),
        op_entry!(MASK_OUT_Y, OP_ST_8_DI, st_8_di),
        op_entry!(MASK_OUT_Y, OP_ST_8_IX, st_8_ix),
        op_entry!(MASK_OUT_Y, OP_ST_8_PD, st_8_pd),
        op_entry!(MASK_OUT_Y, OP_ST_8_PI, st_8_pi),

        op_entry!(MASK_OUT_Y, OP_SVC_8_AI, svc_8_ai),
        op_entry!(MASK_EXACT, OP_SVC_8_AL, svc_8_al),
        op_entry!(MASK_EXACT, OP_SVC_8_AW, svc_8_aw),
        op_entry!(MASK_OUT_Y, OP_SVC_8_DN, svc_8_dn),
        op_entry!(MASK_OUT_Y, OP_SVC_8_DI, svc_8_di),
        op_entry!(MASK_OUT_Y, OP_SVC_8_IX, svc_8_ix),
        op_entry!(MASK_OUT_Y, OP_SVC_8_PD, svc_8_pd),
        op_entry!(MASK_OUT_Y, OP_SVC_8_PI, svc_8_pi),

        op_entry!(MASK_OUT_Y, OP_SVS_8_AI, svs_8_ai),
        op_entry!(MASK_EXACT, OP_SVS_8_AL, svs_8_al),
        op_entry!(MASK_EXACT, OP_SVS_8_AW, svs_8_aw),
        op_entry!(MASK_OUT_Y, OP_SVS_8_DN, svs_8_dn),
        op_entry!(MASK_OUT_Y, OP_SVS_8_DI, svs_8_di),
        op_entry!(MASK_OUT_Y, OP_SVS_8_IX, svs_8_ix),
        op_entry!(MASK_OUT_Y, OP_SVS_8_PD, svs_8_pd),
        op_entry!(MASK_OUT_Y, OP_SVS_8_PI, svs_8_pi),

        // Put op-entries for STOP here
        op_entry!(MASK_EXACT, OP_STOP, stop),

        // Put op-entries for SUB here
        op_entry!(MASK_OUT_X_Y, OP_SUB_8_ER_DN,   sub_8_er_dn),
        op_entry!(MASK_OUT_X_Y, OP_SUB_8_ER_AI,   sub_8_er_ai),
        op_entry!(MASK_OUT_X_Y, OP_SUB_8_ER_PI,   sub_8_er_pi),
        op_entry!(MASK_OUT_X_Y, OP_SUB_8_ER_PD,   sub_8_er_pd),
        op_entry!(MASK_OUT_X_Y, OP_SUB_8_ER_DI,   sub_8_er_di),
        op_entry!(MASK_OUT_X_Y, OP_SUB_8_ER_IX,   sub_8_er_ix),
        op_entry!(MASK_OUT_X,   OP_SUB_8_ER_AW,   sub_8_er_aw),
        op_entry!(MASK_OUT_X,   OP_SUB_8_ER_AL,   sub_8_er_al),
        op_entry!(MASK_OUT_X,   OP_SUB_8_ER_PCDI, sub_8_er_pcdi),
        op_entry!(MASK_OUT_X,   OP_SUB_8_ER_PCIX, sub_8_er_pcix),
        op_entry!(MASK_OUT_X,   OP_SUB_8_ER_IMM,  sub_8_er_imm),

        op_entry!(MASK_OUT_X_Y, OP_SUB_8_RE_AI,   sub_8_re_ai),
        op_entry!(MASK_OUT_X_Y, OP_SUB_8_RE_PI,   sub_8_re_pi),
        op_entry!(MASK_OUT_X_Y, OP_SUB_8_RE_PD,   sub_8_re_pd),
        op_entry!(MASK_OUT_X_Y, OP_SUB_8_RE_DI,   sub_8_re_di),
        op_entry!(MASK_OUT_X_Y, OP_SUB_8_RE_IX,   sub_8_re_ix),
        op_entry!(MASK_OUT_X,   OP_SUB_8_RE_AW,   sub_8_re_aw),
        op_entry!(MASK_OUT_X,   OP_SUB_8_RE_AL,   sub_8_re_al),

        op_entry!(MASK_OUT_X_Y, OP_SUB_16_ER_DN,   sub_16_er_dn),
        op_entry!(MASK_OUT_X_Y, OP_SUB_16_ER_AN,   sub_16_er_an),
        op_entry!(MASK_OUT_X_Y, OP_SUB_16_ER_AI,   sub_16_er_ai),
        op_entry!(MASK_OUT_X_Y, OP_SUB_16_ER_PI,   sub_16_er_pi),
        op_entry!(MASK_OUT_X_Y, OP_SUB_16_ER_PD,   sub_16_er_pd),
        op_entry!(MASK_OUT_X_Y, OP_SUB_16_ER_DI,   sub_16_er_di),
        op_entry!(MASK_OUT_X_Y, OP_SUB_16_ER_IX,   sub_16_er_ix),
        op_entry!(MASK_OUT_X,   OP_SUB_16_ER_AW,   sub_16_er_aw),
        op_entry!(MASK_OUT_X,   OP_SUB_16_ER_AL,   sub_16_er_al),
        op_entry!(MASK_OUT_X,   OP_SUB_16_ER_PCDI, sub_16_er_pcdi),
        op_entry!(MASK_OUT_X,   OP_SUB_16_ER_PCIX, sub_16_er_pcix),
        op_entry!(MASK_OUT_X,   OP_SUB_16_ER_IMM,  sub_16_er_imm),

        op_entry!(MASK_OUT_X_Y, OP_SUB_16_RE_AI,   sub_16_re_ai),
        op_entry!(MASK_OUT_X_Y, OP_SUB_16_RE_PI,   sub_16_re_pi),
        op_entry!(MASK_OUT_X_Y, OP_SUB_16_RE_PD,   sub_16_re_pd),
        op_entry!(MASK_OUT_X_Y, OP_SUB_16_RE_DI,   sub_16_re_di),
        op_entry!(MASK_OUT_X_Y, OP_SUB_16_RE_IX,   sub_16_re_ix),
        op_entry!(MASK_OUT_X,   OP_SUB_16_RE_AW,   sub_16_re_aw),
        op_entry!(MASK_OUT_X,   OP_SUB_16_RE_AL,   sub_16_re_al),

        op_entry!(MASK_OUT_X_Y, OP_SUB_32_ER_DN,   sub_32_er_dn),
        op_entry!(MASK_OUT_X_Y, OP_SUB_32_ER_AN,   sub_32_er_an),
        op_entry!(MASK_OUT_X_Y, OP_SUB_32_ER_AI,   sub_32_er_ai),
        op_entry!(MASK_OUT_X_Y, OP_SUB_32_ER_PI,   sub_32_er_pi),
        op_entry!(MASK_OUT_X_Y, OP_SUB_32_ER_PD,   sub_32_er_pd),
        op_entry!(MASK_OUT_X_Y, OP_SUB_32_ER_DI,   sub_32_er_di),
        op_entry!(MASK_OUT_X_Y, OP_SUB_32_ER_IX,   sub_32_er_ix),
        op_entry!(MASK_OUT_X,   OP_SUB_32_ER_AW,   sub_32_er_aw),
        op_entry!(MASK_OUT_X,   OP_SUB_32_ER_AL,   sub_32_er_al),
        op_entry!(MASK_OUT_X,   OP_SUB_32_ER_PCDI, sub_32_er_pcdi),
        op_entry!(MASK_OUT_X,   OP_SUB_32_ER_PCIX, sub_32_er_pcix),
        op_entry!(MASK_OUT_X,   OP_SUB_32_ER_IMM,  sub_32_er_imm),

        op_entry!(MASK_OUT_X_Y, OP_SUB_32_RE_AI,   sub_32_re_ai),
        op_entry!(MASK_OUT_X_Y, OP_SUB_32_RE_PI,   sub_32_re_pi),
        op_entry!(MASK_OUT_X_Y, OP_SUB_32_RE_PD,   sub_32_re_pd),
        op_entry!(MASK_OUT_X_Y, OP_SUB_32_RE_DI,   sub_32_re_di),
        op_entry!(MASK_OUT_X_Y, OP_SUB_32_RE_IX,   sub_32_re_ix),
        op_entry!(MASK_OUT_X,   OP_SUB_32_RE_AW,   sub_32_re_aw),
        op_entry!(MASK_OUT_X,   OP_SUB_32_RE_AL,   sub_32_re_al),

        op_entry!(MASK_OUT_X_Y, OP_SUBA_16_DN,   suba_16_dn),
        op_entry!(MASK_OUT_X_Y, OP_SUBA_16_AN,   suba_16_an),
        op_entry!(MASK_OUT_X_Y, OP_SUBA_16_AI,   suba_16_ai),
        op_entry!(MASK_OUT_X_Y, OP_SUBA_16_PI,   suba_16_pi),
        op_entry!(MASK_OUT_X_Y, OP_SUBA_16_PD,   suba_16_pd),
        op_entry!(MASK_OUT_X_Y, OP_SUBA_16_DI,   suba_16_di),
        op_entry!(MASK_OUT_X_Y, OP_SUBA_16_IX,   suba_16_ix),
        op_entry!(MASK_OUT_X,   OP_SUBA_16_AW,   suba_16_aw),
        op_entry!(MASK_OUT_X,   OP_SUBA_16_AL,   suba_16_al),
        op_entry!(MASK_OUT_X,   OP_SUBA_16_PCDI, suba_16_pcdi),
        op_entry!(MASK_OUT_X,   OP_SUBA_16_PCIX, suba_16_pcix),
        op_entry!(MASK_OUT_X,   OP_SUBA_16_IMM,  suba_16_imm),

        op_entry!(MASK_OUT_X_Y, OP_SUBA_32_DN,   suba_32_dn),
        op_entry!(MASK_OUT_X_Y, OP_SUBA_32_AN,   suba_32_an),
        op_entry!(MASK_OUT_X_Y, OP_SUBA_32_AI,   suba_32_ai),
        op_entry!(MASK_OUT_X_Y, OP_SUBA_32_PI,   suba_32_pi),
        op_entry!(MASK_OUT_X_Y, OP_SUBA_32_PD,   suba_32_pd),
        op_entry!(MASK_OUT_X_Y, OP_SUBA_32_DI,   suba_32_di),
        op_entry!(MASK_OUT_X_Y, OP_SUBA_32_IX,   suba_32_ix),
        op_entry!(MASK_OUT_X,   OP_SUBA_32_AW,   suba_32_aw),
        op_entry!(MASK_OUT_X,   OP_SUBA_32_AL,   suba_32_al),
        op_entry!(MASK_OUT_X,   OP_SUBA_32_PCDI, suba_32_pcdi),
        op_entry!(MASK_OUT_X,   OP_SUBA_32_PCIX, suba_32_pcix),
        op_entry!(MASK_OUT_X,   OP_SUBA_32_IMM,  suba_32_imm),

        op_entry!(MASK_OUT_Y, OP_SUBI_8_DN,   subi_8_dn),
        op_entry!(MASK_OUT_Y, OP_SUBI_8_AI,   subi_8_ai),
        op_entry!(MASK_OUT_Y, OP_SUBI_8_PI,   subi_8_pi),
        op_entry!(MASK_OUT_Y, OP_SUBI_8_PD,   subi_8_pd),
        op_entry!(MASK_OUT_Y, OP_SUBI_8_DI,   subi_8_di),
        op_entry!(MASK_OUT_Y, OP_SUBI_8_IX,   subi_8_ix),
        op_entry!(MASK_EXACT, OP_SUBI_8_AW,   subi_8_aw),
        op_entry!(MASK_EXACT, OP_SUBI_8_AL,   subi_8_al),

        op_entry!(MASK_OUT_Y, OP_SUBI_16_DN,   subi_16_dn),
        op_entry!(MASK_OUT_Y, OP_SUBI_16_AI,   subi_16_ai),
        op_entry!(MASK_OUT_Y, OP_SUBI_16_PI,   subi_16_pi),
        op_entry!(MASK_OUT_Y, OP_SUBI_16_PD,   subi_16_pd),
        op_entry!(MASK_OUT_Y, OP_SUBI_16_DI,   subi_16_di),
        op_entry!(MASK_OUT_Y, OP_SUBI_16_IX,   subi_16_ix),
        op_entry!(MASK_EXACT, OP_SUBI_16_AW,   subi_16_aw),
        op_entry!(MASK_EXACT, OP_SUBI_16_AL,   subi_16_al),

        op_entry!(MASK_OUT_Y, OP_SUBI_32_DN,   subi_32_dn),
        op_entry!(MASK_OUT_Y, OP_SUBI_32_AI,   subi_32_ai),
        op_entry!(MASK_OUT_Y, OP_SUBI_32_PI,   subi_32_pi),
        op_entry!(MASK_OUT_Y, OP_SUBI_32_PD,   subi_32_pd),
        op_entry!(MASK_OUT_Y, OP_SUBI_32_DI,   subi_32_di),
        op_entry!(MASK_OUT_Y, OP_SUBI_32_IX,   subi_32_ix),
        op_entry!(MASK_EXACT, OP_SUBI_32_AW,   subi_32_aw),
        op_entry!(MASK_EXACT, OP_SUBI_32_AL,   subi_32_al),

        op_entry!(MASK_OUT_X_Y, OP_SUBQ_8_DN, subq_8_dn),
        op_entry!(MASK_OUT_X_Y, OP_SUBQ_8_AI, subq_8_ai),
        op_entry!(MASK_OUT_X_Y, OP_SUBQ_8_PI, subq_8_pi),
        op_entry!(MASK_OUT_X_Y, OP_SUBQ_8_PD, subq_8_pd),
        op_entry!(MASK_OUT_X_Y, OP_SUBQ_8_DI, subq_8_di),
        op_entry!(MASK_OUT_X_Y, OP_SUBQ_8_IX, subq_8_ix),
        op_entry!(MASK_OUT_X,   OP_SUBQ_8_AW, subq_8_aw),
        op_entry!(MASK_OUT_X,   OP_SUBQ_8_AL, subq_8_al),

        op_entry!(MASK_OUT_X_Y, OP_SUBQ_16_DN, subq_16_dn),
        op_entry!(MASK_OUT_X_Y, OP_SUBQ_16_AN, subq_16_an),
        op_entry!(MASK_OUT_X_Y, OP_SUBQ_16_AI, subq_16_ai),
        op_entry!(MASK_OUT_X_Y, OP_SUBQ_16_PI, subq_16_pi),
        op_entry!(MASK_OUT_X_Y, OP_SUBQ_16_PD, subq_16_pd),
        op_entry!(MASK_OUT_X_Y, OP_SUBQ_16_DI, subq_16_di),
        op_entry!(MASK_OUT_X_Y, OP_SUBQ_16_IX, subq_16_ix),
        op_entry!(MASK_OUT_X,   OP_SUBQ_16_AW, subq_16_aw),
        op_entry!(MASK_OUT_X,   OP_SUBQ_16_AL, subq_16_al),

        op_entry!(MASK_OUT_X_Y, OP_SUBQ_32_DN, subq_32_dn),
        op_entry!(MASK_OUT_X_Y, OP_SUBQ_32_AN, subq_32_an),
        op_entry!(MASK_OUT_X_Y, OP_SUBQ_32_AI, subq_32_ai),
        op_entry!(MASK_OUT_X_Y, OP_SUBQ_32_PI, subq_32_pi),
        op_entry!(MASK_OUT_X_Y, OP_SUBQ_32_PD, subq_32_pd),
        op_entry!(MASK_OUT_X_Y, OP_SUBQ_32_DI, subq_32_di),
        op_entry!(MASK_OUT_X_Y, OP_SUBQ_32_IX, subq_32_ix),
        op_entry!(MASK_OUT_X,   OP_SUBQ_32_AW, subq_32_aw),
        op_entry!(MASK_OUT_X,   OP_SUBQ_32_AL, subq_32_al),

        op_entry!(MASK_OUT_X_Y, OP_SUBX_8_RR,  subx_8_rr),
        op_entry!(MASK_OUT_X_Y, OP_SUBX_8_MM,  subx_8_mm),
        op_entry!(MASK_OUT_X_Y, OP_SUBX_16_RR, subx_16_rr),
        op_entry!(MASK_OUT_X_Y, OP_SUBX_16_MM, subx_16_mm),
        op_entry!(MASK_OUT_X_Y, OP_SUBX_32_RR, subx_32_rr),
        op_entry!(MASK_OUT_X_Y, OP_SUBX_32_MM, subx_32_mm),

        // Put op-entries for SWAP here
        op_entry!(MASK_OUT_Y, OP_SWAP_32_DN, swap_32_dn),

        // Put op-entries for TAS here
        op_entry!(MASK_OUT_Y, OP_TAS_8_DN, tas_8_dn),
        op_entry!(MASK_OUT_Y, OP_TAS_8_AI, tas_8_ai),
        op_entry!(MASK_OUT_Y, OP_TAS_8_PI, tas_8_pi),
        op_entry!(MASK_OUT_Y, OP_TAS_8_PD, tas_8_pd),
        op_entry!(MASK_OUT_Y, OP_TAS_8_DI, tas_8_di),
        op_entry!(MASK_OUT_Y, OP_TAS_8_IX, tas_8_ix),
        op_entry!(MASK_EXACT, OP_TAS_8_AW, tas_8_aw),
        op_entry!(MASK_EXACT, OP_TAS_8_AL, tas_8_al),

        // Put op-entries for TRAP here
        op_entry!(MASK_LONIB, OP_TRAP, trap),

        // Put op-entries for TRAPV here
        op_entry!(MASK_EXACT, OP_TRAPV, trapv),

        // Put op-entries for TST here
        // Put op-entries for UNLK here
    ];
    optable
}

pub fn generate() -> InstructionSet {
    // Covers all possible IR values (64k entries)
    let mut handler: InstructionSet = Vec::with_capacity(0x10000);
    for _ in 0..0x10000 { handler.push(illegal); }

    // two of the commonly used op-masks (MASK_OUT_X (280+ uses) and
    // MASK_OUT_X_Y (500+)) are non-contiguous, so optimize for that.
    // This saves millions of iterations of the innermost loop below.
    // The odd mask MASK_LOBYTX (8 blocks of 256 opcodes) is used only
    // for the MOVEQ instruction, saving 1792 iterations, but was cheap
    // to include.

    // The X register is selected by bit 9-11, which gives the offsets
    // in this table
    fn x_offset(len: u32) -> [(u32, u32); 8] {
                  [  (0, len),
                   (512, len),
                  (1024, len),
                  (1536, len),
                  (2048, len),
                  (2560, len),
                  (3072, len),
                  (3584, len)]
    }
    let mut offset_cache = HashMap::new();
    offset_cache.insert(MASK_OUT_X, x_offset(1));
    offset_cache.insert(MASK_OUT_X_Y, x_offset(8));
    offset_cache.insert(MASK_LOBYTX, x_offset(256));
    let optable = generate_optable();
    let _ops = optable.len();
    let mut _implemented = 0;

    for op in optable {
        match offset_cache.get(&op.mask) {
            Some(offsets) => {
                for opcode in offsets.iter().flat_map(|&(start, len)| (start..(start+len)).map(|o| o + op.matching)) {
                    handler[opcode as usize] = op.handler;
                    _implemented += 1;
                }
            },
            None => {
                // the remaining masks are all contiguous, and already optimal
                let max_count = 1 << (op.mask as u16).count_zeros();
                let mut matching = 0;
                for opcode in op.matching..0x10000 {
                    if (opcode & op.mask) == op.matching {
                        handler[opcode as usize] = op.handler;
                        _implemented += 1;
                        matching += 1;
                        if matching >= max_count {
                            break;
                        }
                    }
                }
            }
        }
    }
    // According to Musashi opcode handler jump table;
    // M68000 implements 54007 opcodes (11529 illegal)
    // M68010 implements 54194 opcodes (11342 illegal)
    // M68020 implements 55611 opcodes (9925 illegal)
    // println!("{:?} opcodes implemented ({:.2}% done) in {:?} instruction variants", _implemented, _implemented as f32 / 540.07f32, _ops);
    handler
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optable_mask_and_matching_makes_sense() {
        let optable = super::generate_optable();

        for op in optable {
            if op.mask & op.matching != op.matching {
                panic!("Error generating op handler table: Op mask {:16b} and matching {:16b} is inconsistent for {}", op.mask, op.matching, op.name);
            }
        }
    }

    #[test]
    fn different_ops() {
        assert!(OP_ADDX_16_MM != OP_ADD_16_ER_AN);
    }
    #[test]
    fn correctly_defined_op_andi_16_toc() {
        assert_eq!(0x023c, OP_ANDI_16_TOC);
    }
    #[test]
    fn correctly_defined_asl_32_s() {
        assert_eq!(0xe180, OP_ASL_32_S);
    }
    #[test]
    fn correctly_defined_asr_16_aw() {
        assert_eq!(0xe0f8, OP_ASR_16_AW);
    }
    #[test]
    fn correctly_defined_asl_16_ix() {
        assert_eq!(0xe1f0, OP_ASL_16_IX);
    }
    #[test]
    fn correctly_defined_asl_16_r() {
        assert_eq!(0xe160, OP_ASL_16_R);
    }
    #[test]
    fn correctly_defined_asr_8_r() {
        assert_eq!(0xe020, OP_ASR_8_R);
    }
    #[test]
    fn correctly_defined_bchg_32_r_dn() {
        assert_eq!(0x0140, OP_BCHG_32_R_DN);
    }
    #[test]
    fn correctly_defined_bchg_32_s_dn() {
        assert_eq!(0x0840, OP_BCHG_32_S_DN);
    }
    #[test]
    fn correctly_defined_bchg_8_r_ix() {
        assert_eq!(0x0170, OP_BCHG_8_R_IX);
    }
    #[test]
    fn correctly_defined_bchg_8_s_aw() {
        assert_eq!(0x0878, OP_BCHG_8_S_AW);
    }
    #[test]
    fn correctly_defined_chk_16_pd() {
        assert_eq!(0x41a0, OP_CHK_16_PD);
    }
    #[test]
    fn correctly_defined_clr_16_pi() {
        assert_eq!(0x4258, OP_CLR_16_PI);
    }
    #[test]
    fn correctly_defined_cmp_32_di() {
        assert_eq!(0xb0a8, OP_CMP_32_DI);
    }
    #[test]
    fn correctly_defined_cmpa_32_an() {
        assert_eq!(0xb1c8, OP_CMPA_32_AN);
    }
    #[test]
    fn correctly_defined_op_cmpi_8_ai() {
        assert_eq!(0x0c10, OP_CMPI_8_AI);
    }
    #[test]
    fn correctly_defined_op_cmpm_16() {
        assert_eq!(0xb148, OP_CMPM_16);
    }
    #[test]
    fn correctly_defined_op_dbhi_16() {
        assert_eq!(0x52c8, OP_DBHI_16);
    }
    #[test]
    fn correctly_defined_op_divs_16_dn() {
        assert_eq!(0x81c0, OP_DIVS_16_DN);
    }
    #[test]
    fn correctly_defined_op_divu_16_ix() {
        assert_eq!(0x80f0, OP_DIVU_16_IX);
    }
    #[test]
    fn correctly_defined_op_eor_8_pd() {
        assert_eq!(0xb120, OP_EOR_8_PD);
    }
    #[test]
    fn correctly_defined_op_eori_32_di() {
        assert_eq!(0x0aa8, OP_EORI_32_DI);
    }
    #[test]
    fn correctly_defined_op_eori_16_tos() {
        assert_eq!(0x0a7c, OP_EORI_16_TOS);
    }
    #[test]
    fn correctly_defined_op_exg_32_aa() {
        assert_eq!(0xc148, OP_EXG_32_AA);
    }
    #[test]
    fn correctly_defined_op_ext_wl() {
        assert_eq!(0x48c0, OP_EXT_WL);
    }
    #[test]
    fn correctly_defined_op_jmp_32_pcdi() {
        assert_eq!(0x4efa, OP_JMP_32_PCDI);
    }
    #[test]
    fn correctly_defined_op_jsr_32_ix() {
        assert_eq!(0x4eb0, OP_JSR_32_IX);
    }
    #[test]
    fn correctly_defined_op_lea_32_ai() {
        assert_eq!(0x41d0, OP_LEA_32_AI);
    }
    #[test]
    fn correctly_defined_op_link_16() {
        assert_eq!(0x4e50, OP_LINK_16);
    }
    #[test]
    fn correctly_defined_op_lsl_8_s() {
        assert_eq!(0xe108, OP_LSL_8_S);
    }
    #[test]
    fn correctly_defined_op_lsr_16_r() {
        assert_eq!(0xe068, OP_LSR_16_R);
    }
    #[test]
    fn correctly_defined_op_lsr_32_r() {
        assert_eq!(0xe0a8, OP_LSR_32_R);
    }
    #[test]
    fn correctly_defined_op_lsl_16_aw() {
        assert_eq!(0xe3f8, OP_LSL_16_AW);
    }
    #[test]
    fn correctly_defined_op_lsr_16_ix() {
        assert_eq!(0xe2f0, OP_LSR_16_IX);
    }
    #[test]
    fn correctly_defined_rol_32_s() {
        assert_eq!(0xe198, OP_ROL_32_S);
    }
    #[test]
    fn correctly_defined_ror_16_al() {
        assert_eq!(0xe6f9, OP_ROR_16_AL);
    }
    #[test]
    fn correctly_defined_rol_16_pd() {
        assert_eq!(0xe7e0, OP_ROL_16_PD);
    }
    #[test]
    fn correctly_defined_rol_16_r() {
        assert_eq!(0xe178, OP_ROL_16_R);
    }
    #[test]
    fn correctly_defined_ror_8_r() {
        assert_eq!(0xe038, OP_ROR_8_R);
    }
    #[test]
    fn correctly_defined_roxl_32_r() {
        assert_eq!(0xe1b0, OP_ROXL_32_R);
    }
    #[test]
    fn correctly_defined_roxr_16_ai() {
        assert_eq!(0xe4d0, OP_ROXR_16_AI);
    }
    #[test]
    fn correctly_defined_roxl_16_pi() {
        assert_eq!(0xe5d8, OP_ROXL_16_PI);
    }
    #[test]
    fn correctly_defined_roxl_16_s() {
        assert_eq!(0xe150, OP_ROXL_16_S);
    }
    #[test]
    fn correctly_defined_roxr_8_r() {
        assert_eq!(0xe030, OP_ROXR_8_R);
    }
    #[test]
    fn correctly_defined_op_scc_8_dn() {
        assert_eq!(0x54c0, OP_SCC_8_DN);
    }
    #[test]
    fn correctly_defined_op_scs_8_dn() {
        assert_eq!(0x55c0, OP_SCS_8_DN);
    }
    #[test]
    fn correctly_defined_op_seq_8_dn() {
        assert_eq!(0x57c0, OP_SEQ_8_DN);
    }
    #[test]
    fn correctly_defined_op_sf_8_dn() {
        assert_eq!(0x51c0, OP_SF_8_DN);
    }
    #[test]
    fn correctly_defined_op_sge_8_dn() {
        assert_eq!(0x5cc0, OP_SGE_8_DN);
    }
    #[test]
    fn correctly_defined_op_sgt_8_dn() {
        assert_eq!(0x5ec0, OP_SGT_8_DN);
    }
    #[test]
    fn correctly_defined_op_shi_8_dn() {
        assert_eq!(0x52c0, OP_SHI_8_DN);
    }
    #[test]
    fn correctly_defined_op_sle_8_dn() {
        assert_eq!(0x5fc0, OP_SLE_8_DN);
    }
    #[test]
    fn correctly_defined_op_sls_8_dn() {
        assert_eq!(0x53c0, OP_SLS_8_DN);
    }
    #[test]
    fn correctly_defined_op_slt_8_dn() {
        assert_eq!(0x5dc0, OP_SLT_8_DN);
    }
    #[test]
    fn correctly_defined_op_smi_8_dn() {
        assert_eq!(0x5bc0, OP_SMI_8_DN);
    }
    #[test]
    fn correctly_defined_op_sne_8_dn() {
        assert_eq!(0x56c0, OP_SNE_8_DN);
    }
    #[test]
    fn correctly_defined_op_spl_8_dn() {
        assert_eq!(0x5ac0, OP_SPL_8_DN);
    }
    #[test]
    fn correctly_defined_op_st_8_dn() {
        assert_eq!(0x50c0, OP_ST_8_DN);
    }
    #[test]
    fn correctly_defined_op_svc_8_dn() {
        assert_eq!(0x58c0, OP_SVC_8_DN);
    }
    #[test]
    fn correctly_defined_op_svs_8_dn() {
        assert_eq!(0x59c0, OP_SVS_8_DN);
    }
    #[test]
    fn correctly_defined_op_move_8_ai_pd() {
        assert_eq!(0x10a0, OP_MOVE_8_AI_PD);
    }
    #[test]
    fn correctly_defined_op_move_8_pd_ai() {
        assert_eq!(0x1110, OP_MOVE_8_PD_AI);
    }
    #[test]
    fn correctly_defined_op_move_8_di_pcix() {
        assert_eq!(0x117b, OP_MOVE_8_DI_PCIX);
    }
    #[test]
    fn correctly_defined_op_move_8_al_di() {
        assert_eq!(0x13e8, OP_MOVE_8_AL_DI);
    }
    #[test]
    fn correctly_defined_op_move_16_dn_al() {
        assert_eq!(0x3039, OP_MOVE_16_DN_AL);
    }
    #[test]
    fn correctly_defined_op_move_16_ai_dn() {
        assert_eq!(0x3080, OP_MOVE_16_AI_DN);
    }
    #[test]
    fn correctly_defined_op_move_16_pd_pcix() {
        assert_eq!(0x313b, OP_MOVE_16_PD_PCIX);
    }
    #[test]
    fn correctly_defined_op_move_16_pi_imm() {
        assert_eq!(0x30fc, OP_MOVE_16_PI_IMM);
    }
    #[test]
    fn correctly_defined_op_move_32_ai_imm() {
        assert_eq!(0x20bc, OP_MOVE_32_AI_IMM);
    }
    #[test]
    fn correctly_defined_op_move_32_pd_pcdi() {
        assert_eq!(0x213a, OP_MOVE_32_PD_PCDI);
    }
    #[test]
    fn correctly_defined_op_move_32_dn_dn() {
        assert_eq!(0x2000, OP_MOVE_32_DN_DN);
    }
    #[test]
    fn correctly_defined_op_move_32_aw_al() {
        assert_eq!(0x21f9, OP_MOVE_32_AW_AL);
    }
    #[test]
    fn correctly_defined_op_movea_16_ai() {
        assert_eq!(0x3050, OP_MOVEA_16_AI);
    }
    #[test]
    fn correctly_defined_op_movea_16_imm() {
        assert_eq!(0x3060, OP_MOVEA_16_PD);
    }
    #[test]
    fn correctly_defined_op_movea_32_imm() {
        assert_eq!(0x207c, OP_MOVEA_32_IMM);
    }
    #[test]
    fn correctly_defined_op_movea_32_di() {
        assert_eq!(0x2068, OP_MOVEA_32_DI);
    }
    #[test]
    fn correctly_defined_op_move_16_toc_ai() {
        assert_eq!(0x44d0, OP_MOVE_16_TOC_AI)
    }
    #[test]
    fn correctly_defined_op_move_16_toc_pcix() {
        assert_eq!(0x44fb, OP_MOVE_16_TOC_PCIX)
    }
    #[test]
    fn correctly_defined_op_move_16_frs_al() {
        assert_eq!(0x40f9, OP_MOVE_16_FRS_AL)
    }
    #[test]
    fn correctly_defined_op_move_16_tos_imm() {
        assert_eq!(0x46fc, OP_MOVE_16_TOS_IMM)
    }
    #[test]
    fn correctly_defined_op_move_32_fru() {
        assert_eq!(0x4e68, OP_MOVE_32_FRU)
    }
    #[test]
    fn correctly_defined_op_move_32_tou() {
        assert_eq!(0x4e60, OP_MOVE_32_TOU)
    }
    #[test]
    fn correctly_defined_op_movem_16_er_ai() {
        assert_eq!(0x4c90, OP_MOVEM_16_ER_AI)
    }
    #[test]
    fn correctly_defined_op_movem_16_re_pd() {
        assert_eq!(0x48a0, OP_MOVEM_16_RE_PD)
    }
    #[test]
    fn correctly_defined_op_movem_32_er_al() {
        assert_eq!(0x4cf9, OP_MOVEM_32_ER_AL)
    }
    #[test]
    fn correctly_defined_op_movem_32_re_di() {
        assert_eq!(0x48e8, OP_MOVEM_32_RE_DI)
    }
    #[test]
    fn correctly_defined_op_movep_16_re() {
        assert_eq!(0x0188, OP_MOVEP_16_RE)
    }
    #[test]
    fn correctly_defined_op_movep_32_er() {
        assert_eq!(0x0148, OP_MOVEP_32_ER)
    }
    #[test]
    fn correctly_defined_op_moveq_32() {
        assert_eq!(0x7000, OP_MOVEQ_32)
    }
    #[test]
    fn correctly_defined_op_muls_16_dn() {
        assert_eq!(0xc1c0, OP_MULS_16_DN)
    }
    #[test]
    fn correctly_defined_op_muls_16_imm() {
        assert_eq!(0xc1fc, OP_MULS_16_IMM)
    }
    #[test]
    fn correctly_defined_op_muls_16_aw() {
        assert_eq!(0xc1f8, OP_MULS_16_AW)
    }
    #[test]
    fn correctly_defined_op_mulu_16_ai() {
        assert_eq!(0xc0d0, OP_MULU_16_AI)
    }
    #[test]
    fn correctly_defined_op_mulu_16_pcdi() {
        assert_eq!(0xc0fa, OP_MULU_16_PCDI)
    }
    #[test]
    fn correctly_defined_op_mulu_16_al() {
        assert_eq!(0xc0f9, OP_MULU_16_AL)
    }
    #[test]
    fn correctly_defined_op_nbcd_8_ix() {
        assert_eq!(0x4830, OP_NBCD_8_IX)
    }
    #[test]
    fn correctly_defined_op_nbcd_8_pd() {
        assert_eq!(0x4820, OP_NBCD_8_PD)
    }
    #[test]
    fn correctly_defined_op_neg_8_pi() {
        assert_eq!(0x4418, OP_NEG_8_PI)
    }
    #[test]
    fn correctly_defined_op_neg_8_di() {
        assert_eq!(0x4428, OP_NEG_8_DI)
    }
    #[test]
    fn correctly_defined_op_neg_16_ai() {
        assert_eq!(0x4450, OP_NEG_16_AI)
    }
    #[test]
    fn correctly_defined_op_neg_16_ix() {
        assert_eq!(0x4470, OP_NEG_16_IX)
    }
    #[test]
    fn correctly_defined_op_neg_32_al() {
        assert_eq!(0x44b9, OP_NEG_32_AL)
    }
    #[test]
    fn correctly_defined_op_neg_32_pd() {
        assert_eq!(0x44a0, OP_NEG_32_PD)
    }
    #[test]
    fn correctly_defined_op_negx_8_pi() {
        assert_eq!(0x4018, OP_NEGX_8_PI)
    }
    #[test]
    fn correctly_defined_op_negx_8_dn() {
        assert_eq!(0x4000, OP_NEGX_8_DN)
    }
    #[test]
    fn correctly_defined_op_negx_16_ai() {
        assert_eq!(0x4050, OP_NEGX_16_AI)
    }
    #[test]
    fn correctly_defined_op_negx_16_ix() {
        assert_eq!(0x4070, OP_NEGX_16_IX)
    }
    #[test]
    fn correctly_defined_op_negx_32_al() {
        assert_eq!(0x40b9, OP_NEGX_32_AL)
    }
    #[test]
    fn correctly_defined_op_negx_32_pi() {
        assert_eq!(0x4098, OP_NEGX_32_PI)
    }
    #[test]
    fn correctly_defined_op_nop() {
        assert_eq!(0x4e71, OP_NOP)
    }
    #[test]
    fn correctly_defined_op_not_8_pi() {
        assert_eq!(0x4618, OP_NOT_8_PI)
    }
    #[test]
    fn correctly_defined_op_not_8_di() {
        assert_eq!(0x4628, OP_NOT_8_DI)
    }
    #[test]
    fn correctly_defined_op_not_16_ai() {
        assert_eq!(0x4650, OP_NOT_16_AI)
    }
    #[test]
    fn correctly_defined_op_not_16_ix() {
        assert_eq!(0x4670, OP_NOT_16_IX)
    }
    #[test]
    fn correctly_defined_op_not_32_al() {
        assert_eq!(0x46b9, OP_NOT_32_AL)
    }
    #[test]
    fn correctly_defined_op_not_32_pd() {
        assert_eq!(0x46a0, OP_NOT_32_PD)
    }
    #[test]
    fn correctly_defined_op_or_8_re_al() {
        assert_eq!(0x8139, OP_OR_8_RE_AL)
    }
    #[test]
    fn correctly_defined_op_or_8_er_pcix() {
        assert_eq!(0x803b, OP_OR_8_ER_PCIX)
    }
    #[test]
    fn correctly_defined_op_or_16_re_pd() {
        assert_eq!(0x8160, OP_OR_16_RE_PD)
    }
    #[test]
    fn correctly_defined_op_or_16_er_ai() {
        assert_eq!(0x8050, OP_OR_16_ER_AI)
    }
    #[test]
    fn correctly_defined_op_or_32_re_pi() {
        assert_eq!(0x8198, OP_OR_32_RE_PI)
    }
    #[test]
    fn correctly_defined_op_or_32_er_ix() {
        assert_eq!(0x80b0, OP_OR_32_ER_IX)
    }
    #[test]
    fn correctly_defined_op_ori_8_di() {
        assert_eq!(0x0028, OP_ORI_8_DI)
    }
    #[test]
    fn correctly_defined_op_ori_16_dn() {
        assert_eq!(0x0040, OP_ORI_16_DN)
    }
    #[test]
    fn correctly_defined_op_ori_32_aw() {
        assert_eq!(0x00b8, OP_ORI_32_AW)
    }
    #[test]
    fn correctly_defined_op_ori_16_toc() {
        assert_eq!(0x003c, OP_ORI_16_TOC);
    }
    #[test]
    fn correctly_defined_op_ori_16_tos() {
        assert_eq!(0x007c, OP_ORI_16_TOS);
    }
    #[test]
    fn correctly_defined_op_pea_32_al() {
        assert_eq!(0x4879, OP_PEA_32_AL);
    }
    #[test]
    fn correctly_defined_op_pea_32_pcdi() {
        assert_eq!(0x487a, OP_PEA_32_PCDI);
    }
    #[test]
    fn correctly_defined_op_reset() {
        assert_eq!(0x4e70, OP_RESET);
    }
    #[test]
    fn correctly_defined_op_stop() {
        assert_eq!(0x4e72, OP_STOP);
    }
    #[test]
    fn correctly_defined_op_rte_32() {
        assert_eq!(0x4e73, OP_RTE_32);
    }
    #[test]
    fn correctly_defined_op_rtr_32() {
        assert_eq!(0x4e77, OP_RTR_32);
    }
    #[test]
    fn correctly_defined_op_rts_32() {
        assert_eq!(0x4e75, OP_RTS_32);
    }
    #[test]
    fn correctly_defined_tas_8_dn() {
        assert_eq!(0x4ac0, OP_TAS_8_DN);
    }
    #[test]
    fn correctly_defined_tas_8_ix() {
        assert_eq!(0x4af0, OP_TAS_8_IX);
    }
    #[test]
    fn correctly_defined_trap() {
        assert_eq!(0x4e40, OP_TRAP);
    }
    #[test]
    fn correctly_defined_trapv() {
        assert_eq!(0x4e76, OP_TRAPV);
    }
}
