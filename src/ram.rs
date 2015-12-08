use std::cell::RefCell;
use std::collections::HashMap;
// The m68k had a 24 bit external address bus with
// (2^24 bytes = ) 16 MB addressable space
const PAGE_SIZE: u32 = 1024; // 1K page size
const ADDR_MASK: u32 = PAGE_SIZE - 1; // 1K page size
const PAGE_MASK: u32 = 0xFFFFFF ^ ADDR_MASK; // 16K pages

type Page = Vec<u8>;

pub struct LoggingMem {
	log: RefCell<Vec<Operation>>,
	pages: RefCell<HashMap<u32, Page>>,
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

pub trait AddressBus {
	fn copy_mem(&self) -> Box<AddressBus>;
	fn read_byte(&self, address_space: AddressSpace, address: u32) -> u32;
	fn read_word(&self, address_space: AddressSpace, address: u32) -> u32;
	fn read_long(&self, address_space: AddressSpace, address: u32) -> u32;
	fn write_byte(&mut self, address_space: AddressSpace, address: u32, value: u32);
	fn write_word(&mut self, address_space: AddressSpace, address: u32, value: u32);
	fn write_long(&mut self, address_space: AddressSpace, address: u32, value: u32);
}

impl Clone for LoggingMem {
	fn clone(&self) -> Self {
		LoggingMem { log: RefCell::new(Vec::new()), pages: RefCell::new(HashMap::new()), initializer: self.initializer }
	}
}

impl LoggingMem {
	pub fn new(initializer: u32) -> LoggingMem {
		LoggingMem { log: RefCell::new(Vec::new()), pages: RefCell::new(HashMap::new()), initializer: initializer }
	}
	fn log_len(&self) -> usize {
		let log = self.log.borrow();
		log.len()
	}
	fn get_log(&self, index: usize) -> Operation {
		let log = self.log.borrow();
		log[index]
	}
	fn allocated_pages(&self) -> usize {
		let pages = self.pages.borrow();
		pages.len()
	}
	fn ensure_page(&self, address: u32) -> u32 {
		let page = address & PAGE_MASK;
		let mut pages = self.pages.borrow_mut();
		if !pages.contains_key(&page) {
			pages.insert(page, Vec::with_capacity(PAGE_SIZE as usize));
			if let Some(mut page) = pages.get_mut(&page) {
				for _ in 0..(PAGE_SIZE / 4) {
					page.push(((self.initializer >> 24) & 0xFF) as u8);
					page.push(((self.initializer >> 16) & 0xFF) as u8);
					page.push(((self.initializer >>  8) & 0xFF) as u8);
					page.push(((self.initializer >>  0) & 0xFF) as u8);
				}
			}
		}
		page
	}
	fn read_u8(&self, address: u32) -> u32 {
		let page = self.ensure_page(address);
		let index = (address & ADDR_MASK) as usize;
		self.pages.borrow()[&page][index] as u32
	}

	fn write_u8(&mut self, address: u32, value: u8) {
		let page = self.ensure_page(address);
		let index = (address & ADDR_MASK) as usize;
		let mut pages = self.pages.borrow_mut();
		if let Some(mut page) = pages.get_mut(&page) {
			page[index] = value;
		}
	}
}

impl AddressBus for LoggingMem {
	fn copy_mem(&self) -> Box<AddressBus> {
		let mut copy = LoggingMem::new(self.initializer);
		for i in 0..1024u32 {
			copy.write_byte(SUPERVISOR_PROGRAM, i, self.read_byte(SUPERVISOR_PROGRAM, i));
		}
		Box::new(copy)
	}

	fn read_byte(&self, address_space: AddressSpace, address: u32) -> u32 {
		let mut log = self.log.borrow_mut();
		log.push(Operation::ReadByte(address_space, address));
		self.read_u8(address)
	}

	fn read_word(&self, address_space: AddressSpace, address: u32) -> u32 {
		let mut log = self.log.borrow_mut();
		log.push(Operation::ReadWord(address_space, address));
		(self.read_u8(address+0) << 8
		|self.read_u8(address+1) << 0) as u32
	}

	fn read_long(&self, address_space: AddressSpace, address: u32) -> u32 {
		let mut log = self.log.borrow_mut();
		log.push(Operation::ReadLong(address_space, address));
		(self.read_u8(address+0) << 24
		|self.read_u8(address+1) << 16
		|self.read_u8(address+2) <<  8
		|self.read_u8(address+3) <<  0) as u32
	}

	fn write_byte(&mut self, address_space: AddressSpace, address: u32, value: u32) {
		{
			let mut log = self.log.borrow_mut();
			log.push(Operation::WriteByte(address_space, address, value));
		}
		self.write_u8(address, (value & 0xFF) as u8);
	}

	fn write_word(&mut self, address_space: AddressSpace, address: u32, value: u32) {
		{
			let mut log = self.log.borrow_mut();
			log.push(Operation::WriteWord(address_space, address, value));
		}
		self.write_u8(address+0, ((value >>  8) & 0xFF) as u8);
		self.write_u8(address+1, ((value >>  0) & 0xFF) as u8);
	}

	fn write_long(&mut self, address_space: AddressSpace, address: u32, value: u32) {
		{
			let mut log = self.log.borrow_mut();
			log.push(Operation::WriteLong(address_space, address, value));
		}
		self.write_u8(address+0, ((value >> 24) & 0xFF) as u8);
		self.write_u8(address+1, ((value >> 16) & 0xFF) as u8);
		self.write_u8(address+2, ((value >>  8) & 0xFF) as u8);
		self.write_u8(address+3, ((value >>  0) & 0xFF) as u8);
	}
}

#[cfg(test)]
mod tests {
	use super::{LoggingMem, AddressBus, Operation, SUPERVISOR_DATA, SUPERVISOR_PROGRAM, USER_DATA, USER_PROGRAM, PAGE_SIZE};

	#[test]
	fn read_initialized_memory() {
		let mem = LoggingMem::new(0x01020304);
		for v in 0..256 {
			assert_eq!(0x01, mem.read_byte(SUPERVISOR_DATA, 4*v+0));
			assert_eq!(0x02, mem.read_byte(SUPERVISOR_DATA, 4*v+1));
			assert_eq!(0x03, mem.read_byte(SUPERVISOR_DATA, 4*v+2));
			assert_eq!(0x04, mem.read_byte(SUPERVISOR_DATA, 4*v+3));
		}
		for v in 0..256 {
			assert_eq!(0x0102, mem.read_word(SUPERVISOR_DATA, 4*v+0));
			assert_eq!(0x0203, mem.read_word(SUPERVISOR_DATA, 4*v+1));
			assert_eq!(0x0304, mem.read_word(SUPERVISOR_DATA, 4*v+2));
			if 4*v+3 < 1023 {
				assert_eq!(0x0401, mem.read_word(SUPERVISOR_DATA, 4*v+3));
			}
		}
		for v in 0..255 {
			assert_eq!(0x01020304, mem.read_long(SUPERVISOR_DATA, 4*v+0));
			assert_eq!(0x02030401, mem.read_long(SUPERVISOR_DATA, 4*v+1));
			assert_eq!(0x03040102, mem.read_long(SUPERVISOR_DATA, 4*v+2));
			assert_eq!(0x04010203, mem.read_long(SUPERVISOR_DATA, 4*v+3));
		}
		assert_eq!(0x01020304, mem.read_long(SUPERVISOR_DATA, 4*255));
	}

	#[test]
	fn read_your_u32_writes() {
		let mut mem = LoggingMem::new(0x01020304);
		let pattern = 0xAAAA7777;
		let address = 128;
		assert!(pattern != mem.read_long(SUPERVISOR_DATA, address));
		mem.write_long(SUPERVISOR_DATA, address, pattern);
		assert_eq!(pattern, mem.read_long(SUPERVISOR_DATA, address));
	}

	#[test]
	fn read_your_u16_writes() {
		let mut mem = LoggingMem::new(0x01020304);
		let pattern = 0xAAAA7777;
		let address = 128;
		assert!(pattern != mem.read_word(SUPERVISOR_DATA, address));
		mem.write_word(SUPERVISOR_DATA, address, pattern);
		assert_eq!(pattern & 0xFFFF, mem.read_word(SUPERVISOR_DATA, address));
	}

	#[test]
	fn read_your_u8_writes() {
		let mut mem = LoggingMem::new(0x01020304);
		let pattern = 0xAAAA7777;
		let address = 128;
		assert!(pattern != mem.read_byte(SUPERVISOR_DATA, address));
		mem.write_byte(SUPERVISOR_DATA, address, pattern);
		assert_eq!(pattern & 0xFF, mem.read_byte(SUPERVISOR_DATA, address));
	}

	#[test]
	fn read_byte_is_logged() {
		let mem = LoggingMem::new(0x01020304);
		let address = 128;
		mem.read_byte(SUPERVISOR_DATA, address);
		assert!(mem.log_len() > 0);
		assert_eq!(Operation::ReadByte(SUPERVISOR_DATA, address), mem.get_log(0));
	}

	#[test]
	fn read_word_is_logged() {
		let mem = LoggingMem::new(0x01020304);
		let address = 128;
		mem.read_word(SUPERVISOR_PROGRAM, address);
		assert!(mem.log_len() > 0);
		assert_eq!(Operation::ReadWord(SUPERVISOR_PROGRAM, address), mem.get_log(0));
	}

	#[test]
	fn read_long_is_logged() {
		let mem = LoggingMem::new(0x01020304);
		let address = 128;
		mem.read_long(USER_DATA, address);
		assert!(mem.log_len() > 0);
		assert_eq!(Operation::ReadLong(USER_DATA, address), mem.get_log(0));
	}

	#[test]
	fn write_byte_is_logged() {
		let mut mem = LoggingMem::new(0x01020304);
		let address = 128;
		let pattern = 0xAAAA7777;
		mem.write_byte(SUPERVISOR_DATA, address, pattern);
		assert!(mem.log_len() > 0);
		assert_eq!(Operation::WriteByte(SUPERVISOR_DATA, address, pattern), mem.get_log(0));
	}

	#[test]
	fn write_word_is_logged() {
		let mut mem = LoggingMem::new(0x01020304);
		let address = 128;
		let pattern = 0xAAAA7777;
		mem.write_word(SUPERVISOR_PROGRAM, address, pattern);
		assert!(mem.log_len() > 0);
		assert_eq!(Operation::WriteWord(SUPERVISOR_PROGRAM, address, pattern), mem.get_log(0));
	}

	#[test]
	fn write_long_is_logged() {
		let mut mem = LoggingMem::new(0x01020304);
		let address = 128;
		let pattern = 0xAAAA7777;
		mem.write_long(USER_DATA, address, pattern);
		assert!(mem.log_len() > 0);
		assert_eq!(Operation::WriteLong(USER_DATA, address, pattern), mem.get_log(0));
	}

	#[test]
	fn shared_address_space() {
		let mut mem = LoggingMem::new(0x01020304);
		let pattern = 0xAAAA7777;
		let address = 128;
		assert!(pattern != mem.read_long(SUPERVISOR_DATA, address));
		assert!(pattern != mem.read_long(SUPERVISOR_PROGRAM, address));
		assert!(pattern != mem.read_long(USER_DATA, address));
		assert!(pattern != mem.read_long(USER_PROGRAM, address));
		mem.write_long(SUPERVISOR_DATA, address, pattern);

		assert_eq!(pattern, mem.read_long(SUPERVISOR_DATA, address));
		assert_eq!(pattern, mem.read_long(SUPERVISOR_PROGRAM, address));
		assert_eq!(pattern, mem.read_long(USER_DATA, address));
		assert_eq!(pattern, mem.read_long(USER_PROGRAM, address));
	}

	#[test]
	fn page_allocation()
	{
		let mem = LoggingMem::new(0x01020304);
		let address = 0xFF0000;
		// no pages allocated
		assert_eq!(0, mem.allocated_pages());
		// one page allocated after read
		mem.read_long(SUPERVISOR_DATA, address);
		assert_eq!(1, mem.allocated_pages());
		// no more pages allocated after reading on same page
		mem.read_long(SUPERVISOR_DATA, address + 1);
		assert_eq!(1, mem.allocated_pages());
		// an additional page allocated after reading on new page
		mem.read_long(SUPERVISOR_DATA, address + PAGE_SIZE * 10);
		assert_eq!(2, mem.allocated_pages());
		// two additional pages allocated after reading over new page boundary
		mem.read_long(SUPERVISOR_DATA, address + 4*PAGE_SIZE - 2);
		assert_eq!(4, mem.allocated_pages());
	}
}