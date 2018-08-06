#![feature(test)]

extern crate test;
extern crate r68k_emu;
use test::Bencher;

use r68k_emu::cpu::{ConfiguredCore, Core, ProcessingState, Result, Cycles, Exception, Callbacks};
use r68k_emu::cpu::ops::opcodes;
use r68k_emu::ram::PagedMem;
use r68k_emu::interrupts::AutoInterruptController;

struct LogAllExceptions {
    count: isize
}
impl Callbacks for LogAllExceptions {
    fn exception_callback(&mut self, _: &mut impl Core, ex: Exception) -> Result<Cycles> {
        println!("{:?}", ex);
        self.count += 1;
        Err(ex)
    }
}

#[bench]
fn bench_100k_cycles(b: &mut Bencher) {
    let mut cpu = ConfiguredCore::new_with(0, AutoInterruptController::new(), PagedMem::new(0xAAAAAAAA));
    let regregops = [opcodes::OP_ADD_16_ER_DN, opcodes::OP_SUB_16_ER_DN, opcodes::OP_AND_16_ER_DN, opcodes::OP_OR_16_ER_DN];
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
    cpu.write_data_word(generic_handler, opcodes::OP_RTE_32).unwrap(); // handler is just RTE
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