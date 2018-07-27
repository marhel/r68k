pub const OP_ABCD   : u32 = 0b1100_0001_0000_0000;
pub const OP_ADD    : u32 = 0b1101_0000_0000_0000;
pub const OP_ADDI   : u32 = 0b0000_0110_0000_0000;
pub const OP_ADDQ   : u32 = 0b0101_0000_0000_0000;
pub const OP_ADDX   : u32 = 0b1101_0001_0000_0000;
pub const OP_AND    : u32 = 0b1100_0000_0000_0000;
pub const OP_ANDI   : u32 = 0b0000_0010_0000_0000;
pub const OP_BITOPS : u32 = 0b0000_0000_0000_0000;
pub const OP_BRANCH : u32 = 0b0110_0000_0000_0000;
pub const OP_CHK    : u32 = 0b0100_0000_0000_0000;
pub const OP_CLR    : u32 = 0b0100_0010_0000_0000;
pub const OP_CMP    : u32 = 0b1011_0000_0000_0000;
pub const OP_CMPI   : u32 = 0b0000_1100_0000_0000;
pub const OP_CMPM   : u32 = 0b1011_0001_0000_0000;
pub const OP_DBCC   : u32 = 0b0101_0000_1100_1000;
pub const OP_DIVS   : u32 = 0b1000_0001_1100_0000;
pub const OP_DIVU   : u32 = 0b1000_0000_1100_0000;
pub const OP_EOR    : u32 = 0b1011_0000_0000_0000;
pub const OP_EORI   : u32 = 0b0000_1010_0000_0000;
pub const OP_EXG    : u32 = 0b1100_0001_0000_0000;
pub const OP_EXT    : u32 = 0b0100_1000_0000_0000;
pub const OP_ILLEGAL: u32 = 0b0100_1010_1111_1100;
pub const OP_JMP    : u32 = 0b0100_1110_1100_0000;
pub const OP_JSR    : u32 = 0b0100_1110_1000_0000;
pub const OP_LEA    : u32 = 0b0100_0001_1100_0000;
pub const OP_LINK   : u32 = 0b0100_1110_0101_0000;
pub const OP_MOVE   : u32 = 0b0000_0000_0000_0000;
pub const OP_MOVE2  : u32 = 0b0100_0000_0000_0000;
pub const OP_MOVEM  : u32 = 0b0100_1000_1000_0000;
pub const OP_MOVEP  : u32 = 0b0000_0000_0000_1000;
pub const OP_MOVEQ  : u32 = 0b0111_0000_0000_0000;
pub const OP_MULS   : u32 = 0b1100_0001_1100_0000;
pub const OP_MULU   : u32 = 0b1100_0000_1100_0000;
pub const OP_NBCD   : u32 = 0b0100_1000_0000_0000;
pub const OP_NEG    : u32 = 0b0100_0100_0000_0000;
pub const OP_NEGX   : u32 = 0b0100_0000_0000_0000;
pub const OP_NOT    : u32 = 0b0100_0110_0000_0000;
pub const OP_OR     : u32 = 0b1000_0000_0000_0000;
pub const OP_ORI    : u32 = 0b0000_0000_0000_0000;
pub const OP_PEA    : u32 = 0b0100_1000_0100_0000;
pub const OP_RESET  : u32 = 0b0100_1110_0111_0000;
pub const OP_RTE    : u32 = 0b0100_1110_0111_0011;
pub const OP_RTR    : u32 = 0b0100_1110_0111_0111;
pub const OP_RTS    : u32 = 0b0100_1110_0111_0101;
pub const OP_SBCD   : u32 = 0b1000_0001_0000_0000;
pub const OP_SCC    : u32 = 0b0101_0000_1100_0000;
pub const OP_SHIFT  : u32 = 0b1110_0000_0000_0000;
pub const OP_STOP   : u32 = 0b0100_1110_0111_0010;
pub const OP_SUB    : u32 = 0b1001_0000_0000_0000;
pub const OP_SUBI   : u32 = 0b0000_0100_0000_0000;
pub const OP_SUBQ   : u32 = 0b0101_0001_0000_0000;
pub const OP_SUBX   : u32 = 0b1001_0001_0000_0000;
pub const OP_SWAP   : u32 = 0b0100_1000_0000_0000;
pub const OP_TAS    : u32 = 0b0100_1010_1100_0000;
pub const OP_TRAP   : u32 = 0b0100_1110_0100_0000;
pub const OP_TRAPV  : u32 = 0b0100_1110_0111_0110;
pub const OP_TST    : u32 = 0b0100_1010_0000_0000;
pub const OP_UNLK   : u32 = 0b0100_1110_0101_1000;

pub const BYTE_SIZED: u32 = 0x00;
#[allow(dead_code)]
pub const WORD_SIZED: u32 = 0x40;
#[allow(dead_code)]
pub const LONG_SIZED: u32 = 0x80;

pub const MASK_OUT_X_Y: u32 = 0b1111000111111000; // masks out X and Y register bits (????xxx??????yyy)
pub const MASK_OUT_X  : u32 = 0b1111000111111111; // masks out X register bits (????xxx?????????)
pub const MASK_OUT_Y  : u32 = 0b1111111111111000; // masks out Y register bits (?????????????yyy)
pub const MASK_EXACT  : u32 = 0b1111111111111111; // masks out no register bits, exact match
pub const MASK_LOBYTE : u32 = 0b1111111100000000; // masks out low byte
pub const MASK_LOBYTX : u32 = 0b1111000100000000; // masks out low byte and X register bits
pub const MASK_LO3NIB : u32 = 0b1111000000000000; // masks out lower three nibbles
pub const MASK_LONIB  : u32 = 0b1111111111110000; // masks out low nibble

pub const IF_T : u32 = 0b0000_0000_0000; // True            1
pub const IF_F : u32 = 0b0001_0000_0000; // False           0
pub const IF_HI: u32 = 0b0010_0000_0000; // High            !C & !Z
pub const IF_LS: u32 = 0b0011_0000_0000; // LowOrSame       C | Z
pub const IF_CC: u32 = 0b0100_0000_0000; // CarryClearHI    !C
pub const IF_CS: u32 = 0b0101_0000_0000; // CarrySetLO      C
pub const IF_NE: u32 = 0b0110_0000_0000; // NotEqual        !Z
pub const IF_EQ: u32 = 0b0111_0000_0000; // Equal           Z
pub const IF_VC: u32 = 0b1000_0000_0000; // OverflowClear   !V
pub const IF_VS: u32 = 0b1001_0000_0000; // OverflowSet     V
pub const IF_PL: u32 = 0b1010_0000_0000; // Plus            !N
pub const IF_MI: u32 = 0b1011_0000_0000; // Minus           N
pub const IF_GE: u32 = 0b1100_0000_0000; // GreaterOrEqual  N & V | !N & !V
pub const IF_LT: u32 = 0b1101_0000_0000; // LessThan        N & !V | !N & V
pub const IF_GT: u32 = 0b1110_0000_0000; // GreaterThan     N & V & !Z | !N & !V & !Z
pub const IF_LE: u32 = 0b1111_0000_0000; // LessOrEqual     Z | N & !V | !N & V

pub const DISPLACEMENT_16: u32 = 0x00;
pub const DISPLACEMENT_32: u32 = 0xFF;

pub const OPER_DN  : u32 = 0x00;
pub const OPER_AN  : u32 = 0x08;
pub const OPER_AI  : u32 = 0x10;
pub const OPER_PI  : u32 = 0x18;
pub const OPER_PD  : u32 = 0x20;
pub const OPER_DI  : u32 = 0x28;
pub const OPER_IX  : u32 = 0x30;
pub const OPER_AW  : u32 = 0x38;
pub const OPER_AL  : u32 = 0x39;
pub const OPER_PCDI: u32 = 0x3a;
pub const OPER_PCIX: u32 = 0x3b;
pub const OPER_IMM : u32 = 0x3c;

pub const RR_MODE: u32 = 0x00;  // Register to Register
pub const MM_MODE: u32 = 0x08;  // Memory to Memory

pub const SHIFT_RIGHT: u32 = 0x000;
pub const SHIFT_LEFT : u32 = 0x100;
pub const IMM_COUNT  : u32 = 0x00;
pub const REG_COUNT  : u32 = 0x20;
// Seems that ASL, LSL, ROXL and ROL differs in just two bits;
// However, these bits are placed differently in MEM_SHIFTs and REG_SHIFTs
// so we need 2*4 constants, not 2+4
pub const ARIT_REG_SHIFT  : u32 = 0b00000;
pub const LOGI_REG_SHIFT  : u32 = 0b01000;
pub const ROTX_REG_SHIFT  : u32 = 0b10000;
pub const ROTA_REG_SHIFT  : u32 = 0b11000;
pub const ARIT_MEM_SHIFT  : u32 = 0xC0 | (ARIT_REG_SHIFT << 6);
pub const LOGI_MEM_SHIFT  : u32 = 0xC0 | (LOGI_REG_SHIFT << 6);
pub const ROTX_MEM_SHIFT  : u32 = 0xC0 | (ROTX_REG_SHIFT << 6);
pub const ROTA_MEM_SHIFT  : u32 = 0xC0 | (ROTA_REG_SHIFT << 6);

pub const MOVE_FROM_SR : u32 = 0x0c0;
// pub const MOVE_FROM_CCR : u32 = 0x2c0; // Only 010+
pub const MOVE_TO_CCR  : u32 = 0x4c0;
pub const MOVE_TO_SR   : u32 = 0x6c0;

pub const DEST_DX: u32 = 0x000;
pub const DEST_EA: u32 = 0x100;
pub const DEST_SR: u32 = 0x03c;

// ADDA does not follow the ADD pattern for 'oper' so we cannot use the
// above constants
pub const DEST_AX_WORD: u32 = 0x0C0;
pub const DEST_AX_LONG: u32 = 0x1C0;

pub const OP_UNIMPLEMENTED_1010 : u32 = 0b1010_0000_0000_0000;
pub const OP_UNIMPLEMENTED_1111 : u32 = 0b1111_0000_0000_0000;

pub const BYTE_MOVE: u32 = 0x1000;
pub const WORD_MOVE: u32 = 0x3000;
pub const LONG_MOVE: u32 = 0x2000;

// OPER_XX:s are the 6 least significant bits structured as mmmrrr and
// we need to swap and shift that into place as rrrmmm000000
// to generate the MOVE_TO_XX:s
pub const MOVE_TO_DN  : u32 = (OPER_DN & 0b11_1000) << 3; // rrr == 0
pub const MOVE_TO_AN  : u32 = (OPER_AN & 0b11_1000) << 3; // rrr == 0
pub const MOVE_TO_AI  : u32 = (OPER_AI & 0b11_1000) << 3; // rrr == 0
pub const MOVE_TO_PI  : u32 = (OPER_PI & 0b11_1000) << 3; // rrr == 0
pub const MOVE_TO_PD  : u32 = (OPER_PD & 0b11_1000) << 3; // rrr == 0
pub const MOVE_TO_DI  : u32 = (OPER_DI & 0b11_1000) << 3; // rrr == 0
pub const MOVE_TO_IX  : u32 = (OPER_IX & 0b11_1000) << 3; // rrr == 0
pub const MOVE_TO_AW  : u32 = (OPER_AW & 0b11_1000) << 3; // rrr == 0
pub const MOVE_TO_AL  : u32 = (OPER_AL & 0b11_1000) << 3 | (OPER_AL & 0b111) << 9;
// const MOVE_IMM : u32 = (OPER_IMM & 0b111000) << 3 | (OPER_IMM & 0b111) << 9;

// Bit operations
pub const SRC_REG: u32 = 0x100;
pub const SRC_IMM: u32 = 0x800;
pub const BIT_TST: u32 = 0x00;
pub const BIT_CHG: u32 = 0x40;
pub const BIT_CLR: u32 = 0x80;
pub const BIT_SET: u32 = 0xC0;

// MOVEM constants
pub const WORD_TRANSFER: u32 = 0x00;
pub const LONG_TRANSFER: u32 = 0x40;
pub const REGISTER_TO_MEMORY: u32 = 0x000;
pub const MEMORY_TO_REGISTER: u32 = 0x400;

// MOVE USP constants
pub const MOVE_USP: u32 = 0xe60;
pub const TO_AN: u32 = 0x0;
pub const FROM_AN: u32 = 0x8;

// MOVEP constants
pub const MOVEP_MEMORY_TO_REGISTER: u32 = 0x100;
pub const MOVEP_REGISTER_TO_MEMORY: u32 = 0x180;

// EXG constants
pub const EXG_DATA_DATA: u32 = 0x40; // Exchange two data registers
pub const EXG_ADDR_ADDR: u32 = 0x48; // Exchange two address registers
pub const EXG_DATA_ADDR: u32 = 0x88; // Exchange a data register and an address register

// EXT constants (these are the same as DEST_AX_WORD,
// DEST_AX_LONG, perhaps there's a better common name somewhere)
pub const BYTE_TO_WORD: u32 = 0x080;
pub const WORD_TO_LONG: u32 = 0x0C0;
// const BYTE_TO_LONG: u32 = 0x1C0; // 020+

// CHK constants
pub const WORD_OP: u32 = 0x180;
// const LONG_OP: u32 = 0x100;  only implemented by MC68020+
