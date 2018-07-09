use std::cell::RefCell;
use super::{AddressSpace, AddressBus, ADDRBUS_MASK};
use ram::pagedmem::{PagedMem, DiffIter};

#[derive(Copy, Clone, PartialEq)]
pub enum Operation {
    None,
    ReadByte(AddressSpace, u32, u8),
    ReadWord(AddressSpace, u32, u16),
    ReadLong(AddressSpace, u32, u32),
    WriteByte(AddressSpace, u32, u32),
    WriteWord(AddressSpace, u32, u32),
    WriteLong(AddressSpace, u32, u32),
}
use std::fmt;
impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operation::None => write!(f, "None"),
            Operation::ReadByte(aspace, addr, byte) => write!(f, "ReadByte{:?} @{:06x} => {:02x}", aspace, addr, byte),
            Operation::ReadWord(aspace, addr, word) => write!(f, "ReadWord{:?} @{:06x} => {:04x}", aspace, addr, word),
            Operation::ReadLong(aspace, addr, long) => write!(f, "ReadLong{:?} @{:06x} => {:08x}", aspace, addr, long),
            Operation::WriteByte(aspace, addr, byte) => write!(f, "WriteByte{:?} @{:06x} <= {:02x}", aspace, addr, byte),
            Operation::WriteWord(aspace, addr, word) => write!(f, "WriteWord{:?} @{:06x} <= {:04x}", aspace, addr, word),
            Operation::WriteLong(aspace, addr, long) => write!(f, "WriteLong{:?} @{:06x} <= {:08x}", aspace, addr, long),
        }
    }
}

pub trait OpsLogging {
    fn log(&self, op: Operation);
}

#[derive(Default)]
pub struct OpsLogger {
    log: RefCell<Vec<Operation>>,
}

impl OpsLogger {
    pub fn new() -> OpsLogger {
        OpsLogger { log: RefCell::new(Vec::new()) }
    }
    pub fn ops(&self) -> Vec<Operation>
    {
        self.log.borrow_mut().clone()
    }
    pub fn len(&self) -> usize {
        self.log.borrow_mut().len()
    }
    pub fn is_empty(&self) -> bool {
        self.log.borrow_mut().is_empty()
    }
}
impl OpsLogging for OpsLogger {
    fn log(&self, op: Operation) {
        self.log.borrow_mut().push(op);
    }
}

pub struct LoggingMem<T: OpsLogging> {
    pub logger: T,
    mem: PagedMem,
    pub initializer: u32,
}


impl<T: OpsLogging> LoggingMem<T> {
    pub fn new(initializer: u32, logger: T) -> LoggingMem<T> {
        LoggingMem { logger, mem: PagedMem::new(initializer), initializer }
    }
    pub fn read_u8(&self, address: u32) -> u32 {
        self.mem.read_u8(address)
    }

    pub fn write_u8(&mut self, address: u32, value: u32) {
        self.mem.write_u8(address, value)
    }
    pub fn diffs(&self) -> DiffIter {
        self.mem.diffs()
    }
}

impl<T: OpsLogging> AddressBus for LoggingMem<T> {
    fn copy_from(&mut self, other: &Self) {
        for (addr, byte) in other.diffs() {
            self.write_u8(addr, u32::from(byte));
        }
    }

    fn read_byte(&self, address_space: AddressSpace, address: u32) -> u32 {
        let value = self.read_u8(address);
        self.logger.log(Operation::ReadByte(address_space, address & ADDRBUS_MASK, value as u8));
        value
    }

    fn read_word(&self, address_space: AddressSpace, address: u32) -> u32 {
        let value = (self.read_u8(address) << 8
                    |self.read_u8(address.wrapping_add(1))) as u32;
        self.logger.log(Operation::ReadWord(address_space, address & ADDRBUS_MASK, value as u16));
        value
    }

    fn read_long(&self, address_space: AddressSpace, address: u32) -> u32 {
        let value = (self.read_u8(address) << 24
                    |self.read_u8(address.wrapping_add(1)) << 16
                    |self.read_u8(address.wrapping_add(2)) <<  8
                    |self.read_u8(address.wrapping_add(3))) as u32;
        self.logger.log(Operation::ReadLong(address_space, address & ADDRBUS_MASK, value));
        value
    }

    fn write_byte(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        self.logger.log(Operation::WriteByte(address_space, address & ADDRBUS_MASK, value));
        self.write_u8(address, value);
    }

    fn write_word(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        self.logger.log(Operation::WriteWord(address_space, address & ADDRBUS_MASK, value));
        self.write_u8(address, value >> 8);
        self.write_u8(address.wrapping_add(1), value);
    }

    fn write_long(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        self.logger.log(Operation::WriteLong(address_space, address & ADDRBUS_MASK, value));
        self.write_u8(address, value >> 24);
        self.write_u8(address.wrapping_add(1), value >> 16);
        self.write_u8(address.wrapping_add(2), value >>  8);
        self.write_u8(address.wrapping_add(3), value);
    }
}

#[cfg(test)]
mod tests {
    use super::{LoggingMem, AddressBus, OpsLogger, Operation};
    use ram::{SUPERVISOR_DATA, SUPERVISOR_PROGRAM, USER_DATA, USER_PROGRAM, ADDRBUS_MASK};

    #[test]
    fn read_byte_is_logged() {
        do_read_byte_is_logged(0x80);
        // but the following read should be from address 0x180
        // i.e. limited by the 24-bit address bus width
        do_read_byte_is_logged(0xFF000180);
    }
    #[test]
    fn read_word_is_logged() {
        do_read_word_is_logged(0x80);
        // but the following read should be from address 0x180
        // i.e. limited by the 24-bit address bus width
        do_read_word_is_logged(0xFF000180);
    }

    #[test]
    fn read_long_is_logged() {
        do_read_long_is_logged(0x80);
        // but the following read should be from address 0x180
        // i.e. limited by the 24-bit address bus width
        do_read_long_is_logged(0xFF000180);
    }

    #[test]
    fn write_byte_is_logged() {
        do_write_byte_is_logged(0x80);
        // but the following write should be from address 0x180
        // i.e. limited by the 24-bit address bus width
        do_write_byte_is_logged(0xFF000180);
    }

    #[test]
    fn write_word_is_logged() {
        do_write_word_is_logged(0x80);
        // but the following write should be from address 0x180
        // i.e. limited by the 24-bit address bus width
        do_write_word_is_logged(0xFF000180);
    }

    #[test]
    fn write_long_is_logged() {
        do_write_long_is_logged(0x80);
        // but the following write should be from address 0x180
        // i.e. limited by the 24-bit address bus width
        do_write_long_is_logged(0xFF000180);
    }

    fn do_read_byte_is_logged(address: u32) {
        let mem = LoggingMem::new(0x01020304, OpsLogger::new());
        mem.read_byte(SUPERVISOR_DATA, address);
        assert!(mem.logger.len() > 0);
        assert_eq!(Operation::ReadByte(SUPERVISOR_DATA, address & ADDRBUS_MASK, 0x01), mem.logger.ops()[0]);
    }

    fn do_read_word_is_logged(address: u32) {
        let mem = LoggingMem::new(0x01020304, OpsLogger::new());
        mem.read_word(SUPERVISOR_PROGRAM, address);
        assert!(mem.logger.len() > 0);
        assert_eq!(Operation::ReadWord(SUPERVISOR_PROGRAM, address & ADDRBUS_MASK, 0x0102), mem.logger.ops()[0]);
    }

    fn do_read_long_is_logged(address: u32) {
        let mem = LoggingMem::new(0x01020304, OpsLogger::new());
        mem.read_long(USER_DATA, address);
        assert!(mem.logger.len() > 0);
        assert_eq!(Operation::ReadLong(USER_DATA, address & ADDRBUS_MASK, 0x01020304), mem.logger.ops()[0]);
    }

    fn do_write_byte_is_logged(address: u32) {
        let mut mem = LoggingMem::new(0x01020304, OpsLogger::new());
        let pattern = 0xAAAA7777;
        mem.write_byte(USER_PROGRAM, address, pattern);
        assert!(mem.logger.len() > 0);
        assert_eq!(Operation::WriteByte(USER_PROGRAM, address & ADDRBUS_MASK, pattern), mem.logger.ops()[0]);
    }

    fn do_write_word_is_logged(address: u32) {
        let mut mem = LoggingMem::new(0x01020304, OpsLogger::new());
        let pattern = 0xAAAA7777;
        mem.write_word(SUPERVISOR_PROGRAM, address, pattern);
        assert!(mem.logger.len() > 0);
        assert_eq!(Operation::WriteWord(SUPERVISOR_PROGRAM, address & ADDRBUS_MASK, pattern), mem.logger.ops()[0]);
    }

    fn do_write_long_is_logged(address: u32) {
        let mut mem = LoggingMem::new(0x01020304, OpsLogger::new());
        let pattern = 0xAAAA7777;
        mem.write_long(USER_DATA, address, pattern);
        assert!(mem.logger.len() > 0);
        assert_eq!(Operation::WriteLong(USER_DATA, address & ADDRBUS_MASK, pattern), mem.logger.ops()[0]);
    }
}