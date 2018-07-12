use std::fmt;
use memory::Memory;
use super::Size;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operand {
    DataRegisterDirect(u8),
    AddressRegisterDirect(u8),
    AddressRegisterIndirect(u8),
    AddressRegisterIndirectWithPredecrement(u8),
    AddressRegisterIndirectWithPostincrement(u8),
    AddressRegisterIndirectWithDisplacement(u8, i16),
    AddressRegisterIndirectWithIndex(u8, u8, i8),
    PcWithDisplacement(i16),
    PcWithIndex(u8, i8),
    AbsoluteWord(u16),
    AbsoluteLong(u32),
    Immediate(Size, u32),
    StatusRegister(Size),
    Displacement(Size, u32),
    Number(Size, u32),
}

fn encode_extension_word(xreg_ndx_size: u8, displacement: i8) -> u16 {
    // top four bits = (D/A RRR) matches our register array layout
    (((xreg_ndx_size as u16) << 12) | (displacement as u8 as u16)) as u16
}

impl Operand {
    pub fn extension_words(&self) -> u32 {
        match *self {
            Operand::DataRegisterDirect(_) => 0,
            Operand::AddressRegisterDirect(_) => 0,
            Operand::AddressRegisterIndirect(_) => 0,
            Operand::AddressRegisterIndirectWithPredecrement(_) => 0,
            Operand::AddressRegisterIndirectWithPostincrement(_) => 0,
            Operand::AddressRegisterIndirectWithDisplacement(_, _) => 1,
            Operand::AddressRegisterIndirectWithIndex(_, _, _) => 1,
            Operand::AbsoluteWord(_) => 1,
            Operand::AbsoluteLong(_) => 2,
            Operand::Displacement(Size::Byte, _) => 0,
            Operand::Displacement(Size::Word, _) => 1,
            Operand::Displacement(Size::Long, _) => 2,
            Operand::Displacement(Size::Unsized, _) => panic!("unsized {:?}", self),
            Operand::Number(_, _) => panic!("unsized {:?}", self),
            Operand::PcWithDisplacement(_) => 1,
            Operand::PcWithIndex(_, _) => 1,
            Operand::Immediate(Size::Byte, _) => 1,
            Operand::Immediate(Size::Word, _) => 1,
            Operand::Immediate(Size::Long, _) => 2,
            Operand::Immediate(Size::Unsized, _) => panic!("unsized {:?}", self),
            Operand::StatusRegister(_) => 0,
        }
    }

    pub fn add_extension_words(&self, pc: u32, mem: &mut Memory) -> u32 {
        match *self {
            Operand::DataRegisterDirect(_) => pc,
            Operand::AddressRegisterDirect(_) => pc,
            Operand::AddressRegisterIndirect(_) => pc,
            Operand::AddressRegisterIndirectWithPredecrement(_) => pc,
            Operand::AddressRegisterIndirectWithPostincrement(_) => pc,
            Operand::AddressRegisterIndirectWithDisplacement(_, displacement) =>
                mem.write_word(pc, displacement as u16),
            Operand::AddressRegisterIndirectWithIndex(_, indexinfo, displacement) =>
                mem.write_word(pc, encode_extension_word(indexinfo, displacement)),
            Operand::AbsoluteWord(val) => mem.write_word(pc, val as u16),
            Operand::AbsoluteLong(val) => {
                mem.write_word(pc, (val >> 16) as u16);
                mem.write_word(pc + 2, val as u16)
            },
            Operand::Displacement(Size::Byte, val) => pc,
            Operand::Displacement(Size::Word, val) => mem.write_word(pc, val as u16),
            Operand::Displacement(Size::Long, val) => {
                mem.write_word(pc, (val >> 16) as u16);
                mem.write_word(pc + 2, val as u16)
            },
            Operand::Displacement(Size::Unsized, _) => panic!("unsized {:?}", self),
            Operand::Number(_, _) => panic!("unsized {:?}", self),
            Operand::PcWithDisplacement(displacement) => mem.write_word(pc, displacement as u16),
            Operand::PcWithIndex(indexinfo, displacement) => mem.write_word(pc, encode_extension_word(indexinfo, displacement)),
            Operand::Immediate(Size::Byte, val) => mem.write_word(pc, (val & 0xff) as u16),
            Operand::Immediate(Size::Word, val) => mem.write_word(pc, val as u16),
            Operand::Immediate(Size::Long, val) => {
                mem.write_word(pc, (val >> 16) as u16);
                mem.write_word(pc + 2, val as u16)
            }
            Operand::Immediate(Size::Unsized, _) => panic!("unsized {:?}", self),
            Operand::StatusRegister(_) => pc,
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::DataRegisterDirect(reg) => write!(f, "D{}", reg),
            Operand::AddressRegisterDirect(reg) => write!(f, "A{}", reg),
            Operand::AddressRegisterIndirect(reg) => write!(f, "(A{})", reg),
            Operand::AddressRegisterIndirectWithPredecrement(reg) => write!(f, "-(A{})", reg),
            Operand::AddressRegisterIndirectWithPostincrement(reg) => write!(f, "(A{})+", reg),
            Operand::AddressRegisterIndirectWithDisplacement(reg, dis) => write!(f, "{}(A{})", dis, reg),
            Operand::AddressRegisterIndirectWithIndex(reg, ireg, dis) => write!(f, "{}(A{},{})", dis, reg, xreg(ireg)),
            Operand::PcWithDisplacement(dis) => write!(f, "{}(PC)", dis),
            Operand::PcWithIndex(ireg, dis) => write!(f, "{}(PC,{})", dis, xreg(ireg)),
            Operand::AbsoluteWord(val) => write!(f, "${:04X}", val),
            Operand::AbsoluteLong(val) => write!(f, "${:08X}", val),
            Operand::Number(Size::Byte, val) => write!(f, "${:02X}", val),
            Operand::Number(Size::Word, val) => write!(f, "${:04X}", val),
            Operand::Number(Size::Long, val) => write!(f, "${:08X}", val),
            Operand::Number(Size::Unsized, val) => write!(f, "${:08X}.?", val),
            Operand::Displacement(Size::Byte, val) => write!(f, "${:02X}", val),
            Operand::Displacement(Size::Word, val) => write!(f, "${:04X}", val),
            Operand::Displacement(Size::Long, val) => write!(f, "${:08X}", val),
            Operand::Displacement(Size::Unsized, val) => write!(f, "${:08X}.?", val),
            Operand::Immediate(Size::Byte, val) => write!(f, "#${:02X}", val),
            Operand::Immediate(Size::Word, val) => write!(f, "#${:04X}", val),
            Operand::Immediate(Size::Long, val) => write!(f, "#${:08X}", val),
            Operand::Immediate(Size::Unsized, val) => write!(f, "#${:08X}.?", val),
            Operand::StatusRegister(Size::Byte) => write!(f, "CCR"),
            Operand::StatusRegister(_) => write!(f, "SR"),
         }
    }
}

fn xreg(xreg: u8) -> Operand {
    if xreg & 8 > 0 {
        Operand::AddressRegisterDirect(xreg & 7)
    } else {
        Operand::DataRegisterDirect(xreg & 7)
    }
}

