use operand::Operand;
use memory::Memory;
use constants::*;
use super::{Result, Size, Exception,OpcodeInstance,generate};
use PC;
use Words;

fn decode_destination_ea(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Operand) {
    let mode = ((opcode >> 6) & 0b111) as u8;
    let reg_y = ((opcode >> 9) & 0b111) as u8;
    effective_address(size, pc, mem, mode, reg_y)
}

fn decode_ea(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Operand) {
    let mode = ((opcode >> 3) & 0b111) as u8;
    let reg_y = (opcode & 0b111) as u8;
    effective_address(size, pc, mem, mode, reg_y)
}

fn effective_address(size: Size, pc: PC, mem: &Memory, mode: u8, reg_y: u8) -> (Words, Operand) {
    match mode {
        0b000 => (Words(0), Operand::DataRegisterDirect(reg_y)),
        0b001 => (Words(0), Operand::AddressRegisterDirect(reg_y)),
        0b010 => (Words(0), Operand::AddressRegisterIndirect(reg_y)),
        0b011 => (Words(0), Operand::AddressRegisterIndirectWithPostincrement(reg_y)),
        0b100 => (Words(0), Operand::AddressRegisterIndirectWithPredecrement(reg_y)),
        0b101 => (Words(1), Operand::AddressRegisterIndirectWithDisplacement(reg_y, mem.read_word(pc + 2) as i16)),
        0b110 => {
            let (indexinfo, displacement) = decode_extension_word(mem.read_word(pc + 2));
            (Words(1), Operand::AddressRegisterIndirectWithIndex(reg_y, indexinfo, displacement))
        },
        0b111 => match reg_y {
            0b010 => (Words(1), Operand::PcWithDisplacement(mem.read_word(pc + 2) as i16)),
            0b011 => {
                let (indexinfo, displacement) = decode_extension_word(mem.read_word(pc + 2));
                (Words(1), Operand::PcWithIndex(indexinfo, displacement))
            },
            0b000 => (Words(1), Operand::AbsoluteWord(mem.read_word(pc + 2))),
            0b001 => (Words(2), Operand::AbsoluteLong((mem.read_word(pc + 2) as u32) << 16 | mem.read_word(pc + 4) as u32)),
            0b100 =>
                match size {
                    Size::Byte => (Words(1), Operand::Immediate(size, (mem.read_word(pc + 2) & 0xFF) as u32)),
                    Size::Word => (Words(1), Operand::Immediate(size, mem.read_word(pc + 2) as u32)),
                    Size::Long => (Words(2), Operand::Immediate(size, (mem.read_word(pc + 2) as u32) << 16 | mem.read_word(pc + 4) as u32)),
                    Size::Unsized => panic!("unsized Immediate"),
                },
            _ => panic!("Unknown addressing mode {:03b} reg {:03b}", mode, reg_y),
        },
        _ => panic!("Unknown addressing mode {:03b} reg {:03b}", mode, reg_y),
    }
}

fn decode_extension_word(extension: u16) -> (u8, i8) {
    // top four bits = (D/A RRR) matches our register array layout
    let xreg_ndx_size = (extension>>12) as u8;
    let displacement = extension as i8;
    (xreg_ndx_size, displacement)
}
#[allow(unused_variables)]
fn decode_dx(opcode: u16) -> Operand {
    Operand::DataRegisterDirect(((opcode >> 9) & 7) as u8)
}
fn decode_dy(opcode: u16) -> Operand {
    Operand::DataRegisterDirect((opcode & 0b111) as u8)
}
fn decode_ax(opcode: u16) -> Operand {
    Operand::AddressRegisterDirect(((opcode >> 9) & 7) as u8)
}
fn decode_ay(opcode: u16) -> Operand {
    Operand::AddressRegisterDirect((opcode & 0b111) as u8)
}
fn decode_pdx(opcode: u16) -> Operand {
    Operand::AddressRegisterIndirectWithPredecrement(((opcode >> 9) & 7) as u8)
}
fn decode_pdy(opcode: u16) -> Operand {
    Operand::AddressRegisterIndirectWithPredecrement((opcode & 0b111) as u8)
}
fn decode_pix(opcode: u16) -> Operand {
    Operand::AddressRegisterIndirectWithPostincrement(((opcode >> 9) & 7) as u8)
}
fn decode_piy(opcode: u16) -> Operand {
    Operand::AddressRegisterIndirectWithPostincrement((opcode & 0b111) as u8)
}
fn decode_imm(size: Size, pc: PC, mem: &Memory) -> (Words, Operand) {
    match size {
        Size::Byte => (Words(1), Operand::Immediate(size, (mem.read_word(pc+2) & 0xFF) as u32)),
        Size::Word => (Words(1), Operand::Immediate(size, mem.read_word(pc+2) as u32)),
        Size::Long => (Words(2), Operand::Immediate(size, (mem.read_word(pc+2) as u32) << 16 | mem.read_word(pc+4) as u32)),
        Size::Unsized => panic!("unsized Immediate"),
    }
}
fn decode_movem(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Operand) {
    (Words(1), Operand::Registers(mem.read_word(pc+2), false))
}
fn decode_diy(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Operand) {
    (Words(1), Operand::AddressRegisterIndirectWithDisplacement((opcode & 0b111) as u8, mem.read_word(pc+2) as i16))
}
fn decode_quick(opcode: u16) -> Operand {
    // Three bits of immediate data (0-7)
    let quick = match ((opcode >> 9) & 7) as u32 {
        0 => 8, // zero represents eight
        x => x, // 1 – 7 represent themselves
    };
    Operand::Immediate(Size::Byte, quick)
}
fn decode_imm4(opcode: u16) -> Operand {
    // Four bits of immediate data (0-15)
    Operand::Immediate(Size::Byte, (opcode & 0b1111) as u32)
}
pub fn decode_ea_sr(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, ea) = decode_ea(opcode, size, pc, mem);
    (words, vec![ea, Operand::StatusRegister(Size::Word)])
}
pub fn decode_sr_ea(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, ea) = decode_ea(opcode, size, pc, mem);
    (words, vec![Operand::StatusRegister(Size::Word), ea])
}
pub fn decode_ea_ccr(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, ea) = decode_ea(opcode, size, pc, mem);
    (words, vec![ea, Operand::StatusRegister(Size::Byte)])
}
pub fn decode_imm_sr(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, imm) = decode_imm(Size::Word, pc, mem);
    (words, vec![imm, Operand::StatusRegister(Size::Word)])
}
pub fn decode_imm_ccr(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, imm) = decode_imm(Size::Byte, pc, mem);
    (words, vec![imm, Operand::StatusRegister(Size::Byte)])
}
pub fn decode_usp_ay(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let ay = decode_ay(opcode);
    (Words(0), vec![Operand::UserStackPointer, ay])
}
pub fn decode_ay_usp(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let ay = decode_ay(opcode);
    (Words(0), vec![ay, Operand::UserStackPointer])
}
pub fn decode_ay_imm16(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let ay = decode_ay(opcode);
    let (words, imm) = decode_imm(Size::Word, pc, mem);

    (words, vec![ay, imm])
}
pub fn decode_just_ay(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let ay = decode_ay(opcode);
    (Words(0), vec![ay])
}
pub fn decode_ea_dx(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, ea) = decode_ea(opcode, size, pc, mem);
    (words, vec![ea, decode_dx(opcode)])
}
pub fn decode_diy_dx(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, di) = decode_diy(opcode, size, pc, mem);
    (words, vec![di, decode_dx(opcode)])
}
pub fn decode_dx_diy(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, di) = decode_diy(opcode, size, pc, mem);
    (words, vec![decode_dx(opcode), di])
}
pub fn decode_moveq(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    (Words(0), vec![Operand::Number(Size::Byte, (opcode & 0xff) as i32), decode_dx(opcode)])
}
pub fn decode_none(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    (Words(0), vec![])
}
pub fn decode_just_ea(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, ea) = decode_ea(opcode, size, pc, mem);
    (words, vec![ea])
}
pub fn decode_ea_ea(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, src) = decode_ea(opcode, size, pc, mem);
    let (words2, dst) = decode_destination_ea(opcode, size, pc + words, mem);
    (words+words2, vec![src, dst])
}
pub fn decode_ea_ax(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, ea) = decode_ea(opcode, size, pc, mem);
    (words, vec![ea, decode_ax(opcode)])
}
pub fn decode_dx_ea(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, ea) = decode_ea(opcode, size, pc, mem);
    (words, vec![decode_dx(opcode), ea])
}
pub fn decode_pdx_pdy(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let pdx = decode_pdx(opcode);
    let pdy = decode_pdy(opcode);
    (Words(0), vec![pdx, pdy])
}
pub fn decode_pix_piy(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let pix = decode_pix(opcode);
    let piy = decode_piy(opcode);
    (Words(0), vec![pix, piy])
}

pub fn decode_imm_ea(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, imm) = decode_imm(size, pc, mem);
    let (words2, ea) = decode_ea(opcode, size, pc + words, mem);
    (words + words2, vec![imm, ea])
}
pub fn decode_dy_imm(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, imm) = decode_imm(size, pc, mem);
    (words, vec![decode_dy(opcode), imm])
}
pub fn decode_imm8_dy(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, imm) = decode_imm(Size::Byte, pc, mem);
    (words, vec![imm, decode_dy(opcode)])
}
pub fn decode_just_imm16(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, imm) = decode_imm(Size::Word, pc, mem);
    (words, vec![imm])
}
pub fn decode_quick_ea(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let quick = decode_quick(opcode);
    let (words, ea) = decode_ea(opcode, size, pc, mem);
    (words, vec![quick, ea])
}
pub fn decode_quick_dy(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let quick = decode_quick(opcode);
    let dy = decode_dy(opcode);
    (Words(0), vec![quick, dy])
}
pub fn decode_just_dy(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let dy = decode_dy(opcode);
    (Words(0), vec![dy])
}
pub fn decode_just_imm4(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let imm4 = decode_imm4(opcode);
    (Words(0), vec![imm4])
}
pub fn decode_dx_dy(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let dx = decode_dx(opcode);
    let dy = decode_dy(opcode);
    (Words(0), vec![dx, dy])
}
pub fn decode_dx_ay(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let dx = decode_dx(opcode);
    let ay = decode_ay(opcode);
    (Words(0), vec![dx, ay])
}
pub fn decode_ax_ay(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let ax = decode_ax(opcode);
    let ay = decode_ay(opcode);
    (Words(0), vec![ax, ay])
}
pub fn decode_branch(opcode: u16, _size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let disp8 = opcode & 0xFF;
    let new_pc: PC = pc + 2;
    let (words, displacement) = if disp8 > 0 && disp8 < 0xff {
        (Words(0), Operand::Branch(Size::Byte, (new_pc + (disp8 as u8 as i8 as i32)).0))
    } else if disp8 == 00 {
        (Words(1), Operand::Branch(Size::Word, (new_pc + mem.read_word(new_pc) as i16 as i32).0))
    } else if disp8 == 0xff {
        (Words(2), Operand::Branch(Size::Long, (new_pc + ((mem.read_word(new_pc) as u32) << 16 | mem.read_word(new_pc + 2) as u32) as i32).0))
    } else {
        unreachable!()
    };
    (words, vec![displacement])
}
pub fn decode_movem_ea(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, movem) = decode_movem(opcode, size, pc, mem);
    let (words2, ea) = decode_ea(opcode, size, pc + words, mem);
    let possibly_reversed = if let Operand::AddressRegisterIndirectWithPredecrement(_) = ea {
        if let Operand::Registers(reglist, _) = movem {
            Operand::Registers(reglist, true)
        } else {
            movem
        }
    } else {
        movem
    };
    (words + words2, vec![possibly_reversed, ea])
}
pub fn decode_ea_movem(opcode: u16, size: Size, pc: PC, mem: &Memory) -> (Words, Vec<Operand>) {
    let (words, movem) = decode_movem(opcode, size, pc, mem);
    let (words2, ea) = decode_ea(opcode, size, pc + words, mem);
    (words + words2, vec![ea, movem])
}
/* Check if opcode is using a valid ea mode */
pub fn valid_ea(opcode: u16, ea_mask: u16) -> bool
{
    if ea_mask == 0 {
        true
    } else {
        // ea is the lower six bits of the opcode
        match opcode & 0x3f {
            0x00 ... 0x07 => (ea_mask & EA_DATA_REGISTER_DIRECT) != 0,
            0x08 ... 0x0f => (ea_mask & EA_ADDRESS_REGISTER_DIRECT) != 0,
            0x10 ... 0x17 => (ea_mask & EA_ADDRESS_REGISTER_INDIRECT) != 0,
            0x18 ... 0x1f => (ea_mask & EA_ARI_POSTINCREMENT) != 0,
            0x20 ... 0x27 => (ea_mask & EA_ARI_PREDECREMENT) != 0,
            0x28 ... 0x2f => (ea_mask & EA_ARI_DISPLACEMENT) != 0,
            0x30 ... 0x37 => (ea_mask & EA_ARI_INDEX) != 0,
            0x38 => (ea_mask & EA_ABSOLUTE_SHORT) != 0,
            0x39 => (ea_mask & EA_ABSOLUTE_LONG) != 0,
            0x3a => (ea_mask & EA_PC_DISPLACEMENT) != 0,
            0x3b => (ea_mask & EA_PC_INDEX) != 0,
            0x3c => (ea_mask & EA_IMMEDIATE) != 0,
            _ => false
        }
    }
}

fn get_dest_ea(opcode: u16) -> u16 {
    // normally ea are the 6 least significant bits structured as mmmrrr
    // but for move op codes, the destination ea is stored as rrrmmm000000
    // (where the lower six bits are the source ea)
    // we need to swap and shift that into place
    let mode = (opcode >> 3) & 0b111000;
    let reg_y = (opcode >> 9) & 0b111;
    mode | reg_y
}

pub fn ea_memory_alterable(opcode: u16) -> bool { valid_ea(opcode, EA_MEMORY_ALTERABLE) }
pub fn ea_all_except_an(opcode: u16) -> bool { valid_ea(opcode, EA_ALL_EXCEPT_AN) }
pub fn ea_all(opcode: u16) -> bool { valid_ea(opcode, EA_ALL) }
pub fn ea_data_alterable(opcode: u16) -> bool { valid_ea(opcode, EA_DATA_ALTERABLE) }
pub fn ea_data_alterable_except_dn(opcode: u16) -> bool { valid_ea(opcode, EA_DATA_ALTERABLE & !EA_DATA_REGISTER_DIRECT) }
pub fn ea_all_except_an_to_data_alterable(opcode: u16) -> bool { valid_ea(opcode, EA_ALL_EXCEPT_AN) && valid_ea(get_dest_ea(opcode), EA_DATA_ALTERABLE) }
pub fn ea_all_to_data_alterable(opcode: u16) -> bool { valid_ea(opcode, EA_ALL) && valid_ea(get_dest_ea(opcode), EA_DATA_ALTERABLE) }
pub fn ea_data(opcode: u16) -> bool { valid_ea(opcode, EA_DATA) }
pub fn ea_data_except_dn(opcode: u16) -> bool { valid_ea(opcode, EA_DATA & !EA_DATA_REGISTER_DIRECT) }
pub fn ea_dn(opcode: u16) -> bool { valid_ea(opcode, EA_DATA_REGISTER_DIRECT) }
pub fn ea_alterable(opcode: u16) -> bool { valid_ea(opcode, EA_ALTERABLE) }
pub fn ea_alterable_except_an(opcode: u16) -> bool { valid_ea(opcode, EA_ALTERABLE & !EA_ADDRESS_REGISTER_DIRECT) }
pub fn ea_control(opcode: u16) -> bool { valid_ea(opcode, EA_CONTROL) }
pub fn ea_control_or_pi(opcode: u16) -> bool { valid_ea(opcode, EA_CONTROL_OR_PI) }
pub fn ea_control_alterable_or_pd(opcode: u16) -> bool { valid_ea(opcode, EA_CONTROL_ALTERABLE_OR_PD) }

pub fn always(_opcode: u16) -> bool { true }
pub fn valid_byte_displacement(opcode: u16) -> bool {
    let disp8 = opcode & 0xff;
    disp8 > 0 && disp8 < 0xff
}
pub fn never(_opcode: u16) -> bool { false }

pub fn disassemble_first(mem: &Memory) -> (PC, OpcodeInstance) {
    disassemble(PC(0), mem).unwrap()
}
const INSTRUCTION_SIZE: Words = Words(1);

pub fn disassemble(pc: PC, mem: &Memory) -> Result<(PC, OpcodeInstance)> {
    let optable = generate();
    let opcode = mem.read_word(pc);
    // println!("opcode read was {:04x}", opcode);
    for op in optable {
        // check for mask/opcode inconsistency
        assert!(op.mask & op.matching == op.matching, format!("mask/matching mismatch {:04x} & {:04x} for {}{}", op.mask, op.matching, op.mnemonic, op.size));
        if ((opcode as u32) & op.mask) == op.matching && (op.validator)(opcode) {
            let decoder = op.decoder;
            let (extension_words, operands) = decoder(opcode, op.size, pc, mem);
            return Ok((pc + INSTRUCTION_SIZE + extension_words, OpcodeInstance {mnemonic: op.mnemonic, size: op.size, operands: operands }));
        } else if ((opcode as u32) & op.mask) == op.matching {
            // println!("{:04x}: match for {}{} without passing validator", opcode, op.mnemonic, op.size);
        }
    }
    Err(Exception::IllegalInstruction(opcode, pc))
}


#[cfg(test)]
mod tests {
    use operand::Operand;
    use memory::MemoryVec;
    use super::disassemble_first;
    use super::super::Size;
    use PC;
    use Words;
    use disassembler::disassemble;

    #[test]
    fn decodes_short_forward_jump() {
        // jump forward 6 more bytes (because PC is 1002 after reading the instruction)
        let mem = MemoryVec::new16(PC(0x1000), vec![0x6006]);
        let (pc, inst) = disassemble(PC(0x1000), &mem).unwrap();

        assert_eq!("BRA", inst.mnemonic);
        assert_eq!(Size::Byte, inst.size);
        assert_eq!(Operand::Branch(Size::Byte, 0x1008), inst.operands[0]);
        assert_eq!("$1008", format!("{}", inst.operands[0]));
        assert_eq!("BRA.B\t$1008", format!("{}", inst));
        assert_eq!(pc, PC(0x1002))
    }
    #[test]
    fn decodes_short_next_jump() {
        // jump forwards (00 == offset in next extension word) + two bytes (to skip the extension word)
        // PRM says "A branch to the immediately following instruction automatically uses the 16-bit
        // displacement format because the 8-bit displacement field contains $00 (zero offset)"
        let mem = MemoryVec::new16(PC(0x1000), vec![0x6000, 0x0002]);
        let (pc, inst) = disassemble(PC(0x1000), &mem).unwrap();

        assert_eq!("BRA", inst.mnemonic);
        assert_eq!(Size::Word, inst.size);
        assert_eq!(Operand::Branch(Size::Word, 0x1004), inst.operands[0]);
        assert_eq!("$1004", format!("{}", inst.operands[0]));
        assert_eq!("BRA.W\t$1004", format!("{}", inst));
        assert_eq!(pc, PC(0x1004))
    }
    #[test]
    fn decodes_short_self_jump() {
        // jump back two bytes (because PC is 1002 after reading the instruction)
        let mem = MemoryVec::new16(PC(0x1000), vec![0x60FE]);
        let (pc, inst) = disassemble(PC(0x1000), &mem).unwrap();

        assert_eq!("BRA", inst.mnemonic);
        assert_eq!(Size::Byte, inst.size);
        assert_eq!(Operand::Branch(Size::Byte, 0x1000), inst.operands[0]);
        assert_eq!("$1000", format!("{}", inst.operands[0]));
        assert_eq!("BRA.B\t$1000", format!("{}", inst));
        assert_eq!(pc, PC(0x1002))
    }
    #[test]
    fn decodes_add_8_er() {
        let mem = MemoryVec::new16(PC(0), vec![0xd411]);
        let (pc, inst) = disassemble_first(&mem);
        assert_eq!("ADD", inst.mnemonic);
        assert_eq!(Size::Byte, inst.size);
        assert_eq!(Operand::AddressRegisterIndirect(1), inst.operands[0]);
        assert_eq!(Operand::DataRegisterDirect(2), inst.operands[1]);
        assert_eq!("(A1)", format!("{}", inst.operands[0]));
        assert_eq!("D2", format!("{}", inst.operands[1]));
        assert_eq!("ADD.B\t(A1),D2", format!("{}", inst));
        assert_eq!(pc, PC(2))
    }
    #[test]
    fn decodes_add_8_re() {
        let mem = MemoryVec::new16(PC(0), vec![0xd511]);
        let (pc, inst) = disassemble_first(&mem);

        assert_eq!("ADD", inst.mnemonic);
        assert_eq!(Size::Byte, inst.size);
        assert_eq!(Operand::DataRegisterDirect(2), inst.operands[0]);
        assert_eq!(Operand::AddressRegisterIndirect(1), inst.operands[1]);
        assert_eq!("D2", format!("{}", inst.operands[0]));
        assert_eq!("(A1)", format!("{}", inst.operands[1]));
        assert_eq!("ADD.B\tD2,(A1)", format!("{}", inst));
        assert_eq!(pc, PC(2))
    }

    #[test]
    fn decodes_subq_zero_is_eight() {
        let mem = MemoryVec::new16(PC(0), vec![0x5100]); // quick data is zero, which should be interpreted as 8
        let (pc, inst) = disassemble_first(&mem);

        assert_eq!("SUBQ", inst.mnemonic);
        assert_eq!(Size::Byte, inst.size);
        assert_eq!(Operand::Immediate(Size::Byte, 8), inst.operands[0]);
        assert_eq!(Operand::DataRegisterDirect(0), inst.operands[1]);
        assert_eq!("SUBQ.B\t#$08,D0", format!("{}", inst));
        assert_eq!(pc, PC(2))
    }

    #[test]
    fn two_word_decode_imm_ea() {
        // ADDI #$12,$34(A0) is 0x0668 0x0012 0x0034
        let opcode = 0x0668;
        let dasm_mem = &mut MemoryVec::new16(PC(0), vec![opcode, 0x0012, 0x0034]);
        let (words, ops) = super::decode_imm_ea(opcode, Size::Byte, PC(0), dasm_mem);
        assert_eq!(ops[0], Operand::Immediate(Size::Byte, 0x12));
        assert_eq!(ops[1], Operand::AddressRegisterIndirectWithDisplacement(0, 0x34));
        assert_eq!(words, Words(2));
    }
    #[test]
    fn three_word_decode_imm_ea_di() {
        // ADDI.L #$1F,$77(A6) is 0x06AE 0x0000 0x001F 0x0077
        let opcode = 0x06AE;
        let dasm_mem = &mut MemoryVec::new16(PC(0), vec![opcode, 0x0000, 0x001F, 0x0077]);
        let (words, ops) = super::decode_imm_ea(opcode, Size::Long, PC(0), dasm_mem);
        assert_eq!(ops[0], Operand::Immediate(Size::Long, 0x1F));
        assert_eq!(ops[1], Operand::AddressRegisterIndirectWithDisplacement(6, 0x77));
        assert_eq!(words, Words(3));
    }
    #[test]
    fn three_word_decode_imm_ea_ix() {
        // ADDI.L #$1F00A4,52(A5,D2) is 0x06B5 0x001F 0x00A4 0x2034
        let opcode = 0x06B5;
        let dasm_mem = &mut MemoryVec::new16(PC(0), vec![opcode, 0x001F, 0x00A4, 0x2034]);
        let (words, ops) = super::decode_imm_ea(opcode, Size::Long, PC(0), dasm_mem);
        assert_eq!(ops[0], Operand::Immediate(Size::Long, 0x1F00A4));
        assert_eq!(ops[1], Operand::AddressRegisterIndirectWithIndex(5, 2, 0x34));
        assert_eq!(words, Words(3));
    }
    #[test]
    fn four_word_decode_imm_ea_al() {
        // ADDI.L #$1F00A4,$12345678 is 0x06B9 0x001F 0x00A4 0x1234 0x5678
        let opcode = 0x06B9;
        let dasm_mem = &mut MemoryVec::new16(PC(0), vec![opcode, 0x001F, 0x00A4, 0x1234, 0x5678]);
        let (words, ops) = super::decode_imm_ea(opcode, Size::Long, PC(0), dasm_mem);
        assert_eq!(ops[0], Operand::Immediate(Size::Long, 0x1F00A4));
        assert_eq!(ops[1], Operand::AbsoluteLong(0x12345678));
        assert_eq!(words, Words(4));
    }
}

