// these are valid complete opcodes
pub const OP_ILLEGAL: u32 = 0b0100_1010_1111_1100;
pub const OP_NOP    : u32 = 0b0100_1110_0111_0001;
pub const OP_RESET  : u32 = 0b0100_1110_0111_0000;
pub const OP_STOP   : u32 = 0b0100_1110_0111_0010;
pub const OP_TRAP   : u32 = 0b0100_1110_0100_0000;
pub const OP_TRAPV  : u32 = 0b0100_1110_0111_0110;
