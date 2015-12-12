use std::cell::RefCell;
use std::collections::HashMap;
// The m68k had a 24 bit external address bus with
// (2^24 bytes = ) 16 MB addressable space
const PAGE_SIZE: u32 = 1024; // 1K page size
const ADDR_MASK: u32 = PAGE_SIZE - 1; // 1K page size
const PAGE_MASK: u32 = 0xFFFFFF ^ ADDR_MASK; // 16K pages

type Page = Vec<u8>;

pub trait OpsLogging {
	fn log(&self, op: Operation);
}

struct NopLogger;
pub struct OpsLogger {
	log: RefCell<Vec<Operation>>,
}

impl OpsLogging for NopLogger {
	#![allow(unused_variables)]
	fn log(&self, op: Operation) {
	}
}
impl OpsLogger {
	pub fn new() -> OpsLogger {
		OpsLogger { log: RefCell::new(Vec::new()) }
	}
	fn ops(&self) -> Vec<Operation>
	{
		self.log.borrow_mut().clone()
	}
	fn len(&self) -> usize {
		self.log.borrow_mut().len()
	}
}
impl OpsLogging for OpsLogger {
	fn log(&self, op: Operation) {
		self.log.borrow_mut().push(op);
	}
}

pub struct LoggingMem<T: OpsLogging> {
	logger: T,
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

pub trait AddressBus {
	fn copy_from(&mut self, other: &Self);
	fn read_byte(&self, address_space: AddressSpace, address: u32) -> u32;
	fn read_word(&self, address_space: AddressSpace, address: u32) -> u32;
	fn read_long(&self, address_space: AddressSpace, address: u32) -> u32;
	fn write_byte(&mut self, address_space: AddressSpace, address: u32, value: u32);
	fn write_word(&mut self, address_space: AddressSpace, address: u32, value: u32);
	fn write_long(&mut self, address_space: AddressSpace, address: u32, value: u32);
}

impl<T: OpsLogging> LoggingMem<T> {
	pub fn new(initializer: u32, logger: T) -> LoggingMem<T> {
		LoggingMem { logger: logger, pages: HashMap::new(), initializer: initializer }
	}
	fn allocated_pages(&self) -> usize {
		self.pages.len()
	}
	fn new_page_is_needed(&self, address: u32, value_to_write: u8) -> bool {
		let pageno = address & PAGE_MASK;
		let write_differs_from_initializer = value_to_write as u8 != self.read_initializer(address);
		!self.pages.contains_key(&pageno) && write_differs_from_initializer
	}
	// returns a page if it is already allocated, but inserts a missing page
	// only if we are going to need to write an interesting value to it
	// i.e. one that differs from the initializer
	fn page_if_needed(&mut self, address: u32, value_to_write: u8) -> Option<&mut Page> {
		let pageno = address & PAGE_MASK;
		if self.new_page_is_needed(address, value_to_write) {
			self.create_initialized_page(pageno);
		}
		self.pages.get_mut(&pageno)
	}
	fn create_initialized_page(&mut self, pageno: u32) {
		let mut page = Vec::with_capacity(PAGE_SIZE as usize);
		// initialize page
		for offset in 0..PAGE_SIZE {
			page.push(self.read_initializer(offset));
		}
		self.pages.insert(pageno, page);
	}
	// read uninitialized bytes from initializer instead
	fn read_initializer(&self, address: u32) -> u8 {
		let shift = match address % 4 {
		    0 => 24,
		    1 => 16,
		    2 =>  8,
		    _ =>  0,
		};
		((self.initializer >> shift) & 0xFF) as u8
	}
	pub fn read_u8(&self, address: u32) -> u32 {
		let pageno = address & PAGE_MASK;
		if let Some(page) = self.pages.get(&pageno) {
			let index = (address & ADDR_MASK) as usize;
			page[index] as u32
		} else {
			self.read_initializer(address) as u32
		}
	}

	pub fn write_u8(&mut self, address: u32, value: u32) {
		if let Some(page) = self.page_if_needed(address, value as u8) {
			let index = (address & ADDR_MASK) as usize;
			page[index] = (value & 0xFF) as u8;
		}
	}
}

impl<T: OpsLogging> AddressBus for LoggingMem<T> {
	fn copy_from(&mut self, other: &Self) {
		// copy first page, at least
		for i in 0..PAGE_SIZE {
			self.write_u8(i, other.read_u8(i));
		}
	}

	fn read_byte(&self, address_space: AddressSpace, address: u32) -> u32 {
		self.logger.log(Operation::ReadByte(address_space, address));
		self.read_u8(address)
	}

	fn read_word(&self, address_space: AddressSpace, address: u32) -> u32 {
		self.logger.log(Operation::ReadWord(address_space, address));
		(self.read_u8(address+0) << 8
		|self.read_u8(address+1) << 0) as u32
	}

	fn read_long(&self, address_space: AddressSpace, address: u32) -> u32 {
		self.logger.log(Operation::ReadLong(address_space, address));
		(self.read_u8(address+0) << 24
		|self.read_u8(address+1) << 16
		|self.read_u8(address+2) <<  8
		|self.read_u8(address+3) <<  0) as u32
	}

	fn write_byte(&mut self, address_space: AddressSpace, address: u32, value: u32) {
		self.logger.log(Operation::WriteByte(address_space, address, value));
		self.write_u8(address, value);
	}

	fn write_word(&mut self, address_space: AddressSpace, address: u32, value: u32) {
		self.logger.log(Operation::WriteWord(address_space, address, value));
		self.write_u8(address+0, (value >>  8));
		self.write_u8(address+1, (value >>  0));
	}

	fn write_long(&mut self, address_space: AddressSpace, address: u32, value: u32) {
		self.logger.log(Operation::WriteLong(address_space, address, value));
		self.write_u8(address+0, (value >> 24));
		self.write_u8(address+1, (value >> 16));
		self.write_u8(address+2, (value >>  8));
		self.write_u8(address+3, (value >>  0));
	}
}

#[cfg(test)]
mod tests {
	use super::{LoggingMem, AddressBus, OpsLogger, NopLogger, Operation, SUPERVISOR_DATA, SUPERVISOR_PROGRAM, USER_DATA, USER_PROGRAM, PAGE_SIZE};
	use std::cell::RefCell;

	#[test]
	fn read_initialized_memory() {
		let mem = LoggingMem::new(0x01020304, OpsLogger	{ log: RefCell::new(Vec::new()) });
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
		let mut mem = LoggingMem::new(0x01020304, NopLogger);
		let pattern = 0xAAAA7777;
		let address = 128;
		assert!(pattern != mem.read_long(SUPERVISOR_DATA, address));
		mem.write_long(SUPERVISOR_DATA, address, pattern);
		assert_eq!(pattern, mem.read_long(SUPERVISOR_DATA, address));
	}

	#[test]
	fn read_your_u16_writes() {
		let mut mem = LoggingMem::new(0x01020304, NopLogger);
		let pattern = 0xAAAA7777;
		let address = 128;
		assert!(pattern != mem.read_word(SUPERVISOR_DATA, address));
		mem.write_word(SUPERVISOR_DATA, address, pattern);
		assert_eq!(pattern & 0xFFFF, mem.read_word(SUPERVISOR_DATA, address));
	}

	#[test]
	fn read_your_u8_writes() {
		let mut mem = LoggingMem::new(0x01020304, NopLogger);
		let pattern = 0xAAAA7777;
		let address = 128;
		assert!(pattern != mem.read_byte(SUPERVISOR_DATA, address));
		mem.write_byte(SUPERVISOR_DATA, address, pattern);
		assert_eq!(pattern & 0xFF, mem.read_byte(SUPERVISOR_DATA, address));
	}

	#[test]
	fn read_byte_is_logged() {
		let mem = LoggingMem::new(0x01020304, OpsLogger::new());
		let address = 128;
		mem.read_byte(SUPERVISOR_DATA, address);
		assert!(mem.logger.len() > 0);
		assert_eq!(Operation::ReadByte(SUPERVISOR_DATA, address), mem.logger.ops()[0]);
	}

	#[test]
	fn read_word_is_logged() {
		let mem = LoggingMem::new(0x01020304, OpsLogger::new());
		let address = 128;
		mem.read_word(SUPERVISOR_PROGRAM, address);
		assert!(mem.logger.len() > 0);
		assert_eq!(Operation::ReadWord(SUPERVISOR_PROGRAM, address), mem.logger.ops()[0]);
	}

	#[test]
	fn read_long_is_logged() {
		let mem = LoggingMem::new(0x01020304, OpsLogger::new());
		let address = 128;
		mem.read_long(USER_DATA, address);
		assert!(mem.logger.len() > 0);
		assert_eq!(Operation::ReadLong(USER_DATA, address), mem.logger.ops()[0]);
	}

	#[test]
	fn write_byte_is_logged() {
		let mut mem = LoggingMem::new(0x01020304, OpsLogger::new());
		let address = 128;
		let pattern = 0xAAAA7777;
		mem.write_byte(SUPERVISOR_DATA, address, pattern);
		assert!(mem.logger.len() > 0);
		assert_eq!(Operation::WriteByte(SUPERVISOR_DATA, address, pattern), mem.logger.ops()[0]);
	}

	#[test]
	fn write_word_is_logged() {
		let mut mem = LoggingMem::new(0x01020304, OpsLogger::new());
		let address = 128;
		let pattern = 0xAAAA7777;
		mem.write_word(SUPERVISOR_PROGRAM, address, pattern);
		assert!(mem.logger.len() > 0);
		assert_eq!(Operation::WriteWord(SUPERVISOR_PROGRAM, address, pattern), mem.logger.ops()[0]);
	}

	#[test]
	fn write_long_is_logged() {
		let mut mem = LoggingMem::new(0x01020304, OpsLogger::new());
		let address = 128;
		let pattern = 0xAAAA7777;
		mem.write_long(USER_DATA, address, pattern);
		assert!(mem.logger.len() > 0);
		assert_eq!(Operation::WriteLong(USER_DATA, address, pattern), mem.logger.ops()[0]);
	}

	#[test]
	fn shared_address_space() {
		let mut mem = LoggingMem::new(0x01020304, NopLogger);
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
	fn page_allocation_on_write()
	{
		let mut mem = LoggingMem::new(0x01020304, NopLogger);
		let data = 12345678;
		let address = 0xFF0000;
		// no pages allocated
		assert_eq!(0, mem.allocated_pages());
		// no pages allocated after read
		mem.read_long(SUPERVISOR_DATA, address);
		// no pages allocated after read of different page
		mem.read_long(SUPERVISOR_DATA, address + PAGE_SIZE * 10);
		assert_eq!(0, mem.allocated_pages());
		// one page allocated after write
		mem.write_long(SUPERVISOR_DATA, address, data);
		assert_eq!(1, mem.allocated_pages());
		// no more pages allocated after more writing on same page
		mem.write_long(SUPERVISOR_DATA, address + 1, data);
		assert_eq!(1, mem.allocated_pages());
		// an additional page allocated after writing on new page
		mem.write_long(SUPERVISOR_DATA, address + PAGE_SIZE * 10, data);
		assert_eq!(2, mem.allocated_pages());
		// no additional pages allocated after reading over new page boundary
		mem.read_long(SUPERVISOR_DATA, address + 4*PAGE_SIZE - 2);
		assert_eq!(2, mem.allocated_pages());
		// two additional pages allocated after writing over new page boundary
		mem.write_long(SUPERVISOR_DATA, address + 4*PAGE_SIZE - 2, data);
		assert_eq!(4, mem.allocated_pages());
	}

	#[test]
	fn page_allocation_on_write_unless_matching_initializer()
	{
		let data = 0x01020304;
		let mut mem = LoggingMem::new(data, OpsLogger::new());
		for offset in 0..PAGE_SIZE/4 {
			mem.write_long(SUPERVISOR_DATA, 4*offset, data);
		}
		mem.write_byte(SUPERVISOR_DATA, 0, 0x1);
		mem.write_byte(SUPERVISOR_DATA, 1, 0x2);
		mem.write_byte(SUPERVISOR_DATA, 2, 0x3);
		mem.write_byte(SUPERVISOR_DATA, 3, 0x4);

		mem.write_word(SUPERVISOR_DATA, 3, 0x0401);

		// no pages allocated
		assert_eq!(0, mem.allocated_pages());
		// but as soon as we write something different
		mem.write_byte(SUPERVISOR_DATA, 2, 0x2);
		// a page is allocated
		assert_eq!(1, mem.allocated_pages());
		assert_eq!(262, mem.logger.len());
		assert_eq!(262, mem.logger.ops().len());
	}
}