use PC;

pub trait Memory {
    fn offset(&self) -> u32;
    fn data(&self) -> &[u8];
    fn read_word(&self, pc: PC) -> u16;
    fn read_byte(&self, pc: PC) -> u8;
    fn write_byte(&mut self, pc: PC, byte: u8) -> PC;
    fn write_word(&mut self, pc: PC, word: u16) -> PC;
    fn write_vec(&mut self, pc: PC, bytes: Vec<u8>) -> PC;
}

#[derive(Debug)]
pub struct MemoryVec {
    offset: Option<PC>,
    mem: Vec<u8>
}

impl MemoryVec {
    pub fn new() -> MemoryVec {
        MemoryVec { offset: None, mem: vec![]}
    }
    pub fn new8(offset: PC, bytes: Vec<u8>) -> MemoryVec {
        MemoryVec { offset: Some(offset), mem: bytes}
    }
    pub fn new16(offset: PC, init: Vec<u16>) -> MemoryVec {
        let mut mem = MemoryVec { offset: Some(offset), mem: vec![]};
        let mut pc = PC(0);
        for word in init {
            pc = mem.write_word(pc, word);
        };
        mem
    }
}

impl Memory for MemoryVec {
    fn offset(&self) -> u32 {
        self.offset.unwrap().0
    }
    fn data(&self) -> &[u8] {
        &self.mem
    }
    fn read_word(&self, pc: PC) -> u16 {
        if pc.is_odd() { panic!("Odd PC!") }
        let index = usize::from(pc - self.offset.unwrap());
        (self.mem[index] as u16) << 8 | self.mem[index+1] as u16
    }
    fn read_byte(&self, pc: PC) -> u8 {
        let index = usize::from(pc - self.offset.unwrap());
        self.mem[index]
    }
    fn write_byte(&mut self, pc: PC, byte: u8) -> PC {
        if self.offset.is_none() {
            self.offset = Some(pc);
        }
        let index = usize::from(pc - self.offset.unwrap());
        let size = self.mem.len();
        match index {
            i if i < size => self.mem[i] = byte,
            i if i == size => self.mem.push(byte),
            i => panic!("Index {} out of bounds for size {}", i, size),
        };
        pc + 1
    }
    fn write_word(&mut self, pc: PC, word: u16) -> PC {
        if pc.is_odd() { panic!("Odd PC!") }
        self.write_byte(pc, (word >> 8) as u8);
        self.write_byte(pc + 1, word as u8)
    }
    fn write_vec(&mut self, pc: PC, bytes: Vec<u8>) -> PC {
        let mut pc = pc;
        for b in bytes {
            pc = self.write_byte(pc, b);
        }
        pc
    }
}

#[cfg(test)]
mod tests {
    use super::{MemoryVec, Memory};
    use PC;

    #[test]
    fn byte_writes_can_be_read() {
        let mut mem = MemoryVec { offset: Some(PC(0)), mem: vec![0x01,0x02] };
        let pc = PC(0);
        let value = 0x34;
        mem.write_byte(pc, value);
        assert_eq!(value, mem.read_byte(pc));
    }

    #[test]
    fn consecutive_byte_writes_will_append() {
        let pc = PC(0x1000);
        let mut mem = MemoryVec { offset: Some(pc), mem: vec![] };
        let value = 0x34;
        let value2 = 0x57;
        let pc2 = mem.write_byte(pc, value);
        mem.write_byte(pc2, value2);
        assert_eq!(value, mem.read_byte(pc));
        assert_eq!(value2, mem.read_byte(pc2));
    }

    #[test]
    fn word_writes_can_be_read() {
        let mut mem = MemoryVec { offset: Some(PC(0)), mem: vec![0x01,0x02] };
        let pc = PC(0);
        let value = 0x3456;
        mem.write_word(pc, value);
        assert_eq!(value, mem.read_word(pc));
    }

    #[test]
    fn consecutive_word_writes_will_append() {
        let mut mem = MemoryVec { offset: Some(PC(0)), mem: vec![] };
        let pc = PC(0);
        let value = 0x3456;
        let value2 = 0x3457;
        let pc2 = mem.write_word(pc, value);
        mem.write_word(pc2, value2);
        assert_eq!(value, mem.read_word(pc));
        assert_eq!(value2, mem.read_word(pc2));
    }

    #[test]
    fn initial_write_can_set_offset_if_none_set() {
        let mut mem = MemoryVec { offset: None, mem: vec![] };
        let pc = PC(0x100);
        let value = 0x3456;
        mem.write_word(pc, value);
        assert_eq!(pc, mem.offset());
    }

    #[test]
    #[should_panic]
    fn additional_write_write_cannot_move_offset_even_if_none_set_initially() {
        let mut mem = MemoryVec { offset: None, mem: vec![] };
        let pc = PC(0x100);
        let value = 0x3456;
        mem.write_word(pc, value);
        let pc2 = PC(0x200);
        mem.write_word(pc2, value);
    }

    #[test]
    fn mem_can_be_offset() {
        let pc = PC(0x10000);
        let mem = MemoryVec { offset: Some(pc), mem: vec![0x01, 0x02, 0x03, 0x03] };
        let value = 0x0102;
        assert_eq!(value, mem.read_word(pc));
    }

    #[test]
    #[should_panic]
    fn unaligned_word_write_panics() {
        let mut mem = MemoryVec { offset: Some(PC(0)), mem: vec![0x01, 0x02, 0x03, 0x03] };
        let pc = PC(1); // unaligned
        let value = 0x3456;
        mem.write_word(pc, value);
    }

    #[test]
    #[should_panic]
    fn gapped_word_write_panics() {
        let mut mem = MemoryVec { offset: Some(PC(0)), mem: vec![] };
        let pc = PC(2); // write will not append consecutively
        let value = 0x3456;
        mem.write_word(pc, value);
    }

    #[test]
    #[should_panic]
    fn offset_word_write_before_offset_panics() {
        let mut mem = MemoryVec { offset: Some(PC(0x10000)), mem: vec![] };
        let pc = PC(0); // write will be before offset
        let value = 0x3456;
        mem.write_word(pc, value);
    }

    #[test]
    fn can_write_vec() {
        let data: Vec<u8> = (0u8 .. 0xA0u8).collect();
        let pc = PC(0x2000);
        let mut mem = MemoryVec::new8(pc, vec![]);
        let pc = mem.write_vec(pc, data);
        assert_eq!(0x20A0, pc);
        assert_eq!(0x0A0B, mem.read_word(PC(0x200A)));
    }

}