pub use r68k_common::ops::*;
use r68k_common::constants::*;

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

pub const OP_ANDI_8_TOC    : u32 = OP_ANDI | BYTE_SIZED | DEST_SR;
pub const OP_ANDI_16_TOS   : u32 = OP_ANDI | WORD_SIZED | DEST_SR;

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

pub const OP_BHI_16            : u32 = OP_BRANCH | IF_HI | DISPLACEMENT_16;
pub const OP_BLS_16            : u32 = OP_BRANCH | IF_LS | DISPLACEMENT_16;
pub const OP_BCC_16            : u32 = OP_BRANCH | IF_CC | DISPLACEMENT_16;
pub const OP_BCS_16            : u32 = OP_BRANCH | IF_CS | DISPLACEMENT_16;
pub const OP_BNE_16            : u32 = OP_BRANCH | IF_NE | DISPLACEMENT_16;
pub const OP_BEQ_16            : u32 = OP_BRANCH | IF_EQ | DISPLACEMENT_16;
pub const OP_BVC_16            : u32 = OP_BRANCH | IF_VC | DISPLACEMENT_16;
pub const OP_BVS_16            : u32 = OP_BRANCH | IF_VS | DISPLACEMENT_16;
pub const OP_BPL_16            : u32 = OP_BRANCH | IF_PL | DISPLACEMENT_16;
pub const OP_BMI_16            : u32 = OP_BRANCH | IF_MI | DISPLACEMENT_16;
pub const OP_BGE_16            : u32 = OP_BRANCH | IF_GE | DISPLACEMENT_16;
pub const OP_BLT_16            : u32 = OP_BRANCH | IF_LT | DISPLACEMENT_16;
pub const OP_BGT_16            : u32 = OP_BRANCH | IF_GT | DISPLACEMENT_16;
pub const OP_BLE_16            : u32 = OP_BRANCH | IF_LE | DISPLACEMENT_16;
pub const OP_BRA_16            : u32 = OP_BRANCH | IF_T  | DISPLACEMENT_16;
pub const OP_BSR_16            : u32 = OP_BRANCH | IF_F  | DISPLACEMENT_16;

pub const OP_BHI_32            : u32 = OP_BRANCH | IF_HI | DISPLACEMENT_32;
pub const OP_BLS_32            : u32 = OP_BRANCH | IF_LS | DISPLACEMENT_32;
pub const OP_BCC_32            : u32 = OP_BRANCH | IF_CC | DISPLACEMENT_32;
pub const OP_BCS_32            : u32 = OP_BRANCH | IF_CS | DISPLACEMENT_32;
pub const OP_BNE_32            : u32 = OP_BRANCH | IF_NE | DISPLACEMENT_32;
pub const OP_BEQ_32            : u32 = OP_BRANCH | IF_EQ | DISPLACEMENT_32;
pub const OP_BVC_32            : u32 = OP_BRANCH | IF_VC | DISPLACEMENT_32;
pub const OP_BVS_32            : u32 = OP_BRANCH | IF_VS | DISPLACEMENT_32;
pub const OP_BPL_32            : u32 = OP_BRANCH | IF_PL | DISPLACEMENT_32;
pub const OP_BMI_32            : u32 = OP_BRANCH | IF_MI | DISPLACEMENT_32;
pub const OP_BGE_32            : u32 = OP_BRANCH | IF_GE | DISPLACEMENT_32;
pub const OP_BLT_32            : u32 = OP_BRANCH | IF_LT | DISPLACEMENT_32;
pub const OP_BGT_32            : u32 = OP_BRANCH | IF_GT | DISPLACEMENT_32;
pub const OP_BLE_32            : u32 = OP_BRANCH | IF_LE | DISPLACEMENT_32;
pub const OP_BRA_32            : u32 = OP_BRANCH | IF_T  | DISPLACEMENT_32;
pub const OP_BSR_32            : u32 = OP_BRANCH | IF_F  | DISPLACEMENT_32;

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
pub const OP_BTST_8_R_PCDI  : u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_PCDI;
pub const OP_BTST_8_R_PCIX  : u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_PCIX;
pub const OP_BTST_8_R_IMM   : u32 = OP_BITOPS | BIT_TST | SRC_REG | OPER_IMM;

pub const OP_BTST_8_S_AI    : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_AI;
pub const OP_BTST_8_S_PI    : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_PI;
pub const OP_BTST_8_S_PD    : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_PD;
pub const OP_BTST_8_S_DI    : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_DI;
pub const OP_BTST_8_S_IX    : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_IX;
pub const OP_BTST_8_S_AW    : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_AW;
pub const OP_BTST_8_S_AL    : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_AL;
pub const OP_BTST_8_S_PCDI  : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_PCDI;
pub const OP_BTST_8_S_PCIX  : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_PCIX;
pub const OP_BTST_8_S_IMM   : u32 = OP_BITOPS | BIT_TST | SRC_IMM | OPER_IMM;

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

pub const OP_EORI_8_TOC    : u32 = OP_EORI | BYTE_SIZED | DEST_SR;
pub const OP_EORI_16_TOS   : u32 = OP_EORI | WORD_SIZED | DEST_SR;

pub const OP_EXG_32_DD: u32 = OP_EXG | EXG_DATA_DATA;
pub const OP_EXG_32_AA: u32 = OP_EXG | EXG_ADDR_ADDR;
pub const OP_EXG_32_DA: u32 = OP_EXG | EXG_DATA_ADDR;

pub const OP_EXT_BW: u32 = OP_EXT | BYTE_TO_WORD;
pub const OP_EXT_WL: u32 = OP_EXT | WORD_TO_LONG;
// pub const OP_EXT_BL: u32 = OP_EXT | BYTE_TO_LONG; // 020+

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
pub const OP_LINK_16     : u32 = OP_LINK;

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

pub const OP_MOVE_16_DN_AN   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DN | OPER_AN;
pub const OP_MOVE_16_AI_AN   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AI | OPER_AN;
pub const OP_MOVE_16_PI_AN   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PI | OPER_AN;
pub const OP_MOVE_16_PD_AN   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_PD | OPER_AN;
pub const OP_MOVE_16_DI_AN   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_DI | OPER_AN;
pub const OP_MOVE_16_IX_AN   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_IX | OPER_AN;
pub const OP_MOVE_16_AW_AN   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AW | OPER_AN;
pub const OP_MOVE_16_AL_AN   : u32 = OP_MOVE | WORD_MOVE | MOVE_TO_AL | OPER_AN;

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

pub const OP_MOVE_32_DN_AN   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DN | OPER_AN;
pub const OP_MOVE_32_AI_AN   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AI | OPER_AN;
pub const OP_MOVE_32_PI_AN   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PI | OPER_AN;
pub const OP_MOVE_32_PD_AN   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_PD | OPER_AN;
pub const OP_MOVE_32_DI_AN   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_DI | OPER_AN;
pub const OP_MOVE_32_IX_AN   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_IX | OPER_AN;
pub const OP_MOVE_32_AW_AN   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AW | OPER_AN;
pub const OP_MOVE_32_AL_AN   : u32 = OP_MOVE | LONG_MOVE | MOVE_TO_AL | OPER_AN;

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

pub const OP_MOVE_32_TOU : u32 = OP_MOVE2 | MOVE_USP | TO_AN;
pub const OP_MOVE_32_FRU : u32 = OP_MOVE2 | MOVE_USP | FROM_AN;

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

pub const OP_MOVEP_16_ER: u32 = OP_MOVEP | WORD_TRANSFER | MOVEP_MEMORY_TO_REGISTER;
pub const OP_MOVEP_16_RE: u32 = OP_MOVEP | WORD_TRANSFER | MOVEP_REGISTER_TO_MEMORY;
pub const OP_MOVEP_32_ER: u32 = OP_MOVEP | LONG_TRANSFER | MOVEP_MEMORY_TO_REGISTER;
pub const OP_MOVEP_32_RE: u32 = OP_MOVEP | LONG_TRANSFER | MOVEP_REGISTER_TO_MEMORY;

// Put constants for MOVEQ here
pub const OP_MOVEQ_32: u32 = OP_MOVEQ;

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
pub const OP_ORI_8_TOC    : u32 = OP_ORI | BYTE_SIZED | DEST_SR;

// Put constants for ORI to SR here
pub const OP_ORI_16_TOS   : u32 = OP_ORI | WORD_SIZED | DEST_SR;

// Put constants for PEA here
pub const OP_PEA_32_AI   : u32 = OP_PEA | OPER_AI;
pub const OP_PEA_32_DI   : u32 = OP_PEA | OPER_DI;
pub const OP_PEA_32_IX   : u32 = OP_PEA | OPER_IX;
pub const OP_PEA_32_AW   : u32 = OP_PEA | OPER_AW;
pub const OP_PEA_32_AL   : u32 = OP_PEA | OPER_AL;
pub const OP_PEA_32_PCDI : u32 = OP_PEA | OPER_PCDI;
pub const OP_PEA_32_PCIX : u32 = OP_PEA | OPER_PCIX;

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

pub const OP_RTE_32 : u32 = OP_RTE;
pub const OP_RTR_32 : u32 = OP_RTR;
pub const OP_RTS_32 : u32 = OP_RTS;

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

// Put constants for TST here
pub const OP_TST_8_DN   : u32 = OP_TST | BYTE_SIZED | OPER_DN;
pub const OP_TST_8_AI   : u32 = OP_TST | BYTE_SIZED | OPER_AI;
pub const OP_TST_8_PI   : u32 = OP_TST | BYTE_SIZED | OPER_PI;
pub const OP_TST_8_PD   : u32 = OP_TST | BYTE_SIZED | OPER_PD;
pub const OP_TST_8_DI   : u32 = OP_TST | BYTE_SIZED | OPER_DI;
pub const OP_TST_8_IX   : u32 = OP_TST | BYTE_SIZED | OPER_IX;
pub const OP_TST_8_AW   : u32 = OP_TST | BYTE_SIZED | OPER_AW;
pub const OP_TST_8_AL   : u32 = OP_TST | BYTE_SIZED | OPER_AL;
pub const OP_TST_8_PCDI : u32 = OP_TST | BYTE_SIZED | OPER_PCDI; // NOT MC68000 according to PRM
pub const OP_TST_8_PCIX : u32 = OP_TST | BYTE_SIZED | OPER_PCIX; // NOT MC68000 according to PRM
pub const OP_TST_8_IMM  : u32 = OP_TST | BYTE_SIZED | OPER_IMM;  // NOT MC68000 according to PRM

pub const OP_TST_16_DN   : u32 = OP_TST | WORD_SIZED | OPER_DN;
pub const OP_TST_16_AN   : u32 = OP_TST | WORD_SIZED | OPER_AN; // NOT MC68000 according to PRM
pub const OP_TST_16_AI   : u32 = OP_TST | WORD_SIZED | OPER_AI;
pub const OP_TST_16_PI   : u32 = OP_TST | WORD_SIZED | OPER_PI;
pub const OP_TST_16_PD   : u32 = OP_TST | WORD_SIZED | OPER_PD;
pub const OP_TST_16_DI   : u32 = OP_TST | WORD_SIZED | OPER_DI;
pub const OP_TST_16_IX   : u32 = OP_TST | WORD_SIZED | OPER_IX;
pub const OP_TST_16_AW   : u32 = OP_TST | WORD_SIZED | OPER_AW;
pub const OP_TST_16_AL   : u32 = OP_TST | WORD_SIZED | OPER_AL;
pub const OP_TST_16_PCDI : u32 = OP_TST | WORD_SIZED | OPER_PCDI; // NOT MC68000 according to PRM
pub const OP_TST_16_PCIX : u32 = OP_TST | WORD_SIZED | OPER_PCIX; // NOT MC68000 according to PRM
pub const OP_TST_16_IMM  : u32 = OP_TST | WORD_SIZED | OPER_IMM;  // NOT MC68000 according to PRM

pub const OP_TST_32_DN   : u32 = OP_TST | LONG_SIZED | OPER_DN;
pub const OP_TST_32_AN   : u32 = OP_TST | LONG_SIZED | OPER_AN; // NOT MC68000 according to PRM
pub const OP_TST_32_AI   : u32 = OP_TST | LONG_SIZED | OPER_AI;
pub const OP_TST_32_PI   : u32 = OP_TST | LONG_SIZED | OPER_PI;
pub const OP_TST_32_PD   : u32 = OP_TST | LONG_SIZED | OPER_PD;
pub const OP_TST_32_DI   : u32 = OP_TST | LONG_SIZED | OPER_DI;
pub const OP_TST_32_IX   : u32 = OP_TST | LONG_SIZED | OPER_IX;
pub const OP_TST_32_AW   : u32 = OP_TST | LONG_SIZED | OPER_AW;
pub const OP_TST_32_AL   : u32 = OP_TST | LONG_SIZED | OPER_AL;
pub const OP_TST_32_PCDI : u32 = OP_TST | LONG_SIZED | OPER_PCDI; // NOT MC68000 according to PRM
pub const OP_TST_32_PCIX : u32 = OP_TST | LONG_SIZED | OPER_PCIX; // NOT MC68000 according to PRM
pub const OP_TST_32_IMM  : u32 = OP_TST | LONG_SIZED | OPER_IMM;  // NOT MC68000 according to PRM

// Put constants for UNLK here
pub const OP_UNLK_32     : u32 = OP_UNLK;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_ops() {
        assert!(OP_ADDX_16_MM != OP_ADD_16_ER_AN);
    }
    #[test]
    fn correctly_defined_op_andi_8_toc() {
        assert_eq!(0x023c, OP_ANDI_8_TOC);
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
    fn correctly_defined_op_move_16_pd_an() {
        assert_eq!(0x3108, OP_MOVE_16_PD_AN);
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
    fn correctly_defined_op_move_32_ix_an() {
        assert_eq!(0x2188, OP_MOVE_32_IX_AN);
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
    fn correctly_defined_op_ori_8_toc() {
        assert_eq!(0x003c, OP_ORI_8_TOC);
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
    #[test]
    fn correctly_defined_op_unlk_32() {
        assert_eq!(0x4e58, OP_UNLK_32);
    }
    #[test]
    fn correctly_defined_op_tst_8_imm() {
        assert_eq!(0x4a3c, OP_TST_8_IMM);
    }
    #[test]
    fn correctly_defined_op_tst_8_ai() {
        assert_eq!(0x4a10, OP_TST_8_AI);
    }
    #[test]
    fn correctly_defined_op_tst_16_an() {
        assert_eq!(0x4a48, OP_TST_16_AN);
    }
    #[test]
    fn correctly_defined_op_tst_16_di() {
        assert_eq!(0x4a68, OP_TST_16_DI);
    }
    #[test]
    fn correctly_defined_op_tst_32_pi() {
        assert_eq!(0x4a98, OP_TST_32_PI);
    }
    #[test]
    fn correctly_defined_op_tst_32_ix() {
        assert_eq!(0x4ab0, OP_TST_32_IX);
    }
}
