use std::fmt;

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

impl Operand {
    pub fn size(&self) -> usize {
        match *self {
            Operand::AddressRegisterIndirectWithDisplacement(_, _) => 2,
            _ => 1
        }
    }
    pub fn extension_word(&self) -> u16 {
        match *self {
            Operand::AddressRegisterIndirectWithDisplacement(_, displacement) => displacement as u16,
            _ => panic!("Unknown extension word for {:?}", *self)
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
            Operand::AbsoluteWord(word) => write!(f, "${:4x}", word),
            Operand::AbsoluteLong(long) => write!(f, "${:8x}", long),
            Operand::Immediate(imm) => write!(f, "#${:8x}", imm),
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

