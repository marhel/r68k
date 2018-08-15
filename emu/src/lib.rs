#[macro_use]
#[cfg(test)]
extern crate lazy_static;
#[cfg(test)]
extern crate itertools;
extern crate r68k_common;
#[cfg(test)]
extern crate r68k_tools;

pub mod cpu;
pub mod ram;
pub mod interrupts;
pub mod musashi;


#[cfg(test)]
mod tests {
    use cpu::TestCore;
    use r68k_tools::memory::MemoryVec;
    use r68k_tools::PC;
    use r68k_tools::disassembler::disassemble;
    use r68k_tools::Exception;
    use cpu::ops::handlers::InstructionSetGenerator;
    use cpu::ops::handlers::OpcodeHandler;
    use r68k_tools::disassembler::Disassembler;

    #[test]
    // #[ignore]
    fn roundtrips() {
        let mut over = 0;
        let mut under = 0;
        let mut wrong = 0;
        let gen = InstructionSetGenerator::<TestCore>::new();
        let optable: Vec<&str> = gen.generate_with("???", |ref op| op.name);
        let d = Disassembler::new();
        for opcode in 0x0000..0xffff {
            let op = optable[opcode];
            let parts:Vec<&str> = op.split('_').collect();
            let mnemonic = parts[0];
            let mut pc = PC(0);
            let extension_word_mask = 0b1111_1000_1111_1111;
            // bits 8-10 should always be zero in the ea extension word
            // as we don't know which word will be seen as the ea extension word
            // (as opposed to immediate operand values) just make sure these aren't set.
            let dasm_mem = &mut MemoryVec::new16(pc, vec![opcode as u16, 0x001f, 0x00a4, 0x1234 & extension_word_mask, 0x5678 & extension_word_mask]);
            // println!("PREDASM {:04x}", opcode);
            match d.disassemble(pc, dasm_mem) {
                Err(Exception::IllegalInstruction(_opcode, _)) => if op != "???" && op != "unimplemented_1111" && op != "unimplemented_1010" && op != "illegal" {
                    under += 1;
                    println!("{:04x}: {} disasm under", opcode, op);
                }
                , //println!("{:04x}:\t\tover", opcode),
                Ok((new_pc, dis_inst)) => if op == "???" || op == "unimplemented_1111" || op == "unimplemented_1010" || op == "illegal" {
                    over += 1;
                    println!("{:04x}: {} disasm over, {}", opcode, op, dis_inst);
                } else if dis_inst.mnemonic.to_lowercase() != mnemonic && mnemonic != "real" { // ILLEGAL == real_illegal
                    wrong += 1;
                    println!("{:04x}: {} disasm different {}", opcode, op, dis_inst);
                },
            }
        };
        println!("{}  opcodes over, {} under, {} wrong", over, under, wrong);
    }
}
