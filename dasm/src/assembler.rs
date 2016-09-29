use operand::Operand;
use memory::Memory;
use super::{OpcodeInstance, Size};

fn encode_ea(op: &Operand) -> u16 {
    (match *op {
        Operand::DataRegisterDirect(reg_y) => 0b000000 | reg_y,
        Operand::AddressRegisterDirect(reg_y) => 0b001000 | reg_y,
        Operand::AddressRegisterIndirect(reg_y) => 0b010000 | reg_y,
        Operand::AddressRegisterIndirectWithPostincrement(reg_y) => 0b011000 | reg_y,
        Operand::AddressRegisterIndirectWithPredecrement(reg_y) => 0b100000 | reg_y,
        Operand::AddressRegisterIndirectWithDisplacement(reg_y, _) => 0b101000 | reg_y,
        Operand::AddressRegisterIndirectWithIndex(reg_y, _, _) => 0b110000 | reg_y,
        Operand::AbsoluteWord(_) => 0b111000,
        Operand::AbsoluteLong(_) => 0b111001,
        Operand::PcWithDisplacement(_) => 0b111010,
        Operand::PcWithIndex(_, _) => 0b111011,
        Operand::Immediate(_, _) => 0b111100,
    }) as u16
}

fn encode_dx(op: &Operand) -> u16 {
    match *op {
        Operand::DataRegisterDirect(reg_x) => (reg_x as u16) << 9,
        _ => panic!("not dx-encodable: {:?}", *op)
    }
}

fn encode_ax(op: &Operand) -> u16 {
    match *op {
        Operand::AddressRegisterDirect(reg_x) => (reg_x as u16) << 9,
        _ => panic!("not ax-encodable: {:?}", *op)
    }
}

pub fn encode_ea_dx(op: &OpcodeInstance, template: u16, pc: u32, mem: &mut Memory) -> u32 {
    let ea = encode_ea(&op.operands[0]);
    let dx = encode_dx(&op.operands[1]);
    // println!("{} EA/DX {:4x}, ea {:2x}, dx {:4x}", op.mnemonic, template, ea, dx);
    if template & ea & dx > 0 { panic!("template {:4x}, ea {:2x}, dx {:4x} overlaps for {}", template, ea, dx, op); };
    mem.write_word(pc, template | ea | dx);
    op.operands[0].add_extension_words(pc + 2, mem)
}

pub fn encode_ea_ax(op: &OpcodeInstance, template: u16, pc: u32, mem: &mut Memory) -> u32 {
    let ea = encode_ea(&op.operands[0]);
    let ax = encode_ax(&op.operands[1]);
    // println!("{} EA/AX {:4x}, ea {:2x}, ax {:4x}", op.mnemonic, template, ea, ax);
    if template & ea & ax > 0 { panic!("template {:4x}, ea {:2x}, ax {:4x} overlaps for {}", template, ea, ax, op); };
    mem.write_word(pc, template | ea | ax);
    op.operands[0].add_extension_words(pc + 2, mem)
}

pub fn encode_dx_ea(op: &OpcodeInstance, template: u16, pc: u32, mem: &mut Memory) -> u32 {
    let ea = encode_ea(&op.operands[1]);
    let dx = encode_dx(&op.operands[0]);
    // println!("{} DX/EA {:4x}, ea {:2x}, dx {:4x}", op.mnemonic, template, ea, dx);
    if template & ea & dx > 0 { panic!("template {:4x}, ea {:2x}, dx {:4x} overlaps for {}", template, ea, dx, op); };
    mem.write_word(pc, template | ea | dx);
    op.operands[1].add_extension_words(pc + 2, mem)
}

pub fn encode_imm_ea(op: &OpcodeInstance, template: u16, pc: u32, mem: &mut Memory) -> u32 {
    let ea = encode_ea(&op.operands[1]);
    // println!("{} imm/EA {:4x}, ea {:2x}, dx {:4x}", op.mnemonic, template, ea, dx);
    if template & ea > 0 { panic!("template {:4x}, ea {:2x} overlaps for {}", template, ea, op); };
    let pc = mem.write_word(pc, template | ea);
    let pc = op.operands[0].add_extension_words(pc, mem);
    op.operands[1].add_extension_words(pc, mem)
}

pub fn encode_instruction(instruction: &str, op_inst: &OpcodeInstance, pc: u32, mem: &mut Memory) -> u32
{
    let optable = super::generate();
    for op in optable {
        if op_inst.mnemonic == op.mnemonic && op_inst.size == op.size && (op.selector)(op_inst) { 
            let encoder = op.encoder;
            return encoder(op_inst, op.matching as u16, pc, mem);
        }
    }
    panic!("Could not assemble {} ({})", instruction, op_inst);
}

use regex::RegexSet;
use regex::Regex;

pub fn parse_assembler(instruction: &str) -> OpcodeInstance {
    let re = Regex::new(r"^(\w+)(\.\w)?(\s+(\w\d|-?\$?[\dA-F]*\([\w,0-9]+\)\+?|#?\$?[\dA-F]+(?:\.\w)?)(,(\w\d|-?\$?[\dA-F]*\([\w,0-9]+\)\+?|#?-?\$?[\dA-F]+(?:\.\w)?))?)$").unwrap();
    let im = re.captures(instruction);
    if im.is_none() {
        panic!("Syntax Error: {:?} does not match instruction pattern {:?}", instruction, re);
    }
    let imatch = im.unwrap();
    let (ins, size, op1, op2) = (imatch.at(1).unwrap_or(""), imatch.at(2).unwrap_or(""), imatch.at(4).unwrap_or(""), imatch.at(6).unwrap_or(""));
    let size = match size {
        ".B" => Size::Byte,
        ".W" => Size::Word,
        ".L" => Size::Long,
        _ => Size::Unsized,
    };

    let drd = Regex::new(r"^D([0-7])$").unwrap();
    let ard = Regex::new(r"^A([0-7])$").unwrap();
    let ari = Regex::new(r"^\(A([0-7])\)$").unwrap();
    let api = Regex::new(r"^\(A([0-7])\)\+$").unwrap();
    let apd = Regex::new(r"^-\(A([0-7])\)$").unwrap();
    let adi = Regex::new(r"^(-?\d+)\(A([0-7])\)$",).unwrap();
    let aix = Regex::new(r"^(-?\d+)\(A([0-7]),([DA])([0-7])\)$").unwrap();
    let hex = Regex::new(r"^\$([\dA-F]+)$").unwrap();
    let lng = Regex::new(r"^\$([\dA-F]+)\.L$").unwrap();
    let pcd = Regex::new(r"^(-?\d+)\(PC\)$").unwrap();
    let pci = Regex::new(r"^(-?\d+)\(PC,([DA])([0-7])\)$").unwrap();
    let imm = Regex::new(r"^#\$([\dA-F]+)$").unwrap();
    let modes = RegexSet::new(&[
        drd.as_str(),
        ard.as_str(),
        ari.as_str(),
        api.as_str(),
        apd.as_str(),
        adi.as_str(),
        aix.as_str(),
        hex.as_str(),
        lng.as_str(),
        pcd.as_str(),
        pci.as_str(),
        imm.as_str(),
    ]).unwrap();
    println!("{}", instruction);
    let mode1 = modes.matches(op1).into_iter().nth(0);
    let mode2 = modes.matches(op2).into_iter().nth(0);
    let get_chr = |rx: &Regex, op: &str, at: usize| rx.captures(op).unwrap().at(at).unwrap().chars().next().unwrap();
    let get_num = |rx: &Regex, op: &str, at: usize|->i32 { rx.captures(op).unwrap().at(at).unwrap().parse().unwrap() };
    let get_hex = |rx: &Regex, op: &str, at: usize|->u32 { u32::from_str_radix(rx.captures(op).unwrap().at(at).unwrap(), 16).unwrap() };
    let to_op = |opinfo:(Option<usize>, &str)| {
        let (v, op) = opinfo;
        match v {
            None => if !op.is_empty() { panic!("operand {:?} couldn't be matched", op)} else { None },
            Some(0) => Some(Operand::DataRegisterDirect(get_num(&drd, op, 1) as u8)),
            Some(1) => Some(Operand::AddressRegisterDirect(get_num(&ard, op, 1) as u8)),
            Some(2) => Some(Operand::AddressRegisterIndirect(get_num(&ari, op, 1) as u8)),
            Some(3) => Some(Operand::AddressRegisterIndirectWithPostincrement(get_num(&api, op, 1) as u8)),
            Some(4) => Some(Operand::AddressRegisterIndirectWithPredecrement(get_num(&apd, op, 1) as u8)),
            Some(5) => Some(Operand::AddressRegisterIndirectWithDisplacement(get_num(&adi, op, 2) as u8, get_num(&adi, op, 1) as i16)),
            Some(6) => {
                let offset = if get_chr(&aix, op, 3) == 'D' {
                    0
                } else {
                    8
                };
                let i_reg = offset + get_num(&aix, op, 4) as u8;
                Some(Operand::AddressRegisterIndirectWithIndex(get_num(&aix, op, 2) as u8, i_reg, get_num(&aix, op, 1) as i8))
            },
            Some(7) => {
                let hex: u32 = get_hex(&hex, op, 1);
                if hex > 0x7FFF {
                    Some(Operand::AbsoluteLong(hex))
                } else {
                    Some(Operand::AbsoluteWord(hex as u16))
                }
            },
            Some(8) => Some(Operand::AbsoluteLong(get_hex(&lng, op, 1))),
            Some(9) => Some(Operand::PcWithDisplacement(get_num(&pcd, op, 1) as i16)),
            Some(10) => {
                let offset = if get_chr(&pci, op, 2) == 'D' {
                    0
                } else {
                    8
                };
                let i_reg = offset + get_num(&pci, op, 3) as u8;
                Some(Operand::PcWithIndex(i_reg, get_num(&pci, op, 1) as i8))
            },
            Some(11) => Some(Operand::Immediate(size, get_hex(&imm, op, 1) as u32)),
            // TODO: Handle the remaining addressing modes
            _ => panic!("Operand syntax error {:?} {:?}", v, op)
        }
    };
    OpcodeInstance {mnemonic: ins, size: size, operands: vec![(mode1, op1), (mode2, op2)].into_iter().filter_map(to_op).collect::<Vec<_>>()}
}

#[cfg(test)]
mod tests {
    use operand::Operand;
    use memory::{MemoryVec, Memory};
    use super::{parse_assembler, encode_instruction};
    use super::super::Size;

    #[test]
    fn encodes_add_8_er() {
        let asm = "ADD.B\t(A1),D2";
        let inst = parse_assembler(asm);
        assert_eq!("ADD", inst.mnemonic);
        assert_eq!(Size::Byte, inst.size);
        assert_eq!(Operand::AddressRegisterIndirect(1), inst.operands[0]);
        assert_eq!(Operand::DataRegisterDirect(2), inst.operands[1]);
        let mut mem = &mut MemoryVec { mem: vec![0x00, 0x00, 0x00, 0x00]};
        let pc = 0;
        let new_pc = encode_instruction(asm, &inst, pc, mem);
        assert_eq!(2, new_pc);
        assert_eq!(0xd411, mem.read_word(pc));
    }
    #[test]
    fn encodes_add_8_re() {
        let asm = "ADD.B\tD2,(A1)";
        let inst = parse_assembler(asm);
        assert_eq!("ADD", inst.mnemonic);
        assert_eq!(Size::Byte, inst.size);
        assert_eq!(Operand::DataRegisterDirect(2), inst.operands[0]);
        assert_eq!(Operand::AddressRegisterIndirect(1), inst.operands[1]);
        let mut mem = &mut MemoryVec { mem: vec![0x00, 0x00, 0x00, 0x00]};
        let pc = 0;
        let new_pc = encode_instruction(asm, &inst, pc, mem);
        assert_eq!(2, new_pc);
        assert_eq!(0xd511, mem.read_word(pc));
    }

}