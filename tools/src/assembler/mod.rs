use operand::Operand;
use memory::{Memory, MemoryVec};
use super::{OpcodeInstance, Size};
pub mod parser;

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

fn assert_no_overlap(op: &OpcodeInstance, template: u16, ea: u16, xreg: u16) {
    assert!(template & ea | template & xreg | ea & xreg == 0, "template {:016b}, ea {:06b}, xreg {:012b} overlaps for {}", template, ea, xreg, op);
}

pub fn encode_ea_dx(op: &OpcodeInstance, template: u16, pc: u32, mem: &mut Memory) -> u32 {
    let ea = encode_ea(&op.operands[0]);
    let dx = encode_dx(&op.operands[1]);
    assert_no_overlap(&op, template, ea, dx);
    mem.write_word(pc, template | ea | dx);
    op.operands[0].add_extension_words(pc + 2, mem)
}

pub fn encode_ea_ax(op: &OpcodeInstance, template: u16, pc: u32, mem: &mut Memory) -> u32 {
    let ea = encode_ea(&op.operands[0]);
    let ax = encode_ax(&op.operands[1]);
    assert_no_overlap(&op, template, ea, ax);
    mem.write_word(pc, template | ea | ax);
    op.operands[0].add_extension_words(pc + 2, mem)
}

pub fn encode_dx_ea(op: &OpcodeInstance, template: u16, pc: u32, mem: &mut Memory) -> u32 {
    let ea = encode_ea(&op.operands[1]);
    let dx = encode_dx(&op.operands[0]);
    assert_no_overlap(&op, template, ea, dx);
    mem.write_word(pc, template | ea | dx);
    op.operands[1].add_extension_words(pc + 2, mem)
}

pub fn encode_imm_ea(op: &OpcodeInstance, template: u16, pc: u32, mem: &mut Memory) -> u32 {
    let ea = encode_ea(&op.operands[1]);
    assert_no_overlap(&op, template, ea, 0);
    let pc = mem.write_word(pc, template | ea);
    let pc = op.operands[0].add_extension_words(pc, mem);
    op.operands[1].add_extension_words(pc, mem)
}
#[allow(unused_variables)]
pub fn nop_encoder(op: &OpcodeInstance, template: u16, pc: u32, mem: &mut Memory) -> u32 {
    pc
}
#[allow(unused_variables)]
pub fn nop_selector(op: &OpcodeInstance) -> bool {
    false
}
pub fn is_ea_ax(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    match op.operands[1] {
        Operand::AddressRegisterDirect(_) => true,
        _ => false,
    }
}
pub fn is_ea_dx(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    match op.operands[1] {
        Operand::DataRegisterDirect(_) => true,
        _ => false,
    }
}
pub fn is_dx_ea(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    match op.operands[1] {
        Operand::DataRegisterDirect(_) => false,
        _ => true,
    }
}
pub fn is_imm_ea(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    match op.operands[0] {
        Operand::Immediate(_, _) => true,
        _ => false,
    }
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

use std::io;
use std::io::BufRead;
use self::parser::{Rdp, Rule, Directive};
use pest::{StringInput, Parser};
pub struct Assembler;

impl Assembler {
    pub fn new() -> Assembler {
        Assembler
    }

    pub fn assemble(&self, reader: &mut BufRead) ->  io::Result<(u32, MemoryVec)> {
        let mut mem = MemoryVec::new();
        let mut pc = 0;

        for line in reader.lines() {
            let asm = line.unwrap();
            let mut parser = Rdp::new(StringInput::new(&asm));
            assert!(parser.statement());
            assert!(parser.end());
            let queue = parser.queue_with_captures();
            match queue[0].0.rule {
                Rule::a_directive => {
                    match parser.process_directive() {
                        (label, Directive::Origin(expr)) => {
                            pc = expr.eval().unwrap() as u32;
                        },
                        (label, directive) => panic!("Doesn't yet handle directive {:?}", directive),
                    }
                },
                Rule::an_instruction => {
                    let op = parser.process_instruction();
                    pc = encode_instruction(&queue[0].1, &op, pc, &mut mem);
                },
                Rule::asm_comment => continue,
                other_rule => panic!("Does not yet handle {:?}", other_rule),
            }
        }
        Ok((pc, mem))
    }

    pub fn parse_assembler<'a>(&'a self, instruction: &'a str) -> OpcodeInstance {
        let mut parser = Rdp::new(StringInput::new(instruction));
        assert!(parser.statement());
        assert!(parser.end());
        parser.process_instruction()
    }
}

#[cfg(test)]
mod tests {
    use operand::Operand;
    use memory::{MemoryVec, Memory};
    use super::{Assembler, encode_instruction};
    use super::super::Size;
    use std::io::BufReader;

    #[test]
    fn encodes_add_8_er() {
        let asm = " ADD.B\t(A1),D2";
        let a = Assembler::new();
        let inst = a.parse_assembler(asm);
        assert_eq!("ADD", inst.mnemonic);
        assert_eq!(Size::Byte, inst.size);
        assert_eq!(Operand::AddressRegisterIndirect(1), inst.operands[0]);
        assert_eq!(Operand::DataRegisterDirect(2), inst.operands[1]);
        let mut mem = &mut MemoryVec::new();
        let pc = 0;
        let new_pc = encode_instruction(asm, &inst, pc, mem);
        assert_eq!(2, new_pc);
        assert_eq!(0xd411, mem.read_word(pc));
    }
    #[test]
    fn encodes_add_8_re() {
        let asm = " ADD.B\tD2,(A1)";
        let a = Assembler::new();
        let inst = a.parse_assembler(asm);
        assert_eq!("ADD", inst.mnemonic);
        assert_eq!(Size::Byte, inst.size);
        assert_eq!(Operand::DataRegisterDirect(2), inst.operands[0]);
        assert_eq!(Operand::AddressRegisterIndirect(1), inst.operands[1]);
        let mut mem = &mut MemoryVec::new();
        let pc = 0;
        let new_pc = encode_instruction(asm, &inst, pc, mem);
        assert_eq!(2, new_pc);
        assert_eq!(0xd511, mem.read_word(pc));
    }

    #[test]
    fn can_assemble() {
        let r68k = Assembler::new();

        let asm = r#"
        ADD.B   #$3,D0
        ADD.B   D0,D1"#;

        println!("{}", asm);
        let mut reader = BufReader::new(asm.as_bytes());
        let (last_pc, mem) = r68k.assemble(&mut reader).unwrap();
        assert_eq!(6, last_pc);
        assert_eq!(0, mem.offset());
    }

    #[test]
    fn supports_org_directive() {
        let r68k = Assembler::new();

        let asm = r#"
        ; let's start off with a comment, and then set PC to $1000
    ORG $1000

    ADD.B   #$3,D0
    ADD.B   D0,D1"#;

        println!("{}", asm);
        let org = 0x1000;
        let mut reader = BufReader::new(asm.as_bytes());
        let (end, mem) = r68k.assemble(&mut reader).unwrap();
        assert_eq!(0x1000 + 6, end);
        assert_eq!(0x1000, mem.offset());
    }
}