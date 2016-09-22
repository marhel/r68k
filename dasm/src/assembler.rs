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
        _ => panic!("not ea-encodable: {:?}", *op)
    }) as u16
}

fn encode_dx(op: &Operand) -> u16 {
    match *op {
        Operand::DataRegisterDirect(reg_x) => (reg_x as u16) << 9,
        _ => panic!("not dx-encodable: {:?}", *op)
    }
}

pub fn encode_ea_dx(op: &OpcodeInstance, template: u16, pc: u32, mem: &mut Memory) -> u32 {
    let ea = encode_ea(&op.operands[0]);
    let dx = encode_dx(&op.operands[1]);
    println!("{} EA/DX {:4x}, ea {:2x}, dx {:4x}", op.mnemonic, template, ea, dx);
    if template & ea & dx > 0 { panic!("template {:4x}, ea {:2x}, dx {:4x} overlaps for {}", template, ea, dx, op); };
    mem.write_word(pc, template | ea | dx);
    if op.operands[0].size() == 2 {
        mem.write_word(pc + 2, op.operands[0].extension_word());
        pc + 4
    } else {
        pc + 2
    }
}

pub fn encode_dx_ea(op: &OpcodeInstance, template: u16, pc: u32, mem: &mut Memory) -> u32 {
    let ea = encode_ea(&op.operands[1]);
    let dx = encode_dx(&op.operands[0]);
    println!("{} DX/EA {:4x}, ea {:2x}, dx {:4x}", op.mnemonic, template, ea, dx);
    if template & ea & dx > 0 { panic!("template {:4x}, ea {:2x}, dx {:4x} overlaps for {}", template, ea, dx, op); };
    mem.write_word(pc, template | ea | dx);
    pc + 2
}

pub fn encode_instruction(op_inst: &OpcodeInstance, pc: u32, mem: &mut Memory) -> u32
{
    let optable = super::generate();
    for op in optable {
        if op_inst.mnemonic == op.mnemonic && op_inst.size == op.size && (op.selector)(op_inst) {
            let encoder = op.encoder;
            return encoder(op_inst, op.matching as u16, pc, mem);
        }
    }
    panic!("Could not assemble {}", op_inst);
}

use regex::RegexSet;
use regex::Regex;

pub fn parse_assembler(instruction: &str) -> OpcodeInstance {
    println!("parse_assembler on {:?}", instruction);
    let re = Regex::new(r"^(\w+)(\.\w)?(\s+(\w\d|\d*-?\([\w,0-9]+\)\+?|\$\d+(?:\.\w)?)(,(\w\d|\d*-?\([DAPC,0-9]+\)\+?))?|\$\d+(?:\.\w)?)$").unwrap();
    let ins = re.captures(instruction).unwrap();
    let (ins, size, op1, op2) = (ins.at(1).unwrap_or(""), ins.at(2).unwrap_or(""), ins.at(4).unwrap_or(""), ins.at(6).unwrap_or(""));
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
    let adi = Regex::new(r"^(\d+)\(A([0-7])\)$",).unwrap();
    let aix = Regex::new(r"^(\d+)\(A([0-7]),([DA])([0-7])\)$").unwrap();
    let hex = Regex::new(r"^\$(\d+)$").unwrap();
    //let lng = Regex::new(r"^\$(\d+)\.L$").unwrap();

    let modes = RegexSet::new(&[
        drd.as_str(),
        ard.as_str(),
        ari.as_str(),
        api.as_str(),
        apd.as_str(),
        adi.as_str(),
        aix.as_str(),
        hex.as_str(),
        //lng.as_str(),
        // TODO: turn the rest into regexes as well
        r"^\d+\(PC\)$",
        r"^\d+\(PC,[DA][0-7]\)$",
    ]).unwrap();

    let mode1 = modes.matches(op1).into_iter().nth(0);
    let mode2 = modes.matches(op2).into_iter().nth(0);
    let get_chr = |rx: &Regex, op: &str, at: usize| rx.captures(op).unwrap().at(at).unwrap().chars().next().unwrap();
    let get_num = |rx: &Regex, op: &str, at: usize| rx.captures(op).unwrap().at(at).unwrap().parse().unwrap();
    let get_hex = |rx: &Regex, op: &str, at: usize| rx.captures(op).unwrap().at(at).unwrap().parse().unwrap();
    let to_op = |opinfo| {
        let (v, op) = opinfo;
        match v {
            None => None,
            Some(0) => Some(Operand::DataRegisterDirect(get_num(&drd, op, 1))),
            Some(1) => Some(Operand::AddressRegisterDirect(get_num(&ard, op, 1))),
            Some(2) => Some(Operand::AddressRegisterIndirect(get_num(&ari, op, 1))),
            Some(3) => Some(Operand::AddressRegisterIndirectWithPostincrement(get_num(&api, op, 1))),
            Some(4) => Some(Operand::AddressRegisterIndirectWithPredecrement(get_num(&apd, op, 1))),
            Some(5) => Some(Operand::AddressRegisterIndirectWithDisplacement(get_num(&adi, op, 2), get_num(&adi, op, 1) as i16)),
            Some(6) => {
                let offset = if get_chr(&aix, op, 4) == 'D' {
                    0
                } else {
                    8
                };
                let i_reg = offset + get_num(&aix, op, 4);
                Some(Operand::AddressRegisterIndirectWithIndex(get_num(&aix, op, 2), i_reg, get_num(&aix, op, 1) as i8))
            },
            Some(7) => {
                let hex: u32 = get_hex(&hex, op, 1);
                if hex > 0x7FFF {
                    Some(Operand::AbsoluteLong(hex))
                } else {
                    Some(Operand::AbsoluteWord(hex as u16))
                }
            },
            Some(8) => {
                Some(Operand::AbsoluteLong(get_hex(&hex, op, 1)))
            },
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
        let inst = parse_assembler("ADD.B\t(A1),D2");
        assert_eq!("ADD", inst.mnemonic);
        assert_eq!(Size::Byte, inst.size);
        assert_eq!(Operand::AddressRegisterIndirect(1), inst.operands[0]);
        assert_eq!(Operand::DataRegisterDirect(2), inst.operands[1]);
        let mut mem = &mut MemoryVec { mem: vec![0x00, 0x00, 0x00, 0x00]};
        let pc = 0;
        let new_pc = encode_instruction(&inst, pc, mem);
        assert_eq!(2, new_pc);
        assert_eq!(0xd411, mem.read_word(pc));
    }
    #[test]
    fn encodes_add_8_re() {
        let inst = parse_assembler("ADD.B\tD2,(A1)");
        assert_eq!("ADD", inst.mnemonic);
        assert_eq!(Size::Byte, inst.size);
        assert_eq!(Operand::DataRegisterDirect(2), inst.operands[0]);
        assert_eq!(Operand::AddressRegisterIndirect(1), inst.operands[1]);
        let mut mem = &mut MemoryVec { mem: vec![0x00, 0x00, 0x00, 0x00]};
        let pc = 0;
        let new_pc = encode_instruction(&inst, pc, mem);
        assert_eq!(2, new_pc);
        assert_eq!(0xd511, mem.read_word(pc));
    }

}