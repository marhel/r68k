use std::collections::HashMap;

pub struct LoggingMem {
	ops: Vec<Operation>,
	mem: HashMap<AddressSpace, [u8; 1024]>
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddressSpace(Mode, Segment);
#[derive(Copy, Clone, Debug, PartialEq)]
enum Segment {
	Program, Data
}
#[derive(Copy, Clone, Debug, PartialEq)]
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
	fn read_u8(address_space: AddressSpace, address: u32) -> u32;
	fn read_u16(address_space: AddressSpace, address: u32) -> u32;
	fn read_u32(address_space: AddressSpace, address: u32) -> u32;
	fn write_u8(address_space: AddressSpace, address: u32, value: u32);
	fn write_u16(address_space: AddressSpace, address: u32, value: u32);
	fn write_u32(address_space: AddressSpace, address: u32, value: u32);
}

impl AddressBus for LoggingMem {
	fn read_u8(address_space: AddressSpace, address: u32) -> u32 {
		return 0;
	}
	fn read_u16(address_space: AddressSpace, address: u32) -> u32 {
		return 0;
	}

	fn read_u32(address_space: AddressSpace, address: u32) -> u32 {
		return 0;
	}

	fn write_u8(address_space: AddressSpace, address: u32, value: u32) {
	}

	fn write_u16(address_space: AddressSpace, address: u32, value: u32) {
	}

	fn write_u32(address_space: AddressSpace, address: u32, value: u32) {
	}
}

#[cfg(test)]
mod tests {
	use super::LoggingMem;

	#[test]
	fn new_sets_pc() {
	}
}