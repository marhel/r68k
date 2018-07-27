extern crate r68k_tools;

use r68k_tools::srecords::write_s68;
use std::io::LineWriter;
use r68k_tools::memory::MemoryVec;
use r68k_tools::PC;

fn main() {
    let mut lw = LineWriter::new(vec![]);

    let data: Vec<u8> = (0u8 .. 0xFFu8).collect();
    let start = 0x2016;
    let mem = MemoryVec::new8(PC(start), data);
    write_s68(&mut lw, vec![&mem], start).unwrap();

    println!("{}", String::from_utf8(lw.into_inner().unwrap()).unwrap());
}