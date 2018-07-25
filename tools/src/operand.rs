use std::fmt;
use memory::Memory;
use super::Size;
use PC;

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
    Registers(u16, bool), // reglist, reversed
    UserStackPointer,
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
            Operand::Registers(_, _) => 1,
            Operand::UserStackPointer => 0,
        }
    }

    pub fn add_extension_words(&self, pc: PC, mem: &mut Memory) -> PC {
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
            Operand::Displacement(Size::Byte, _) => pc,
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
            Operand::Registers(reglist, false) => mem.write_word(pc, reglist),
            Operand::Registers(reglist, true) => mem.write_word(pc, bit_reverse(reglist)),
            Operand::UserStackPointer => pc,
        }
    }
}

fn bit_reverse(x: u16) -> u16 {
    let x = (x & 0b1010_1010_1010_1010) >> 1 | (x & 0b0101_0101_0101_0101) << 1;
    let x = (x & 0b1100_1100_1100_1100) >> 2 | (x & 0b0011_0011_0011_0011) << 2;
    let x = (x & 0b1111_0000_1111_0000) >> 4 | (x & 0b0000_1111_0000_1111) << 4;
    (x >> 8 | x << 8)
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn write_registers(f: &mut fmt::Formatter, reglist: u16) -> fmt::Result {
            let mut result: fmt::Result = Ok(());
            let mut reglist = reglist;
            let mut first = true;
            for bit in 0..16u16 {
                if (1 << bit) & reglist != 0 {
                    let mut span = bit;
                    let span = loop {
                        if span < 16 {
                            let spanbit = (1 << span);
                            if spanbit & reglist != 0 {
                                reglist &= !spanbit;
                                span += 1;
                                continue;
                            }
                        }
                        break span - 1;
                    };
                    if !first {
                        result = write!(f, "/");
                    };
                    first = false;
                    if span == bit {
                        if bit > 7 {
                            result = write!(f, "A{}", bit-8);
                        } else {
                            result = write!(f, "D{}", bit);
                        }
                    } else {
                        if bit > 7 {
                            result = write!(f, "A{}-", bit-8);
                        } else {
                            result = write!(f, "D{}-", bit);
                        }
                        if span > 7 {
                            result = write!(f, "A{}", span-8);
                        } else {
                            result = write!(f, "D{}", span);
                        }
                    }
                }
            };
            result
        };
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
            Operand::Registers(reglist, false) => write_registers(f, reglist),
            Operand::Registers(reglist, true) => write_registers(f, bit_reverse(reglist)),
            Operand::UserStackPointer => write!(f, "USP"),
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


#[cfg(test)]
mod tests {
    use operand::bit_reverse;
    extern crate rand;
    use self::rand::Rng;
    use operand::Operand;

    #[test]
    fn simple_bit_reversal() {
        assert_eq!(0b1101_0011_1010_0001, bit_reverse(0b1000_0101_1100_1011));
    }

    #[test]
    fn bit_reversal() {
        let mut r = rand::thread_rng();
        for _ in 0..1000 {
            let v = r.gen();
            assert_eq!(v, bit_reverse(bit_reverse(v)));
        }
    }

    #[test]
    fn single_bit_registers_operand() {
        for bit in 0..16 {
            let expected = if bit < 8 {
                format!("D{}", bit)
            } else {
                format!("A{}", bit-8)
            };
            assert_eq!(expected, format!("{}", Operand::Registers(1 << bit, false)));
        }
    }
    #[test]
    fn separate_registers_operand() {
        assert_eq!("A1/A4", format!("{}", Operand::Registers(0b0001_0010_0000_0000, false)));
        assert_eq!("D5/A2", format!("{}", Operand::Registers(0b0000_0100_0010_0000, false)));
    }
    #[test]
    fn spanning_registers_operand() {
        assert_eq!("A1-A4", format!("{}", Operand::Registers(0b0001_1110_0000_0000, false)));
        assert_eq!("D5-A2", format!("{}", Operand::Registers(0b0000_0111_1110_0000, false)));
    }
    #[test]
    fn combined_registers_operand() {
        assert_eq!("D4/A1-A4", format!("{}", Operand::Registers(0b0001_1110_0001_0000, false)));
        assert_eq!("D5-A2/A5", format!("{}", Operand::Registers(0b0010_0111_1110_0000, false)));
    }
}