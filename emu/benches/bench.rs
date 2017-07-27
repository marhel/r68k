#![feature(test)]

extern crate test;
extern crate r68k_emu;
use test::Bencher;

use r68k_emu::cpu::{Core, ProcessingState, Result, Cycles, Exception, Callbacks};
use r68k_emu::cpu::ops::handlers;

struct LogAllExceptions {
    count: isize
}
impl Callbacks for LogAllExceptions {
    fn exception_callback(&mut self, _: &mut Core, ex: Exception) -> Result<Cycles> {
        println!("{:?}", ex);
        self.count += 1;
        Err(ex)
    }
}

#[bench]
fn bench_100k_cycles(b: &mut Bencher) {
    let mut cpu = Core::new_auto();
    let regregops = [handlers::OP_ADD_16_ER_DN, handlers::OP_SUB_16_ER_DN, handlers::OP_AND_16_ER_DN, handlers::OP_OR_16_ER_DN];
    let pc_base = 0x1000;
    // write an instruction sequence of simple reg-to-reg operations
    for i in 0..0x10000 {
        cpu.write_program_word(pc_base + i*2, regregops[(i % regregops.len() as u32) as usize]).unwrap();
    }
    cpu.write_program_long(0, 0x100000).unwrap(); // SSP
    cpu.write_program_long(4, pc_base).unwrap(); // PC

    let generic_handler = 0x900;
    for exception in 2..256 {
        cpu.write_data_long(exception * 4, generic_handler).unwrap(); // set up exception vector
    }
    cpu.write_data_word(generic_handler, handlers::OP_RTE_32).unwrap(); // handler is just RTE
    let cycles_per_instruction = 4;
    let num_instructions = 25_000;
    let bytes_per_instruction = 2;
    let mut handler = LogAllExceptions { count: 0 };
    cpu.reset();
    assert_eq!(pc_base, cpu.pc);
    b.iter(|| {
        cpu.pc = pc_base;
        cpu.execute_with_state(num_instructions * cycles_per_instruction, &mut handler);
    });
    assert_eq!(0, handler.count);
    assert_eq!(ProcessingState::Normal, cpu.processing_state);
    assert_eq!(pc_base + (num_instructions * bytes_per_instruction) as u32, cpu.pc);
}