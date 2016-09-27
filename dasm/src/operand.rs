use std::fmt;
use memory::Memory;

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
    Immediate(u16),
}

fn encode_extension_word(xreg_ndx_size: u8, displacement: i8) -> u16 {
    // top four bits = (D/A RRR) matches our register array layout
    (((xreg_ndx_size as u16) << 11) | (displacement as u8 as u16)) as u16
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
            Operand::PcWithDisplacement(_) => 1,
            Operand::PcWithIndex(_, _) => 1,
            Operand::Immediate(_) => 1,
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
            Operand::AbsoluteWord(wrd) => mem.write_word(pc, wrd),
            Operand::AbsoluteLong(lng) => {
                mem.write_word(pc, (lng >> 16) as u16);
                mem.write_word(pc + 2, lng as u16)
            },
            Operand::PcWithDisplacement(displacement) => mem.write_word(pc, displacement as u16),
            Operand::PcWithIndex(indexinfo, displacement) => mem.write_word(pc, encode_extension_word(indexinfo, displacement)),
            Operand::Immediate(imm) => mem.write_word(pc, imm),
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
            Operand::AbsoluteWord(word) => write!(f, "${:04X}", word),
            Operand::AbsoluteLong(long) => write!(f, "${:08X}.L", long),
            Operand::Immediate(imm) => write!(f, "#${:04X}", imm),
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

