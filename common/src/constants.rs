pub const OP_ADD   : u32 = 0b1101_0000_0000_0000;
pub const OP_ADDX  : u32 = 0b1101_0001_0000_0000;
pub const OP_ADDI  : u32 = 0b0000_0110_0000_0000;
pub const OP_ADDQ  : u32 = 0b0101_0000_0000_0000;

pub const BYTE_SIZED: u32 = 0x00;
#[allow(dead_code)]
pub const WORD_SIZED: u32 = 0x40;
#[allow(dead_code)]
pub const LONG_SIZED: u32 = 0x80;

pub const DEST_DX: u32 = 0x000;
pub const DEST_EA: u32 = 0x100;
// ADDA does not follow the ADD pattern for 'oper' so we cannot use the
// above constants
pub const DEST_AX_WORD: u32 = 0x0C0;
pub const DEST_AX_LONG: u32 = 0x1C0;
