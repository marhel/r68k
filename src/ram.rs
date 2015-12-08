use std::cell::RefCell;
use std::collections::HashMap;
const PAGE_MASK: u32 = 0b1111_1111_1111_1100_0000_0000; // 16K pages
const ADDR_MASK: u32 = 0b0000_0000_0000_0011_1111_1111; // 1K page size
type Page = [u8; (ADDR_MASK+1) as usize];

pub struct LoggingMem {
	pub log: RefCell<Vec<Operation>>,
	pages: HashMap<u32, Page>,
	initializer: u32,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct AddressSpace(Mode, Segment);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Segment {
	Program, Data
}
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Mode {
	User, Supervisor
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Operation {
	None,
	ReadByte(AddressSpace, u32),
	ReadWord(AddressSpace, u32),
	ReadLong(AddressSpace, u32),
	WriteByte(AddressSpace, u32, u32),
	WriteWord(AddressSpace, u32, u32),
	WriteLong(AddressSpace, u32, u32),
}
pub const SUPERVISOR_PROGRAM: AddressSpace = AddressSpace(Mode::Supervisor, Segment::Program);
pub const SUPERVISOR_DATA: AddressSpace = AddressSpace(Mode::Supervisor, Segment::Data);
pub const USER_PROGRAM: AddressSpace = AddressSpace(Mode::User, Segment::Program);
pub const USER_DATA: AddressSpace = AddressSpace(Mode::User, Segment::Data);

trait AddressBus {
	fn read_u8(&self, address_space: AddressSpace, address: u32) -> u32;
	fn read_u16(&self, address_space: AddressSpace, address: u32) -> u32;
	fn read_u32(&self, address_space: AddressSpace, address: u32) -> u32;
	fn write_u8(&mut self, address_space: AddressSpace, address: u32, value: u32);
	fn write_u16(&mut self, address_space: AddressSpace, address: u32, value: u32);
	fn write_u32(&mut self, address_space: AddressSpace, address: u32, value: u32);
}

impl LoggingMem {
	fn new(initializer: u32) -> LoggingMem {
		let mut lmem = LoggingMem { log: RefCell::new(Vec::new()), pages: HashMap::new(), initializer: initializer };
		lmem
	}
	fn loglen(&self) -> usize {
		let log = self.log.borrow();
		log.len()
	}
	fn getlog(&self, index: usize) -> Operation {
		let log = self.log.borrow();
		log[index]
	}
	fn ensure_page(&self, address: u32) -> u32 {
		let page = address & PAGE_MASK;
		if !self.pages.contains_key(&page) {
			self.pages.insert(page, [0u8; (ADDR_MASK + 1) as usize]);
			let mut page = self.pages[&page];
			for v in 0..((ADDR_MASK+1) / 4) {
				page[(4*v+0) as usize] = ((self.initializer >> 24) & 0xFF) as u8;
				page[(4*v+1) as usize] = ((self.initializer >> 16) & 0xFF) as u8;
				page[(4*v+2) as usize] = ((self.initializer >>  8) & 0xFF) as u8;
				page[(4*v+3) as usize] = ((self.initializer >>  0) & 0xFF) as u8;
			}
		}
		page
	}
	fn read_byte(&self, address: u32) -> u32 {
		let index = (address & ADDR_MASK) as usize;
		//self.pages[&self.ensure_page(address)][index] as u32
		0
	}

	fn write_byte(&mut self, address: u32, value: u8) {
		let index = (address & ADDR_MASK) as usize;
		let page = self.ensure_page(address);
		let mut page = self.pages[&page];
		page[index] = value;
	}
}

impl AddressBus for LoggingMem {
	fn read_u8(&self, address_space: AddressSpace, address: u32) -> u32 {
		let mut log = self.log.borrow_mut();
		log.push(Operation::ReadByte(address_space, address));
		self.read_byte(address)
	}

	fn read_u16(&self, address_space: AddressSpace, address: u32) -> u32 {
		let mut log = self.log.borrow_mut();
		log.push(Operation::ReadWord(address_space, address));
		((self.read_byte(address+0) as u16) << 8
		|(self.read_byte(address+1) as u16) << 0
		) as u32
	}

	fn read_u32(&self, address_space: AddressSpace, address: u32) -> u32 {
		let mut log = self.log.borrow_mut();
		log.push(Operation::ReadLong(address_space, address));
		((self.read_byte(address+0) as u32) << 24
		|(self.read_byte(address+1) as u32) << 16
		|(self.read_byte(address+2) as u32) <<  8
		|(self.read_byte(address+3) as u32) <<  0) as u32
	}

	fn write_u8(&mut self, address_space: AddressSpace, address: u32, value: u32) {
		{
			let mut log = self.log.borrow_mut();
			log.push(Operation::WriteByte(address_space, address, value));
		}
		self.write_byte(address, (value & 0xFF) as u8);
	}

	fn write_u16(&mut self, address_space: AddressSpace, address: u32, value: u32) {
		{
			let mut log = self.log.borrow_mut();
			log.push(Operation::WriteWord(address_space, address, value));
		}
		self.write_byte(address+0, ((value >>  8) & 0xFF) as u8);
		self.write_byte(address+1, ((value >>  0) & 0xFF) as u8);
	}

	fn write_u32(&mut self, address_space: AddressSpace, address: u32, value: u32) {
		{
			let mut log = self.log.borrow_mut();
			log.push(Operation::WriteLong(address_space, address, value));
		}
		self.write_byte(address+0, ((value >> 24) & 0xFF) as u8);
		self.write_byte(address+1, ((value >> 16) & 0xFF) as u8);
		self.write_byte(address+2, ((value >>  8) & 0xFF) as u8);
		self.write_byte(address+3, ((value >>  0) & 0xFF) as u8);
	}
}

#[cfg(test)]
mod tests {
	use super::{LoggingMem, AddressBus, Operation, SUPERVISOR_DATA, SUPERVISOR_PROGRAM, USER_DATA, USER_PROGRAM};

	#[test]
	fn read_initialized_memory() {
		let mem = LoggingMem::new(0x01020304);
		for v in 0..256 {
			assert_eq!(0x01, mem.read_u8(SUPERVISOR_DATA, 4*v+0));
			assert_eq!(0x02, mem.read_u8(SUPERVISOR_DATA, 4*v+1));
			assert_eq!(0x03, mem.read_u8(SUPERVISOR_DATA, 4*v+2));
			assert_eq!(0x04, mem.read_u8(SUPERVISOR_DATA, 4*v+3));
		}
		for v in 0..256 {
			assert_eq!(0x0102, mem.read_u16(SUPERVISOR_DATA, 4*v+0));
			assert_eq!(0x0203, mem.read_u16(SUPERVISOR_DATA, 4*v+1));
			assert_eq!(0x0304, mem.read_u16(SUPERVISOR_DATA, 4*v+2));
			if 4*v+3 < 1023 {
				assert_eq!(0x0401, mem.read_u16(SUPERVISOR_DATA, 4*v+3));
			}
		}
		for v in 0..255 {
			assert_eq!(0x01020304, mem.read_u32(SUPERVISOR_DATA, 4*v+0));
			assert_eq!(0x02030401, mem.read_u32(SUPERVISOR_DATA, 4*v+1));
			assert_eq!(0x03040102, mem.read_u32(SUPERVISOR_DATA, 4*v+2));
			assert_eq!(0x04010203, mem.read_u32(SUPERVISOR_DATA, 4*v+3));
		}
		assert_eq!(0x01020304, mem.read_u32(SUPERVISOR_DATA, 4*255));
	}

	#[test]
	fn read_your_u32_writes() {
		let mut mem = LoggingMem::new(0x01020304);
		let pattern = 0xAAAA7777;
		let address = 128;
		assert!(pattern != mem.read_u32(SUPERVISOR_DATA, address));
		mem.write_u32(SUPERVISOR_DATA, address, pattern);
		assert_eq!(pattern, mem.read_u32(SUPERVISOR_DATA, address));
	}

	#[test]
	fn read_your_u16_writes() {
		let mut mem = LoggingMem::new(0x01020304);
		let pattern = 0xAAAA7777;
		let address = 128;
		assert!(pattern != mem.read_u16(SUPERVISOR_DATA, address));
		mem.write_u16(SUPERVISOR_DATA, address, pattern);
		assert_eq!(pattern & 0xFFFF, mem.read_u16(SUPERVISOR_DATA, address));
	}

	#[test]
	fn read_your_u8_writes() {
		let mut mem = LoggingMem::new(0x01020304);
		let pattern = 0xAAAA7777;
		let address = 128;
		assert!(pattern != mem.read_u8(SUPERVISOR_DATA, address));
		mem.write_u8(SUPERVISOR_DATA, address, pattern);
		assert_eq!(pattern & 0xFF, mem.read_u8(SUPERVISOR_DATA, address));
	}

	#[test]
	fn read_u8_is_logged() {
		let mem = LoggingMem::new(0x01020304);
		let address = 128;
		mem.read_u8(SUPERVISOR_DATA, address);
		assert!(mem.loglen() > 0);
		assert_eq!(Operation::ReadByte(SUPERVISOR_DATA, address), mem.getlog(0));
	}

	#[test]
	fn read_u16_is_logged() {
		let mem = LoggingMem::new(0x01020304);
		let address = 128;
		mem.read_u16(SUPERVISOR_PROGRAM, address);
		assert!(mem.loglen() > 0);
		assert_eq!(Operation::ReadWord(SUPERVISOR_PROGRAM, address), mem.getlog(0));
	}

	#[test]
	fn read_u32_is_logged() {
		let mem = LoggingMem::new(0x01020304);
		let address = 128;
		mem.read_u32(USER_DATA, address);
		assert!(mem.loglen() > 0);
		assert_eq!(Operation::ReadLong(USER_DATA, address), mem.getlog(0));
	}

	#[test]
	fn write_u8_is_logged() {
		let mut mem = LoggingMem::new(0x01020304);
		let address = 128;
		let pattern = 0xAAAA7777;
		mem.write_u8(SUPERVISOR_DATA, address, pattern);
		assert!(mem.loglen() > 0);
		assert_eq!(Operation::WriteByte(SUPERVISOR_DATA, address, pattern), mem.getlog(0));
	}

	#[test]
	fn write_u16_is_logged() {
		let mut mem = LoggingMem::new(0x01020304);
		let address = 128;
		let pattern = 0xAAAA7777;
		mem.write_u16(SUPERVISOR_PROGRAM, address, pattern);
		assert!(mem.loglen() > 0);
		assert_eq!(Operation::WriteWord(SUPERVISOR_PROGRAM, address, pattern), mem.getlog(0));
	}

	#[test]
	fn write_u32_is_logged() {
		let mut mem = LoggingMem::new(0x01020304);
		let address = 128;
		let pattern = 0xAAAA7777;
		mem.write_u32(USER_DATA, address, pattern);
		assert!(mem.loglen() > 0);
		assert_eq!(Operation::WriteLong(USER_DATA, address, pattern), mem.getlog(0));
	}

	#[test]
	fn shared_address_space() {
		let mut mem = LoggingMem::new(0x01020304);
		let pattern = 0xAAAA7777;
		let address = 128;
		assert!(pattern != mem.read_u32(SUPERVISOR_DATA, address));
		assert!(pattern != mem.read_u32(SUPERVISOR_PROGRAM, address));
		assert!(pattern != mem.read_u32(USER_DATA, address));
		assert!(pattern != mem.read_u32(USER_PROGRAM, address));
		mem.write_u32(SUPERVISOR_DATA, address, pattern);

		assert_eq!(pattern, mem.read_u32(SUPERVISOR_DATA, address));
		assert_eq!(pattern, mem.read_u32(SUPERVISOR_PROGRAM, address));
		assert_eq!(pattern, mem.read_u32(USER_DATA, address));
		assert_eq!(pattern, mem.read_u32(USER_PROGRAM, address));
	}
}