pub trait Memory {
    fn read_word(&self, pc: u32) -> u16;
    fn write_word(&mut self, pc: u32, word: u16) -> u16;
}

#[derive(Debug)] 
pub struct MemoryVec {
    pub mem: Vec<u16>
}

impl Memory for MemoryVec {
    fn read_word(&self, pc: u32) -> u16 {
        if pc % 1 == 1 { panic!("Odd PC!") }
        self.mem[(pc/2) as usize]
    }
    fn write_word(&mut self, pc: u32, word: u16) -> u16 {
        if pc % 1 == 1 { panic!("Odd PC!") }
        let old = self.mem[(pc/2) as usize];
        self.mem[(pc/2) as usize] = word;
        old
    }
}
