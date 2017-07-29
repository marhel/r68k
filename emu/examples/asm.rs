extern crate r68k_emu;
extern crate r68k_tools;

use r68k_emu::cpu::TestCore;
use r68k_tools::assembler::Assembler;
use r68k_tools::memory::Memory;
use std::io;
use std::io::BufReader;
use r68k_tools::srecords::write_s68;

fn main() {
    let r68k_asm = Assembler::new();

    let asm = r#"
    ; let's start off with a comment, and then set PC to $1000
    ORG $1000

    ADD.B   #$3,D0
    ADD.B   D0,D1
"#;

    println!("{}", asm);
    let mut reader = BufReader::new(asm.as_bytes());
    let (end, mem) = r68k_asm.assemble(&mut reader).unwrap();
    let offset = mem.offset();
    let mut r68k_emu = TestCore::new_mem(offset, mem.data());
    println!("assembled {:06x} - {:06x} and PC is {:06x}", offset, end, r68k_emu.pc);
    let mut stdout = io::stdout();
    write_s68(&mut stdout, vec![&mem], offset).unwrap();
    r68k_emu.execute1();
}
