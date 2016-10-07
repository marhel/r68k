extern crate r68k_tools;
use r68k_tools::srecords::write_s68;
use std::io::Write;
use std::io::LineWriter;

fn main() {
    let mut v:Vec<u8> = vec![];
    let mut lw = LineWriter::new(v);

    let data: Vec<u8> = (0u8 .. 0xFFu8).collect();

    write_s68(&mut lw, vec![(0x2016, data)], 0x2016);

    println!("{}", String::from_utf8(lw.into_inner().unwrap()).unwrap());
}