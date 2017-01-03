extern crate r68k_tools;

use r68k_tools::assembler::Assembler;
use std::io;
use std::io::BufReader;
use r68k_tools::srecords::write_s68;
use r68k_tools::memory::Memory;

fn main() {
    let r68k = Assembler::new();

    let asm = r#"
    ; let's start off with a comment, and then set PC to $1000
    ORG $1000

    ADD.B   #$3,D0
    ADD.B   D0,D1
"#;

    println!("{}", asm);
    let mut reader = BufReader::new(asm.as_bytes());
    let (end, mem) = r68k.assemble(&mut reader).unwrap();
    let offset = mem.offset();
    println!("assembled {:06x} - {:06x}", offset, end);
    let mut stdout = io::stdout();
    write_s68(&mut stdout, vec![&mem], 0).unwrap();
}
