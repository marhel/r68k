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
        _ => panic!("not ea-encodable: {:?}", *op)
    }) as u16
}

fn encode_destination_ea(op: &Operand) -> u16 {
    // normally ea are the 6 least significant bits structured as mmmrrr and
    // we need to swap and shift that into place as rrrmmm000000
    let ea = encode_ea(op);
    (ea & 0b11_1000) << 3 | (ea & 0b111) << 9
}

fn encode_dx(op: &Operand) -> u16 {
    match *op {
        Operand::DataRegisterDirect(reg_x) => (reg_x as u16) << 9,
        _ => panic!("not dx-encodable: {:?}", *op)
    }
}
fn encode_pdx(op: &Operand) -> u16 {
    match *op {
        Operand::AddressRegisterIndirectWithPredecrement(reg_x) => (reg_x as u16) << 9,
        _ => panic!("not dx-encodable: {:?}", *op)
    }
}
fn encode_quick(op: &Operand) -> u16 {
    match *op {
        Operand::Immediate(Size::Byte, val) => ((val & 0b111) << 9) as u16,
        _ => panic!("not quick-encodable: {:?}", *op)
    }
}
fn encode_imm4(op: &Operand) -> u16 {
    match *op {
        Operand::Immediate(Size::Byte, val) => (val & 0b1111) as u16,
        _ => panic!("not imm4-encodable: {:?}", *op)
    }
}
fn encode_dy(op: &Operand) -> u16 {
    match *op {
        Operand::DataRegisterDirect(reg_y) => (reg_y & 0b111) as u16,
        _ => panic!("not dy-encodable: {:?}", *op)
    }
}
fn encode_pdy(op: &Operand) -> u16 {
    match *op {
        Operand::AddressRegisterIndirectWithPredecrement(reg_y) => (reg_y & 0b111) as u16,
        _ => panic!("not dy-encodable: {:?}", *op)
    }
}
fn encode_ay(op: &Operand) -> u16 {
    match *op {
        Operand::AddressRegisterDirect(reg_y) => (reg_y & 0b111) as u16,
        _ => panic!("not ay-encodable: {:?}", *op)
    }
}

fn encode_ax(op: &Operand) -> u16 {
    match *op {
        Operand::AddressRegisterDirect(reg_x) => (reg_x as u16) << 9,
        _ => panic!("not ax-encodable: {:?}", *op)
    }
}

fn assert_no_overlap(op: &OpcodeInstance, template: u16, ea: u16, xreg: u16) {
    assert!(template & ea | template & xreg | ea & xreg == 0, "\ntemplate {:016b}\nea       {:16b}\nxreg     {:16b}\noverlaps for {}", template, ea, xreg, op);
}

pub fn encode_ea_dx(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let ea = encode_ea(&op.operands[0]);
    let dx = encode_dx(&op.operands[1]);
    assert_no_overlap(&op, template, ea, dx);
    let pc = mem.write_word(pc, template | ea | dx);
    op.operands[0].add_extension_words(pc, mem)
}

pub fn encode_ea_ax(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let ea = encode_ea(&op.operands[0]);
    let ax = encode_ax(&op.operands[1]);
    assert_no_overlap(&op, template, ea, ax);
    let pc = mem.write_word(pc, template | ea | ax);
    op.operands[0].add_extension_words(pc, mem)
}

pub fn encode_dx_ea(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let ea = encode_ea(&op.operands[1]);
    let dx = encode_dx(&op.operands[0]);
    assert_no_overlap(&op, template, ea, dx);
    let pc = mem.write_word(pc, template | ea | dx);
    op.operands[1].add_extension_words(pc, mem)
}
pub fn encode_dy_imm(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let dy = encode_dy(&op.operands[0]);
    assert_no_overlap(&op, template, 0, dy);
    let pc = mem.write_word(pc, template | dy);
    op.operands[1].add_extension_words(pc, mem)
}
pub fn encode_imm8_dy(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let dy = encode_dy(&op.operands[1]);
    assert_no_overlap(&op, template, 0, dy);
    let pc = mem.write_word(pc, template | dy);
    op.operands[0].add_extension_words(pc, mem)
}
pub fn encode_just_ea(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let ea_index = if let Operand::StatusRegister(_) = &op.operands[0] {
        1
    } else {
        0
    };
    let ea = encode_ea(&op.operands[ea_index]);
    assert_no_overlap(&op, template, ea, 0);
    let pc = mem.write_word(pc, template | ea);
    op.operands[ea_index].add_extension_words(pc, mem)
}

pub fn encode_just_ay(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let ea_index = if let Operand::UserStackPointer = &op.operands[0] {
        1
    } else {
        0
    };
    let ay = encode_ay(&op.operands[ea_index]);
    assert_no_overlap(&op, template, 0, ay);
    mem.write_word(pc, template | ay)
}
pub fn encode_ay_imm16(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let ay = encode_ay(&op.operands[0]);
    assert_no_overlap(&op, template, 0, ay);
    let pc = mem.write_word(pc, template | ay);
    op.operands[1].add_extension_words(pc, mem)
}

pub fn encode_none(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    mem.write_word(pc, template)
}

pub fn encode_ea_ea(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let src_ea = encode_ea(&op.operands[0]);
    let dst_ea = encode_destination_ea(&op.operands[1]);
    assert_no_overlap(&op, template, src_ea, dst_ea & !template);
    let pc = mem.write_word(pc, template | src_ea | dst_ea);
    let pc = op.operands[0].add_extension_words(pc, mem);
    op.operands[1].add_extension_words(pc, mem)
}

pub fn encode_imm_ea(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let ea = encode_ea(&op.operands[1]);
    assert_no_overlap(&op, template, ea, 0);
    let pc = mem.write_word(pc, template | ea);
    let pc = op.operands[0].add_extension_words(pc, mem);
    op.operands[1].add_extension_words(pc, mem)
}
pub fn encode_just_imm4(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let imm = encode_imm4(&op.operands[0]);
    assert_no_overlap(&op, template, 0, imm);
    mem.write_word(pc, template | imm)
}
pub fn encode_just_imm16(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let pc = mem.write_word(pc, template);
    op.operands[0].add_extension_words(pc, mem)
}
pub fn encode_quick_ea(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let quick = encode_quick(&op.operands[0]);
    let ea = encode_ea(&op.operands[1]);
    assert_no_overlap(&op, template, ea, quick);
    let pc = mem.write_word(pc, template | ea | quick);
    op.operands[1].add_extension_words(pc, mem)
}
pub fn encode_quick_dy(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let quick = encode_quick(&op.operands[0]);
    let dy = encode_dy(&op.operands[1]);
    assert_no_overlap(&op, template, quick, dy);
    mem.write_word(pc, template | dy | quick)
}
pub fn encode_just_dy(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let dy = encode_dy(&op.operands[0]);
    assert_no_overlap(&op, template, 0, dy);
    mem.write_word(pc, template | dy)
}
pub fn encode_dx_dy(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let dx = encode_dx(&op.operands[0]);
    let dy = encode_dy(&op.operands[1]);
    assert_no_overlap(&op, template, dx, dy);
    mem.write_word(pc, template | dx | dy)
}
pub fn encode_dx_ay(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let dx = encode_dx(&op.operands[0]);
    let ay = encode_ay(&op.operands[1]);
    assert_no_overlap(&op, template, dx, ay);
    mem.write_word(pc, template | dx | ay)
}
pub fn encode_ax_ay(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let ax = encode_ax(&op.operands[0]);
    let ay = encode_ay(&op.operands[1]);
    assert_no_overlap(&op, template, ax, ay);
    mem.write_word(pc, template | ax | ay)
}
pub fn encode_pdx_pdy(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let pdx = encode_pdx(&op.operands[0]);
    let pdy = encode_pdy(&op.operands[1]);
    assert_no_overlap(&op, template, pdx, pdy);
    mem.write_word(pc, template | pdx | pdy)
}
fn encode_8bit_displacement(operand: &Operand) -> u16 {
    match operand {
        Operand::Displacement(Size::Byte, disp) if *disp > 0 && *disp < 0xff => (*disp as u16) & 0xff,
        Operand::Displacement(Size::Word, _) => 0x00,
        Operand::Displacement(Size::Long, _) => 0xff,
        _ => unreachable!(),
    }
}

pub fn encode_branch(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let disp8 = encode_8bit_displacement(&op.operands[0]);
    assert_no_overlap(&op, template, disp8, 0);
    let pc = mem.write_word(pc, template | disp8);
    op.operands[0].add_extension_words(pc, mem)
}
pub fn encode_moveq(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let data = if let Operand::Displacement(Size::Byte, val) = op.operands[0] {
        val as u8 as u16
    } else {
        unreachable!()
    };
    let dx = encode_dx(&op.operands[1]);
    assert_no_overlap(&op, template, data, dx);
    mem.write_word(pc, template | data | dx)
}
pub fn encode_movem_ea(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let ea = encode_ea(&op.operands[1]);
    assert_no_overlap(&op, template, ea, 0);
    let pc = mem.write_word(pc, template | ea);
    let possibly_reversed = if let Operand::AddressRegisterIndirectWithPredecrement(_) = op.operands[1] {
        if let Operand::Registers(reglist, _) = op.operands[0] {
            Operand::Registers(reglist, true)
        } else {
            op.operands[0]
        }
    } else {
        op.operands[0]
    };
    let pc = possibly_reversed.add_extension_words(pc, mem);
    op.operands[1].add_extension_words(pc, mem)
}
pub fn encode_ea_movem(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    let ea = encode_ea(&op.operands[0]);
    assert_no_overlap(&op, template, ea, 0);
    let pc = mem.write_word(pc, template | ea);
    let pc = op.operands[1].add_extension_words(pc, mem);
    op.operands[0].add_extension_words(pc, mem)
}
#[allow(unused_variables)]
pub fn nop_encoder(op: &OpcodeInstance, template: u16, pc: PC, mem: &mut Memory) -> PC {
    pc
}
#[allow(unused_variables)]
pub fn nop_selector(op: &OpcodeInstance) -> bool {
    false
}
pub fn is_ea_an(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    match op.operands[1] {
        Operand::AddressRegisterDirect(_) => true,
        _ => false,
    }
}
pub fn is_branch(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 1 { return false };
    match op.operands[0] {
        Operand::Displacement(_, _) => true,
        _ => false,
    }
}
pub fn is_dn(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 1 { return false };
    match op.operands[0] {
        Operand::DataRegisterDirect(_) => true,
        _ => false,
    }
}
pub fn is_moveq(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    (match op.operands[0] {
        Operand::Displacement(Size::Byte, _) => true,
        _ => false,
    }) && (match op.operands[1] {
        Operand::DataRegisterDirect(_) => true,
        _ => false,
    })
}
pub fn is_quick_dn(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    (match op.operands[0] {
        Operand::Immediate(_, _) => true,
        _ => false,
    }) && (match op.operands[1] {
        Operand::DataRegisterDirect(_) => true,
        _ => false,
    })
}
pub fn is_dn_imm(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    (match op.operands[0] {
        Operand::DataRegisterDirect(_) => true,
        _ => false,
    }) && (match op.operands[1] {
        Operand::Immediate(_, _) => true,
        _ => false,
    })
}
pub fn is_an_imm16(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    (match op.operands[0] {
        Operand::AddressRegisterDirect(_) => true,
        _ => false,
    }) && (match op.operands[1] {
        Operand::Immediate(Size::Word, _) => true,
        _ => false,
    })
}
pub fn is_imm16(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 1 { return false };
    match op.operands[0] {
        Operand::Immediate(Size::Word, _) => true,
        _ => false,
    }
}
pub fn is_an(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 1 { return false };
    match op.operands[0] {
        Operand::AddressRegisterDirect(_) => true,
        _ => false,
    }
}
pub fn is_imm8_dn(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    (match op.operands[0] {
        Operand::Immediate(Size::Byte, _) => true,
        _ => false,
    }) && (match op.operands[1] {
        Operand::DataRegisterDirect(_) => true,
        _ => false,
    })
}
pub fn is_dn_dn(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    (match op.operands[0] {
        Operand::DataRegisterDirect(_) => true,
        _ => false,
    }) && (match op.operands[1] {
        Operand::DataRegisterDirect(_) => true,
        _ => false,
    })
}
pub fn is_dn_an(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    (match op.operands[0] {
        Operand::DataRegisterDirect(_) => true,
        _ => false,
    }) && (match op.operands[1] {
        Operand::AddressRegisterDirect(_) => true,
        _ => false,
    })
}
pub fn is_an_an(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    (match op.operands[0] {
        Operand::AddressRegisterDirect(_) => true,
        _ => false,
    }) && (match op.operands[1] {
        Operand::AddressRegisterDirect(_) => true,
        _ => false,
    })
}
pub fn is_disp_ea(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    match op.operands[0] {
        Operand::Displacement(_, _) => true,
        _ => false,
    }
}
pub fn is_movem_ea(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    match op.operands[0] {
        Operand::Registers(_, _) => true,
        _ => false,
    }
}
pub fn is_ea_movem(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    match op.operands[1] {
        Operand::Registers(_, _) => true,
        _ => false,
    }
}
pub fn is_ea_dn(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    match op.operands[1] {
        Operand::DataRegisterDirect(_) => true,
        _ => false,
    }
}
pub fn is_dn_ea(op: &OpcodeInstance) -> bool {
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
pub fn is_imm4(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 1 { return false };
    match op.operands[0] {
        Operand::Immediate(Size::Byte, _) => true,
        _ => false,
    }
}
pub fn is_sr_ea(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    match op.operands[0] {
        Operand::StatusRegister(Size::Word) => true,
        _ => false,
    }
}
pub fn is_ea_sr(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    match op.operands[1] {
        Operand::StatusRegister(Size::Word) => true,
        _ => false,
    }
}
pub fn is_usp_an(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    (match op.operands[0] {
        Operand::UserStackPointer => true,
        _ => false,
    }) && (match op.operands[1] {
        Operand::AddressRegisterDirect(_) => true,
        _ => false,
    })
}
pub fn is_an_usp(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    (match op.operands[0] {
        Operand::AddressRegisterDirect(_) => true,
        _ => false,
    }) && (match op.operands[1] {
        Operand::UserStackPointer => true,
        _ => false,
    })
}
pub fn is_pd_pd(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    (match op.operands[0] {
        Operand::AddressRegisterIndirectWithPredecrement(_) => true,
        _ => false,
    }) && (match op.operands[1] {
        Operand::AddressRegisterIndirectWithPredecrement(_) => true,
        _ => false,
    })
}
pub fn is_ea_ccr(op: &OpcodeInstance) -> bool {
    if op.operands.len() != 2 { return false };
    match op.operands[1] {
        Operand::StatusRegister(Size::Byte) => true,
        _ => false,
    }
}
pub fn is_none(op: &OpcodeInstance) -> bool {
    op.operands.len() == 0
}
pub fn is_ea(op: &OpcodeInstance) -> bool {
    op.operands.len() == 1
}
pub fn is_ea_ea(op: &OpcodeInstance) -> bool {
    op.operands.len() == 2
}


pub fn encode_instruction(instruction: &str, op_inst: &OpcodeInstance, pc: PC, mem: &mut Memory) -> PC
{
    let optable = super::generate();
    for op in optable {
        assert!(op.mask & op.matching == op.matching, format!("mask/matching mismatch {:04x} & {:04x} for {}{}", op.mask, op.matching, op.mnemonic, op.size));
        if op_inst.mnemonic == op.mnemonic && op_inst.size == op.size && (op.selector)(op_inst) {
            let encoder = op.encoder;
            return encoder(op_inst, op.matching as u16, pc, mem);
        }
    }
    panic!("Could not assemble {} ({:?})", instruction, op_inst);
}

use std::io;
use std::io::BufRead;
use self::parser::{Rdp, Rule, Directive};
use pest::{StringInput, Parser};
use std::collections::HashSet;
use PC;

pub struct Assembler<'a> {
    branches: HashSet<&'a str>,
    unsizeds: HashSet<&'a str>,
}

impl<'b> Assembler<'b> {
    pub fn new() -> Assembler<'b> {
        let mut unsizeds: HashSet<&str> = HashSet::new();
        unsizeds.insert("RTS");
        unsizeds.insert("RTR");
        unsizeds.insert("RTE");
        unsizeds.insert("JSR");
        unsizeds.insert("JMP");
        unsizeds.insert("TRAP");
        unsizeds.insert("TRAPV");
        unsizeds.insert("UNLK");
        unsizeds.insert("ILLEGAL");
        unsizeds.insert("STOP");
        unsizeds.insert("RESET");

        let mut branches: HashSet<&str> = HashSet::new();
        branches.insert("BHI");
        branches.insert("BLS");
        branches.insert("BCC");
        branches.insert("BCS");
        branches.insert("BNE");
        branches.insert("BEQ");
        branches.insert("BVC");
        branches.insert("BVS");
        branches.insert("BPL");
        branches.insert("BMI");
        branches.insert("BGE");
        branches.insert("BLT");
        branches.insert("BGT");
        branches.insert("BLE");
        branches.insert("BRA");
        branches.insert("BSR");
        branches.insert("MOVEQ");

        Assembler { branches, unsizeds }
    }

    pub fn adjust_size<'a>(&self, op_inst: &OpcodeInstance<'a>) -> OpcodeInstance<'a> {
        let mut clone: OpcodeInstance = (*op_inst).clone();
        clone.size = if op_inst.size == Size::Unsized && !self.unsizeds.contains(op_inst.mnemonic) { Size::Word } else { op_inst.size };
        if self.branches.contains(op_inst.mnemonic) {
            clone.operands = op_inst.operands.iter().map(|&op| match op {
                Operand::Number(Size::Unsized, x) if op_inst.mnemonic == "MOVEQ" => Operand::Displacement(Size::Byte, x),
                Operand::Number(Size::Unsized, x) if op_inst.size == Size::Byte && x > 0 && x < 0xFF => Operand::Displacement(Size::Byte, x),
                Operand::Number(Size::Unsized, x) if x <= 0xFFFF => Operand::Displacement(Size::Word, x),
                Operand::Number(Size::Unsized, x) => Operand::Displacement(Size::Long, x),
                Operand::Number(size, x) => Operand::Displacement(size, x),
                x => x,
            }).collect();
        } else {
            clone.operands = op_inst.operands.iter().map(|&op| match op {
                Operand::Immediate(Size::Unsized, x) if op_inst.mnemonic == "SUBQ" => Operand::Immediate(Size::Byte, x),
                Operand::Immediate(Size::Unsized, x) if op_inst.mnemonic == "ADDQ" => Operand::Immediate(Size::Byte, x),
                Operand::Immediate(Size::Unsized, x) if op_inst.mnemonic == "ROL" => Operand::Immediate(Size::Byte, x),
                Operand::Immediate(Size::Unsized, x) if op_inst.mnemonic == "ROR" => Operand::Immediate(Size::Byte, x),
                Operand::Immediate(Size::Unsized, x) if op_inst.mnemonic == "ROXL" => Operand::Immediate(Size::Byte, x),
                Operand::Immediate(Size::Unsized, x) if op_inst.mnemonic == "ROXR" => Operand::Immediate(Size::Byte, x),
                Operand::Immediate(Size::Unsized, x) if op_inst.mnemonic == "LSL" => Operand::Immediate(Size::Byte, x),
                Operand::Immediate(Size::Unsized, x) if op_inst.mnemonic == "LSR" => Operand::Immediate(Size::Byte, x),
                Operand::Immediate(Size::Unsized, x) if op_inst.mnemonic == "ASL" => Operand::Immediate(Size::Byte, x),
                Operand::Immediate(Size::Unsized, x) if op_inst.mnemonic == "ASR" => Operand::Immediate(Size::Byte, x),
                Operand::Immediate(Size::Unsized, x) if op_inst.mnemonic == "BTST" => Operand::Immediate(Size::Byte, x),
                Operand::Immediate(Size::Unsized, x) if op_inst.mnemonic == "BSET" => Operand::Immediate(Size::Byte, x),
                Operand::Immediate(Size::Unsized, x) if op_inst.mnemonic == "BCHG" => Operand::Immediate(Size::Byte, x),
                Operand::Immediate(Size::Unsized, x) if op_inst.mnemonic == "BCLR" => Operand::Immediate(Size::Byte, x),
                Operand::Immediate(Size::Unsized, x) if op_inst.mnemonic == "TRAP" => Operand::Immediate(Size::Byte, x),
                Operand::Immediate(Size::Unsized, x) if op_inst.mnemonic == "STOP" => Operand::Immediate(Size::Word, x),
                Operand::Immediate(Size::Unsized, x) => Operand::Immediate(clone.size, x),
                Operand::Number(Size::Byte, x) => Operand::AbsoluteWord(x as u8 as u16),
                Operand::Number(Size::Word, x) => Operand::AbsoluteWord(x as u16),
                Operand::Number(Size::Long, x) => Operand::AbsoluteLong(x),
                Operand::Number(Size::Unsized, x) if x <= 0xFF => Operand::AbsoluteWord(x as u16),
                Operand::Number(Size::Unsized, x) if x <= 0xFFFF => Operand::AbsoluteWord(x as u16),
                Operand::Number(Size::Unsized, x) => Operand::AbsoluteLong(x),
                x => x,
            }).collect();
        }
        clone
    }

    pub fn assemble(&self, reader: &mut BufRead) ->  io::Result<(PC, MemoryVec)> {
        let mut mem = MemoryVec::new();
        let mut pc = PC(0);

        for line in reader.lines() {
            let asm = line.unwrap();
            let mut parser = Rdp::new(StringInput::new(&asm));
            assert!(parser.statement());
            assert!(parser.end());
            let queue = parser.queue_with_captures();
            match queue[0].0.rule {
                Rule::a_directive => {
                    match parser.process_directive() {
                        (_label, Directive::Origin(expr)) => {
                            pc = PC(expr.eval().unwrap() as u32);
                        },
                        (_label, directive) => panic!("Doesn't yet handle directive {:?}", directive),
                    }
                },
                Rule::an_instruction => {
                    let unsized_inst = parser.process_instruction();
                    let sized_inst = self.adjust_size(&unsized_inst);
                    pc = encode_instruction(&queue[0].1, &sized_inst, pc, &mut mem);
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
    use OpcodeInstance;
    use PC;

    #[test]
    fn encodes_add_8_er() {
        let asm = " ADD.B\t(A1),D2";
        let a = Assembler::new();
        let inst = a.parse_assembler(asm);
        assert_eq!("ADD", inst.mnemonic);
        assert_eq!(Size::Byte, inst.size);
        assert_eq!(Operand::AddressRegisterIndirect(1), inst.operands[0]);
        assert_eq!(Operand::DataRegisterDirect(2), inst.operands[1]);
        let mem = &mut MemoryVec::new();
        let pc = PC(0);
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
        let mem = &mut MemoryVec::new();
        let pc = PC(0);
        let new_pc = encode_instruction(asm, &inst, pc, mem);
        assert_eq!(2, new_pc);
        assert_eq!(0xd511, mem.read_word(pc));
    }
    #[test]
    fn can_adjust_size() {
        let addi_op = OpcodeInstance {
            mnemonic: "ADDI",
            size: Size::Byte,
            operands: vec![Operand::Immediate(Size::Unsized, 31), Operand::DataRegisterDirect(0)]
        };
        let r68k = Assembler::new();
        let adjusted = r68k.adjust_size(&addi_op);
        assert_eq!(Operand::Immediate(Size::Byte, 31), adjusted.operands[0]);
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
        let _org = 0x1000;
        let mut reader = BufReader::new(asm.as_bytes());
        let (end, mem) = r68k.assemble(&mut reader).unwrap();
        assert_eq!(0x1000 + 6, end);
        assert_eq!(0x1000, mem.offset());
    }
}