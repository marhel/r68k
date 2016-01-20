
use super::super::Handler;
#[allow(dead_code)]
struct OpcodeHandler {
	mask: u32,
	matching: u32,
	name: String,
	handler: Handler
}

use super::super::InstructionSet;
use super::*;
macro_rules! op_entry {
	($mask:expr, $matching:expr, $handler:ident) => (OpcodeHandler { mask: $mask, matching: $matching, handler: $handler, name: stringify!($handler).to_string() })
}

pub const MASK_OUT_X_Y: u32 = 0b1111000111111000; // masks out X and Y register bits (????xxx??????yyy)
pub const MASK_OUT_X: u32 = 0b1111000111111111; // masks out X register bits (????xxx?????????)
pub const MASK_OUT_Y: u32 = 0b1111111111111000; // masks out Y register bits (?????????????yyy)
pub const MASK_EXACT: u32 = 0b1111111111111111; // masks out no register bits, exact match
pub const MASK_LOBYTE:u32 = 0b1111111100000000; // masks out low byte

const OP_ABCD  : u32 = 0b1100_0001_0000_0000;
const OP_ADD   : u32 = 0b1101_0000_0000_0000;
const OP_ADDX  : u32 = 0b1101_0001_0000_0000;
const OP_ADDI  : u32 = 0b0000_0110_0000_0000;
const OP_ADDQ  : u32 = 0b0101_0000_0000_0000;
const OP_AND   : u32 = 0b1100_0000_0000_0000;
const OP_ANDI  : u32 = 0b0000_0010_0000_0000;
const OP_ASHIFT: u32 = 0b1110_0000_0000_0000;
const OP_BRANCH: u32 = 0b0110_0000_0000_0000;
const OP_BITOPS: u32 = 0b0000_0000_0000_0000;

const IF_T : u32 = 0b0000_0000_0000; // True				1
const IF_F : u32 = 0b0001_0000_0000; // False				0
const IF_HI: u32 = 0b0010_0000_0000; // High				!C & !Z
const IF_LS: u32 = 0b0011_0000_0000; // LowOrSame 			C | Z
const IF_CC: u32 = 0b0100_0000_0000; // CarryClearHI 		!C
const IF_CS: u32 = 0b0101_0000_0000; // CarrySetLO 			C
const IF_NE: u32 = 0b0110_0000_0000; // NotEqual 			!Z
const IF_EQ: u32 = 0b0111_0000_0000; // Equal 				Z
const IF_VC: u32 = 0b1000_0000_0000; // OverflowClear 		!V
const IF_VS: u32 = 0b1001_0000_0000; // OverflowSet 		V
const IF_PL: u32 = 0b1010_0000_0000; // Plus 				!N
const IF_MI: u32 = 0b1011_0000_0000; // Minus 				N
const IF_GE: u32 = 0b1100_0000_0000; // GreaterOrEqual 		N & V | !N & !V
const IF_LT: u32 = 0b1101_0000_0000; // LessThan 			N & !V | !N & V
const IF_GT: u32 = 0b1110_0000_0000; // GreaterThan 		N & V & !Z | !N & !V & !Z
const IF_LE: u32 = 0b1111_0000_0000; // LessOrEqual 		Z | N & !V | !N & V

const OPER_D   : u32 = 0x00;
const OPER_A   : u32 = 0x08;
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

const RR_MODE: u32 = 0x00;
const MM_MODE: u32 = 0x08;

const SHIFT_RIGHT: u32 = 0x000;
const SHIFT_LEFT : u32 = 0x100;
const IMM_COUNT  : u32 = 0x00;
const REG_COUNT  : u32 = 0x20;
const MEM_SHIFT  : u32 = 0xC0;
const REG_SHIFT  : u32 = 0x00;

// ADDA does not follow the ADD pattern for 'oper' so we cannot use the
// above constants
const DEST_AX_WORD: u32 = 0x0C0;
const DEST_AX_LONG: u32 = 0x1C0;

// -- OP-constants -------------------------------
pub const OP_ABCD_8_RR: u32 = OP_ABCD | BYTE_SIZED | RR_MODE;
pub const OP_ABCD_8_MM: u32 = OP_ABCD | BYTE_SIZED | MM_MODE;

pub const OP_ADD_8_ER_D    : u32 = OP_ADD | BYTE_SIZED | DEST_DX | OPER_D;
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

pub const OP_ADD_16_ER_D   : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_D;
pub const OP_ADD_16_ER_A   : u32 = OP_ADD | WORD_SIZED | DEST_DX | OPER_A;
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

pub const OP_ADD_32_ER_D   : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_D;
pub const OP_ADD_32_ER_A   : u32 = OP_ADD | LONG_SIZED | DEST_DX | OPER_A;
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

pub const OP_ADDA_16_D     : u32 = OP_ADD | DEST_AX_WORD | OPER_D;
pub const OP_ADDA_16_A     : u32 = OP_ADD | DEST_AX_WORD | OPER_A;
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

pub const OP_ADDA_32_D     : u32 = OP_ADD | DEST_AX_LONG | OPER_D;
pub const OP_ADDA_32_A     : u32 = OP_ADD | DEST_AX_LONG | OPER_A;
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

pub const OP_ADDI_8_D      : u32 = OP_ADDI | BYTE_SIZED | OPER_D;
pub const OP_ADDI_8_AI     : u32 = OP_ADDI | BYTE_SIZED | OPER_AI;
pub const OP_ADDI_8_PI     : u32 = OP_ADDI | BYTE_SIZED | OPER_PI;
pub const OP_ADDI_8_PD     : u32 = OP_ADDI | BYTE_SIZED | OPER_PD;
pub const OP_ADDI_8_DI     : u32 = OP_ADDI | BYTE_SIZED | OPER_DI;
pub const OP_ADDI_8_IX     : u32 = OP_ADDI | BYTE_SIZED | OPER_IX;
pub const OP_ADDI_8_AW     : u32 = OP_ADDI | BYTE_SIZED | OPER_AW;
pub const OP_ADDI_8_AL     : u32 = OP_ADDI | BYTE_SIZED | OPER_AL;

pub const OP_ADDI_16_D     : u32 = OP_ADDI | WORD_SIZED | OPER_D;
pub const OP_ADDI_16_AI    : u32 = OP_ADDI | WORD_SIZED | OPER_AI;
pub const OP_ADDI_16_PI    : u32 = OP_ADDI | WORD_SIZED | OPER_PI;
pub const OP_ADDI_16_PD    : u32 = OP_ADDI | WORD_SIZED | OPER_PD;
pub const OP_ADDI_16_DI    : u32 = OP_ADDI | WORD_SIZED | OPER_DI;
pub const OP_ADDI_16_IX    : u32 = OP_ADDI | WORD_SIZED | OPER_IX;
pub const OP_ADDI_16_AW    : u32 = OP_ADDI | WORD_SIZED | OPER_AW;
pub const OP_ADDI_16_AL    : u32 = OP_ADDI | WORD_SIZED | OPER_AL;

pub const OP_ADDI_32_D     : u32 = OP_ADDI | LONG_SIZED | OPER_D;
pub const OP_ADDI_32_AI    : u32 = OP_ADDI | LONG_SIZED | OPER_AI;
pub const OP_ADDI_32_PI    : u32 = OP_ADDI | LONG_SIZED | OPER_PI;
pub const OP_ADDI_32_PD    : u32 = OP_ADDI | LONG_SIZED | OPER_PD;
pub const OP_ADDI_32_DI    : u32 = OP_ADDI | LONG_SIZED | OPER_DI;
pub const OP_ADDI_32_IX    : u32 = OP_ADDI | LONG_SIZED | OPER_IX;
pub const OP_ADDI_32_AW    : u32 = OP_ADDI | LONG_SIZED | OPER_AW;
pub const OP_ADDI_32_AL    : u32 = OP_ADDI | LONG_SIZED | OPER_AL;

pub const OP_ADDQ_8_D      : u32 = OP_ADDQ | BYTE_SIZED | OPER_D;
pub const OP_ADDQ_8_AI     : u32 = OP_ADDQ | BYTE_SIZED | OPER_AI;
pub const OP_ADDQ_8_PI     : u32 = OP_ADDQ | BYTE_SIZED | OPER_PI;
pub const OP_ADDQ_8_PD     : u32 = OP_ADDQ | BYTE_SIZED | OPER_PD;
pub const OP_ADDQ_8_DI     : u32 = OP_ADDQ | BYTE_SIZED | OPER_DI;
pub const OP_ADDQ_8_IX     : u32 = OP_ADDQ | BYTE_SIZED | OPER_IX;
pub const OP_ADDQ_8_AW     : u32 = OP_ADDQ | BYTE_SIZED | OPER_AW;
pub const OP_ADDQ_8_AL     : u32 = OP_ADDQ | BYTE_SIZED | OPER_AL;

pub const OP_ADDQ_16_D     : u32 = OP_ADDQ | WORD_SIZED | OPER_D;
pub const OP_ADDQ_16_A     : u32 = OP_ADDQ | WORD_SIZED | OPER_A;
pub const OP_ADDQ_16_AI    : u32 = OP_ADDQ | WORD_SIZED | OPER_AI;
pub const OP_ADDQ_16_PI    : u32 = OP_ADDQ | WORD_SIZED | OPER_PI;
pub const OP_ADDQ_16_PD    : u32 = OP_ADDQ | WORD_SIZED | OPER_PD;
pub const OP_ADDQ_16_DI    : u32 = OP_ADDQ | WORD_SIZED | OPER_DI;
pub const OP_ADDQ_16_IX    : u32 = OP_ADDQ | WORD_SIZED | OPER_IX;
pub const OP_ADDQ_16_AW    : u32 = OP_ADDQ | WORD_SIZED | OPER_AW;
pub const OP_ADDQ_16_AL    : u32 = OP_ADDQ | WORD_SIZED | OPER_AL;

pub const OP_ADDQ_32_D     : u32 = OP_ADDQ | LONG_SIZED | OPER_D;
pub const OP_ADDQ_32_A     : u32 = OP_ADDQ | LONG_SIZED | OPER_A;
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

pub const OP_AND_8_ER_D    : u32 = OP_AND | BYTE_SIZED | DEST_DX | OPER_D;
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

pub const OP_AND_16_ER_D   : u32 = OP_AND | WORD_SIZED | DEST_DX | OPER_D;
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

pub const OP_AND_32_ER_D   : u32 = OP_AND | LONG_SIZED | DEST_DX | OPER_D;
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

pub const OP_ANDI_8_D      : u32 = OP_ANDI | BYTE_SIZED | OPER_D;
pub const OP_ANDI_8_AI     : u32 = OP_ANDI | BYTE_SIZED | OPER_AI;
pub const OP_ANDI_8_PI     : u32 = OP_ANDI | BYTE_SIZED | OPER_PI;
pub const OP_ANDI_8_PD     : u32 = OP_ANDI | BYTE_SIZED | OPER_PD;
pub const OP_ANDI_8_DI     : u32 = OP_ANDI | BYTE_SIZED | OPER_DI;
pub const OP_ANDI_8_IX     : u32 = OP_ANDI | BYTE_SIZED | OPER_IX;
pub const OP_ANDI_8_AW     : u32 = OP_ANDI | BYTE_SIZED | OPER_AW;
pub const OP_ANDI_8_AL     : u32 = OP_ANDI | BYTE_SIZED | OPER_AL;

pub const OP_ANDI_16_D     : u32 = OP_ANDI | WORD_SIZED | OPER_D;
pub const OP_ANDI_16_AI    : u32 = OP_ANDI | WORD_SIZED | OPER_AI;
pub const OP_ANDI_16_PI    : u32 = OP_ANDI | WORD_SIZED | OPER_PI;
pub const OP_ANDI_16_PD    : u32 = OP_ANDI | WORD_SIZED | OPER_PD;
pub const OP_ANDI_16_DI    : u32 = OP_ANDI | WORD_SIZED | OPER_DI;
pub const OP_ANDI_16_IX    : u32 = OP_ANDI | WORD_SIZED | OPER_IX;
pub const OP_ANDI_16_AW    : u32 = OP_ANDI | WORD_SIZED | OPER_AW;
pub const OP_ANDI_16_AL    : u32 = OP_ANDI | WORD_SIZED | OPER_AL;

pub const OP_ANDI_32_D     : u32 = OP_ANDI | LONG_SIZED | OPER_D;
pub const OP_ANDI_32_AI    : u32 = OP_ANDI | LONG_SIZED | OPER_AI;
pub const OP_ANDI_32_PI    : u32 = OP_ANDI | LONG_SIZED | OPER_PI;
pub const OP_ANDI_32_PD    : u32 = OP_ANDI | LONG_SIZED | OPER_PD;
pub const OP_ANDI_32_DI    : u32 = OP_ANDI | LONG_SIZED | OPER_DI;
pub const OP_ANDI_32_IX    : u32 = OP_ANDI | LONG_SIZED | OPER_IX;
pub const OP_ANDI_32_AW    : u32 = OP_ANDI | LONG_SIZED | OPER_AW;
pub const OP_ANDI_32_AL    : u32 = OP_ANDI | LONG_SIZED | OPER_AL;

pub const OP_ANDI_16_TOC   : u32 = OP_ANDI | DEST_CCR;

pub const OP_ASL_8_R        : u32 = OP_ASHIFT | SHIFT_LEFT  | BYTE_SIZED | REG_SHIFT | REG_COUNT;
pub const OP_ASL_8_S        : u32 = OP_ASHIFT | SHIFT_LEFT  | BYTE_SIZED | REG_SHIFT | IMM_COUNT;
pub const OP_ASL_16_R       : u32 = OP_ASHIFT | SHIFT_LEFT  | WORD_SIZED | REG_SHIFT | REG_COUNT;
pub const OP_ASL_16_S       : u32 = OP_ASHIFT | SHIFT_LEFT  | WORD_SIZED | REG_SHIFT | IMM_COUNT;
pub const OP_ASL_32_R       : u32 = OP_ASHIFT | SHIFT_LEFT  | LONG_SIZED | REG_SHIFT | REG_COUNT;
pub const OP_ASL_32_S       : u32 = OP_ASHIFT | SHIFT_LEFT  | LONG_SIZED | REG_SHIFT | IMM_COUNT;

pub const OP_ASL_16_AI      : u32 = OP_ASHIFT | SHIFT_LEFT  | WORD_SIZED | MEM_SHIFT | OPER_AI;
pub const OP_ASL_16_PI      : u32 = OP_ASHIFT | SHIFT_LEFT  | WORD_SIZED | MEM_SHIFT | OPER_PI;
pub const OP_ASL_16_PD      : u32 = OP_ASHIFT | SHIFT_LEFT  | WORD_SIZED | MEM_SHIFT | OPER_PD;
pub const OP_ASL_16_DI      : u32 = OP_ASHIFT | SHIFT_LEFT  | WORD_SIZED | MEM_SHIFT | OPER_DI;
pub const OP_ASL_16_IX      : u32 = OP_ASHIFT | SHIFT_LEFT  | WORD_SIZED | MEM_SHIFT | OPER_IX;
pub const OP_ASL_16_AW      : u32 = OP_ASHIFT | SHIFT_LEFT  | WORD_SIZED | MEM_SHIFT | OPER_AW;
pub const OP_ASL_16_AL      : u32 = OP_ASHIFT | SHIFT_LEFT  | WORD_SIZED | MEM_SHIFT | OPER_AL;

pub const OP_ASR_8_R        : u32 = OP_ASHIFT | SHIFT_RIGHT | BYTE_SIZED | REG_SHIFT | REG_COUNT;
pub const OP_ASR_8_S        : u32 = OP_ASHIFT | SHIFT_RIGHT | BYTE_SIZED | REG_SHIFT | IMM_COUNT;
pub const OP_ASR_16_R       : u32 = OP_ASHIFT | SHIFT_RIGHT | WORD_SIZED | REG_SHIFT | REG_COUNT;
pub const OP_ASR_16_S       : u32 = OP_ASHIFT | SHIFT_RIGHT | WORD_SIZED | REG_SHIFT | IMM_COUNT;
pub const OP_ASR_32_R       : u32 = OP_ASHIFT | SHIFT_RIGHT | LONG_SIZED | REG_SHIFT | REG_COUNT;
pub const OP_ASR_32_S       : u32 = OP_ASHIFT | SHIFT_RIGHT | LONG_SIZED | REG_SHIFT | IMM_COUNT;

pub const OP_ASR_16_AI      : u32 = OP_ASHIFT | SHIFT_RIGHT | WORD_SIZED | MEM_SHIFT | OPER_AI;
pub const OP_ASR_16_PI      : u32 = OP_ASHIFT | SHIFT_RIGHT | WORD_SIZED | MEM_SHIFT | OPER_PI;
pub const OP_ASR_16_PD      : u32 = OP_ASHIFT | SHIFT_RIGHT | WORD_SIZED | MEM_SHIFT | OPER_PD;
pub const OP_ASR_16_DI      : u32 = OP_ASHIFT | SHIFT_RIGHT | WORD_SIZED | MEM_SHIFT | OPER_DI;
pub const OP_ASR_16_IX      : u32 = OP_ASHIFT | SHIFT_RIGHT | WORD_SIZED | MEM_SHIFT | OPER_IX;
pub const OP_ASR_16_AW      : u32 = OP_ASHIFT | SHIFT_RIGHT | WORD_SIZED | MEM_SHIFT | OPER_AW;
pub const OP_ASR_16_AL      : u32 = OP_ASHIFT | SHIFT_RIGHT | WORD_SIZED | MEM_SHIFT | OPER_AL;

pub const OP_BHI_8			: u32 = OP_BRANCH | IF_HI;
pub const OP_BLS_8			: u32 = OP_BRANCH | IF_LS;
pub const OP_BCC_8			: u32 = OP_BRANCH | IF_CC;
pub const OP_BCS_8			: u32 = OP_BRANCH | IF_CS;
pub const OP_BNE_8			: u32 = OP_BRANCH | IF_NE;
pub const OP_BEQ_8			: u32 = OP_BRANCH | IF_EQ;
pub const OP_BVC_8			: u32 = OP_BRANCH | IF_VC;
pub const OP_BVS_8			: u32 = OP_BRANCH | IF_VS;
pub const OP_BPL_8			: u32 = OP_BRANCH | IF_PL;
pub const OP_BMI_8			: u32 = OP_BRANCH | IF_MI;
pub const OP_BGE_8			: u32 = OP_BRANCH | IF_GE;
pub const OP_BLT_8			: u32 = OP_BRANCH | IF_LT;
pub const OP_BGT_8			: u32 = OP_BRANCH | IF_GT;
pub const OP_BLE_8			: u32 = OP_BRANCH | IF_LE;
pub const OP_BRA_8			: u32 = OP_BRANCH | IF_T;
pub const OP_BSR_8			: u32 = OP_BRANCH | IF_F;

pub const OP_BHI_16			: u32 = OP_BRANCH | IF_HI;
pub const OP_BLS_16			: u32 = OP_BRANCH | IF_LS;
pub const OP_BCC_16			: u32 = OP_BRANCH | IF_CC;
pub const OP_BCS_16			: u32 = OP_BRANCH | IF_CS;
pub const OP_BNE_16			: u32 = OP_BRANCH | IF_NE;
pub const OP_BEQ_16			: u32 = OP_BRANCH | IF_EQ;
pub const OP_BVC_16			: u32 = OP_BRANCH | IF_VC;
pub const OP_BVS_16			: u32 = OP_BRANCH | IF_VS;
pub const OP_BPL_16			: u32 = OP_BRANCH | IF_PL;
pub const OP_BMI_16			: u32 = OP_BRANCH | IF_MI;
pub const OP_BGE_16			: u32 = OP_BRANCH | IF_GE;
pub const OP_BLT_16			: u32 = OP_BRANCH | IF_LT;
pub const OP_BGT_16			: u32 = OP_BRANCH | IF_GT;
pub const OP_BLE_16			: u32 = OP_BRANCH | IF_LE;
pub const OP_BRA_16			: u32 = OP_BRANCH | IF_T;
pub const OP_BSR_16			: u32 = OP_BRANCH | IF_F;

const SRC_REG: u32 = 0x100;
const SRC_IMM: u32 = 0x800;
const BIT_TST: u32 = 0x00;
const BIT_CHG: u32 = 0x40;
const BIT_CLR: u32 = 0x80;
const BIT_SET: u32 = 0xC0;

pub const OP_BCHG_32_R_D	: u32 = OP_BITOPS | BIT_CHG | SRC_REG;
pub const OP_BCHG_32_S_D	: u32 = OP_BITOPS | BIT_CHG | SRC_IMM;
pub const OP_BCHG_8_R_AI	: u32 = OP_BITOPS | BIT_CHG | SRC_REG | OPER_AI;
pub const OP_BCHG_8_R_PI	: u32 = OP_BITOPS | BIT_CHG | SRC_REG | OPER_PI;
pub const OP_BCHG_8_R_PD	: u32 = OP_BITOPS | BIT_CHG | SRC_REG | OPER_PD;
pub const OP_BCHG_8_R_DI	: u32 = OP_BITOPS | BIT_CHG | SRC_REG | OPER_DI;
pub const OP_BCHG_8_R_IX	: u32 = OP_BITOPS | BIT_CHG | SRC_REG | OPER_IX;
pub const OP_BCHG_8_R_AW	: u32 = OP_BITOPS | BIT_CHG | SRC_REG | OPER_AW;
pub const OP_BCHG_8_R_AL	: u32 = OP_BITOPS | BIT_CHG | SRC_REG | OPER_AL;
pub const OP_BCHG_8_S_AI	: u32 = OP_BITOPS | BIT_CHG | SRC_IMM | OPER_AI;
pub const OP_BCHG_8_S_PI	: u32 = OP_BITOPS | BIT_CHG | SRC_IMM | OPER_PI;
pub const OP_BCHG_8_S_PD	: u32 = OP_BITOPS | BIT_CHG | SRC_IMM | OPER_PD;
pub const OP_BCHG_8_S_DI	: u32 = OP_BITOPS | BIT_CHG | SRC_IMM | OPER_DI;
pub const OP_BCHG_8_S_IX	: u32 = OP_BITOPS | BIT_CHG | SRC_IMM | OPER_IX;
pub const OP_BCHG_8_S_AW	: u32 = OP_BITOPS | BIT_CHG | SRC_IMM | OPER_AW;
pub const OP_BCHG_8_S_AL	: u32 = OP_BITOPS | BIT_CHG | SRC_IMM | OPER_AL;

pub const OP_BCLR_32_R_D	: u32 = OP_BITOPS | BIT_CLR | SRC_REG;
pub const OP_BCLR_32_S_D	: u32 = OP_BITOPS | BIT_CLR | SRC_IMM;
pub const OP_BCLR_8_R_AI	: u32 = OP_BITOPS | BIT_CLR | SRC_REG | OPER_AI;
pub const OP_BCLR_8_R_PI	: u32 = OP_BITOPS | BIT_CLR | SRC_REG | OPER_PI;
pub const OP_BCLR_8_R_PD	: u32 = OP_BITOPS | BIT_CLR | SRC_REG | OPER_PD;
pub const OP_BCLR_8_R_DI	: u32 = OP_BITOPS | BIT_CLR | SRC_REG | OPER_DI;
pub const OP_BCLR_8_R_IX	: u32 = OP_BITOPS | BIT_CLR | SRC_REG | OPER_IX;
pub const OP_BCLR_8_R_AW	: u32 = OP_BITOPS | BIT_CLR | SRC_REG | OPER_AW;
pub const OP_BCLR_8_R_AL	: u32 = OP_BITOPS | BIT_CLR | SRC_REG | OPER_AL;
pub const OP_BCLR_8_S_AI	: u32 = OP_BITOPS | BIT_CLR | SRC_IMM | OPER_AI;
pub const OP_BCLR_8_S_PI	: u32 = OP_BITOPS | BIT_CLR | SRC_IMM | OPER_PI;
pub const OP_BCLR_8_S_PD	: u32 = OP_BITOPS | BIT_CLR | SRC_IMM | OPER_PD;
pub const OP_BCLR_8_S_DI	: u32 = OP_BITOPS | BIT_CLR | SRC_IMM | OPER_DI;
pub const OP_BCLR_8_S_IX	: u32 = OP_BITOPS | BIT_CLR | SRC_IMM | OPER_IX;
pub const OP_BCLR_8_S_AW	: u32 = OP_BITOPS | BIT_CLR | SRC_IMM | OPER_AW;
pub const OP_BCLR_8_S_AL	: u32 = OP_BITOPS | BIT_CLR | SRC_IMM | OPER_AL;

pub const OP_BSET_32_R_D	: u32 = OP_BITOPS | BIT_SET | SRC_REG;
pub const OP_BSET_32_S_D	: u32 = OP_BITOPS | BIT_SET | SRC_IMM;
pub const OP_BSET_8_R_AI	: u32 = OP_BITOPS | BIT_SET | SRC_REG | OPER_AI;
pub const OP_BSET_8_R_PI	: u32 = OP_BITOPS | BIT_SET | SRC_REG | OPER_PI;
pub const OP_BSET_8_R_PD	: u32 = OP_BITOPS | BIT_SET | SRC_REG | OPER_PD;
pub const OP_BSET_8_R_DI	: u32 = OP_BITOPS | BIT_SET | SRC_REG | OPER_DI;
pub const OP_BSET_8_R_IX	: u32 = OP_BITOPS | BIT_SET | SRC_REG | OPER_IX;
pub const OP_BSET_8_R_AW	: u32 = OP_BITOPS | BIT_SET | SRC_REG | OPER_AW;
pub const OP_BSET_8_R_AL	: u32 = OP_BITOPS | BIT_SET | SRC_REG | OPER_AL;
pub const OP_BSET_8_S_AI	: u32 = OP_BITOPS | BIT_SET | SRC_IMM | OPER_AI;
pub const OP_BSET_8_S_PI	: u32 = OP_BITOPS | BIT_SET | SRC_IMM | OPER_PI;
pub const OP_BSET_8_S_PD	: u32 = OP_BITOPS | BIT_SET | SRC_IMM | OPER_PD;
pub const OP_BSET_8_S_DI	: u32 = OP_BITOPS | BIT_SET | SRC_IMM | OPER_DI;
pub const OP_BSET_8_S_IX	: u32 = OP_BITOPS | BIT_SET | SRC_IMM | OPER_IX;
pub const OP_BSET_8_S_AW	: u32 = OP_BITOPS | BIT_SET | SRC_IMM | OPER_AW;
pub const OP_BSET_8_S_AL	: u32 = OP_BITOPS | BIT_SET | SRC_IMM | OPER_AL;

pub const OP_BTST_32_R_D	: u32 = OP_BITOPS | BIT_TST | SRC_REG;
pub const OP_BTST_32_S_D	: u32 = OP_BITOPS | BIT_TST | SRC_IMM;
pub const OP_BTST_8_R_AI	: u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_AI;
pub const OP_BTST_8_R_PI	: u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_PI;
pub const OP_BTST_8_R_PD	: u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_PD;
pub const OP_BTST_8_R_DI	: u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_DI;
pub const OP_BTST_8_R_IX	: u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_IX;
pub const OP_BTST_8_R_AW	: u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_AW;
pub const OP_BTST_8_R_AL	: u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_AL;
pub const OP_BTST_8_S_AI	: u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_AI;
pub const OP_BTST_8_S_PI	: u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_PI;
pub const OP_BTST_8_S_PD	: u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_PD;
pub const OP_BTST_8_S_DI	: u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_DI;
pub const OP_BTST_8_S_IX	: u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_IX;
pub const OP_BTST_8_S_AW	: u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_AW;
pub const OP_BTST_8_S_AL	: u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_AL;

pub fn generate() -> InstructionSet {
	// Covers all possible IR values (64k entries)
	let mut handler: InstructionSet = Vec::with_capacity(0x10000);
	for _ in 0..0x10000 { handler.push(illegal); }
	//let handler = [illegal].iter().cycle().take(0x10000).collect::<InstructionSet>();
	// (0..0x10000).map(|_| illegal).collect::<InstructionSet>();
	// the optable contains opcode mask, matching mask and the corresponding handler + name
	let optable = vec![
		op_entry!(MASK_OUT_X_Y, OP_ABCD_8_RR, abcd_8_rr),
		op_entry!(MASK_OUT_X_Y, OP_ABCD_8_MM, abcd_8_mm),

		op_entry!(MASK_OUT_X_Y, OP_ADD_8_ER_D,    add_8_er_d),
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

		op_entry!(MASK_OUT_X_Y, OP_ADD_16_ER_D,    add_16_er_d),
		op_entry!(MASK_OUT_X_Y, OP_ADD_16_ER_A,    add_16_er_a),
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

		op_entry!(MASK_OUT_X_Y, OP_ADD_32_ER_D,    add_32_er_d),
		op_entry!(MASK_OUT_X_Y, OP_ADD_32_ER_A,    add_32_er_a),
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

		op_entry!(MASK_OUT_X_Y, OP_ADDA_16_D,    adda_16_d),
		op_entry!(MASK_OUT_X_Y, OP_ADDA_16_A,    adda_16_a),
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

		op_entry!(MASK_OUT_X_Y, OP_ADDA_32_D,    adda_32_d),
		op_entry!(MASK_OUT_X_Y, OP_ADDA_32_A,    adda_32_a),
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

		op_entry!(MASK_OUT_Y, OP_ADDI_8_D,    addi_8_d),
		op_entry!(MASK_OUT_Y, OP_ADDI_8_AI,   addi_8_ai),
		op_entry!(MASK_OUT_Y, OP_ADDI_8_PI,   addi_8_pi),
		op_entry!(MASK_OUT_Y, OP_ADDI_8_PD,   addi_8_pd),
		op_entry!(MASK_OUT_Y, OP_ADDI_8_DI,   addi_8_di),
		op_entry!(MASK_OUT_Y, OP_ADDI_8_IX,   addi_8_ix),
		op_entry!(MASK_EXACT, OP_ADDI_8_AW,   addi_8_aw),
		op_entry!(MASK_EXACT, OP_ADDI_8_AL,   addi_8_al),

		op_entry!(MASK_OUT_Y, OP_ADDI_16_D,    addi_16_d),
		op_entry!(MASK_OUT_Y, OP_ADDI_16_AI,   addi_16_ai),
		op_entry!(MASK_OUT_Y, OP_ADDI_16_PI,   addi_16_pi),
		op_entry!(MASK_OUT_Y, OP_ADDI_16_PD,   addi_16_pd),
		op_entry!(MASK_OUT_Y, OP_ADDI_16_DI,   addi_16_di),
		op_entry!(MASK_OUT_Y, OP_ADDI_16_IX,   addi_16_ix),
		op_entry!(MASK_EXACT, OP_ADDI_16_AW,   addi_16_aw),
		op_entry!(MASK_EXACT, OP_ADDI_16_AL,   addi_16_al),

		op_entry!(MASK_OUT_Y, OP_ADDI_32_D,    addi_32_d),
		op_entry!(MASK_OUT_Y, OP_ADDI_32_AI,   addi_32_ai),
		op_entry!(MASK_OUT_Y, OP_ADDI_32_PI,   addi_32_pi),
		op_entry!(MASK_OUT_Y, OP_ADDI_32_PD,   addi_32_pd),
		op_entry!(MASK_OUT_Y, OP_ADDI_32_DI,   addi_32_di),
		op_entry!(MASK_OUT_Y, OP_ADDI_32_IX,   addi_32_ix),
		op_entry!(MASK_EXACT, OP_ADDI_32_AW,   addi_32_aw),
		op_entry!(MASK_EXACT, OP_ADDI_32_AL,   addi_32_al),

		op_entry!(MASK_OUT_X_Y, OP_ADDQ_8_D,  addq_8_d),
		op_entry!(MASK_OUT_X_Y, OP_ADDQ_8_AI, addq_8_ai),
		op_entry!(MASK_OUT_X_Y, OP_ADDQ_8_PI, addq_8_pi),
		op_entry!(MASK_OUT_X_Y, OP_ADDQ_8_PD, addq_8_pd),
		op_entry!(MASK_OUT_X_Y, OP_ADDQ_8_DI, addq_8_di),
		op_entry!(MASK_OUT_X_Y, OP_ADDQ_8_IX, addq_8_ix),
		op_entry!(MASK_OUT_X,   OP_ADDQ_8_AW, addq_8_aw),
		op_entry!(MASK_OUT_X,   OP_ADDQ_8_AL, addq_8_al),

		op_entry!(MASK_OUT_X_Y, OP_ADDQ_16_D,  addq_16_d),
		op_entry!(MASK_OUT_X_Y, OP_ADDQ_16_A,  addq_16_a),
		op_entry!(MASK_OUT_X_Y, OP_ADDQ_16_AI, addq_16_ai),
		op_entry!(MASK_OUT_X_Y, OP_ADDQ_16_PI, addq_16_pi),
		op_entry!(MASK_OUT_X_Y, OP_ADDQ_16_PD, addq_16_pd),
		op_entry!(MASK_OUT_X_Y, OP_ADDQ_16_DI, addq_16_di),
		op_entry!(MASK_OUT_X_Y, OP_ADDQ_16_IX, addq_16_ix),
		op_entry!(MASK_OUT_X,   OP_ADDQ_16_AW, addq_16_aw),
		op_entry!(MASK_OUT_X,   OP_ADDQ_16_AL, addq_16_al),

		op_entry!(MASK_OUT_X_Y, OP_ADDQ_32_D,  addq_32_d),
		op_entry!(MASK_OUT_X_Y, OP_ADDQ_32_A,  addq_32_a),
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

		op_entry!(MASK_OUT_X_Y, OP_AND_8_ER_D,    and_8_er_d),
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

		op_entry!(MASK_OUT_X_Y, OP_AND_16_ER_D,    and_16_er_d),
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

		op_entry!(MASK_OUT_X_Y, OP_AND_32_ER_D,    and_32_er_d),
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

		op_entry!(MASK_OUT_Y, OP_ANDI_8_D,    andi_8_d),
		op_entry!(MASK_OUT_Y, OP_ANDI_8_AI,   andi_8_ai),
		op_entry!(MASK_OUT_Y, OP_ANDI_8_PI,   andi_8_pi),
		op_entry!(MASK_OUT_Y, OP_ANDI_8_PD,   andi_8_pd),
		op_entry!(MASK_OUT_Y, OP_ANDI_8_DI,   andi_8_di),
		op_entry!(MASK_OUT_Y, OP_ANDI_8_IX,   andi_8_ix),
		op_entry!(MASK_EXACT, OP_ANDI_8_AW,   andi_8_aw),
		op_entry!(MASK_EXACT, OP_ANDI_8_AL,   andi_8_al),

		op_entry!(MASK_OUT_Y, OP_ANDI_16_D,    andi_16_d),
		op_entry!(MASK_OUT_Y, OP_ANDI_16_AI,   andi_16_ai),
		op_entry!(MASK_OUT_Y, OP_ANDI_16_PI,   andi_16_pi),
		op_entry!(MASK_OUT_Y, OP_ANDI_16_PD,   andi_16_pd),
		op_entry!(MASK_OUT_Y, OP_ANDI_16_DI,   andi_16_di),
		op_entry!(MASK_OUT_Y, OP_ANDI_16_IX,   andi_16_ix),
		op_entry!(MASK_EXACT, OP_ANDI_16_AW,   andi_16_aw),
		op_entry!(MASK_EXACT, OP_ANDI_16_AL,   andi_16_al),

		op_entry!(MASK_OUT_Y, OP_ANDI_32_D,    andi_32_d),
		op_entry!(MASK_OUT_Y, OP_ANDI_32_AI,   andi_32_ai),
		op_entry!(MASK_OUT_Y, OP_ANDI_32_PI,   andi_32_pi),
		op_entry!(MASK_OUT_Y, OP_ANDI_32_PD,   andi_32_pd),
		op_entry!(MASK_OUT_Y, OP_ANDI_32_DI,   andi_32_di),
		op_entry!(MASK_OUT_Y, OP_ANDI_32_IX,   andi_32_ix),
		op_entry!(MASK_EXACT, OP_ANDI_32_AW,   andi_32_aw),
		op_entry!(MASK_EXACT, OP_ANDI_32_AL,   andi_32_al),

		op_entry!(MASK_EXACT, OP_ANDI_16_TOC,   andi_16_toc),

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

		op_entry!(MASK_OUT_X_Y, OP_BCHG_32_R_D, bchg_32_r_d),
		op_entry!(MASK_OUT_Y,   OP_BCHG_32_S_D, bchg_32_s_d),
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

		op_entry!(MASK_OUT_X_Y, OP_BCLR_32_R_D, bclr_32_r_d),
		op_entry!(MASK_OUT_Y,   OP_BCLR_32_S_D, bclr_32_s_d),
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

		op_entry!(MASK_OUT_X_Y, OP_BSET_32_R_D, bset_32_r_d),
		op_entry!(MASK_OUT_Y,   OP_BSET_32_S_D, bset_32_s_d),
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

		op_entry!(MASK_OUT_X_Y, OP_BTST_32_R_D, btst_32_r_d),
		op_entry!(MASK_OUT_Y,   OP_BTST_32_S_D, btst_32_s_d),
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
	];
	// let mut implemented = 0;
	for op in optable {
		for opcode in op.matching..0x10000 {
			if (opcode & op.mask) == op.matching {
				// println!("{:16b}: {}", opcode, op.name);
				handler[opcode as usize] = op.handler;
				// implemented += 1;
			}
		}
	}
	// According to Musashi opcode handler jump table;
	// M68000 implements 54007 opcodes (11529 illegal)
	// M68010 implements 54194 opcodes (11342 illegal)
	// M68020 implements 55611 opcodes (9925 illegal)
	// println!("{:?} opcodes implemented ({:.2}% done)", implemented, implemented as f32 / 540.07f32);
	handler
}
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn different_ops() {
		assert!(OP_ADDX_16_MM != OP_ADD_16_ER_A);
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
	fn correctly_defined_asl_8_r() {
		assert_eq!(0xe020, OP_ASR_8_R);
	}
	#[test]
	fn correctly_defined_bchg_32_r_d() {
		assert_eq!(0x0140, OP_BCHG_32_R_D);
	}
	#[test]
	fn correctly_defined_bchg_32_s_d() {
		assert_eq!(0x0840, OP_BCHG_32_S_D);
	}
	#[test]
	fn correctly_defined_bchg_8_r_ix() {
		assert_eq!(0x0170, OP_BCHG_8_R_IX);
	}
	#[test]
	fn correctly_defined_bchg_8_s_aw() {
		assert_eq!(0x0878, OP_BCHG_8_S_AW);
	}
}