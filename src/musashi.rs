// Integration with Musashi
extern crate libc;


// Register enum copied from Musashi's m68k_register_t enum
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Register {
    /* Real registers */
    D0,        /* Data registers */
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    A0,        /* Address registers */
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    PC,        /* Program Counter */
    SR,        /* Status Register */
    SP,        /* The current Stack Pointer (located in A7) */
    USP,        /* User Stack Pointer */
    ISP,        /* Interrupt Stack Pointer */
    MSP,        /* Master Stack Pointer */
    SFC,        /* Source Function Code */
    DFC,        /* Destination Function Code */
    VBR,        /* Vector Base Register */
    CACR,        /* Cache Control Register */
    CAAR,        /* Cache Address Register */

    /* Assumed registers */
    /* These are cheat registers which emulate the 1-longword prefetch
     * present in the 68000 and 68010.
     */
    PrefAddr,    /* Last prefetch address */
    PrefData,    /* Last prefetch data */

    /* Convenience registers */
    PPC,        /* Previous value in the program counter */
    IR,            /* Instruction register */
    CpuType    /* Type of CPU being run */
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
enum CpuType
{
    Invalid,
    M68000,
    M68010,
    M68EC020,
    M68020,
    M68030,        /* Supported by disassembler ONLY */
    M68040        /* Supported by disassembler ONLY */
}
#[link(name = "musashi", kind = "static")]
extern {
    fn m68k_init();
    fn m68k_set_cpu_type(cputype: CpuType);
    fn m68k_pulse_reset();
    fn m68k_execute(num_cycles: i32) -> i32;
    fn m68k_get_reg(context: *mut libc::c_void, regnum: Register) -> u32;
    fn m68k_set_reg(regnum: Register, value: u32);
}
use ram::{Operation, AddressSpace, SUPERVISOR_PROGRAM, SUPERVISOR_DATA, USER_PROGRAM, USER_DATA, ADDRBUS_MASK};
static mut musashi_locations_used: usize = 0;
static mut musashi_memory_initializer: u32 = 0xaaaaaaaa;
static mut musashi_memory_location:  [u32; 1024] = [0; 1024];
static mut musashi_memory_data:  [u8; 1024] = [0; 1024];

// as statics are not allowed to have destructors, allocate a
// big enough array to hold the small number of operations
// expected from executing a very limited number of opcodes
static mut musashi_ops: [Operation; 512] = [Operation::None; 512];
static mut musashi_opcount: usize = 0;
static mut musashi_address_space: AddressSpace = SUPERVISOR_PROGRAM;

unsafe fn register_op(op: Operation) {
    if musashi_opcount < musashi_ops.len() {
        // println!("mem_op {:?}", op);
        musashi_ops[musashi_opcount] = op;
        musashi_opcount += 1;
    }
}
// callbacks from Musashi
#[no_mangle]
pub extern fn m68k_read_memory_8(address: u32) -> u32 {
    unsafe {
        let address = address & ADDRBUS_MASK;
        let value = read_musashi_byte(address);
        let op = Operation::ReadByte(musashi_address_space, address, value);
        register_op(op);
        value as u32
    }
}
#[no_mangle]
pub extern fn m68k_read_memory_16(address: u32) -> u32 {
    unsafe {
        let address = address & ADDRBUS_MASK;
        let value =  (read_musashi_byte(address+0) as u16) << 8
                    |(read_musashi_byte(address+1) as u16) << 0;
        let op = Operation::ReadWord(musashi_address_space, address, value);
        register_op(op);
        value as u32
    }
}
#[no_mangle]
pub extern fn m68k_read_memory_32(address: u32) -> u32 {
    unsafe {
        let value = ((read_musashi_byte(address+0) as u32) << 24
                    |(read_musashi_byte(address+1) as u32) << 16
                    |(read_musashi_byte(address+2) as u32) <<  8
                    |(read_musashi_byte(address+3) as u32) <<  0) as u32;
        let op = Operation::ReadLong(musashi_address_space, address, value);
        register_op(op);
        value
    }
}

#[no_mangle]
pub extern fn m68k_write_memory_8(address: u32, value: u32) {
    unsafe {
        let op = Operation::WriteByte(musashi_address_space, address, value);
        register_op(op);
        write_musashi_byte(address+0, (value & 0xff) as u8);
    }
}
#[no_mangle]
pub extern fn m68k_write_memory_16(address: u32, value: u32) {
    unsafe {
        let op = Operation::WriteWord(musashi_address_space, address, value);
        register_op(op);
        write_musashi_byte(address+0, ((value & 0xff00) >> 8) as u8);
        write_musashi_byte(address+1, ((value & 0x00ff) >> 0) as u8);
    }
}
#[no_mangle]
pub extern fn m68k_write_memory_32(address: u32, value: u32) {
    unsafe {
        let op = Operation::WriteLong(musashi_address_space, address, value);
        register_op(op);
        write_musashi_byte(address+0, ((value & 0xff000000) >> 24) as u8);
        write_musashi_byte(address+1, ((value & 0x00ff0000) >> 16) as u8);
        write_musashi_byte(address+2, ((value & 0x0000ff00) >>  8) as u8);
        write_musashi_byte(address+3, ((value & 0x000000ff) >>  0) as u8);
    }
}
// read uninitialized bytes from initializer instead
unsafe fn read_initializer(address: u32) -> u8 {
    let shift = match address % 4 {
        0 => 24,
        1 => 16,
        2 =>  8,
        _ =>  0,
    };
    ((musashi_memory_initializer >> shift) & 0xFF) as u8
}
unsafe fn find_musashi_location(address: u32) -> Option<usize> {
    for i in 0..musashi_locations_used {
        if musashi_memory_location[i as usize] == address {
            return Some(i as usize)
        }
    };
    None
}
unsafe fn read_musashi_byte(address: u32) -> u8 {
    if let Some(index) = find_musashi_location(address) {
        musashi_memory_data[index]
    } else {
        read_initializer(address)
    }
}
unsafe fn write_musashi_byte(address: u32, data: u8) {
    let write_differs_from_initializer = data != read_initializer(address);
    if write_differs_from_initializer {
        if let Some(index) = find_musashi_location(address) {
            musashi_memory_data[index] = data;
        } else {
            musashi_memory_location[musashi_locations_used] = address;
            musashi_memory_data[musashi_locations_used] = data;
            musashi_locations_used += 1;
        }
    }
}

#[no_mangle]
pub extern fn cpu_pulse_reset() {panic!("pr")}
#[no_mangle]
pub extern fn cpu_long_branch() {}
#[no_mangle]
pub extern fn m68k_set_fc(fc: u32) {
    unsafe {
        musashi_address_space = match fc {
            1 => USER_DATA,
            2 => USER_PROGRAM,
            5 => SUPERVISOR_DATA,
            6 => SUPERVISOR_PROGRAM,
            _ => panic!("unknown fc: {}", fc),
        };
        // println!("set_fc {:?}", musashi_address_space);
    }
}
#[allow(unused_variables)]
#[no_mangle]
pub extern fn cpu_irq_ack(level: i32) -> i32 {panic!("ia")}
#[no_mangle]
pub extern fn cpu_instr_callback() {}

use std::ptr;

#[allow(unused_variables)]
pub fn experimental_communication() {
    let _mutex = MUSASHI_LOCK.lock().unwrap();

    unsafe {
        m68k_init();
        m68k_set_cpu_type(CpuType::M68000);
        m68k_set_reg(Register::D0, 123);
        println!("D0: {}", m68k_get_reg(ptr::null_mut(), Register::D0));
    }
}

#[allow(unused_variables)]
pub fn roundtrip_register(reg: Register, value: u32) -> u32 {
    let _mutex = MUSASHI_LOCK.lock().unwrap();

    unsafe {
        m68k_init();
        m68k_set_cpu_type(CpuType::M68000);
        m68k_set_reg(reg, value);
        m68k_get_reg(ptr::null_mut(), reg)
    }
}

use cpu::{Core, Cycles};

static REGS:[Register; 16] = [Register::D0, Register::D1, Register::D2, Register::D3, Register::D4, Register::D5, Register::D6, Register::D7, Register::A0, Register::A1, Register::A2, Register::A3, Register::A4, Register::A5, Register::A6, Register::A7];

fn get_ops() -> Vec<Operation> {
    let mut res: Vec<Operation> = vec![];
    unsafe {
        for i in 0..musashi_opcount {
            res.push(musashi_ops[i]);
        }
    }
    res
}

pub fn initialize_musashi(core: &mut Core, memory_initializer: u32) {
    // println!("initialize_musashi {:?}", thread::current());
    unsafe {
        initialize_musashi_memory(memory_initializer);
        m68k_init();
        m68k_set_cpu_type(CpuType::M68000);
        m68k_write_memory_32(0, core.ssp());
        m68k_write_memory_32(4, core.pc);
        m68k_pulse_reset();
        // Resetting opcount, because m68k_pulse_reset causes irrelevant
        // reads from 0x00000000 to set PC/SP, a jump to PC and
        // resetting of state. But we don't want to test those ops.
        musashi_opcount = 0;
        //m68k_set_reg(Register::PC, core.pc);
        m68k_set_reg(Register::USP, core.usp());
        // if SR clears S_FLAG then SSP <- A7, A7 <- USP
        m68k_set_reg(Register::SR, core.status_register() as u32);
        for (i, &reg) in REGS.iter().enumerate() {
            if i != 15 {
                m68k_set_reg(reg, core.dar[i]);
            }
        }
        // just copy diffs, as it takes too long to reset all 16MB
        for (addr, byte) in core.mem.diffs() {
            write_musashi_byte(addr, byte);
        }
    }
}

pub fn initialize_musashi_memory(initializer: u32) {
    unsafe {
        musashi_memory_initializer = initializer;
        musashi_opcount = 0;
        musashi_locations_used = 0;
        m68k_set_fc(SUPERVISOR_PROGRAM.fc());
    }
}
pub fn musashi_written_bytes() -> u16 {
    unsafe {
        musashi_locations_used as u16
    }
}
pub fn execute1(core: &mut Core) -> Cycles {
    // println!("execute1 mushashi {:?}", thread::current());
    unsafe {
        let cycle_count = m68k_execute(1);

        for (i, &reg) in REGS.iter().enumerate() {
            core.dar[i] = m68k_get_reg(ptr::null_mut(), reg);
        }
        core.pc = m68k_get_reg(ptr::null_mut(), Register::PC);
        core.sr_to_flags(m68k_get_reg(ptr::null_mut(), Register::SR) as u16);
        if core.s_flag > 0 {
            core.inactive_usp = m68k_get_reg(ptr::null_mut(), Register::USP);
            core.dar[15] = m68k_get_reg(ptr::null_mut(), Register::ISP);
        } else {
            core.dar[15] = m68k_get_reg(ptr::null_mut(), Register::USP);
            core.inactive_ssp = m68k_get_reg(ptr::null_mut(), Register::ISP);
        }

        Cycles(cycle_count)
    }
}

#[allow(unused_variables)]
pub fn reset_and_execute1(core: &mut Core, memory_initializer: u32) -> Cycles {
    initialize_musashi(core, memory_initializer);
    execute1(core)
}


// Talking to Musashi isn't thread-safe, and the tests are running
// threaded, which cause intermittent test failures unless serializing
// access using something like a mutex. Musashi functions are called in
// global/static context, and statics are not allowed to have
// destructors
use std::sync::{Arc, Mutex};
// using lazy_static! to work-around "statics are not allowed to have destructors [E0493]""
lazy_static! {
    static ref MUSASHI_LOCK: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    static ref QUICKCHECK_LOCK: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ram::SUPERVISOR_PROGRAM;
    use super::MUSASHI_LOCK;
    use super::QUICKCHECK_LOCK;
    use ram::{Operation, AddressBus};
    use cpu::Core;

    extern crate quickcheck;
    use self::quickcheck::*;
    #[derive(Copy, Clone, Debug, PartialEq)]
    struct Bitpattern(u32);
    impl Arbitrary for Bitpattern {
        fn arbitrary<G: Gen>(g: &mut G) -> Bitpattern {
            // when size 256, could generate any 32 bit pattern
            let nonuniform: u32 = Arbitrary::arbitrary(g);
            // increase likelihood of returning all zeros to 1:32
            if nonuniform < 8 {return Bitpattern(0)}
            // increase likelihood of returning all ones to 1:32
            if nonuniform < 16 {return Bitpattern(0xffffffff)}
            let i1: u32 = Arbitrary::arbitrary(g);
            let i2: u32 = Arbitrary::arbitrary(g);
            let i3: u32 = Arbitrary::arbitrary(g);
            let i4: u32 = Arbitrary::arbitrary(g);
            let sum: u32 = (i1 << 24) | (i2 << 16) | (i3 << 8) | i4;
            Bitpattern(sum)
        }
        fn shrink(&self) -> Box<Iterator<Item=Self>> {
            match *self {
                Bitpattern(x) => {
                    let xs = x.shrink(); // should shrink Bitpattern by clearing bits, not setting new ones
                    let tagged = xs //.inspect(|x|println!("{}", x))
                    .map(Bitpattern);
                    Box::new(tagged)
                }
            }
        }
    }

    impl Arbitrary for Register {
        fn arbitrary<G: Gen>(g: &mut G) -> Register {
            let regs = [Register::D0, Register::D1, Register::D2, Register::D3, Register::D4, Register::D5, Register::D6, Register::D7, Register::A0, Register::A1, Register::A2, Register::A3, Register::A4, Register::A5, Register::A6,
            Register::SR, // Register::A7, Register::SP, Register::PC
            ];
            //println!("{}",i);
            if let Some(&reg) = g.choose(&regs) {
                reg
            } else {
                unreachable!();
            }
        }
    }

    extern crate rand;

    use itertools::{Itertools, assert_equal};
    use cpu::ops::handlers::*;
    use super::get_ops;
    // struct OpSeq {
    //     mask: u32,
    //     matching: u32,
    //     current_op: u32,
    // }
    // impl OpSeq {
    //     fn new(mask: u32, matching: u32) -> OpSeq {
    //         OpSeq { mask: mask, matching: matching, current_op: 0 }
    //     }
    // }
    // impl Iterator for OpSeq {
    //     type Item = u32;
    //     fn next(&mut self) -> Option<u32> {
    //         if self.current_op == 0x10000 {
    //             None
    //         } else {
    //             while (self.current_op & self.mask) != self.matching && self.current_op < 0x10000 {
    //                 self.current_op += 1;
    //             }
    //             if self.current_op == 0x10000 {
    //                 return None;
    //             }
    //             let res = Some(self.current_op);
    //             self.current_op += 1;
    //             res
    //         }
    //     }
    // }

    fn opcodes(mask: u32, matching: u32) -> Vec<u16> {
        (matching..0x10000u32)
            .filter(|opcode| (opcode & mask) == matching)
            .map(|v|v as u16).collect::<Vec<u16>>()
    }
    macro_rules! opcodes {
        ($mask:expr , $matching:expr) => {($matching..0x10000).filter(|opcode| (opcode & $mask) == $matching)}
    }

    #[test]
    fn opcodes_from_mask_and_matching(){
        let mut opseq = Vec::new();
        opseq.extend(opcodes!(MASK_OUT_X_Y, OP_ABCD_8_RR));
        assert_eq!(64, opseq.len());
        let ops = opseq.iter().unique();
        assert_eq!(64, ops.count());
        if let Some(&min) = opseq.iter().min() {
            assert_eq!(0b1100000100000000, min);
        }
        if let Some(&max) = opseq.iter().max() {
            assert_eq!(0b1100111100000111, max);
        }
        for code in opseq.iter() {
            assert_eq!(OP_ABCD_8_RR, code & OP_ABCD_8_RR);
        }
    }

    static mut opcode_under_test: u16 = 0;

    fn hammer_cores_even_addresses(memory_pattern: Bitpattern, rs: Vec<(Register, Bitpattern)>) -> TestResult {
        let mem_mask = (2<<24)-2; // keep even
        hammer_cores_with(mem_mask, memory_pattern, rs)
    }
    fn hammer_cores(memory_pattern: Bitpattern, rs: Vec<(Register, Bitpattern)>) -> TestResult {
        let mem_mask = (2<<24)-1; // allow odd
        hammer_cores_with(mem_mask, memory_pattern, rs)
    }

    fn hammer_cores_with(mem_mask: u32, memory_pattern: Bitpattern, rs: Vec<(Register, Bitpattern)>) -> TestResult {
        let pc = 0x40;
        let mem = unsafe {
            [((opcode_under_test >> 8) & 0xff) as u8, (opcode_under_test & 0xff) as u8]
        };
        let Bitpattern(memory_initializer) = memory_pattern;
        let mut musashi = Core::new_mem_init(pc, &mem, memory_initializer & mem_mask);
        const STACK_MASK:u32 = (1024-16); // keep even
        musashi.inactive_ssp = 0x128;
        musashi.inactive_usp = 0x256;
        for r in 0..8 {
            musashi.dar[r] = 0;
            musashi.dar[8+r] = 0x128;
        }
        // set up RESET vector in memory
        let (ssp, pc) = (musashi.ssp(), musashi.pc);
        musashi.write_program_long(0, ssp).unwrap();
        musashi.write_program_long(4, pc).unwrap();
        for r in rs {
            match r {
                (Register::D0, Bitpattern(bp)) => musashi.dar[0] = bp,
                (Register::D1, Bitpattern(bp)) => musashi.dar[1] = bp,
                (Register::D2, Bitpattern(bp)) => musashi.dar[2] = bp,
                (Register::D3, Bitpattern(bp)) => musashi.dar[3] = bp,
                (Register::D4, Bitpattern(bp)) => musashi.dar[4] = bp,
                (Register::D5, Bitpattern(bp)) => musashi.dar[5] = bp,
                (Register::D6, Bitpattern(bp)) => musashi.dar[6] = bp,
                (Register::D7, Bitpattern(bp)) => musashi.dar[7] = bp,
                // must ensure Addresses are within musashi memory space!
                (Register::A0, Bitpattern(bp)) => musashi.dar[0+8] = bp & mem_mask,
                (Register::A1, Bitpattern(bp)) => musashi.dar[1+8] = bp & mem_mask,
                (Register::A2, Bitpattern(bp)) => musashi.dar[2+8] = bp & mem_mask,
                (Register::A3, Bitpattern(bp)) => musashi.dar[3+8] = bp & mem_mask,
                (Register::A4, Bitpattern(bp)) => musashi.dar[4+8] = bp & mem_mask,
                (Register::A5, Bitpattern(bp)) => musashi.dar[5+8] = bp & mem_mask,
                (Register::A6, Bitpattern(bp)) => musashi.dar[6+8] = bp & mem_mask,
                (Register::A7, Bitpattern(bp)) => musashi.dar[7+8] = bp & STACK_MASK + 8,
                (Register::USP, Bitpattern(bp)) => musashi.inactive_usp = bp & STACK_MASK + 8,
                (Register::SR, Bitpattern(bp)) => musashi.sr_to_flags(bp as u16),
                _ => {
                    panic!("No idea how to set {:?}", r.0)
                },
            }
        }
        let mut r68k = musashi.clone(); // so very self-aware!
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        let musashi_cycles = reset_and_execute1(&mut musashi, memory_initializer & mem_mask);
        let r68k_cycles = r68k.execute1();
        let res = assert_cores_equal(&musashi, &r68k);
        if !res {
            println!("discarding test");
            return TestResult::discard();
        }
        assert_eq!(musashi_cycles, r68k_cycles);
        TestResult::from_bool(res)
    }

    macro_rules! qc8 {
        ($opmask:ident, $opcode:ident, $fn_name:ident) => (qc!($opmask, $opcode, $fn_name, hammer_cores););
    }
    macro_rules! qc {
        ($opmask:ident, $opcode:ident, $fn_name:ident) => (qc!($opmask, $opcode, $fn_name, hammer_cores_even_addresses););
        ($opmask:ident, $opcode:ident, $fn_name:ident, $hammer:ident) => (
        #[test]
        #[ignore]
            fn $fn_name() {
            // Musashi isn't thread safe, and the construct with opcode_under_test
            // isn't either. :(
            let _mutex = QUICKCHECK_LOCK.lock().unwrap();
            // check for mask/opcode inconsistency
            assert!($opmask & $opcode == $opcode);
            for opcode in opcodes($opmask, $opcode)
            {
                println!("Will hammer {:016b}", opcode);
                unsafe {
                    // this is because I don't know how to make
                    // hammer_cores take the opcode as a parameter and
                    // we cannot simply use a closure either; see
                    // https://github.com/BurntSushi/quickcheck/issues/56
                    opcode_under_test = opcode;
                }
                QuickCheck::new()
                .gen(StdGen::new(rand::thread_rng(), 256))
                .tests(10)
                .quickcheck($hammer as fn(_, _) -> _);
            }
        })
    }

    qc8!(MASK_OUT_X_Y, OP_ABCD_8_RR, qc_abcd_rr);
    qc8!(MASK_OUT_X_Y, OP_ABCD_8_MM, qc_abcd_mm);

    qc8!(MASK_OUT_X_Y, OP_ADD_8_ER_DN, qc_add_8_er_dn);
    qc8!(MASK_OUT_X_Y, OP_ADD_8_ER_PI, qc_add_8_er_pi);
    qc8!(MASK_OUT_X_Y, OP_ADD_8_ER_PD, qc_add_8_er_pd);
    qc8!(MASK_OUT_X_Y, OP_ADD_8_ER_AI, qc_add_8_er_ai);
    qc8!(MASK_OUT_X_Y, OP_ADD_8_ER_DI, qc_add_8_er_di);
    qc8!(MASK_OUT_X_Y, OP_ADD_8_ER_IX, qc_add_8_er_ix);
    qc8!(MASK_OUT_X, OP_ADD_8_ER_AW, qc_add_8_er_aw);
    qc8!(MASK_OUT_X, OP_ADD_8_ER_AL, qc_add_8_er_al);
    qc8!(MASK_OUT_X, OP_ADD_8_ER_PCDI, qc_add_8_er_pcdi);
    qc8!(MASK_OUT_X, OP_ADD_8_ER_PCIX, qc_add_8_er_pcix);
    qc8!(MASK_OUT_X, OP_ADD_8_ER_IMM, qc_add_8_er_imm);

    qc8!(MASK_OUT_X_Y, OP_ADD_8_RE_PI, qc_add_8_re_pi);
    qc8!(MASK_OUT_X_Y, OP_ADD_8_RE_PD, qc_add_8_re_pd);
    qc8!(MASK_OUT_X_Y, OP_ADD_8_RE_AI, qc_add_8_re_ai);
    qc8!(MASK_OUT_X_Y, OP_ADD_8_RE_DI, qc_add_8_re_di);
    qc8!(MASK_OUT_X_Y, OP_ADD_8_RE_IX, qc_add_8_re_ix);
    qc8!(MASK_OUT_X, OP_ADD_8_RE_AW, qc_add_8_re_aw);
    qc8!(MASK_OUT_X, OP_ADD_8_RE_AL, qc_add_8_re_al);

    qc!(MASK_OUT_X_Y, OP_ADD_16_ER_DN, qc_add_16_er_dn);
    qc!(MASK_OUT_X_Y, OP_ADD_16_ER_AN, qc_add_16_er_an);
    qc!(MASK_OUT_X_Y, OP_ADD_16_ER_PI, qc_add_16_er_pi);
    qc!(MASK_OUT_X_Y, OP_ADD_16_ER_PD, qc_add_16_er_pd);
    qc!(MASK_OUT_X_Y, OP_ADD_16_ER_AI, qc_add_16_er_ai);
    qc!(MASK_OUT_X_Y, OP_ADD_16_ER_DI, qc_add_16_er_di);
    qc!(MASK_OUT_X_Y, OP_ADD_16_ER_IX, qc_add_16_er_ix);
    qc!(MASK_OUT_X, OP_ADD_16_ER_AW, qc_add_16_er_aw);
    qc!(MASK_OUT_X, OP_ADD_16_ER_AL, qc_add_16_er_al);
    qc!(MASK_OUT_X, OP_ADD_16_ER_PCDI, qc_add_16_er_pcdi);
    qc!(MASK_OUT_X, OP_ADD_16_ER_PCIX, qc_add_16_er_pcix);
    qc!(MASK_OUT_X, OP_ADD_16_ER_IMM, qc_add_16_er_imm);

    qc!(MASK_OUT_X_Y, OP_ADD_16_RE_PI, qc_add_16_re_pi);
    qc!(MASK_OUT_X_Y, OP_ADD_16_RE_PD, qc_add_16_re_pd);
    qc!(MASK_OUT_X_Y, OP_ADD_16_RE_AI, qc_add_16_re_ai);
    qc!(MASK_OUT_X_Y, OP_ADD_16_RE_DI, qc_add_16_re_di);
    qc!(MASK_OUT_X_Y, OP_ADD_16_RE_IX, qc_add_16_re_ix);
    qc!(MASK_OUT_X, OP_ADD_16_RE_AW, qc_add_16_re_aw);
    qc!(MASK_OUT_X, OP_ADD_16_RE_AL, qc_add_16_re_al);

    qc!(MASK_OUT_X_Y, OP_ADD_32_ER_DN, qc_add_32_er_dn);
    qc!(MASK_OUT_X_Y, OP_ADD_32_ER_AN, qc_add_32_er_an);
    qc!(MASK_OUT_X_Y, OP_ADD_32_ER_PI, qc_add_32_er_pi);
    qc!(MASK_OUT_X_Y, OP_ADD_32_ER_PD, qc_add_32_er_pd);
    qc!(MASK_OUT_X_Y, OP_ADD_32_ER_AI, qc_add_32_er_ai);
    qc!(MASK_OUT_X_Y, OP_ADD_32_ER_DI, qc_add_32_er_di);
    qc!(MASK_OUT_X_Y, OP_ADD_32_ER_IX, qc_add_32_er_ix);
    qc!(MASK_OUT_X, OP_ADD_32_ER_AW, qc_add_32_er_aw);
    qc!(MASK_OUT_X, OP_ADD_32_ER_AL, qc_add_32_er_al);
    qc!(MASK_OUT_X, OP_ADD_32_ER_PCDI, qc_add_32_er_pcdi);
    qc!(MASK_OUT_X, OP_ADD_32_ER_PCIX, qc_add_32_er_pcix);
    qc!(MASK_OUT_X, OP_ADD_32_ER_IMM, qc_add_32_er_imm);

    qc!(MASK_OUT_X_Y, OP_ADD_32_RE_PI, qc_add_32_re_pi);
    qc!(MASK_OUT_X_Y, OP_ADD_32_RE_PD, qc_add_32_re_pd);
    qc!(MASK_OUT_X_Y, OP_ADD_32_RE_AI, qc_add_32_re_ai);
    qc!(MASK_OUT_X_Y, OP_ADD_32_RE_DI, qc_add_32_re_di);
    qc!(MASK_OUT_X_Y, OP_ADD_32_RE_IX, qc_add_32_re_ix);
    qc!(MASK_OUT_X, OP_ADD_32_RE_AW, qc_add_32_re_aw);
    qc!(MASK_OUT_X, OP_ADD_32_RE_AL, qc_add_32_re_al);

    qc!(MASK_OUT_X_Y, OP_ADDA_16_DN, qc_adda_16_dn);
    qc!(MASK_OUT_X_Y, OP_ADDA_16_AN, qc_adda_16_an);
    qc!(MASK_OUT_X_Y, OP_ADDA_16_PI, qc_adda_16_pi);
    qc!(MASK_OUT_X_Y, OP_ADDA_16_PD, qc_adda_16_pd);
    qc!(MASK_OUT_X_Y, OP_ADDA_16_AI, qc_adda_16_ai);
    qc!(MASK_OUT_X_Y, OP_ADDA_16_DI, qc_adda_16_di);
    qc!(MASK_OUT_X_Y, OP_ADDA_16_IX, qc_adda_16_ix);
    qc!(MASK_OUT_X, OP_ADDA_16_AW, qc_adda_16_aw);
    qc!(MASK_OUT_X, OP_ADDA_16_AL, qc_adda_16_al);
    qc!(MASK_OUT_X, OP_ADDA_16_PCDI, qc_adda_16_pcdi);
    qc!(MASK_OUT_X, OP_ADDA_16_PCIX, qc_adda_16_pcix);
    qc!(MASK_OUT_X, OP_ADDA_16_IMM, qc_adda_16_imm);

    qc!(MASK_OUT_X_Y, OP_ADDA_32_DN, qc_adda_32_dn);
    qc!(MASK_OUT_X_Y, OP_ADDA_32_AN, qc_adda_32_an);
    qc!(MASK_OUT_X_Y, OP_ADDA_32_PI, qc_adda_32_pi);
    qc!(MASK_OUT_X_Y, OP_ADDA_32_PD, qc_adda_32_pd);
    qc!(MASK_OUT_X_Y, OP_ADDA_32_AI, qc_adda_32_ai);
    qc!(MASK_OUT_X_Y, OP_ADDA_32_DI, qc_adda_32_di);
    qc!(MASK_OUT_X_Y, OP_ADDA_32_IX, qc_adda_32_ix);
    qc!(MASK_OUT_X, OP_ADDA_32_AW, qc_adda_32_aw);
    qc!(MASK_OUT_X, OP_ADDA_32_AL, qc_adda_32_al);
    qc!(MASK_OUT_X, OP_ADDA_32_PCDI, qc_adda_32_pcdi);
    qc!(MASK_OUT_X, OP_ADDA_32_PCIX, qc_adda_32_pcix);
    qc!(MASK_OUT_X, OP_ADDA_32_IMM, qc_adda_32_imm);

    qc8!(MASK_OUT_Y, OP_ADDI_8_DN, qc_addi_8_dn);
    qc8!(MASK_OUT_Y, OP_ADDI_8_PI, qc_addi_8_pi);
    qc8!(MASK_OUT_Y, OP_ADDI_8_PD, qc_addi_8_pd);
    qc8!(MASK_OUT_Y, OP_ADDI_8_AI, qc_addi_8_ai);
    qc8!(MASK_OUT_Y, OP_ADDI_8_DI, qc_addi_8_di);
    qc8!(MASK_OUT_Y, OP_ADDI_8_IX, qc_addi_8_ix);
    qc8!(MASK_EXACT, OP_ADDI_8_AW, qc_addi_8_aw);
    qc8!(MASK_EXACT, OP_ADDI_8_AL, qc_addi_8_al);

    qc!(MASK_OUT_Y, OP_ADDI_16_DN, qc_addi_16_dn);
    qc!(MASK_OUT_Y, OP_ADDI_16_PI, qc_addi_16_pi);
    qc!(MASK_OUT_Y, OP_ADDI_16_PD, qc_addi_16_pd);
    qc!(MASK_OUT_Y, OP_ADDI_16_AI, qc_addi_16_ai);
    qc!(MASK_OUT_Y, OP_ADDI_16_DI, qc_addi_16_di);
    qc!(MASK_OUT_Y, OP_ADDI_16_IX, qc_addi_16_ix);
    qc!(MASK_EXACT, OP_ADDI_16_AW, qc_addi_16_aw);
    qc!(MASK_EXACT, OP_ADDI_16_AL, qc_addi_16_al);

    qc!(MASK_OUT_Y, OP_ADDI_32_DN, qc_addi_32_dn);
    qc!(MASK_OUT_Y, OP_ADDI_32_PI, qc_addi_32_pi);
    qc!(MASK_OUT_Y, OP_ADDI_32_PD, qc_addi_32_pd);
    qc!(MASK_OUT_Y, OP_ADDI_32_AI, qc_addi_32_ai);
    qc!(MASK_OUT_Y, OP_ADDI_32_DI, qc_addi_32_di);
    qc!(MASK_OUT_Y, OP_ADDI_32_IX, qc_addi_32_ix);
    qc!(MASK_EXACT, OP_ADDI_32_AW, qc_addi_32_aw);
    qc!(MASK_EXACT, OP_ADDI_32_AL, qc_addi_32_al);

    qc8!(MASK_OUT_X_Y, OP_ADDQ_8_DN, qc_addq_8_dn);
    qc8!(MASK_OUT_X_Y, OP_ADDQ_8_PI, qc_addq_8_pi);
    qc8!(MASK_OUT_X_Y, OP_ADDQ_8_PD, qc_addq_8_pd);
    qc8!(MASK_OUT_X_Y, OP_ADDQ_8_AI, qc_addq_8_ai);
    qc8!(MASK_OUT_X_Y, OP_ADDQ_8_DI, qc_addq_8_di);
    qc8!(MASK_OUT_X_Y, OP_ADDQ_8_IX, qc_addq_8_ix);
    qc8!(MASK_OUT_X, OP_ADDQ_8_AW, qc_addq_8_aw);
    qc8!(MASK_OUT_X, OP_ADDQ_8_AL, qc_addq_8_al);

    qc!(MASK_OUT_X_Y, OP_ADDQ_16_DN, qc_addq_16_dn);
    qc!(MASK_OUT_X_Y, OP_ADDQ_16_AN, qc_addq_16_an);
    qc!(MASK_OUT_X_Y, OP_ADDQ_16_PI, qc_addq_16_pi);
    qc!(MASK_OUT_X_Y, OP_ADDQ_16_PD, qc_addq_16_pd);
    qc!(MASK_OUT_X_Y, OP_ADDQ_16_AI, qc_addq_16_ai);
    qc!(MASK_OUT_X_Y, OP_ADDQ_16_DI, qc_addq_16_di);
    qc!(MASK_OUT_X_Y, OP_ADDQ_16_IX, qc_addq_16_ix);
    qc!(MASK_OUT_X, OP_ADDQ_16_AW, qc_addq_16_aw);
    qc!(MASK_OUT_X, OP_ADDQ_16_AL, qc_addq_16_al);

    qc!(MASK_OUT_X_Y, OP_ADDQ_32_DN, qc_addq_32_dn);
    qc!(MASK_OUT_X_Y, OP_ADDQ_32_AN, qc_addq_32_an);
    qc!(MASK_OUT_X_Y, OP_ADDQ_32_PI, qc_addq_32_pi);
    qc!(MASK_OUT_X_Y, OP_ADDQ_32_PD, qc_addq_32_pd);
    qc!(MASK_OUT_X_Y, OP_ADDQ_32_AI, qc_addq_32_ai);
    qc!(MASK_OUT_X_Y, OP_ADDQ_32_DI, qc_addq_32_di);
    qc!(MASK_OUT_X_Y, OP_ADDQ_32_IX, qc_addq_32_ix);
    qc!(MASK_OUT_X, OP_ADDQ_32_AW, qc_addq_32_aw);
    qc!(MASK_OUT_X, OP_ADDQ_32_AL, qc_addq_32_al);

    qc8!(MASK_OUT_X_Y, OP_ADDX_8_RR, qc_addx_8_rr);
    qc8!(MASK_OUT_X_Y, OP_ADDX_8_MM, qc_addx_8_mm);
    qc!(MASK_OUT_X_Y, OP_ADDX_16_RR, qc_addx_16_rr);
    qc!(MASK_OUT_X_Y, OP_ADDX_16_MM, qc_addx_16_mm);
    qc!(MASK_OUT_X_Y, OP_ADDX_32_RR, qc_addx_32_rr);
    qc!(MASK_OUT_X_Y, OP_ADDX_32_MM, qc_addx_32_mm);

    qc8!(MASK_OUT_X_Y, OP_AND_8_ER_DN, qc_and_8_er_dn);
    qc8!(MASK_OUT_X_Y, OP_AND_8_ER_PI, qc_and_8_er_pi);
    qc8!(MASK_OUT_X_Y, OP_AND_8_ER_PD, qc_and_8_er_pd);
    qc8!(MASK_OUT_X_Y, OP_AND_8_ER_AI, qc_and_8_er_ai);
    qc8!(MASK_OUT_X_Y, OP_AND_8_ER_DI, qc_and_8_er_di);
    qc8!(MASK_OUT_X_Y, OP_AND_8_ER_IX, qc_and_8_er_ix);
    qc8!(MASK_OUT_X, OP_AND_8_ER_AW, qc_and_8_er_aw);
    qc8!(MASK_OUT_X, OP_AND_8_ER_AL, qc_and_8_er_al);
    qc8!(MASK_OUT_X, OP_AND_8_ER_PCDI, qc_and_8_er_pcdi);
    qc8!(MASK_OUT_X, OP_AND_8_ER_PCIX, qc_and_8_er_pcix);
    qc8!(MASK_OUT_X, OP_AND_8_ER_IMM, qc_and_8_er_imm);

    qc8!(MASK_OUT_X_Y, OP_AND_8_RE_PI, qc_and_8_re_pi);
    qc8!(MASK_OUT_X_Y, OP_AND_8_RE_PD, qc_and_8_re_pd);
    qc8!(MASK_OUT_X_Y, OP_AND_8_RE_AI, qc_and_8_re_ai);
    qc8!(MASK_OUT_X_Y, OP_AND_8_RE_DI, qc_and_8_re_di);
    qc8!(MASK_OUT_X_Y, OP_AND_8_RE_IX, qc_and_8_re_ix);
    qc8!(MASK_OUT_X, OP_AND_8_RE_AW, qc_and_8_re_aw);
    qc8!(MASK_OUT_X, OP_AND_8_RE_AL, qc_and_8_re_al);

    qc!(MASK_OUT_X_Y, OP_AND_16_ER_DN, qc_and_16_er_dn);
    qc!(MASK_OUT_X_Y, OP_AND_16_ER_PI, qc_and_16_er_pi);
    qc!(MASK_OUT_X_Y, OP_AND_16_ER_PD, qc_and_16_er_pd);
    qc!(MASK_OUT_X_Y, OP_AND_16_ER_AI, qc_and_16_er_ai);
    qc!(MASK_OUT_X_Y, OP_AND_16_ER_DI, qc_and_16_er_di);
    qc!(MASK_OUT_X_Y, OP_AND_16_ER_IX, qc_and_16_er_ix);
    qc!(MASK_OUT_X, OP_AND_16_ER_AW, qc_and_16_er_aw);
    qc!(MASK_OUT_X, OP_AND_16_ER_AL, qc_and_16_er_al);
    qc!(MASK_OUT_X, OP_AND_16_ER_PCDI, qc_and_16_er_pcdi);
    qc!(MASK_OUT_X, OP_AND_16_ER_PCIX, qc_and_16_er_pcix);
    qc!(MASK_OUT_X, OP_AND_16_ER_IMM, qc_and_16_er_imm);

    qc!(MASK_OUT_X_Y, OP_AND_16_RE_PI, qc_and_16_re_pi);
    qc!(MASK_OUT_X_Y, OP_AND_16_RE_PD, qc_and_16_re_pd);
    qc!(MASK_OUT_X_Y, OP_AND_16_RE_AI, qc_and_16_re_ai);
    qc!(MASK_OUT_X_Y, OP_AND_16_RE_DI, qc_and_16_re_di);
    qc!(MASK_OUT_X_Y, OP_AND_16_RE_IX, qc_and_16_re_ix);
    qc!(MASK_OUT_X, OP_AND_16_RE_AW, qc_and_16_re_aw);
    qc!(MASK_OUT_X, OP_AND_16_RE_AL, qc_and_16_re_al);

    qc!(MASK_OUT_X_Y, OP_AND_32_ER_DN, qc_and_32_er_dn);
    qc!(MASK_OUT_X_Y, OP_AND_32_ER_PI, qc_and_32_er_pi);
    qc!(MASK_OUT_X_Y, OP_AND_32_ER_PD, qc_and_32_er_pd);
    qc!(MASK_OUT_X_Y, OP_AND_32_ER_AI, qc_and_32_er_ai);
    qc!(MASK_OUT_X_Y, OP_AND_32_ER_DI, qc_and_32_er_di);
    qc!(MASK_OUT_X_Y, OP_AND_32_ER_IX, qc_and_32_er_ix);
    qc!(MASK_OUT_X, OP_AND_32_ER_AW, qc_and_32_er_aw);
    qc!(MASK_OUT_X, OP_AND_32_ER_AL, qc_and_32_er_al);
    qc!(MASK_OUT_X, OP_AND_32_ER_PCDI, qc_and_32_er_pcdi);
    qc!(MASK_OUT_X, OP_AND_32_ER_PCIX, qc_and_32_er_pcix);
    qc!(MASK_OUT_X, OP_AND_32_ER_IMM, qc_and_32_er_imm);

    qc!(MASK_OUT_X_Y, OP_AND_32_RE_PI, qc_and_32_re_pi);
    qc!(MASK_OUT_X_Y, OP_AND_32_RE_PD, qc_and_32_re_pd);
    qc!(MASK_OUT_X_Y, OP_AND_32_RE_AI, qc_and_32_re_ai);
    qc!(MASK_OUT_X_Y, OP_AND_32_RE_DI, qc_and_32_re_di);
    qc!(MASK_OUT_X_Y, OP_AND_32_RE_IX, qc_and_32_re_ix);
    qc!(MASK_OUT_X, OP_AND_32_RE_AW, qc_and_32_re_aw);
    qc!(MASK_OUT_X, OP_AND_32_RE_AL, qc_and_32_re_al);

    qc8!(MASK_OUT_Y, OP_ANDI_8_DN, qc_andi_8_dn);
    qc8!(MASK_OUT_Y, OP_ANDI_8_PI, qc_andi_8_pi);
    qc8!(MASK_OUT_Y, OP_ANDI_8_PD, qc_andi_8_pd);
    qc8!(MASK_OUT_Y, OP_ANDI_8_AI, qc_andi_8_ai);
    qc8!(MASK_OUT_Y, OP_ANDI_8_DI, qc_andi_8_di);
    qc8!(MASK_OUT_Y, OP_ANDI_8_IX, qc_andi_8_ix);
    qc8!(MASK_EXACT, OP_ANDI_8_AW, qc_andi_8_aw);
    qc8!(MASK_EXACT, OP_ANDI_8_AL, qc_andi_8_al);

    qc!(MASK_OUT_Y, OP_ANDI_16_DN, qc_andi_16_dn);
    qc!(MASK_OUT_Y, OP_ANDI_16_PI, qc_andi_16_pi);
    qc!(MASK_OUT_Y, OP_ANDI_16_PD, qc_andi_16_pd);
    qc!(MASK_OUT_Y, OP_ANDI_16_AI, qc_andi_16_ai);
    qc!(MASK_OUT_Y, OP_ANDI_16_DI, qc_andi_16_di);
    qc!(MASK_OUT_Y, OP_ANDI_16_IX, qc_andi_16_ix);
    qc!(MASK_EXACT, OP_ANDI_16_AW, qc_andi_16_aw);
    qc!(MASK_EXACT, OP_ANDI_16_AL, qc_andi_16_al);

    qc!(MASK_OUT_Y, OP_ANDI_32_DN, qc_andi_32_dn);
    qc!(MASK_OUT_Y, OP_ANDI_32_PI, qc_andi_32_pi);
    qc!(MASK_OUT_Y, OP_ANDI_32_PD, qc_andi_32_pd);
    qc!(MASK_OUT_Y, OP_ANDI_32_AI, qc_andi_32_ai);
    qc!(MASK_OUT_Y, OP_ANDI_32_DI, qc_andi_32_di);
    qc!(MASK_OUT_Y, OP_ANDI_32_IX, qc_andi_32_ix);
    qc!(MASK_EXACT, OP_ANDI_32_AW, qc_andi_32_aw);
    qc!(MASK_EXACT, OP_ANDI_32_AL, qc_andi_32_al);

    qc!(MASK_EXACT, OP_ANDI_16_TOC, qc_andi_16_toc);
    qc!(MASK_EXACT, OP_ANDI_16_TOS, qc_andi_16_tos);

    qc8!(MASK_OUT_X_Y, OP_ASR_8_S, qc_asr_8_s);
    qc!(MASK_OUT_X_Y, OP_ASR_16_S, qc_asr_16_s);
    qc!(MASK_OUT_X_Y, OP_ASR_32_S, qc_asr_32_s);
    qc8!(MASK_OUT_X_Y, OP_ASR_8_R, qc_asr_8_r);
    qc!(MASK_OUT_X_Y, OP_ASR_16_R, qc_asr_16_r);
    qc!(MASK_OUT_X_Y, OP_ASR_32_R, qc_asr_32_r);

    qc8!(MASK_OUT_X_Y, OP_ASL_8_S, qc_asl_8_s);
    qc!(MASK_OUT_X_Y, OP_ASL_16_S, qc_asl_16_s);
    qc!(MASK_OUT_X_Y, OP_ASL_32_S, qc_asl_32_s);
    qc8!(MASK_OUT_X_Y, OP_ASL_8_R, qc_asl_8_r);
    qc!(MASK_OUT_X_Y, OP_ASL_16_R, qc_asl_16_r);
    qc!(MASK_OUT_X_Y, OP_ASL_32_R, qc_asl_32_r);

    qc!(MASK_OUT_Y, OP_ASL_16_AI, qc_asl_16_ai);
    qc!(MASK_OUT_Y, OP_ASL_16_PI, qc_asl_16_pi);
    qc!(MASK_OUT_Y, OP_ASL_16_PD, qc_asl_16_pd);
    qc!(MASK_OUT_Y, OP_ASL_16_DI, qc_asl_16_di);
    qc!(MASK_OUT_Y, OP_ASL_16_IX, qc_asl_16_ix);
    qc!(MASK_EXACT, OP_ASL_16_AW, qc_asl_16_aw);
    qc!(MASK_EXACT, OP_ASL_16_AL, qc_asl_16_al);

    qc!(MASK_OUT_Y, OP_ASR_16_AI, qc_asr_16_ai);
    qc!(MASK_OUT_Y, OP_ASR_16_PI, qc_asr_16_pi);
    qc!(MASK_OUT_Y, OP_ASR_16_PD, qc_asr_16_pd);
    qc!(MASK_OUT_Y, OP_ASR_16_DI, qc_asr_16_di);
    qc!(MASK_OUT_Y, OP_ASR_16_IX, qc_asr_16_ix);
    qc!(MASK_EXACT, OP_ASR_16_AW, qc_asr_16_aw);
    qc!(MASK_EXACT, OP_ASR_16_AL, qc_asr_16_al);

    const MASK_LOBYTE_QUICKER: u32 = MASK_LOBYTE + 0xe0;
    qc8!(MASK_LOBYTE_QUICKER, OP_BHI_8, qc_bhi_8);
    qc8!(MASK_LOBYTE_QUICKER, OP_BLS_8, qc_bls_8);
    qc8!(MASK_LOBYTE_QUICKER, OP_BCC_8, qc_bcc_8);
    qc8!(MASK_LOBYTE_QUICKER, OP_BCS_8, qc_bcs_8);
    qc8!(MASK_LOBYTE_QUICKER, OP_BNE_8, qc_bne_8);
    qc8!(MASK_LOBYTE_QUICKER, OP_BEQ_8, qc_beq_8);
    qc8!(MASK_LOBYTE_QUICKER, OP_BVC_8, qc_bvc_8);
    qc8!(MASK_LOBYTE_QUICKER, OP_BVS_8, qc_bvs_8);
    qc8!(MASK_LOBYTE_QUICKER, OP_BPL_8, qc_bpl_8);
    qc8!(MASK_LOBYTE_QUICKER, OP_BMI_8, qc_bmi_8);
    qc8!(MASK_LOBYTE_QUICKER, OP_BGE_8, qc_bge_8);
    qc8!(MASK_LOBYTE_QUICKER, OP_BLT_8, qc_blt_8);
    qc8!(MASK_LOBYTE_QUICKER, OP_BGT_8, qc_bgt_8);
    qc8!(MASK_LOBYTE_QUICKER, OP_BLE_8, qc_ble_8);
    qc8!(MASK_LOBYTE_QUICKER, OP_BRA_8, qc_bra_8);
    qc8!(MASK_LOBYTE_QUICKER, OP_BSR_8, qc_bsr_8);

    qc!(MASK_EXACT, OP_BHI_16, qc_bhi_16);
    qc!(MASK_EXACT, OP_BLS_16, qc_bls_16);
    qc!(MASK_EXACT, OP_BCC_16, qc_bcc_16);
    qc!(MASK_EXACT, OP_BCS_16, qc_bcs_16);
    qc!(MASK_EXACT, OP_BNE_16, qc_bne_16);
    qc!(MASK_EXACT, OP_BEQ_16, qc_beq_16);
    qc!(MASK_EXACT, OP_BVC_16, qc_bvc_16);
    qc!(MASK_EXACT, OP_BVS_16, qc_bvs_16);
    qc!(MASK_EXACT, OP_BPL_16, qc_bpl_16);
    qc!(MASK_EXACT, OP_BMI_16, qc_bmi_16);
    qc!(MASK_EXACT, OP_BGE_16, qc_bge_16);
    qc!(MASK_EXACT, OP_BLT_16, qc_blt_16);
    qc!(MASK_EXACT, OP_BGT_16, qc_bgt_16);
    qc!(MASK_EXACT, OP_BLE_16, qc_ble_16);
    qc!(MASK_EXACT, OP_BRA_16, qc_bra_16);
    qc!(MASK_EXACT, OP_BSR_16, qc_bsr_16);

    qc!(MASK_OUT_X_Y, OP_BCHG_32_R_DN, qc_bchg_32_r_dn);
    qc!(MASK_OUT_Y, OP_BCHG_32_S_DN, qc_bchg_32_s_dn);
    qc8!(MASK_OUT_X_Y, OP_BCHG_8_R_AI, qc_bchg_8_r_ai);
    qc8!(MASK_OUT_X_Y, OP_BCHG_8_R_PI, qc_bchg_8_r_pi);
    qc8!(MASK_OUT_X_Y, OP_BCHG_8_R_PD, qc_bchg_8_r_pd);
    qc8!(MASK_OUT_X_Y, OP_BCHG_8_R_DI, qc_bchg_8_r_di);
    qc8!(MASK_OUT_X_Y, OP_BCHG_8_R_IX, qc_bchg_8_r_ix);
    qc8!(MASK_OUT_X, OP_BCHG_8_R_AW, qc_bchg_8_r_aw);
    qc8!(MASK_OUT_X, OP_BCHG_8_R_AL, qc_bchg_8_r_al);
    qc8!(MASK_OUT_Y, OP_BCHG_8_S_AI, qc_bchg_8_s_ai);
    qc8!(MASK_OUT_Y, OP_BCHG_8_S_PI, qc_bchg_8_s_pi);
    qc8!(MASK_OUT_Y, OP_BCHG_8_S_PD, qc_bchg_8_s_pd);
    qc8!(MASK_OUT_Y, OP_BCHG_8_S_DI, qc_bchg_8_s_di);
    qc8!(MASK_OUT_Y, OP_BCHG_8_S_IX, qc_bchg_8_s_ix);
    qc8!(MASK_EXACT, OP_BCHG_8_S_AW, qc_bchg_8_s_aw);
    qc8!(MASK_EXACT, OP_BCHG_8_S_AL, qc_bchg_8_s_al);

    qc!(MASK_OUT_X_Y, OP_BCLR_32_R_DN, qc_bclr_32_r_dn);
    qc!(MASK_OUT_Y, OP_BCLR_32_S_DN, qc_bclr_32_s_dn);
    qc8!(MASK_OUT_X_Y, OP_BCLR_8_R_AI, qc_bclr_8_r_ai);
    qc8!(MASK_OUT_X_Y, OP_BCLR_8_R_PI, qc_bclr_8_r_pi);
    qc8!(MASK_OUT_X_Y, OP_BCLR_8_R_PD, qc_bclr_8_r_pd);
    qc8!(MASK_OUT_X_Y, OP_BCLR_8_R_DI, qc_bclr_8_r_di);
    qc8!(MASK_OUT_X_Y, OP_BCLR_8_R_IX, qc_bclr_8_r_ix);
    qc8!(MASK_OUT_X, OP_BCLR_8_R_AW, qc_bclr_8_r_aw);
    qc8!(MASK_OUT_X, OP_BCLR_8_R_AL, qc_bclr_8_r_al);
    qc8!(MASK_OUT_Y, OP_BCLR_8_S_AI, qc_bclr_8_s_ai);
    qc8!(MASK_OUT_Y, OP_BCLR_8_S_PI, qc_bclr_8_s_pi);
    qc8!(MASK_OUT_Y, OP_BCLR_8_S_PD, qc_bclr_8_s_pd);
    qc8!(MASK_OUT_Y, OP_BCLR_8_S_DI, qc_bclr_8_s_di);
    qc8!(MASK_OUT_Y, OP_BCLR_8_S_IX, qc_bclr_8_s_ix);
    qc8!(MASK_EXACT, OP_BCLR_8_S_AW, qc_bclr_8_s_aw);
    qc8!(MASK_EXACT, OP_BCLR_8_S_AL, qc_bclr_8_s_al);

    qc!(MASK_OUT_X_Y, OP_BSET_32_R_DN, qc_bset_32_r_dn);
    qc!(MASK_OUT_Y, OP_BSET_32_S_DN, qc_bset_32_s_dn);
    qc8!(MASK_OUT_X_Y, OP_BSET_8_R_AI, qc_bset_8_r_ai);
    qc8!(MASK_OUT_X_Y, OP_BSET_8_R_PI, qc_bset_8_r_pi);
    qc8!(MASK_OUT_X_Y, OP_BSET_8_R_PD, qc_bset_8_r_pd);
    qc8!(MASK_OUT_X_Y, OP_BSET_8_R_DI, qc_bset_8_r_di);
    qc8!(MASK_OUT_X_Y, OP_BSET_8_R_IX, qc_bset_8_r_ix);
    qc8!(MASK_OUT_X, OP_BSET_8_R_AW, qc_bset_8_r_aw);
    qc8!(MASK_OUT_X, OP_BSET_8_R_AL, qc_bset_8_r_al);
    qc8!(MASK_OUT_Y, OP_BSET_8_S_AI, qc_bset_8_s_ai);
    qc8!(MASK_OUT_Y, OP_BSET_8_S_PI, qc_bset_8_s_pi);
    qc8!(MASK_OUT_Y, OP_BSET_8_S_PD, qc_bset_8_s_pd);
    qc8!(MASK_OUT_Y, OP_BSET_8_S_DI, qc_bset_8_s_di);
    qc8!(MASK_OUT_Y, OP_BSET_8_S_IX, qc_bset_8_s_ix);
    qc8!(MASK_EXACT, OP_BSET_8_S_AW, qc_bset_8_s_aw);
    qc8!(MASK_EXACT, OP_BSET_8_S_AL, qc_bset_8_s_al);

    qc!(MASK_OUT_X_Y, OP_BTST_32_R_DN, qc_btst_32_r_dn);
    qc!(MASK_OUT_Y, OP_BTST_32_S_DN, qc_btst_32_s_dn);
    qc8!(MASK_OUT_X_Y, OP_BTST_8_R_AI, qc_btst_8_r_ai);
    qc8!(MASK_OUT_X_Y, OP_BTST_8_R_PI, qc_btst_8_r_pi);
    qc8!(MASK_OUT_X_Y, OP_BTST_8_R_PD, qc_btst_8_r_pd);
    qc8!(MASK_OUT_X_Y, OP_BTST_8_R_DI, qc_btst_8_r_di);
    qc8!(MASK_OUT_X_Y, OP_BTST_8_R_IX, qc_btst_8_r_ix);
    qc8!(MASK_OUT_X, OP_BTST_8_R_AW, qc_btst_8_r_aw);
    qc8!(MASK_OUT_X, OP_BTST_8_R_AL, qc_btst_8_r_al);
    qc8!(MASK_OUT_Y, OP_BTST_8_S_AI, qc_btst_8_s_ai);
    qc8!(MASK_OUT_Y, OP_BTST_8_S_PI, qc_btst_8_s_pi);
    qc8!(MASK_OUT_Y, OP_BTST_8_S_PD, qc_btst_8_s_pd);
    qc8!(MASK_OUT_Y, OP_BTST_8_S_DI, qc_btst_8_s_di);
    qc8!(MASK_OUT_Y, OP_BTST_8_S_IX, qc_btst_8_s_ix);
    qc8!(MASK_EXACT, OP_BTST_8_S_AW, qc_btst_8_s_aw);
    qc8!(MASK_EXACT, OP_BTST_8_S_AL, qc_btst_8_s_al);

    qc!(MASK_OUT_X_Y, OP_CHK_16_AI, qc_chk_16_ai);
    qc!(MASK_OUT_X, OP_CHK_16_AL, qc_chk_16_al);
    qc!(MASK_OUT_X, OP_CHK_16_AW, qc_chk_16_aw);
    qc!(MASK_OUT_X_Y, OP_CHK_16_DN, qc_chk_16_dn);
    qc!(MASK_OUT_X_Y, OP_CHK_16_DI, qc_chk_16_di);
    qc!(MASK_OUT_X, OP_CHK_16_IMM, qc_chk_16_imm);
    qc!(MASK_OUT_X_Y, OP_CHK_16_IX, qc_chk_16_ix);
    qc!(MASK_OUT_X, OP_CHK_16_PCDI, qc_chk_16_pcdi);
    qc!(MASK_OUT_X, OP_CHK_16_PCIX, qc_chk_16_pcix);
    qc!(MASK_OUT_X_Y, OP_CHK_16_PD, qc_chk_16_pd);
    qc!(MASK_OUT_X_Y, OP_CHK_16_PI, qc_chk_16_pi);

    qc8!(MASK_OUT_Y, OP_CLR_8_DN, qc_clr_8_dn);
    qc8!(MASK_OUT_Y, OP_CLR_8_AI, qc_clr_8_ai);
    qc8!(MASK_OUT_Y, OP_CLR_8_PI, qc_clr_8_pi);
    qc8!(MASK_OUT_Y, OP_CLR_8_PD, qc_clr_8_pd);
    qc8!(MASK_OUT_Y, OP_CLR_8_DI, qc_clr_8_di);
    qc8!(MASK_OUT_Y, OP_CLR_8_IX, qc_clr_8_ix);
    qc8!(MASK_EXACT, OP_CLR_8_AW, qc_clr_8_aw);
    qc8!(MASK_EXACT, OP_CLR_8_AL, qc_clr_8_al);

    qc!(MASK_OUT_Y, OP_CLR_16_DN, qc_clr_16_dn);
    qc!(MASK_OUT_Y, OP_CLR_16_AI, qc_clr_16_ai);
    qc!(MASK_OUT_Y, OP_CLR_16_PI, qc_clr_16_pi);
    qc!(MASK_OUT_Y, OP_CLR_16_PD, qc_clr_16_pd);
    qc!(MASK_OUT_Y, OP_CLR_16_DI, qc_clr_16_di);
    qc!(MASK_OUT_Y, OP_CLR_16_IX, qc_clr_16_ix);
    qc!(MASK_EXACT, OP_CLR_16_AW, qc_clr_16_aw);
    qc!(MASK_EXACT, OP_CLR_16_AL, qc_clr_16_al);

    qc!(MASK_OUT_Y, OP_CLR_32_DN, qc_clr_32_dn);
    qc!(MASK_OUT_Y, OP_CLR_32_AI, qc_clr_32_ai);
    qc!(MASK_OUT_Y, OP_CLR_32_PI, qc_clr_32_pi);
    qc!(MASK_OUT_Y, OP_CLR_32_PD, qc_clr_32_pd);
    qc!(MASK_OUT_Y, OP_CLR_32_DI, qc_clr_32_di);
    qc!(MASK_OUT_Y, OP_CLR_32_IX, qc_clr_32_ix);
    qc!(MASK_EXACT, OP_CLR_32_AW, qc_clr_32_aw);
    qc!(MASK_EXACT, OP_CLR_32_AL, qc_clr_32_al);

    qc8!(MASK_OUT_X_Y, OP_CMP_8_DN, qc_cmp_8_dn);
    qc8!(MASK_OUT_X_Y, OP_CMP_8_AI, qc_cmp_8_ai);
    qc8!(MASK_OUT_X_Y, OP_CMP_8_PI, qc_cmp_8_pi);
    qc8!(MASK_OUT_X_Y, OP_CMP_8_PD, qc_cmp_8_pd);
    qc8!(MASK_OUT_X_Y, OP_CMP_8_DI, qc_cmp_8_di);
    qc8!(MASK_OUT_X_Y, OP_CMP_8_IX, qc_cmp_8_ix);
    qc8!(MASK_OUT_X, OP_CMP_8_AW, qc_cmp_8_aw);
    qc8!(MASK_OUT_X, OP_CMP_8_AL, qc_cmp_8_al);
    qc8!(MASK_OUT_X, OP_CMP_8_PCDI, qc_cmp_8_pcdi);
    qc8!(MASK_OUT_X, OP_CMP_8_PCIX, qc_cmp_8_pcix);
    qc8!(MASK_OUT_X, OP_CMP_8_IMM, qc_cmp_8_imm);

    qc!(MASK_OUT_X_Y, OP_CMP_16_DN, qc_cmp_16_dn);
    qc!(MASK_OUT_X_Y, OP_CMP_16_AN, qc_cmp_16_an);
    qc!(MASK_OUT_X_Y, OP_CMP_16_AI, qc_cmp_16_ai);
    qc!(MASK_OUT_X_Y, OP_CMP_16_PI, qc_cmp_16_pi);
    qc!(MASK_OUT_X_Y, OP_CMP_16_PD, qc_cmp_16_pd);
    qc!(MASK_OUT_X_Y, OP_CMP_16_DI, qc_cmp_16_di);
    qc!(MASK_OUT_X_Y, OP_CMP_16_IX, qc_cmp_16_ix);
    qc!(MASK_OUT_X, OP_CMP_16_AW, qc_cmp_16_aw);
    qc!(MASK_OUT_X, OP_CMP_16_AL, qc_cmp_16_al);
    qc!(MASK_OUT_X, OP_CMP_16_PCDI, qc_cmp_16_pcdi);
    qc!(MASK_OUT_X, OP_CMP_16_PCIX, qc_cmp_16_pcix);
    qc!(MASK_OUT_X, OP_CMP_16_IMM, qc_cmp_16_imm);

    qc!(MASK_OUT_X_Y, OP_CMP_32_DN, qc_cmp_32_dn);
    qc!(MASK_OUT_X_Y, OP_CMP_32_AN, qc_cmp_32_an);
    qc!(MASK_OUT_X_Y, OP_CMP_32_AI, qc_cmp_32_ai);
    qc!(MASK_OUT_X_Y, OP_CMP_32_PI, qc_cmp_32_pi);
    qc!(MASK_OUT_X_Y, OP_CMP_32_PD, qc_cmp_32_pd);
    qc!(MASK_OUT_X_Y, OP_CMP_32_DI, qc_cmp_32_di);
    qc!(MASK_OUT_X_Y, OP_CMP_32_IX, qc_cmp_32_ix);
    qc!(MASK_OUT_X, OP_CMP_32_AW, qc_cmp_32_aw);
    qc!(MASK_OUT_X, OP_CMP_32_AL, qc_cmp_32_al);
    qc!(MASK_OUT_X, OP_CMP_32_PCDI, qc_cmp_32_pcdi);
    qc!(MASK_OUT_X, OP_CMP_32_PCIX, qc_cmp_32_pcix);
    qc!(MASK_OUT_X, OP_CMP_32_IMM, qc_cmp_32_imm);

    qc!(MASK_OUT_X_Y, OP_CMPA_16_DN, qc_cmpa_16_dn);
    qc!(MASK_OUT_X_Y, OP_CMPA_16_AN, qc_cmpa_16_an);
    qc!(MASK_OUT_X_Y, OP_CMPA_16_PI, qc_cmpa_16_pi);
    qc!(MASK_OUT_X_Y, OP_CMPA_16_PD, qc_cmpa_16_pd);
    qc!(MASK_OUT_X_Y, OP_CMPA_16_AI, qc_cmpa_16_ai);
    qc!(MASK_OUT_X_Y, OP_CMPA_16_DI, qc_cmpa_16_di);
    qc!(MASK_OUT_X_Y, OP_CMPA_16_IX, qc_cmpa_16_ix);
    qc!(MASK_OUT_X, OP_CMPA_16_AW, qc_cmpa_16_aw);
    qc!(MASK_OUT_X, OP_CMPA_16_AL, qc_cmpa_16_al);
    qc!(MASK_OUT_X, OP_CMPA_16_PCDI, qc_cmpa_16_pcdi);
    qc!(MASK_OUT_X, OP_CMPA_16_PCIX, qc_cmpa_16_pcix);
    qc!(MASK_OUT_X, OP_CMPA_16_IMM, qc_cmpa_16_imm);

    qc!(MASK_OUT_X_Y, OP_CMPA_32_DN, qc_cmpa_32_dn);
    qc!(MASK_OUT_X_Y, OP_CMPA_32_AN, qc_cmpa_32_an);
    qc!(MASK_OUT_X_Y, OP_CMPA_32_PI, qc_cmpa_32_pi);
    qc!(MASK_OUT_X_Y, OP_CMPA_32_PD, qc_cmpa_32_pd);
    qc!(MASK_OUT_X_Y, OP_CMPA_32_AI, qc_cmpa_32_ai);
    qc!(MASK_OUT_X_Y, OP_CMPA_32_DI, qc_cmpa_32_di);
    qc!(MASK_OUT_X_Y, OP_CMPA_32_IX, qc_cmpa_32_ix);
    qc!(MASK_OUT_X, OP_CMPA_32_AW, qc_cmpa_32_aw);
    qc!(MASK_OUT_X, OP_CMPA_32_AL, qc_cmpa_32_al);
    qc!(MASK_OUT_X, OP_CMPA_32_PCDI, qc_cmpa_32_pcdi);
    qc!(MASK_OUT_X, OP_CMPA_32_PCIX, qc_cmpa_32_pcix);
    qc!(MASK_OUT_X, OP_CMPA_32_IMM, qc_cmpa_32_imm);

    qc8!(MASK_OUT_Y, OP_CMPI_8_DN, qc_cmpi_8_dn);
    qc8!(MASK_OUT_Y, OP_CMPI_8_AI, qc_cmpi_8_ai);
    qc8!(MASK_OUT_Y, OP_CMPI_8_PI, qc_cmpi_8_pi);
    qc8!(MASK_OUT_Y, OP_CMPI_8_PD, qc_cmpi_8_pd);
    qc8!(MASK_OUT_Y, OP_CMPI_8_DI, qc_cmpi_8_di);
    qc8!(MASK_OUT_Y, OP_CMPI_8_IX, qc_cmpi_8_ix);
    qc8!(MASK_EXACT, OP_CMPI_8_AW, qc_cmpi_8_aw);
    qc8!(MASK_EXACT, OP_CMPI_8_AL, qc_cmpi_8_al);

    qc!(MASK_OUT_Y, OP_CMPI_16_DN, qc_cmpi_16_dn);
    qc!(MASK_OUT_Y, OP_CMPI_16_AI, qc_cmpi_16_ai);
    qc!(MASK_OUT_Y, OP_CMPI_16_PI, qc_cmpi_16_pi);
    qc!(MASK_OUT_Y, OP_CMPI_16_PD, qc_cmpi_16_pd);
    qc!(MASK_OUT_Y, OP_CMPI_16_DI, qc_cmpi_16_di);
    qc!(MASK_OUT_Y, OP_CMPI_16_IX, qc_cmpi_16_ix);
    qc!(MASK_EXACT, OP_CMPI_16_AW, qc_cmpi_16_aw);
    qc!(MASK_EXACT, OP_CMPI_16_AL, qc_cmpi_16_al);

    qc!(MASK_OUT_Y, OP_CMPI_32_DN, qc_cmpi_32_dn);
    qc!(MASK_OUT_Y, OP_CMPI_32_AI, qc_cmpi_32_ai);
    qc!(MASK_OUT_Y, OP_CMPI_32_PI, qc_cmpi_32_pi);
    qc!(MASK_OUT_Y, OP_CMPI_32_PD, qc_cmpi_32_pd);
    qc!(MASK_OUT_Y, OP_CMPI_32_DI, qc_cmpi_32_di);
    qc!(MASK_OUT_Y, OP_CMPI_32_IX, qc_cmpi_32_ix);
    qc!(MASK_EXACT, OP_CMPI_32_AW, qc_cmpi_32_aw);
    qc!(MASK_EXACT, OP_CMPI_32_AL, qc_cmpi_32_al);

    qc8!(MASK_OUT_X_Y, OP_CMPM_8, qc_cmpm_8);
    qc!(MASK_OUT_X_Y, OP_CMPM_16, qc_cmpm_16);
    qc!(MASK_OUT_X_Y, OP_CMPM_32, qc_cmpm_32);

    // Put qc for DBcc here
    qc!(MASK_OUT_Y, OP_DBT_16, qc_dbt_16);
    qc!(MASK_OUT_Y, OP_DBF_16, qc_dbf_16);
    qc!(MASK_OUT_Y, OP_DBHI_16, qc_dbhi_16);
    qc!(MASK_OUT_Y, OP_DBLS_16, qc_dbls_16);
    qc!(MASK_OUT_Y, OP_DBCC_16, qc_dbcc_16);
    qc!(MASK_OUT_Y, OP_DBCS_16, qc_dbcs_16);
    qc!(MASK_OUT_Y, OP_DBNE_16, qc_dbne_16);
    qc!(MASK_OUT_Y, OP_DBEQ_16, qc_dbeq_16);
    qc!(MASK_OUT_Y, OP_DBVC_16, qc_dbvc_16);
    qc!(MASK_OUT_Y, OP_DBVS_16, qc_dbvs_16);
    qc!(MASK_OUT_Y, OP_DBPL_16, qc_dbpl_16);
    qc!(MASK_OUT_Y, OP_DBMI_16, qc_dbmi_16);
    qc!(MASK_OUT_Y, OP_DBGE_16, qc_dbge_16);
    qc!(MASK_OUT_Y, OP_DBLT_16, qc_dblt_16);
    qc!(MASK_OUT_Y, OP_DBGT_16, qc_dbgt_16);
    qc!(MASK_OUT_Y, OP_DBLE_16, qc_dble_16);

    // Put qc for DIVS here
    qc!(MASK_OUT_X_Y, OP_DIVS_16_AI, qc_divs_16_ai);
    qc!(MASK_OUT_X, OP_DIVS_16_AL, qc_divs_16_al);
    qc!(MASK_OUT_X, OP_DIVS_16_AW, qc_divs_16_aw);
    qc!(MASK_OUT_X_Y, OP_DIVS_16_DN, qc_divs_16_dn);
    qc!(MASK_OUT_X_Y, OP_DIVS_16_DI, qc_divs_16_di);
    qc!(MASK_OUT_X, OP_DIVS_16_IMM, qc_divs_16_imm);
    qc!(MASK_OUT_X_Y, OP_DIVS_16_IX, qc_divs_16_ix);
    qc!(MASK_OUT_X, OP_DIVS_16_PCDI, qc_divs_16_pcdi);
    qc!(MASK_OUT_X, OP_DIVS_16_PCIX, qc_divs_16_pcix);
    qc!(MASK_OUT_X_Y, OP_DIVS_16_PD, qc_divs_16_pd);
    qc!(MASK_OUT_X_Y, OP_DIVS_16_PI, qc_divs_16_pi);

    // Put qc for DIVU here
    qc!(MASK_OUT_X_Y, OP_DIVU_16_AI, qc_divu_16_ai);
    qc!(MASK_OUT_X, OP_DIVU_16_AL, qc_divu_16_al);
    qc!(MASK_OUT_X, OP_DIVU_16_AW, qc_divu_16_aw);
    qc!(MASK_OUT_X_Y, OP_DIVU_16_DN, qc_divu_16_dn);
    qc!(MASK_OUT_X_Y, OP_DIVU_16_DI, qc_divu_16_di);
    qc!(MASK_OUT_X, OP_DIVU_16_IMM, qc_divu_16_imm);
    qc!(MASK_OUT_X_Y, OP_DIVU_16_IX, qc_divu_16_ix);
    qc!(MASK_OUT_X, OP_DIVU_16_PCDI, qc_divu_16_pcdi);
    qc!(MASK_OUT_X, OP_DIVU_16_PCIX, qc_divu_16_pcix);
    qc!(MASK_OUT_X_Y, OP_DIVU_16_PD, qc_divu_16_pd);
    qc!(MASK_OUT_X_Y, OP_DIVU_16_PI, qc_divu_16_pi);

    // Put qc for EOR, EORI, EORI to CCR and EORI to SR here
    qc8!(MASK_OUT_X_Y, OP_EOR_8_DN, qc_eor_8_dn);
    qc8!(MASK_OUT_X_Y, OP_EOR_8_AI, qc_eor_8_ai);
    qc8!(MASK_OUT_X_Y, OP_EOR_8_PI, qc_eor_8_pi);
    qc8!(MASK_OUT_X_Y, OP_EOR_8_PD, qc_eor_8_pd);
    qc8!(MASK_OUT_X_Y, OP_EOR_8_DI, qc_eor_8_di);
    qc8!(MASK_OUT_X_Y, OP_EOR_8_IX, qc_eor_8_ix);
    qc8!(MASK_OUT_X, OP_EOR_8_AW, qc_eor_8_aw);
    qc8!(MASK_OUT_X, OP_EOR_8_AL, qc_eor_8_al);

    qc!(MASK_OUT_X_Y, OP_EOR_16_DN, qc_eor_16_dn);
    qc!(MASK_OUT_X_Y, OP_EOR_16_AI, qc_eor_16_ai);
    qc!(MASK_OUT_X_Y, OP_EOR_16_PI, qc_eor_16_pi);
    qc!(MASK_OUT_X_Y, OP_EOR_16_PD, qc_eor_16_pd);
    qc!(MASK_OUT_X_Y, OP_EOR_16_DI, qc_eor_16_di);
    qc!(MASK_OUT_X_Y, OP_EOR_16_IX, qc_eor_16_ix);
    qc!(MASK_OUT_X, OP_EOR_16_AW, qc_eor_16_aw);
    qc!(MASK_OUT_X, OP_EOR_16_AL, qc_eor_16_al);

    qc!(MASK_OUT_X_Y, OP_EOR_32_DN, qc_eor_32_dn);
    qc!(MASK_OUT_X_Y, OP_EOR_32_AI, qc_eor_32_ai);
    qc!(MASK_OUT_X_Y, OP_EOR_32_PI, qc_eor_32_pi);
    qc!(MASK_OUT_X_Y, OP_EOR_32_PD, qc_eor_32_pd);
    qc!(MASK_OUT_X_Y, OP_EOR_32_DI, qc_eor_32_di);
    qc!(MASK_OUT_X_Y, OP_EOR_32_IX, qc_eor_32_ix);
    qc!(MASK_OUT_X, OP_EOR_32_AW, qc_eor_32_aw);
    qc!(MASK_OUT_X, OP_EOR_32_AL, qc_eor_32_al);

    qc8!(MASK_OUT_Y, OP_EORI_8_DN, qc_eori_8_dn);
    qc8!(MASK_OUT_Y, OP_EORI_8_AI, qc_eori_8_ai);
    qc8!(MASK_OUT_Y, OP_EORI_8_PI, qc_eori_8_pi);
    qc8!(MASK_OUT_Y, OP_EORI_8_PD, qc_eori_8_pd);
    qc8!(MASK_OUT_Y, OP_EORI_8_DI, qc_eori_8_di);
    qc8!(MASK_OUT_Y, OP_EORI_8_IX, qc_eori_8_ix);
    qc8!(MASK_EXACT, OP_EORI_8_AW, qc_eori_8_aw);
    qc8!(MASK_EXACT, OP_EORI_8_AL, qc_eori_8_al);

    qc!(MASK_OUT_Y, OP_EORI_16_DN, qc_eori_16_dn);
    qc!(MASK_OUT_Y, OP_EORI_16_AI, qc_eori_16_ai);
    qc!(MASK_OUT_Y, OP_EORI_16_PI, qc_eori_16_pi);
    qc!(MASK_OUT_Y, OP_EORI_16_PD, qc_eori_16_pd);
    qc!(MASK_OUT_Y, OP_EORI_16_DI, qc_eori_16_di);
    qc!(MASK_OUT_Y, OP_EORI_16_IX, qc_eori_16_ix);
    qc!(MASK_EXACT, OP_EORI_16_AW, qc_eori_16_aw);
    qc!(MASK_EXACT, OP_EORI_16_AL, qc_eori_16_al);

    qc!(MASK_OUT_Y, OP_EORI_32_DN, qc_eori_32_dn);
    qc!(MASK_OUT_Y, OP_EORI_32_AI, qc_eori_32_ai);
    qc!(MASK_OUT_Y, OP_EORI_32_PI, qc_eori_32_pi);
    qc!(MASK_OUT_Y, OP_EORI_32_PD, qc_eori_32_pd);
    qc!(MASK_OUT_Y, OP_EORI_32_DI, qc_eori_32_di);
    qc!(MASK_OUT_Y, OP_EORI_32_IX, qc_eori_32_ix);
    qc!(MASK_EXACT, OP_EORI_32_AW, qc_eori_32_aw);
    qc!(MASK_EXACT, OP_EORI_32_AL, qc_eori_32_al);

    qc!(MASK_EXACT, OP_EORI_16_TOC, qc_eori_16_toc);
    qc!(MASK_EXACT, OP_EORI_16_TOS, qc_eori_16_tos);

    // Put qc for EXG here
    qc!(MASK_OUT_X_Y, OP_EXG_32_DD, qc_exg_32_dd);
    qc!(MASK_OUT_X_Y, OP_EXG_32_AA, qc_exg_32_aa);
    qc!(MASK_OUT_X_Y, OP_EXG_32_DA, qc_exg_32_da);

    // Put qc for EXT here
    qc!(MASK_OUT_Y, OP_EXT_BW, qc_ext_bw);
    qc!(MASK_OUT_Y, OP_EXT_WL, qc_ext_wl);

    // Put qc for ILLEGAL here
    qc!(MASK_EXACT, OP_ILLEGAL, qc_illegal);

    // Put qc for JMP here
    qc!(MASK_OUT_Y, OP_JMP_32_AI, qc_jmp_32_ai);
    qc!(MASK_EXACT, OP_JMP_32_AL, qc_jmp_32_al);
    qc!(MASK_EXACT, OP_JMP_32_AW, qc_jmp_32_aw);
    qc!(MASK_OUT_Y, OP_JMP_32_DI, qc_jmp_32_di);
    qc!(MASK_OUT_Y, OP_JMP_32_IX, qc_jmp_32_ix);
    qc!(MASK_EXACT, OP_JMP_32_PCDI, qc_jmp_32_pcdi);
    qc!(MASK_EXACT, OP_JMP_32_PCIX, qc_jmp_32_pcix);

    // Put qc for JSR here
    qc!(MASK_OUT_Y, OP_JSR_32_AI, qc_jsr_32_ai);
    qc!(MASK_EXACT, OP_JSR_32_AL, qc_jsr_32_al);
    qc!(MASK_EXACT, OP_JSR_32_AW, qc_jsr_32_aw);
    qc!(MASK_OUT_Y, OP_JSR_32_DI, qc_jsr_32_di);
    qc!(MASK_OUT_Y, OP_JSR_32_IX, qc_jsr_32_ix);
    qc!(MASK_EXACT, OP_JSR_32_PCDI, qc_jsr_32_pcdi);
    qc!(MASK_EXACT, OP_JSR_32_PCIX, qc_jsr_32_pcix);

    // Put qc for LEA here
    qc!(MASK_OUT_Y, OP_LEA_32_AI, qc_lea_32_ai);
    qc!(MASK_EXACT, OP_LEA_32_AL, qc_lea_32_al);
    qc!(MASK_EXACT, OP_LEA_32_AW, qc_lea_32_aw);
    qc!(MASK_OUT_Y, OP_LEA_32_DI, qc_lea_32_di);
    qc!(MASK_OUT_Y, OP_LEA_32_IX, qc_lea_32_ix);
    qc!(MASK_EXACT, OP_LEA_32_PCDI, qc_lea_32_pcdi);
    qc!(MASK_EXACT, OP_LEA_32_PCIX, qc_lea_32_pcix);

    // Put qc for LINK here
    qc!(MASK_OUT_Y, OP_LINK_16, qc_link_16);

    // Put qc for LSL, LSR here
    qc8!(MASK_OUT_X_Y, OP_LSR_8_S, qc_lsr_8_s);
    qc!(MASK_OUT_X_Y, OP_LSR_16_S, qc_lsr_16_s);
    qc!(MASK_OUT_X_Y, OP_LSR_32_S, qc_lsr_32_s);
    qc8!(MASK_OUT_X_Y, OP_LSR_8_R, qc_lsr_8_r);
    qc!(MASK_OUT_X_Y, OP_LSR_16_R, qc_lsr_16_r);
    qc!(MASK_OUT_X_Y, OP_LSR_32_R, qc_lsr_32_r);

    qc8!(MASK_OUT_X_Y, OP_LSL_8_S, qc_lsl_8_s);
    qc!(MASK_OUT_X_Y, OP_LSL_16_S, qc_lsl_16_s);
    qc!(MASK_OUT_X_Y, OP_LSL_32_S, qc_lsl_32_s);
    qc8!(MASK_OUT_X_Y, OP_LSL_8_R, qc_lsl_8_r);
    qc!(MASK_OUT_X_Y, OP_LSL_16_R, qc_lsl_16_r);
    qc!(MASK_OUT_X_Y, OP_LSL_32_R, qc_lsl_32_r);

    qc!(MASK_OUT_Y, OP_LSL_16_AI, qc_lsl_16_ai);
    qc!(MASK_OUT_Y, OP_LSL_16_PI, qc_lsl_16_pi);
    qc!(MASK_OUT_Y, OP_LSL_16_PD, qc_lsl_16_pd);
    qc!(MASK_OUT_Y, OP_LSL_16_DI, qc_lsl_16_di);
    qc!(MASK_OUT_Y, OP_LSL_16_IX, qc_lsl_16_ix);
    qc!(MASK_EXACT, OP_LSL_16_AW, qc_lsl_16_aw);
    qc!(MASK_EXACT, OP_LSL_16_AL, qc_lsl_16_al);

    qc!(MASK_OUT_Y, OP_LSR_16_AI, qc_lsr_16_ai);
    qc!(MASK_OUT_Y, OP_LSR_16_PI, qc_lsr_16_pi);
    qc!(MASK_OUT_Y, OP_LSR_16_PD, qc_lsr_16_pd);
    qc!(MASK_OUT_Y, OP_LSR_16_DI, qc_lsr_16_di);
    qc!(MASK_OUT_Y, OP_LSR_16_IX, qc_lsr_16_ix);
    qc!(MASK_EXACT, OP_LSR_16_AW, qc_lsr_16_aw);
    qc!(MASK_EXACT, OP_LSR_16_AL, qc_lsr_16_al);

    // Put qc for MOVE here
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_DN_DN, qc_move_8_dn_dn);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_AI_DN, qc_move_8_ai_dn);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_PI_DN, qc_move_8_pi_dn);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_PD_DN, qc_move_8_pd_dn);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_DI_DN, qc_move_8_di_dn);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_IX_DN, qc_move_8_ix_dn);
    qc8!(MASK_OUT_Y,   OP_MOVE_8_AW_DN, qc_move_8_aw_dn);
    qc8!(MASK_OUT_Y,   OP_MOVE_8_AL_DN, qc_move_8_al_dn);

    qc8!(MASK_OUT_X_Y, OP_MOVE_8_DN_AI, qc_move_8_dn_ai);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_AI_AI, qc_move_8_ai_ai);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_PI_AI, qc_move_8_pi_ai);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_PD_AI, qc_move_8_pd_ai);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_DI_AI, qc_move_8_di_ai);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_IX_AI, qc_move_8_ix_ai);
    qc8!(MASK_OUT_Y,   OP_MOVE_8_AW_AI, qc_move_8_aw_ai);
    qc8!(MASK_OUT_Y,   OP_MOVE_8_AL_AI, qc_move_8_al_ai);

    qc8!(MASK_OUT_X_Y, OP_MOVE_8_DN_PI, qc_move_8_dn_pi);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_AI_PI, qc_move_8_ai_pi);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_PI_PI, qc_move_8_pi_pi);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_PD_PI, qc_move_8_pd_pi);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_DI_PI, qc_move_8_di_pi);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_IX_PI, qc_move_8_ix_pi);
    qc8!(MASK_OUT_Y,   OP_MOVE_8_AW_PI, qc_move_8_aw_pi);
    qc8!(MASK_OUT_Y,   OP_MOVE_8_AL_PI, qc_move_8_al_pi);

    qc8!(MASK_OUT_X_Y, OP_MOVE_8_DN_PD, qc_move_8_dn_pd);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_AI_PD, qc_move_8_ai_pd);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_PI_PD, qc_move_8_pi_pd);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_PD_PD, qc_move_8_pd_pd);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_DI_PD, qc_move_8_di_pd);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_IX_PD, qc_move_8_ix_pd);
    qc8!(MASK_OUT_Y,   OP_MOVE_8_AW_PD, qc_move_8_aw_pd);
    qc8!(MASK_OUT_Y,   OP_MOVE_8_AL_PD, qc_move_8_al_pd);

    qc8!(MASK_OUT_X_Y, OP_MOVE_8_DN_DI, qc_move_8_dn_di);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_AI_DI, qc_move_8_ai_di);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_PI_DI, qc_move_8_pi_di);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_PD_DI, qc_move_8_pd_di);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_DI_DI, qc_move_8_di_di);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_IX_DI, qc_move_8_ix_di);
    qc8!(MASK_OUT_Y,   OP_MOVE_8_AW_DI, qc_move_8_aw_di);
    qc8!(MASK_OUT_Y,   OP_MOVE_8_AL_DI, qc_move_8_al_di);

    qc8!(MASK_OUT_X_Y, OP_MOVE_8_DN_IX, qc_move_8_dn_ix);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_AI_IX, qc_move_8_ai_ix);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_PI_IX, qc_move_8_pi_ix);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_PD_IX, qc_move_8_pd_ix);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_DI_IX, qc_move_8_di_ix);
    qc8!(MASK_OUT_X_Y, OP_MOVE_8_IX_IX, qc_move_8_ix_ix);
    qc8!(MASK_OUT_Y,   OP_MOVE_8_AW_IX, qc_move_8_aw_ix);
    qc8!(MASK_OUT_Y,   OP_MOVE_8_AL_IX, qc_move_8_al_ix);

    qc8!(MASK_OUT_X, OP_MOVE_8_DN_AW, qc_move_8_dn_aw);
    qc8!(MASK_OUT_X, OP_MOVE_8_AI_AW, qc_move_8_ai_aw);
    qc8!(MASK_OUT_X, OP_MOVE_8_PI_AW, qc_move_8_pi_aw);
    qc8!(MASK_OUT_X, OP_MOVE_8_PD_AW, qc_move_8_pd_aw);
    qc8!(MASK_OUT_X, OP_MOVE_8_DI_AW, qc_move_8_di_aw);
    qc8!(MASK_OUT_X, OP_MOVE_8_IX_AW, qc_move_8_ix_aw);
    qc8!(MASK_EXACT, OP_MOVE_8_AW_AW, qc_move_8_aw_aw);
    qc8!(MASK_EXACT, OP_MOVE_8_AL_AW, qc_move_8_al_aw);

    qc8!(MASK_OUT_X, OP_MOVE_8_DN_AL, qc_move_8_dn_al);
    qc8!(MASK_OUT_X, OP_MOVE_8_AI_AL, qc_move_8_ai_al);
    qc8!(MASK_OUT_X, OP_MOVE_8_PI_AL, qc_move_8_pi_al);
    qc8!(MASK_OUT_X, OP_MOVE_8_PD_AL, qc_move_8_pd_al);
    qc8!(MASK_OUT_X, OP_MOVE_8_DI_AL, qc_move_8_di_al);
    qc8!(MASK_OUT_X, OP_MOVE_8_IX_AL, qc_move_8_ix_al);
    qc8!(MASK_EXACT, OP_MOVE_8_AW_AL, qc_move_8_aw_al);
    qc8!(MASK_EXACT, OP_MOVE_8_AL_AL, qc_move_8_al_al);

    qc8!(MASK_OUT_X, OP_MOVE_8_DN_PCDI, qc_move_8_dn_pcdi);
    qc8!(MASK_OUT_X, OP_MOVE_8_AI_PCDI, qc_move_8_ai_pcdi);
    qc8!(MASK_OUT_X, OP_MOVE_8_PI_PCDI, qc_move_8_pi_pcdi);
    qc8!(MASK_OUT_X, OP_MOVE_8_PD_PCDI, qc_move_8_pd_pcdi);
    qc8!(MASK_OUT_X, OP_MOVE_8_DI_PCDI, qc_move_8_di_pcdi);
    qc8!(MASK_OUT_X, OP_MOVE_8_IX_PCDI, qc_move_8_ix_pcdi);
    qc8!(MASK_EXACT, OP_MOVE_8_AW_PCDI, qc_move_8_aw_pcdi);
    qc8!(MASK_EXACT, OP_MOVE_8_AL_PCDI, qc_move_8_al_pcdi);

    qc8!(MASK_OUT_X, OP_MOVE_8_DN_PCIX, qc_move_8_dn_pcix);
    qc8!(MASK_OUT_X, OP_MOVE_8_AI_PCIX, qc_move_8_ai_pcix);
    qc8!(MASK_OUT_X, OP_MOVE_8_PI_PCIX, qc_move_8_pi_pcix);
    qc8!(MASK_OUT_X, OP_MOVE_8_PD_PCIX, qc_move_8_pd_pcix);
    qc8!(MASK_OUT_X, OP_MOVE_8_DI_PCIX, qc_move_8_di_pcix);
    qc8!(MASK_OUT_X, OP_MOVE_8_IX_PCIX, qc_move_8_ix_pcix);
    qc8!(MASK_EXACT, OP_MOVE_8_AW_PCIX, qc_move_8_aw_pcix);
    qc8!(MASK_EXACT, OP_MOVE_8_AL_PCIX, qc_move_8_al_pcix);

    qc8!(MASK_OUT_X, OP_MOVE_8_DN_IMM, qc_move_8_dn_imm);
    qc8!(MASK_OUT_X, OP_MOVE_8_AI_IMM, qc_move_8_ai_imm);
    qc8!(MASK_OUT_X, OP_MOVE_8_PI_IMM, qc_move_8_pi_imm);
    qc8!(MASK_OUT_X, OP_MOVE_8_PD_IMM, qc_move_8_pd_imm);
    qc8!(MASK_OUT_X, OP_MOVE_8_DI_IMM, qc_move_8_di_imm);
    qc8!(MASK_OUT_X, OP_MOVE_8_IX_IMM, qc_move_8_ix_imm);
    qc8!(MASK_EXACT, OP_MOVE_8_AW_IMM, qc_move_8_aw_imm);
    qc8!(MASK_EXACT, OP_MOVE_8_AL_IMM, qc_move_8_al_imm);

    qc!(MASK_OUT_X_Y, OP_MOVE_16_DN_DN, qc_move_16_dn_dn);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_AI_DN, qc_move_16_ai_dn);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_PI_DN, qc_move_16_pi_dn);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_PD_DN, qc_move_16_pd_dn);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_DI_DN, qc_move_16_di_dn);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_IX_DN, qc_move_16_ix_dn);
    qc!(MASK_OUT_Y,   OP_MOVE_16_AW_DN, qc_move_16_aw_dn);
    qc!(MASK_OUT_Y,   OP_MOVE_16_AL_DN, qc_move_16_al_dn);

    qc!(MASK_OUT_X_Y, OP_MOVE_16_DN_AI, qc_move_16_dn_ai);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_AI_AI, qc_move_16_ai_ai);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_PI_AI, qc_move_16_pi_ai);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_PD_AI, qc_move_16_pd_ai);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_DI_AI, qc_move_16_di_ai);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_IX_AI, qc_move_16_ix_ai);
    qc!(MASK_OUT_Y,   OP_MOVE_16_AW_AI, qc_move_16_aw_ai);
    qc!(MASK_OUT_Y,   OP_MOVE_16_AL_AI, qc_move_16_al_ai);

    qc!(MASK_OUT_X_Y, OP_MOVE_16_DN_PI, qc_move_16_dn_pi);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_AI_PI, qc_move_16_ai_pi);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_PI_PI, qc_move_16_pi_pi);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_PD_PI, qc_move_16_pd_pi);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_DI_PI, qc_move_16_di_pi);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_IX_PI, qc_move_16_ix_pi);
    qc!(MASK_OUT_Y,   OP_MOVE_16_AW_PI, qc_move_16_aw_pi);
    qc!(MASK_OUT_Y,   OP_MOVE_16_AL_PI, qc_move_16_al_pi);

    qc!(MASK_OUT_X_Y, OP_MOVE_16_DN_PD, qc_move_16_dn_pd);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_AI_PD, qc_move_16_ai_pd);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_PI_PD, qc_move_16_pi_pd);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_PD_PD, qc_move_16_pd_pd);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_DI_PD, qc_move_16_di_pd);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_IX_PD, qc_move_16_ix_pd);
    qc!(MASK_OUT_Y,   OP_MOVE_16_AW_PD, qc_move_16_aw_pd);
    qc!(MASK_OUT_Y,   OP_MOVE_16_AL_PD, qc_move_16_al_pd);

    qc!(MASK_OUT_X_Y, OP_MOVE_16_DN_DI, qc_move_16_dn_di);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_AI_DI, qc_move_16_ai_di);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_PI_DI, qc_move_16_pi_di);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_PD_DI, qc_move_16_pd_di);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_DI_DI, qc_move_16_di_di);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_IX_DI, qc_move_16_ix_di);
    qc!(MASK_OUT_Y,   OP_MOVE_16_AW_DI, qc_move_16_aw_di);
    qc!(MASK_OUT_Y,   OP_MOVE_16_AL_DI, qc_move_16_al_di);

    qc!(MASK_OUT_X_Y, OP_MOVE_16_DN_IX, qc_move_16_dn_ix);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_AI_IX, qc_move_16_ai_ix);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_PI_IX, qc_move_16_pi_ix);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_PD_IX, qc_move_16_pd_ix);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_DI_IX, qc_move_16_di_ix);
    qc!(MASK_OUT_X_Y, OP_MOVE_16_IX_IX, qc_move_16_ix_ix);
    qc!(MASK_OUT_Y,   OP_MOVE_16_AW_IX, qc_move_16_aw_ix);
    qc!(MASK_OUT_Y,   OP_MOVE_16_AL_IX, qc_move_16_al_ix);

    qc!(MASK_OUT_X, OP_MOVE_16_DN_AW, qc_move_16_dn_aw);
    qc!(MASK_OUT_X, OP_MOVE_16_AI_AW, qc_move_16_ai_aw);
    qc!(MASK_OUT_X, OP_MOVE_16_PI_AW, qc_move_16_pi_aw);
    qc!(MASK_OUT_X, OP_MOVE_16_PD_AW, qc_move_16_pd_aw);
    qc!(MASK_OUT_X, OP_MOVE_16_DI_AW, qc_move_16_di_aw);
    qc!(MASK_OUT_X, OP_MOVE_16_IX_AW, qc_move_16_ix_aw);
    qc!(MASK_EXACT, OP_MOVE_16_AW_AW, qc_move_16_aw_aw);
    qc!(MASK_EXACT, OP_MOVE_16_AL_AW, qc_move_16_al_aw);

    qc!(MASK_OUT_X, OP_MOVE_16_DN_AL, qc_move_16_dn_al);
    qc!(MASK_OUT_X, OP_MOVE_16_AI_AL, qc_move_16_ai_al);
    qc!(MASK_OUT_X, OP_MOVE_16_PI_AL, qc_move_16_pi_al);
    qc!(MASK_OUT_X, OP_MOVE_16_PD_AL, qc_move_16_pd_al);
    qc!(MASK_OUT_X, OP_MOVE_16_DI_AL, qc_move_16_di_al);
    qc!(MASK_OUT_X, OP_MOVE_16_IX_AL, qc_move_16_ix_al);
    qc!(MASK_EXACT, OP_MOVE_16_AW_AL, qc_move_16_aw_al);
    qc!(MASK_EXACT, OP_MOVE_16_AL_AL, qc_move_16_al_al);

    qc!(MASK_OUT_X, OP_MOVE_16_DN_PCDI, qc_move_16_dn_pcdi);
    qc!(MASK_OUT_X, OP_MOVE_16_AI_PCDI, qc_move_16_ai_pcdi);
    qc!(MASK_OUT_X, OP_MOVE_16_PI_PCDI, qc_move_16_pi_pcdi);
    qc!(MASK_OUT_X, OP_MOVE_16_PD_PCDI, qc_move_16_pd_pcdi);
    qc!(MASK_OUT_X, OP_MOVE_16_DI_PCDI, qc_move_16_di_pcdi);
    qc!(MASK_OUT_X, OP_MOVE_16_IX_PCDI, qc_move_16_ix_pcdi);
    qc!(MASK_EXACT, OP_MOVE_16_AW_PCDI, qc_move_16_aw_pcdi);
    qc!(MASK_EXACT, OP_MOVE_16_AL_PCDI, qc_move_16_al_pcdi);

    qc!(MASK_OUT_X, OP_MOVE_16_DN_PCIX, qc_move_16_dn_pcix);
    qc!(MASK_OUT_X, OP_MOVE_16_AI_PCIX, qc_move_16_ai_pcix);
    qc!(MASK_OUT_X, OP_MOVE_16_PI_PCIX, qc_move_16_pi_pcix);
    qc!(MASK_OUT_X, OP_MOVE_16_PD_PCIX, qc_move_16_pd_pcix);
    qc!(MASK_OUT_X, OP_MOVE_16_DI_PCIX, qc_move_16_di_pcix);
    qc!(MASK_OUT_X, OP_MOVE_16_IX_PCIX, qc_move_16_ix_pcix);
    qc!(MASK_EXACT, OP_MOVE_16_AW_PCIX, qc_move_16_aw_pcix);
    qc!(MASK_EXACT, OP_MOVE_16_AL_PCIX, qc_move_16_al_pcix);

    qc!(MASK_OUT_X, OP_MOVE_16_DN_IMM, qc_move_16_dn_imm);
    qc!(MASK_OUT_X, OP_MOVE_16_AI_IMM, qc_move_16_ai_imm);
    qc!(MASK_OUT_X, OP_MOVE_16_PI_IMM, qc_move_16_pi_imm);
    qc!(MASK_OUT_X, OP_MOVE_16_PD_IMM, qc_move_16_pd_imm);
    qc!(MASK_OUT_X, OP_MOVE_16_DI_IMM, qc_move_16_di_imm);
    qc!(MASK_OUT_X, OP_MOVE_16_IX_IMM, qc_move_16_ix_imm);
    qc!(MASK_EXACT, OP_MOVE_16_AW_IMM, qc_move_16_aw_imm);
    qc!(MASK_EXACT, OP_MOVE_16_AL_IMM, qc_move_16_al_imm);

    // Put qc for MOVEA here
    // Put qc for MOVE to CCR here
    // Put qc for MOVE from SR here
    // Put qc for MOVE to SR here
    // Put qc for MOVE USP here
    // Put qc for MOVEM here
    // Put qc for MOVEP here
    // Put qc for MOVEQ here
    // Put qc for MULS here
    // Put qc for MULU here
    // Put qc for NBCD here
    // Put qc for NEG here
    // Put qc for NEGX here
    // Put qc for NOP here
    // Put qc for NOT here
    // Put qc for OR here
    // Put qc for ORI here
    // Put qc for ORI to CCR here
    // Put qc for ORI to SR here
    // Put qc for PEA here
    // Put qc for RESET here
    // Put qc for ROL, ROR here
    qc8!(MASK_OUT_X_Y, OP_ROR_8_S, qc_ror_8_s);
    qc!(MASK_OUT_X_Y, OP_ROR_16_S, qc_ror_16_s);
    qc!(MASK_OUT_X_Y, OP_ROR_32_S, qc_ror_32_s);
    qc8!(MASK_OUT_X_Y, OP_ROR_8_R, qc_ror_8_r);
    qc!(MASK_OUT_X_Y, OP_ROR_16_R, qc_ror_16_r);
    qc!(MASK_OUT_X_Y, OP_ROR_32_R, qc_ror_32_r);

    qc8!(MASK_OUT_X_Y, OP_ROL_8_S, qc_rol_8_s);
    qc!(MASK_OUT_X_Y, OP_ROL_16_S, qc_rol_16_s);
    qc!(MASK_OUT_X_Y, OP_ROL_32_S, qc_rol_32_s);
    qc8!(MASK_OUT_X_Y, OP_ROL_8_R, qc_rol_8_r);
    qc!(MASK_OUT_X_Y, OP_ROL_16_R, qc_rol_16_r);
    qc!(MASK_OUT_X_Y, OP_ROL_32_R, qc_rol_32_r);

    qc!(MASK_OUT_Y, OP_ROL_16_AI, qc_rol_16_ai);
    qc!(MASK_OUT_Y, OP_ROL_16_PI, qc_rol_16_pi);
    qc!(MASK_OUT_Y, OP_ROL_16_PD, qc_rol_16_pd);
    qc!(MASK_OUT_Y, OP_ROL_16_DI, qc_rol_16_di);
    qc!(MASK_OUT_Y, OP_ROL_16_IX, qc_rol_16_ix);
    qc!(MASK_EXACT, OP_ROL_16_AW, qc_rol_16_aw);
    qc!(MASK_EXACT, OP_ROL_16_AL, qc_rol_16_al);

    qc!(MASK_OUT_Y, OP_ROR_16_AI, qc_ror_16_ai);
    qc!(MASK_OUT_Y, OP_ROR_16_PI, qc_ror_16_pi);
    qc!(MASK_OUT_Y, OP_ROR_16_PD, qc_ror_16_pd);
    qc!(MASK_OUT_Y, OP_ROR_16_DI, qc_ror_16_di);
    qc!(MASK_OUT_Y, OP_ROR_16_IX, qc_ror_16_ix);
    qc!(MASK_EXACT, OP_ROR_16_AW, qc_ror_16_aw);
    qc!(MASK_EXACT, OP_ROR_16_AL, qc_ror_16_al);

    // Put qc for ROXL, ROXR here
    qc8!(MASK_OUT_X_Y, OP_ROXR_8_S, qc_roxr_8_s);
    qc!(MASK_OUT_X_Y, OP_ROXR_16_S, qc_roxr_16_s);
    qc!(MASK_OUT_X_Y, OP_ROXR_32_S, qc_roxr_32_s);
    qc8!(MASK_OUT_X_Y, OP_ROXR_8_R, qc_roxr_8_r);
    qc!(MASK_OUT_X_Y, OP_ROXR_16_R, qc_roxr_16_r);
    qc!(MASK_OUT_X_Y, OP_ROXR_32_R, qc_roxr_32_r);

    qc8!(MASK_OUT_X_Y, OP_ROXL_8_S, qc_roxl_8_s);
    qc!(MASK_OUT_X_Y, OP_ROXL_16_S, qc_roxl_16_s);
    qc!(MASK_OUT_X_Y, OP_ROXL_32_S, qc_roxl_32_s);
    qc8!(MASK_OUT_X_Y, OP_ROXL_8_R, qc_roxl_8_r);
    qc!(MASK_OUT_X_Y, OP_ROXL_16_R, qc_roxl_16_r);
    qc!(MASK_OUT_X_Y, OP_ROXL_32_R, qc_roxl_32_r);

    qc!(MASK_OUT_Y, OP_ROXL_16_AI, qc_roxl_16_ai);
    qc!(MASK_OUT_Y, OP_ROXL_16_PI, qc_roxl_16_pi);
    qc!(MASK_OUT_Y, OP_ROXL_16_PD, qc_roxl_16_pd);
    qc!(MASK_OUT_Y, OP_ROXL_16_DI, qc_roxl_16_di);
    qc!(MASK_OUT_Y, OP_ROXL_16_IX, qc_roxl_16_ix);
    qc!(MASK_EXACT, OP_ROXL_16_AW, qc_roxl_16_aw);
    qc!(MASK_EXACT, OP_ROXL_16_AL, qc_roxl_16_al);

    qc!(MASK_OUT_Y, OP_ROXR_16_AI, qc_roxr_16_ai);
    qc!(MASK_OUT_Y, OP_ROXR_16_PI, qc_roxr_16_pi);
    qc!(MASK_OUT_Y, OP_ROXR_16_PD, qc_roxr_16_pd);
    qc!(MASK_OUT_Y, OP_ROXR_16_DI, qc_roxr_16_di);
    qc!(MASK_OUT_Y, OP_ROXR_16_IX, qc_roxr_16_ix);
    qc!(MASK_EXACT, OP_ROXR_16_AW, qc_roxr_16_aw);
    qc!(MASK_EXACT, OP_ROXR_16_AL, qc_roxr_16_al);

    // Put qc for RTE here
    // Put qc for RTR here
    // Put qc for RTS here

    qc8!(MASK_OUT_X_Y, OP_SBCD_8_RR, qc_sbcd_rr);
    qc8!(MASK_OUT_X_Y, OP_SBCD_8_MM, qc_sbcd_mm);

    qc!(MASK_OUT_Y, OP_SCC_8_AI, qc_scc_8_ai);
    qc!(MASK_EXACT, OP_SCC_8_AL, qc_scc_8_al);
    qc!(MASK_EXACT, OP_SCC_8_AW, qc_scc_8_aw);
    qc!(MASK_OUT_Y, OP_SCC_8_DN, qc_scc_8_dn);
    qc!(MASK_OUT_Y, OP_SCC_8_DI, qc_scc_8_di);
    qc!(MASK_OUT_Y, OP_SCC_8_IX, qc_scc_8_ix);
    qc!(MASK_OUT_Y, OP_SCC_8_PD, qc_scc_8_pd);
    qc!(MASK_OUT_Y, OP_SCC_8_PI, qc_scc_8_pi);

    qc!(MASK_OUT_Y, OP_SCS_8_AI, qc_scs_8_ai);
    qc!(MASK_EXACT, OP_SCS_8_AL, qc_scs_8_al);
    qc!(MASK_EXACT, OP_SCS_8_AW, qc_scs_8_aw);
    qc!(MASK_OUT_Y, OP_SCS_8_DN, qc_scs_8_dn);
    qc!(MASK_OUT_Y, OP_SCS_8_DI, qc_scs_8_di);
    qc!(MASK_OUT_Y, OP_SCS_8_IX, qc_scs_8_ix);
    qc!(MASK_OUT_Y, OP_SCS_8_PD, qc_scs_8_pd);
    qc!(MASK_OUT_Y, OP_SCS_8_PI, qc_scs_8_pi);

    qc!(MASK_OUT_Y, OP_SEQ_8_AI, qc_seq_8_ai);
    qc!(MASK_EXACT, OP_SEQ_8_AL, qc_seq_8_al);
    qc!(MASK_EXACT, OP_SEQ_8_AW, qc_seq_8_aw);
    qc!(MASK_OUT_Y, OP_SEQ_8_DN, qc_seq_8_dn);
    qc!(MASK_OUT_Y, OP_SEQ_8_DI, qc_seq_8_di);
    qc!(MASK_OUT_Y, OP_SEQ_8_IX, qc_seq_8_ix);
    qc!(MASK_OUT_Y, OP_SEQ_8_PD, qc_seq_8_pd);
    qc!(MASK_OUT_Y, OP_SEQ_8_PI, qc_seq_8_pi);

    qc!(MASK_OUT_Y, OP_SF_8_AI, qc_sf_8_ai);
    qc!(MASK_EXACT, OP_SF_8_AL, qc_sf_8_al);
    qc!(MASK_EXACT, OP_SF_8_AW, qc_sf_8_aw);
    qc!(MASK_OUT_Y, OP_SF_8_DN, qc_sf_8_dn);
    qc!(MASK_OUT_Y, OP_SF_8_DI, qc_sf_8_di);
    qc!(MASK_OUT_Y, OP_SF_8_IX, qc_sf_8_ix);
    qc!(MASK_OUT_Y, OP_SF_8_PD, qc_sf_8_pd);
    qc!(MASK_OUT_Y, OP_SF_8_PI, qc_sf_8_pi);

    qc!(MASK_OUT_Y, OP_SGE_8_AI, qc_sge_8_ai);
    qc!(MASK_EXACT, OP_SGE_8_AL, qc_sge_8_al);
    qc!(MASK_EXACT, OP_SGE_8_AW, qc_sge_8_aw);
    qc!(MASK_OUT_Y, OP_SGE_8_DN, qc_sge_8_dn);
    qc!(MASK_OUT_Y, OP_SGE_8_DI, qc_sge_8_di);
    qc!(MASK_OUT_Y, OP_SGE_8_IX, qc_sge_8_ix);
    qc!(MASK_OUT_Y, OP_SGE_8_PD, qc_sge_8_pd);
    qc!(MASK_OUT_Y, OP_SGE_8_PI, qc_sge_8_pi);

    qc!(MASK_OUT_Y, OP_SGT_8_AI, qc_sgt_8_ai);
    qc!(MASK_EXACT, OP_SGT_8_AL, qc_sgt_8_al);
    qc!(MASK_EXACT, OP_SGT_8_AW, qc_sgt_8_aw);
    qc!(MASK_OUT_Y, OP_SGT_8_DN, qc_sgt_8_dn);
    qc!(MASK_OUT_Y, OP_SGT_8_DI, qc_sgt_8_di);
    qc!(MASK_OUT_Y, OP_SGT_8_IX, qc_sgt_8_ix);
    qc!(MASK_OUT_Y, OP_SGT_8_PD, qc_sgt_8_pd);
    qc!(MASK_OUT_Y, OP_SGT_8_PI, qc_sgt_8_pi);

    qc!(MASK_OUT_Y, OP_SHI_8_AI, qc_shi_8_ai);
    qc!(MASK_EXACT, OP_SHI_8_AL, qc_shi_8_al);
    qc!(MASK_EXACT, OP_SHI_8_AW, qc_shi_8_aw);
    qc!(MASK_OUT_Y, OP_SHI_8_DN, qc_shi_8_dn);
    qc!(MASK_OUT_Y, OP_SHI_8_DI, qc_shi_8_di);
    qc!(MASK_OUT_Y, OP_SHI_8_IX, qc_shi_8_ix);
    qc!(MASK_OUT_Y, OP_SHI_8_PD, qc_shi_8_pd);
    qc!(MASK_OUT_Y, OP_SHI_8_PI, qc_shi_8_pi);

    qc!(MASK_OUT_Y, OP_SLE_8_AI, qc_sle_8_ai);
    qc!(MASK_EXACT, OP_SLE_8_AL, qc_sle_8_al);
    qc!(MASK_EXACT, OP_SLE_8_AW, qc_sle_8_aw);
    qc!(MASK_OUT_Y, OP_SLE_8_DN, qc_sle_8_dn);
    qc!(MASK_OUT_Y, OP_SLE_8_DI, qc_sle_8_di);
    qc!(MASK_OUT_Y, OP_SLE_8_IX, qc_sle_8_ix);
    qc!(MASK_OUT_Y, OP_SLE_8_PD, qc_sle_8_pd);
    qc!(MASK_OUT_Y, OP_SLE_8_PI, qc_sle_8_pi);

    qc!(MASK_OUT_Y, OP_SLS_8_AI, qc_sls_8_ai);
    qc!(MASK_EXACT, OP_SLS_8_AL, qc_sls_8_al);
    qc!(MASK_EXACT, OP_SLS_8_AW, qc_sls_8_aw);
    qc!(MASK_OUT_Y, OP_SLS_8_DN, qc_sls_8_dn);
    qc!(MASK_OUT_Y, OP_SLS_8_DI, qc_sls_8_di);
    qc!(MASK_OUT_Y, OP_SLS_8_IX, qc_sls_8_ix);
    qc!(MASK_OUT_Y, OP_SLS_8_PD, qc_sls_8_pd);
    qc!(MASK_OUT_Y, OP_SLS_8_PI, qc_sls_8_pi);

    qc!(MASK_OUT_Y, OP_SLT_8_AI, qc_slt_8_ai);
    qc!(MASK_EXACT, OP_SLT_8_AL, qc_slt_8_al);
    qc!(MASK_EXACT, OP_SLT_8_AW, qc_slt_8_aw);
    qc!(MASK_OUT_Y, OP_SLT_8_DN, qc_slt_8_dn);
    qc!(MASK_OUT_Y, OP_SLT_8_DI, qc_slt_8_di);
    qc!(MASK_OUT_Y, OP_SLT_8_IX, qc_slt_8_ix);
    qc!(MASK_OUT_Y, OP_SLT_8_PD, qc_slt_8_pd);
    qc!(MASK_OUT_Y, OP_SLT_8_PI, qc_slt_8_pi);

    qc!(MASK_OUT_Y, OP_SMI_8_AI, qc_smi_8_ai);
    qc!(MASK_EXACT, OP_SMI_8_AL, qc_smi_8_al);
    qc!(MASK_EXACT, OP_SMI_8_AW, qc_smi_8_aw);
    qc!(MASK_OUT_Y, OP_SMI_8_DN, qc_smi_8_dn);
    qc!(MASK_OUT_Y, OP_SMI_8_DI, qc_smi_8_di);
    qc!(MASK_OUT_Y, OP_SMI_8_IX, qc_smi_8_ix);
    qc!(MASK_OUT_Y, OP_SMI_8_PD, qc_smi_8_pd);
    qc!(MASK_OUT_Y, OP_SMI_8_PI, qc_smi_8_pi);

    qc!(MASK_OUT_Y, OP_SNE_8_AI, qc_sne_8_ai);
    qc!(MASK_EXACT, OP_SNE_8_AL, qc_sne_8_al);
    qc!(MASK_EXACT, OP_SNE_8_AW, qc_sne_8_aw);
    qc!(MASK_OUT_Y, OP_SNE_8_DN, qc_sne_8_dn);
    qc!(MASK_OUT_Y, OP_SNE_8_DI, qc_sne_8_di);
    qc!(MASK_OUT_Y, OP_SNE_8_IX, qc_sne_8_ix);
    qc!(MASK_OUT_Y, OP_SNE_8_PD, qc_sne_8_pd);
    qc!(MASK_OUT_Y, OP_SNE_8_PI, qc_sne_8_pi);

    qc!(MASK_OUT_Y, OP_SPL_8_AI, qc_spl_8_ai);
    qc!(MASK_EXACT, OP_SPL_8_AL, qc_spl_8_al);
    qc!(MASK_EXACT, OP_SPL_8_AW, qc_spl_8_aw);
    qc!(MASK_OUT_Y, OP_SPL_8_DN, qc_spl_8_dn);
    qc!(MASK_OUT_Y, OP_SPL_8_DI, qc_spl_8_di);
    qc!(MASK_OUT_Y, OP_SPL_8_IX, qc_spl_8_ix);
    qc!(MASK_OUT_Y, OP_SPL_8_PD, qc_spl_8_pd);
    qc!(MASK_OUT_Y, OP_SPL_8_PI, qc_spl_8_pi);

    qc!(MASK_OUT_Y, OP_ST_8_AI, qc_st_8_ai);
    qc!(MASK_EXACT, OP_ST_8_AL, qc_st_8_al);
    qc!(MASK_EXACT, OP_ST_8_AW, qc_st_8_aw);
    qc!(MASK_OUT_Y, OP_ST_8_DN, qc_st_8_dn);
    qc!(MASK_OUT_Y, OP_ST_8_DI, qc_st_8_di);
    qc!(MASK_OUT_Y, OP_ST_8_IX, qc_st_8_ix);
    qc!(MASK_OUT_Y, OP_ST_8_PD, qc_st_8_pd);
    qc!(MASK_OUT_Y, OP_ST_8_PI, qc_st_8_pi);

    qc!(MASK_OUT_Y, OP_SVC_8_AI, qc_svc_8_ai);
    qc!(MASK_EXACT, OP_SVC_8_AL, qc_svc_8_al);
    qc!(MASK_EXACT, OP_SVC_8_AW, qc_svc_8_aw);
    qc!(MASK_OUT_Y, OP_SVC_8_DN, qc_svc_8_dn);
    qc!(MASK_OUT_Y, OP_SVC_8_DI, qc_svc_8_di);
    qc!(MASK_OUT_Y, OP_SVC_8_IX, qc_svc_8_ix);
    qc!(MASK_OUT_Y, OP_SVC_8_PD, qc_svc_8_pd);
    qc!(MASK_OUT_Y, OP_SVC_8_PI, qc_svc_8_pi);

    qc!(MASK_OUT_Y, OP_SVS_8_AI, qc_svs_8_ai);
    qc!(MASK_EXACT, OP_SVS_8_AL, qc_svs_8_al);
    qc!(MASK_EXACT, OP_SVS_8_AW, qc_svs_8_aw);
    qc!(MASK_OUT_Y, OP_SVS_8_DN, qc_svs_8_dn);
    qc!(MASK_OUT_Y, OP_SVS_8_DI, qc_svs_8_di);
    qc!(MASK_OUT_Y, OP_SVS_8_IX, qc_svs_8_ix);
    qc!(MASK_OUT_Y, OP_SVS_8_PD, qc_svs_8_pd);
    qc!(MASK_OUT_Y, OP_SVS_8_PI, qc_svs_8_pi);

    /*
    qc!(MASK_OUT_Y, OP_SHI_8_AI, qc_shi_8_ai);
    qc!(MASK_EXACT, OP_SHI_8_AL, qc_shi_8_al);
    qc!(MASK_EXACT, OP_SHI_8_AW, qc_shi_8_aw);
    qc!(MASK_OUT_Y, OP_SHI_8_DN, qc_shi_8_dn);
    qc!(MASK_OUT_Y, OP_SHI_8_DI, qc_shi_8_di);
    qc!(MASK_OUT_Y, OP_SHI_8_IX, qc_shi_8_ix);
    qc!(MASK_OUT_Y, OP_SHI_8_PD, qc_shi_8_pd);
    qc!(MASK_OUT_Y, OP_SHI_8_PI, qc_shi_8_pi);
    */

    // Put qc for STOP here
    // Put qc for SUB here

    qc8!(MASK_OUT_X_Y, OP_SUB_8_ER_DN, qc_sub_8_er_dn);
    qc8!(MASK_OUT_X_Y, OP_SUB_8_ER_PI, qc_sub_8_er_pi);
    qc8!(MASK_OUT_X_Y, OP_SUB_8_ER_PD, qc_sub_8_er_pd);
    qc8!(MASK_OUT_X_Y, OP_SUB_8_ER_AI, qc_sub_8_er_ai);
    qc8!(MASK_OUT_X_Y, OP_SUB_8_ER_DI, qc_sub_8_er_di);
    qc8!(MASK_OUT_X_Y, OP_SUB_8_ER_IX, qc_sub_8_er_ix);
    qc8!(MASK_OUT_X, OP_SUB_8_ER_AW, qc_sub_8_er_aw);
    qc8!(MASK_OUT_X, OP_SUB_8_ER_AL, qc_sub_8_er_al);
    qc8!(MASK_OUT_X, OP_SUB_8_ER_PCDI, qc_sub_8_er_pcdi);
    qc8!(MASK_OUT_X, OP_SUB_8_ER_PCIX, qc_sub_8_er_pcix);
    qc8!(MASK_OUT_X, OP_SUB_8_ER_IMM, qc_sub_8_er_imm);

    qc8!(MASK_OUT_X_Y, OP_SUB_8_RE_PI, qc_sub_8_re_pi);
    qc8!(MASK_OUT_X_Y, OP_SUB_8_RE_PD, qc_sub_8_re_pd);
    qc8!(MASK_OUT_X_Y, OP_SUB_8_RE_AI, qc_sub_8_re_ai);
    qc8!(MASK_OUT_X_Y, OP_SUB_8_RE_DI, qc_sub_8_re_di);
    qc8!(MASK_OUT_X_Y, OP_SUB_8_RE_IX, qc_sub_8_re_ix);
    qc8!(MASK_OUT_X, OP_SUB_8_RE_AW, qc_sub_8_re_aw);
    qc8!(MASK_OUT_X, OP_SUB_8_RE_AL, qc_sub_8_re_al);

    qc!(MASK_OUT_X_Y, OP_SUB_16_ER_DN, qc_sub_16_er_dn);
    qc!(MASK_OUT_X_Y, OP_SUB_16_ER_AN, qc_sub_16_er_an);
    qc!(MASK_OUT_X_Y, OP_SUB_16_ER_PI, qc_sub_16_er_pi);
    qc!(MASK_OUT_X_Y, OP_SUB_16_ER_PD, qc_sub_16_er_pd);
    qc!(MASK_OUT_X_Y, OP_SUB_16_ER_AI, qc_sub_16_er_ai);
    qc!(MASK_OUT_X_Y, OP_SUB_16_ER_DI, qc_sub_16_er_di);
    qc!(MASK_OUT_X_Y, OP_SUB_16_ER_IX, qc_sub_16_er_ix);
    qc!(MASK_OUT_X, OP_SUB_16_ER_AW, qc_sub_16_er_aw);
    qc!(MASK_OUT_X, OP_SUB_16_ER_AL, qc_sub_16_er_al);
    qc!(MASK_OUT_X, OP_SUB_16_ER_PCDI, qc_sub_16_er_pcdi);
    qc!(MASK_OUT_X, OP_SUB_16_ER_PCIX, qc_sub_16_er_pcix);
    qc!(MASK_OUT_X, OP_SUB_16_ER_IMM, qc_sub_16_er_imm);

    qc!(MASK_OUT_X_Y, OP_SUB_16_RE_PI, qc_sub_16_re_pi);
    qc!(MASK_OUT_X_Y, OP_SUB_16_RE_PD, qc_sub_16_re_pd);
    qc!(MASK_OUT_X_Y, OP_SUB_16_RE_AI, qc_sub_16_re_ai);
    qc!(MASK_OUT_X_Y, OP_SUB_16_RE_DI, qc_sub_16_re_di);
    qc!(MASK_OUT_X_Y, OP_SUB_16_RE_IX, qc_sub_16_re_ix);
    qc!(MASK_OUT_X, OP_SUB_16_RE_AW, qc_sub_16_re_aw);
    qc!(MASK_OUT_X, OP_SUB_16_RE_AL, qc_sub_16_re_al);

    qc!(MASK_OUT_X_Y, OP_SUB_32_ER_DN, qc_sub_32_er_dn);
    qc!(MASK_OUT_X_Y, OP_SUB_32_ER_AN, qc_sub_32_er_an);
    qc!(MASK_OUT_X_Y, OP_SUB_32_ER_PI, qc_sub_32_er_pi);
    qc!(MASK_OUT_X_Y, OP_SUB_32_ER_PD, qc_sub_32_er_pd);
    qc!(MASK_OUT_X_Y, OP_SUB_32_ER_AI, qc_sub_32_er_ai);
    qc!(MASK_OUT_X_Y, OP_SUB_32_ER_DI, qc_sub_32_er_di);
    qc!(MASK_OUT_X_Y, OP_SUB_32_ER_IX, qc_sub_32_er_ix);
    qc!(MASK_OUT_X, OP_SUB_32_ER_AW, qc_sub_32_er_aw);
    qc!(MASK_OUT_X, OP_SUB_32_ER_AL, qc_sub_32_er_al);
    qc!(MASK_OUT_X, OP_SUB_32_ER_PCDI, qc_sub_32_er_pcdi);
    qc!(MASK_OUT_X, OP_SUB_32_ER_PCIX, qc_sub_32_er_pcix);
    qc!(MASK_OUT_X, OP_SUB_32_ER_IMM, qc_sub_32_er_imm);

    qc!(MASK_OUT_X_Y, OP_SUB_32_RE_PI, qc_sub_32_re_pi);
    qc!(MASK_OUT_X_Y, OP_SUB_32_RE_PD, qc_sub_32_re_pd);
    qc!(MASK_OUT_X_Y, OP_SUB_32_RE_AI, qc_sub_32_re_ai);
    qc!(MASK_OUT_X_Y, OP_SUB_32_RE_DI, qc_sub_32_re_di);
    qc!(MASK_OUT_X_Y, OP_SUB_32_RE_IX, qc_sub_32_re_ix);
    qc!(MASK_OUT_X, OP_SUB_32_RE_AW, qc_sub_32_re_aw);
    qc!(MASK_OUT_X, OP_SUB_32_RE_AL, qc_sub_32_re_al);

    qc!(MASK_OUT_X_Y, OP_SUBA_16_DN, qc_suba_16_dn);
    qc!(MASK_OUT_X_Y, OP_SUBA_16_AN, qc_suba_16_an);
    qc!(MASK_OUT_X_Y, OP_SUBA_16_PI, qc_suba_16_pi);
    qc!(MASK_OUT_X_Y, OP_SUBA_16_PD, qc_suba_16_pd);
    qc!(MASK_OUT_X_Y, OP_SUBA_16_AI, qc_suba_16_ai);
    qc!(MASK_OUT_X_Y, OP_SUBA_16_DI, qc_suba_16_di);
    qc!(MASK_OUT_X_Y, OP_SUBA_16_IX, qc_suba_16_ix);
    qc!(MASK_OUT_X, OP_SUBA_16_AW, qc_suba_16_aw);
    qc!(MASK_OUT_X, OP_SUBA_16_AL, qc_suba_16_al);
    qc!(MASK_OUT_X, OP_SUBA_16_PCDI, qc_suba_16_pcdi);
    qc!(MASK_OUT_X, OP_SUBA_16_PCIX, qc_suba_16_pcix);
    qc!(MASK_OUT_X, OP_SUBA_16_IMM, qc_suba_16_imm);

    qc!(MASK_OUT_X_Y, OP_SUBA_32_DN, qc_suba_32_dn);
    qc!(MASK_OUT_X_Y, OP_SUBA_32_AN, qc_suba_32_an);
    qc!(MASK_OUT_X_Y, OP_SUBA_32_PI, qc_suba_32_pi);
    qc!(MASK_OUT_X_Y, OP_SUBA_32_PD, qc_suba_32_pd);
    qc!(MASK_OUT_X_Y, OP_SUBA_32_AI, qc_suba_32_ai);
    qc!(MASK_OUT_X_Y, OP_SUBA_32_DI, qc_suba_32_di);
    qc!(MASK_OUT_X_Y, OP_SUBA_32_IX, qc_suba_32_ix);
    qc!(MASK_OUT_X, OP_SUBA_32_AW, qc_suba_32_aw);
    qc!(MASK_OUT_X, OP_SUBA_32_AL, qc_suba_32_al);
    qc!(MASK_OUT_X, OP_SUBA_32_PCDI, qc_suba_32_pcdi);
    qc!(MASK_OUT_X, OP_SUBA_32_PCIX, qc_suba_32_pcix);
    qc!(MASK_OUT_X, OP_SUBA_32_IMM, qc_suba_32_imm);

    qc8!(MASK_OUT_Y, OP_SUBI_8_DN, qc_subi_8_dn);
    qc8!(MASK_OUT_Y, OP_SUBI_8_PI, qc_subi_8_pi);
    qc8!(MASK_OUT_Y, OP_SUBI_8_PD, qc_subi_8_pd);
    qc8!(MASK_OUT_Y, OP_SUBI_8_AI, qc_subi_8_ai);
    qc8!(MASK_OUT_Y, OP_SUBI_8_DI, qc_subi_8_di);
    qc8!(MASK_OUT_Y, OP_SUBI_8_IX, qc_subi_8_ix);
    qc8!(MASK_EXACT, OP_SUBI_8_AW, qc_subi_8_aw);
    qc8!(MASK_EXACT, OP_SUBI_8_AL, qc_subi_8_al);

    qc!(MASK_OUT_Y, OP_SUBI_16_DN, qc_subi_16_dn);
    qc!(MASK_OUT_Y, OP_SUBI_16_PI, qc_subi_16_pi);
    qc!(MASK_OUT_Y, OP_SUBI_16_PD, qc_subi_16_pd);
    qc!(MASK_OUT_Y, OP_SUBI_16_AI, qc_subi_16_ai);
    qc!(MASK_OUT_Y, OP_SUBI_16_DI, qc_subi_16_di);
    qc!(MASK_OUT_Y, OP_SUBI_16_IX, qc_subi_16_ix);
    qc!(MASK_EXACT, OP_SUBI_16_AW, qc_subi_16_aw);
    qc!(MASK_EXACT, OP_SUBI_16_AL, qc_subi_16_al);

    qc!(MASK_OUT_Y, OP_SUBI_32_DN, qc_subi_32_dn);
    qc!(MASK_OUT_Y, OP_SUBI_32_PI, qc_subi_32_pi);
    qc!(MASK_OUT_Y, OP_SUBI_32_PD, qc_subi_32_pd);
    qc!(MASK_OUT_Y, OP_SUBI_32_AI, qc_subi_32_ai);
    qc!(MASK_OUT_Y, OP_SUBI_32_DI, qc_subi_32_di);
    qc!(MASK_OUT_Y, OP_SUBI_32_IX, qc_subi_32_ix);
    qc!(MASK_EXACT, OP_SUBI_32_AW, qc_subi_32_aw);
    qc!(MASK_EXACT, OP_SUBI_32_AL, qc_subi_32_al);

    qc8!(MASK_OUT_X_Y, OP_SUBQ_8_DN, qc_subq_8_dn);
    qc8!(MASK_OUT_X_Y, OP_SUBQ_8_PI, qc_subq_8_pi);
    qc8!(MASK_OUT_X_Y, OP_SUBQ_8_PD, qc_subq_8_pd);
    qc8!(MASK_OUT_X_Y, OP_SUBQ_8_AI, qc_subq_8_ai);
    qc8!(MASK_OUT_X_Y, OP_SUBQ_8_DI, qc_subq_8_di);
    qc8!(MASK_OUT_X_Y, OP_SUBQ_8_IX, qc_subq_8_ix);
    qc8!(MASK_OUT_X, OP_SUBQ_8_AW, qc_subq_8_aw);
    qc8!(MASK_OUT_X, OP_SUBQ_8_AL, qc_subq_8_al);

    qc!(MASK_OUT_X_Y, OP_SUBQ_16_DN, qc_subq_16_dn);
    qc!(MASK_OUT_X_Y, OP_SUBQ_16_AN, qc_subq_16_an);
    qc!(MASK_OUT_X_Y, OP_SUBQ_16_PI, qc_subq_16_pi);
    qc!(MASK_OUT_X_Y, OP_SUBQ_16_PD, qc_subq_16_pd);
    qc!(MASK_OUT_X_Y, OP_SUBQ_16_AI, qc_subq_16_ai);
    qc!(MASK_OUT_X_Y, OP_SUBQ_16_DI, qc_subq_16_di);
    qc!(MASK_OUT_X_Y, OP_SUBQ_16_IX, qc_subq_16_ix);
    qc!(MASK_OUT_X, OP_SUBQ_16_AW, qc_subq_16_aw);
    qc!(MASK_OUT_X, OP_SUBQ_16_AL, qc_subq_16_al);

    qc!(MASK_OUT_X_Y, OP_SUBQ_32_DN, qc_subq_32_dn);
    qc!(MASK_OUT_X_Y, OP_SUBQ_32_AN, qc_subq_32_an);
    qc!(MASK_OUT_X_Y, OP_SUBQ_32_PI, qc_subq_32_pi);
    qc!(MASK_OUT_X_Y, OP_SUBQ_32_PD, qc_subq_32_pd);
    qc!(MASK_OUT_X_Y, OP_SUBQ_32_AI, qc_subq_32_ai);
    qc!(MASK_OUT_X_Y, OP_SUBQ_32_DI, qc_subq_32_di);
    qc!(MASK_OUT_X_Y, OP_SUBQ_32_IX, qc_subq_32_ix);
    qc!(MASK_OUT_X, OP_SUBQ_32_AW, qc_subq_32_aw);
    qc!(MASK_OUT_X, OP_SUBQ_32_AL, qc_subq_32_al);

    qc8!(MASK_OUT_X_Y, OP_SUBX_8_RR, qc_subx_8_rr);
    qc8!(MASK_OUT_X_Y, OP_SUBX_8_MM, qc_subx_8_mm);
    qc!(MASK_OUT_X_Y, OP_SUBX_16_RR, qc_subx_16_rr);
    qc!(MASK_OUT_X_Y, OP_SUBX_16_MM, qc_subx_16_mm);
    qc!(MASK_OUT_X_Y, OP_SUBX_32_RR, qc_subx_32_rr);
    qc!(MASK_OUT_X_Y, OP_SUBX_32_MM, qc_subx_32_mm);

    // Put qc for SWAP here
    qc!(MASK_OUT_X_Y, OP_SWAP_32_DN, qc_swap_32_dn);
    //
    // Put qc for TAS here
    // Put qc for TRAP here
    // Put qc for TRAPV here
    // Put qc for TST here
    // Put qc for UNLK here

    macro_rules! core_eq {
        ($left:ident , $right:ident . $field:ident [ $index:expr ]) => ({
            match (&($left.$field[$index]), &($right.$field[$index])) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        println!("core incoherence: `{}[{}]` differs \
                               ({}: `0x{:x}`, {}: `0x{:x}`)", stringify!($field), $index, stringify!($left), left_val, stringify!($right), right_val);
                        return false;
                    }
                }
            }
        });
        ($left:ident , $right:ident . $field:ident () ?) => ({
            match (&($left.$field()), &($right.$field())) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        println!("core incoherence: `{}()` differs \
                               ({}: `{:?}`, {}: `{:?}`)", stringify!($field), stringify!($left), left_val, stringify!($right), right_val);
                        return false;
                    }
                }
            }
        });
        ($left:ident , $right:ident . $field:ident ()) => ({
            match (&($left.$field()), &($right.$field())) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        println!("core incoherence: `{}()` differs \
                               ({}: `0x{:x}`, {}: `0x{:x}`)", stringify!($field), stringify!($left), left_val, stringify!($right), right_val);
                        return false;
                    }
                }
            }
        });
        ($left:ident , $right:ident . $field:ident) => ({
            match (&($left.$field), &($right.$field)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        println!("core incoherence: `{}` differs \
                               ({}: `0x{:x}`, {}: `0x{:x}`)", stringify!($field), stringify!($left), left_val, stringify!($right), right_val);
                        return false;
                    }
                }
            }
        })
    }
    fn assert_cores_equal(musashi: &Core, r68k: &Core) -> bool {
        let is_reading_vector = |&op| match op {
            Operation::ReadLong(SUPERVISOR_DATA, addr, _) =>
                addr % 4 == 0 && addr >= 0x08 && addr < 0x30,
            _ =>
                false
        };
        // check that memory accesses match up
        // if an exception occurred, do not compare beyond which vector was taken
        // as Mushashi also seems to execute the first instruction of the handler
        if let Some(vector_read_index) = r68k.mem.logger.ops().iter().position(is_reading_vector) {
            assert_equal(get_ops().iter().take(vector_read_index+1), r68k.mem.logger.ops().iter().take(vector_read_index+1));

            // TODO: perhaps we could let r68 execute just one more
            // instruction to get back in sync?

            // If we got this far, the memory accesses up to, and
            // including the vector read match up, but we cannot
            // compare further
            return false;
        }
        assert_equal(get_ops(), r68k.mem.logger.ops());

        core_eq!(musashi, r68k.pc);
        core_eq!(musashi, r68k.flags() ?);
        core_eq!(musashi, r68k.status_register());
        core_eq!(musashi, r68k.ssp());
        core_eq!(musashi, r68k.usp());
        for i in (0..16).rev() {
            core_eq!(musashi, r68k.dar[i]);
        }
        true
    }

    #[test]
    fn roundtrip_d0() {
        assert_eq!(256, roundtrip_register(Register::D0, 256));
    }

    #[test]
    fn roundtrip_abcd_rr() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        let pc = 0x40;
        // 0xc101: ABCD        D0, D1
        let mut cpu = Core::new_mem(pc, &[0xc1, 0x01, 0x00, 0x00]);
        cpu.dar[0] = 0x17;
        cpu.dar[1] = 0x27;
        cpu.dar[5] = 0x55555;
        reset_and_execute1(&mut cpu, 0xaaaaaaaa);

        // 17 + 27 is 44
        assert_eq!(0x44, cpu.dar[0]);
        assert_eq!(0x27, cpu.dar[1]);
        assert_eq!(0x55555, cpu.dar[5]);

        let ops = get_ops();
        assert_eq!(1, ops.len());
        assert_eq!(Operation::ReadLong(SUPERVISOR_PROGRAM, pc, 0xc1010000), ops[0]);
    }

    #[test]
    fn compare_abcd_rr() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        let pc = 0x40;
        // 0xc300: ABCD        D1, D0
        let mut musashi = Core::new_mem(pc, &[0xc3, 0x00]);
        musashi.dar[0] = 0x16;
        musashi.dar[1] = 0x26;

        let mut r68k = musashi.clone(); // so very self-aware!
        reset_and_execute1(&mut musashi, 0xaaaaaaaa);
        r68k.execute1();
        assert_eq!(0x42, r68k.dar[1]);

        assert_cores_equal(&musashi, &r68k);
    }


    #[test]
    fn run_abcd_rr_twice() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        let pc = 0x40;
        // 0xc300: ABCD        D1, D0
        // 0xc302: ABCD        D1, D2
        let mut musashi = Core::new_mem(pc, &[0xc3, 0x00, 0xc3, 0x02]);
        musashi.dar[0] = 0x16;
        musashi.dar[1] = 0x26;
        musashi.dar[2] = 0x31;

        let mut r68k = musashi.clone(); // so very self-aware!

        initialize_musashi(&mut musashi, 0xaaaaaaaa);

        // execute ABCD        D1, D0
        execute1(&mut musashi);
        r68k.execute1();
        assert_eq!(0x42, musashi.dar[1]);
        assert_eq!(0x42, r68k.dar[1]);

        // then execute a second instruction (ABCD D1, D2) on the core
        execute1(&mut musashi);
        r68k.execute1();
        assert_eq!(0x73, musashi.dar[1]);
        assert_eq!(0x73, r68k.dar[1]);

        assert_cores_equal(&musashi, &r68k);
    }

    #[test]
    fn compare_address_error_actions() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        // using an odd absolute address should force an address error
        // opcodes d278,0107 is ADD.W    $0107, D1
        let mut musashi = Core::new_mem(0x40, &[0xd2, 0x78, 0x01, 0x07]);
        let vec3handler = 0x1F0000;
        musashi.mem.write_long(SUPERVISOR_PROGRAM, 3*4, vec3handler);
        musashi.mem.write_long(SUPERVISOR_PROGRAM, vec3handler, 0xd2780108);
        musashi.dar[15] = 0x100;
        let mut r68k = musashi.clone(); // so very self-aware!
        initialize_musashi(&mut musashi, 0xaaaaaaaa);
        execute1(&mut musashi);
        //execute1(&mut musashi);
        r68k.execute1();
        r68k.execute1();

        assert_cores_equal(&musashi, &r68k);
    }
    #[test]
    fn compare_illegal_instruction_actions() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        // d208 is ADD.B A0,D0, which is illegal
        let mut musashi = Core::new_mem(0x4000, &[0xd2, 08]);
        let vec4handler = 0x2F0000;
        musashi.mem.write_long(SUPERVISOR_PROGRAM, 4*4, vec4handler);
        musashi.mem.write_long(SUPERVISOR_PROGRAM, vec4handler, 0xd2780108);
        musashi.dar[15] = 0x100;
        let mut r68k = musashi.clone(); // so very self-aware!
        initialize_musashi(&mut musashi, 0xaaaaaaaa);
        execute1(&mut musashi);
        //execute1(&mut musashi);
        r68k.execute1();
        //r68k.execute1();

        assert_cores_equal(&musashi, &r68k);
    }

use std::ptr;
use super::m68k_get_reg;

    #[test]
    fn stackpointers_are_correct_when_starting_in_supervisor_mode() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        let pc = 0x40;
        // 0xc300: ABCD        D1, D0
        // 0xc302: ABCD        D1, D2
        let mut musashi = Core::new_mem(pc, &[0xc3, 0x00, 0xc3, 0x02]);
        musashi.sr_to_flags((1<<13));
        musashi.inactive_usp = 0x200; // User SP
        musashi.dar[15] = 0x100;       // Supa SP
        initialize_musashi(&mut musashi, 0xaaaaaaaa);
        unsafe {
            assert!((1<<13) & m68k_get_reg(ptr::null_mut(), Register::SR) > 0);
            assert_eq!(0x100, m68k_get_reg(ptr::null_mut(), Register::ISP));
            assert_eq!(0x200, m68k_get_reg(ptr::null_mut(), Register::USP));
        }
    }
    #[test]
    fn stackpointers_are_correct_when_starting_in_user_mode() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        let pc = 0x40;
        // 0xc300: ABCD        D1, D0
        // 0xc302: ABCD        D1, D2
        let mut musashi = Core::new_mem(pc, &[0xc3, 0x00, 0xc3, 0x02]);
        musashi.sr_to_flags(0);
        musashi.dar[15] = 0x200;       // User SP
        musashi.inactive_ssp = 0x100; // Supa SP
        initialize_musashi(&mut musashi, 0xaaaaaaaa);
        unsafe {
            assert!((1<<13) & m68k_get_reg(ptr::null_mut(), Register::SR) == 0);
            assert_eq!(0x100, m68k_get_reg(ptr::null_mut(), Register::ISP));
            assert_eq!(0x200, m68k_get_reg(ptr::null_mut(), Register::USP));
        }
    }


use ram::{SUPERVISOR_DATA, USER_PROGRAM, USER_DATA, ADDRBUS_MASK};

    #[test]
    fn read_initialized_memory() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        initialize_musashi_memory(0x01020304);
        for v in 0..256 {
            assert_eq!(0x01, m68k_read_memory_8(4*v+0));
            assert_eq!(0x02, m68k_read_memory_8(4*v+1));
            assert_eq!(0x03, m68k_read_memory_8(4*v+2));
            assert_eq!(0x04, m68k_read_memory_8(4*v+3));
        }
        for v in 0..256 {
            assert_eq!(0x0102, m68k_read_memory_16(4*v+0));
            assert_eq!(0x0203, m68k_read_memory_16(4*v+1));
            assert_eq!(0x0304, m68k_read_memory_16(4*v+2));
            if 4*v+3 < 1023 {
                assert_eq!(0x0401, m68k_read_memory_16(4*v+3));
            }
        }
        for v in 0..255 {
            assert_eq!(0x01020304, m68k_read_memory_32(4*v+0));
            assert_eq!(0x02030401, m68k_read_memory_32(4*v+1));
            assert_eq!(0x03040102, m68k_read_memory_32(4*v+2));
            assert_eq!(0x04010203, m68k_read_memory_32(4*v+3));
        }
        assert_eq!(0x01020304, m68k_read_memory_32(4*255));
    }

    #[test]
    fn read_your_u32_writes() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        initialize_musashi_memory(0x01020304);
        let pattern = 0xAAAA7777;
        let address = 128;
        assert!(pattern != m68k_read_memory_32(address));
        m68k_write_memory_32(address, pattern);
        assert_eq!(pattern, m68k_read_memory_32(address));
    }

    #[test]
    fn read_your_u16_writes() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        initialize_musashi_memory(0x01020304);
        let pattern = 0xAAAA7777;
        let address = 128;
        assert!(pattern != m68k_read_memory_16(address));
        m68k_write_memory_16(address, pattern);
        assert_eq!(pattern & 0xFFFF, m68k_read_memory_16(address));
    }

    #[test]
    fn read_your_u8_writes() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        initialize_musashi_memory(0x01020304);
        let pattern = 0xAAAA7777;
        let address = 128;
        assert!(pattern != m68k_read_memory_8(address));
        m68k_write_memory_8(address, pattern);
        assert_eq!(pattern & 0xFF, m68k_read_memory_8(address));
    }

    #[test]
    fn shared_address_space() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        initialize_musashi_memory(0x01020304);
        let pattern = 0xAAAA7777;
        let address = 128;
        m68k_set_fc(USER_DATA.fc());
        assert!(pattern != m68k_read_memory_32(address));
        m68k_set_fc(USER_PROGRAM.fc());
        assert!(pattern != m68k_read_memory_32(address));
        m68k_set_fc(SUPERVISOR_DATA.fc());
        assert!(pattern != m68k_read_memory_32(address));
        m68k_set_fc(SUPERVISOR_PROGRAM.fc());
        assert!(pattern != m68k_read_memory_32(address));
        m68k_set_fc(USER_DATA.fc());
        m68k_write_memory_32(address, pattern);

        assert_eq!(pattern, m68k_read_memory_32(address));
        m68k_set_fc(USER_PROGRAM.fc());
        assert_eq!(pattern, m68k_read_memory_32(address));
        m68k_set_fc(SUPERVISOR_DATA.fc());
        assert_eq!(pattern, m68k_read_memory_32(address));
        m68k_set_fc(SUPERVISOR_PROGRAM.fc());
        assert_eq!(pattern, m68k_read_memory_32(address));
    }

    #[test]
    fn do_read_byte_is_logged() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        initialize_musashi_memory(0x01020304);
        let address = 0x80;
        m68k_set_fc(SUPERVISOR_DATA.fc());
        m68k_read_memory_8(address);
        let ops = get_ops();
        assert!(ops.len() > 0);
        assert_eq!(Operation::ReadByte(SUPERVISOR_DATA, address & ADDRBUS_MASK, 0x01), ops[0]);
    }

    #[test]
    fn do_read_word_is_logged() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        initialize_musashi_memory(0x01020304);
        let address = 0x80;
        m68k_set_fc(SUPERVISOR_PROGRAM.fc());
        m68k_read_memory_16(address);
        let ops = get_ops();
        assert!(ops.len() > 0);
        assert_eq!(Operation::ReadWord(SUPERVISOR_PROGRAM, address & ADDRBUS_MASK, 0x0102), ops[0]);
    }

    #[test]
    fn do_read_long_is_logged() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        initialize_musashi_memory(0x01020304);
        let address = 0x80;
        m68k_set_fc(USER_DATA.fc());
        m68k_read_memory_32(address);
        let ops = get_ops();
        assert!(ops.len() > 0);
        assert_eq!(Operation::ReadLong(USER_DATA, address & ADDRBUS_MASK, 0x01020304), ops[0]);
    }

    #[test]
    fn do_write_byte_is_logged() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        initialize_musashi_memory(0x01020304);
        let address = 0x80;
        let pattern = 0xAAAA7777;
        m68k_set_fc(USER_PROGRAM.fc());
        m68k_write_memory_8(address, pattern);
        let ops = get_ops();
        assert!(ops.len() > 0);
        assert_eq!(Operation::WriteByte(USER_PROGRAM, address & ADDRBUS_MASK, pattern), ops[0]);
    }

    #[test]
    fn do_write_word_is_logged() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        initialize_musashi_memory(0x01020304);
        let address = 0x80;
        let pattern = 0xAAAA7777;
        m68k_set_fc(SUPERVISOR_PROGRAM.fc());
        m68k_write_memory_16(address, pattern);
        let ops = get_ops();
        assert!(ops.len() > 0);
        assert_eq!(Operation::WriteWord(SUPERVISOR_PROGRAM, address & ADDRBUS_MASK, pattern), ops[0]);
    }

    #[test]
    fn do_write_long_is_logged() {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        initialize_musashi_memory(0x01020304);
        let address = 0x80;
        let pattern = 0xAAAA7777;
        m68k_set_fc(USER_DATA.fc());
        m68k_write_memory_32(address, pattern);
        let ops = get_ops();
        assert!(ops.len() > 0);
        assert_eq!(Operation::WriteLong(USER_DATA, address & ADDRBUS_MASK, pattern), ops[0]);
    }

    #[test]
    fn page_allocation_on_write_unless_matching_initializer()
    {
        let _mutex = MUSASHI_LOCK.lock().unwrap();

        let data = 0x01020304;
        initialize_musashi_memory(data);
        for offset in 0..256 {
            m68k_write_memory_32(4*offset, data);
        }
        m68k_write_memory_8(0, 0x1);
        m68k_write_memory_8(1, 0x2);
        m68k_write_memory_8(2, 0x3);
        m68k_write_memory_8(3, 0x4);

        m68k_write_memory_16(3, 0x0401);

        // no pages allocated
        assert_eq!(0, musashi_written_bytes());
        // but as soon as we write something different
        m68k_write_memory_8(2, 0x2);
        // a page is allocated
        assert_eq!(1, musashi_written_bytes());
        // we don't need to allocate a second page if we overwrite existing data
        m68k_write_memory_8(2, 0x99);
        assert_eq!(1, musashi_written_bytes());
        let ops = get_ops();
        assert_eq!(263, ops.len());
    }
}
