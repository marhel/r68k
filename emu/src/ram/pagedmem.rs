use std::collections::HashMap;
use super::{AddressSpace, AddressBus, ADDRBUS_MASK};

const PAGE_SIZE: u32 = 16; // 16 bytes page size
const ADDR_MASK: u32 = PAGE_SIZE - 1; // all ones
const PAGE_MASK: u32 = ADDRBUS_MASK ^ ADDR_MASK;

type Page = Vec<u8>;

pub struct PagedMem {
    pages: HashMap<u32, Page>,
    pub initializer: u32,
}

impl PagedMem {
    #[allow(dead_code)]
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
            u32::from(page[index])
        } else {
            u32::from(self.read_initializer(address))
        }
    }

    pub fn write_u8(&mut self, address: u32, value: u32) {
        if let Some(page) = self.page_if_needed(address, value as u8) {
            let index = (address & ADDR_MASK) as usize;
            page[index] = (value & 0xFF) as u8;
        }
    }
    pub fn diffs(&self) -> DiffIter {
        let mut keys: Vec<u32> = self.pages.keys().cloned().collect();
        keys.sort();
        DiffIter { pages: &self.pages, keys, offset: 0 }
    }
}

impl PagedMem {
    pub fn new(initializer: u32) -> PagedMem {
        PagedMem { pages: HashMap::new(), initializer }
    }
}

pub struct DiffIter<'a> {
    pages: &'a HashMap<u32, Page>,
    keys: Vec<u32>,
    offset: usize,
}
impl<'a> Iterator for DiffIter<'a> {
    type Item = (u32, u8);
    fn next(&mut self) -> Option<(u32, u8)> {
        if self.offset >= PAGE_SIZE as usize * self.keys.len() {
            None
        } else {
            let pageindex = self.offset / PAGE_SIZE as usize;
            let index = self.offset % PAGE_SIZE as usize;
            let page = self.keys[pageindex];
            self.offset += 1;
            Some((page + index as u32, self.pages[&page][index]))
        }
    }
}

impl AddressBus for PagedMem {
    fn copy_from(&mut self, other: &Self) {
        for (addr, byte) in other.diffs() {
            self.write_u8(addr, u32::from(byte));
        }
    }

    fn read_byte(&self, _address_space: AddressSpace, address: u32) -> u32 {
        self.read_u8(address)
    }

    fn read_word(&self, _address_space: AddressSpace, address: u32) -> u32 {
        (self.read_u8(address) << 8
        |self.read_u8(address.wrapping_add(1))) as u32
    }

    fn read_long(&self, _address_space: AddressSpace, address: u32) -> u32 {
        (self.read_u8(address) << 24
        |self.read_u8(address.wrapping_add(1)) << 16
        |self.read_u8(address.wrapping_add(2)) <<  8
        |self.read_u8(address.wrapping_add(3))) as u32
    }

    fn write_byte(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        self.write_u8(address, value);
    }

    fn write_word(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        self.write_u8(address, value >>  8);
        self.write_u8(address.wrapping_add(1), value);
    }

    fn write_long(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        self.write_u8(address, value >> 24);
        self.write_u8(address.wrapping_add(1), value >> 16);
        self.write_u8(address.wrapping_add(2), value >>  8);
        self.write_u8(address.wrapping_add(3), value);
    }
}

#[cfg(test)]
mod tests {
    use super::{AddressBus, PagedMem, PAGE_SIZE};
    use ram::{SUPERVISOR_DATA, SUPERVISOR_PROGRAM, USER_DATA, USER_PROGRAM, ADDRBUS_MASK};

    #[test]
    fn read_initialized_memory() {
        let mem = PagedMem::new(0x01020304);
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
        let mut mem = PagedMem::new(0x01020304);
        let pattern = 0xAAAA7777;
        let address = 128;
        assert!(pattern != mem.read_long(SUPERVISOR_DATA, address));
        mem.write_long(SUPERVISOR_DATA, address, pattern);
        assert_eq!(pattern, mem.read_long(SUPERVISOR_DATA, address));
    }

    #[test]
    fn read_your_u16_writes() {
        let mut mem = PagedMem::new(0x01020304);
        let pattern = 0xAAAA7777;
        let address = 128;
        assert!(pattern != mem.read_word(SUPERVISOR_DATA, address));
        mem.write_word(SUPERVISOR_DATA, address, pattern);
        assert_eq!(pattern & 0xFFFF, mem.read_word(SUPERVISOR_DATA, address));
    }

    #[test]
    fn read_your_u8_writes() {
        let mut mem = PagedMem::new(0x01020304);
        let pattern = 0xAAAA7777;
        let address = 128;
        assert!(pattern != mem.read_byte(SUPERVISOR_DATA, address));
        mem.write_byte(SUPERVISOR_DATA, address, pattern);
        assert_eq!(pattern & 0xFF, mem.read_byte(SUPERVISOR_DATA, address));
    }

    #[test]
    fn shared_address_space() {
        let mut mem = PagedMem::new(0x01020304);
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
        let mut mem = PagedMem::new(0x01020304);
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
        let mut mem = PagedMem::new(data);
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
        // we don't need to allocate a second page if we overwrite existing data
        mem.write_byte(SUPERVISOR_DATA, 2, 0x99);
        assert_eq!(1, mem.allocated_pages());
    }

    #[test]
    fn no_diff_initially()
    {
        let mem = PagedMem::new(0x01020304);
        assert_eq!(None, mem.diffs().next());
    }

    #[test]
    fn can_extract_diffs()
    {
        let mut mem = PagedMem::new(0x01020304);
        mem.write_byte(SUPERVISOR_DATA, PAGE_SIZE * 10, 0x91);
        mem.write_byte(SUPERVISOR_DATA, PAGE_SIZE * 20, 0x92);
        assert_eq!(2, mem.allocated_pages());
        let diffs: Vec<(u32, u8)> = mem.diffs().collect();
        assert_eq!((PAGE_SIZE * 10, 0x91), diffs[0]);
        assert_eq!((PAGE_SIZE * 20, 0x92), diffs[PAGE_SIZE as usize]);
    }

    #[test]
    fn extracts_two_full_pages_of_diffs()
    {
        let mut mem = PagedMem::new(0x01020304);
        mem.write_byte(SUPERVISOR_DATA, PAGE_SIZE * 10, 0x91);
        mem.write_byte(SUPERVISOR_DATA, PAGE_SIZE * 20, 0x92);

        assert_eq!(PAGE_SIZE as usize * mem.allocated_pages(), mem.diffs().count());
    }

    #[test]
    fn cross_address_bus_boundary_byte_access() {
        let mut mem = PagedMem::new(0x01020304);
        mem.write_byte(SUPERVISOR_DATA, ADDRBUS_MASK, 0x91);
        assert_eq!(0x91, mem.read_byte(SUPERVISOR_DATA, ADDRBUS_MASK));
        mem.write_byte(SUPERVISOR_DATA, ADDRBUS_MASK+1, 0x92);
        assert_eq!(0x92, mem.read_byte(SUPERVISOR_DATA, 0));
    }

    #[test]
    fn cross_address_bus_boundary_word_access() {
        let mut mem = PagedMem::new(0x01020304);
        mem.write_word(SUPERVISOR_DATA, ADDRBUS_MASK+1, 0x9192);
        assert_eq!(0x9192, mem.read_word(SUPERVISOR_DATA, 0));
    }

    #[test]
    fn cross_address_bus_boundary_long_access() {
        let mut mem = PagedMem::new(0x01020304);
        mem.write_long(SUPERVISOR_DATA, ADDRBUS_MASK-1, 0x91929394);
        assert_eq!(0x91929394, mem.read_long(SUPERVISOR_DATA, ADDRBUS_MASK-1));
    }

    #[test]
    fn cross_type_boundary_word_access() {
        let mut mem = PagedMem::new(0x01020304);

        let addr = u32::max_value()-1;
        mem.write_word(SUPERVISOR_DATA, addr, 0x9192);
        assert_eq!(0x9192, mem.read_word(SUPERVISOR_DATA, addr));
    }

    #[test]
    fn cross_type_boundary_long_access() {
        let mut mem = PagedMem::new(0x01020304);

        let addr = u32::max_value()-1;
        mem.write_long(SUPERVISOR_DATA, addr, 0x91929394);
        assert_eq!(0x91929394, mem.read_long(SUPERVISOR_DATA, addr));
    }
}