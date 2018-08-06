// type alias for exception handling
use std::result;
pub type Result<T> = result::Result<T, Exception>;
use interrupts::{InterruptController, AutoInterruptController, SPURIOUS_INTERRUPT};
use ram::loggingmem::{LoggingMem, OpsLogger};
pub type TestCore = ConfiguredCore<AutoInterruptController, LoggingMem<OpsLogger>>;
pub type Handler<T> = fn(&mut T) -> Result<Cycles>;
pub type InstructionSet<T> = Vec<Handler<T>>;
use ram::{AddressBus, SUPERVISOR_PROGRAM, SUPERVISOR_DATA, USER_PROGRAM, USER_DATA};
pub mod ops;
mod effective_address;
mod operator;

pub trait Core {
    fn pc(&mut self) -> &mut  u32;
    fn ir(&mut self) -> u16;
    fn ax(&mut self) -> &mut  u32;
    fn ay(&mut self) -> &mut  u32;
    fn dx(&mut self) -> &mut  u32;
    fn dy(&mut self) -> &mut  u32;
    fn c_flag(&mut self) -> &mut u32;
    fn v_flag(&mut self) -> &mut u32;
    fn n_flag(&mut self) -> &mut u32;
    fn s_flag(&mut self) -> &mut u32;
    fn x_flag(&mut self) -> &mut u32;
    fn not_z_flag(&mut self) -> &mut u32;
    fn x_flag_as_1(&self) -> u32;
    fn dar(&mut self) -> &mut [u32; 16];
    fn read_data_byte(&mut self, address: u32) -> Result<u32>;
    fn read_data_word(&mut self, address: u32) -> Result<u32>;
    fn read_data_long(&mut self, address: u32) -> Result<u32>;
    fn read_program_byte(&mut self, address: u32) -> Result<u32>;
    fn read_program_word(&mut self, address: u32) -> Result<u32>;
    fn read_program_long(&mut self, address: u32) -> Result<u32>;
    fn write_data_byte(&mut self, address: u32, value: u32) -> Result<()>;
    fn write_data_word(&mut self, address: u32, value: u32) -> Result<()>;
    fn write_data_long(&mut self, address: u32, value: u32) -> Result<()>;
    fn write_program_byte(&mut self, address: u32, value: u32) -> Result<()>;
    fn write_program_word(&mut self, address: u32, value: u32) -> Result<()>;
    fn write_program_long(&mut self, address: u32, value: u32) -> Result<()>;
    fn status_register(&self) -> u16;
    fn condition_code_register(&self) -> u16;
    fn sr_to_flags(&mut self, sr: u16);
    fn ccr_to_flags(&mut self, ccr: u16);
    fn cond_t(&self) -> bool;
    fn cond_f(&self) -> bool;
    fn cond_hi(&self) -> bool;
    fn cond_ls(&self) -> bool;
    fn cond_cc(&self) -> bool;
    fn cond_cs(&self) -> bool;
    fn cond_ne(&self) -> bool;
    fn cond_eq(&self) -> bool;
    fn cond_vc(&self) -> bool;
    fn cond_vs(&self) -> bool;
    fn cond_pl(&self) -> bool;
    fn cond_mi(&self) -> bool;
    fn cond_ge(&self) -> bool;
    fn cond_lt(&self) -> bool;
    fn cond_gt(&self) -> bool;
    fn cond_le(&self) -> bool;
    fn branch_8(&mut self, offset: i8);
    fn branch_16(&mut self, offset: i16);
    fn read_imm_i16(&mut self) -> Result<i16>;
    fn read_imm_u16(&mut self) -> Result<u16>;
    fn read_imm_u32(&mut self) -> Result<u32>;
    fn jump(&mut self, pc: u32);
    fn push_32(&mut self, value: u32) -> u32;
    fn pop_32(&mut self) -> u32;
    fn push_16(&mut self, value: u16) -> u32;
    fn pop_16(&mut self) -> u16;
    fn push_sp(&mut self) -> u32;
    fn inactive_ssp(&self) -> u32;
    fn inactive_usp(&mut self) -> &mut u32;
    fn reset_external_devices(&mut self);
    fn resume_normal_processing(&mut self);
    fn stop_instruction_processing(&mut self);
    fn allow_tas_writeback(&mut self) -> bool;
}

pub struct ConfiguredCore<T: InterruptController, A: AddressBus> {
    pub pc: u32,
    pub inactive_ssp: u32, // when in user mode
    pub inactive_usp: u32, // when in supervisor mode
    pub ir: u16,
    pub dar: [u32; 16],
    instruction_set: InstructionSet<ConfiguredCore<T, A>>,
    pub s_flag: u32,
    pub irq_level: u8,
    pub int_mask: u32,
    pub int_ctrl: T,
    pub x_flag: u32,
    pub c_flag: u32,
    pub v_flag: u32,
    pub n_flag: u32,
    pub prefetch_addr: u32,
    pub prefetch_data: u32,
    pub not_z_flag: u32,
    pub processing_state: ProcessingState,
    pub mem: A,
}
impl<T: InterruptController, A: AddressBus> Core for ConfiguredCore<T, A> {
    fn dar(&mut self) -> &mut [u32; 16] {
        &mut self.dar
    }
    fn pc(&mut self) -> &mut u32 {
        &mut self.pc
    }
    fn ir(&mut self) -> u16 {
        self.ir
    }
    fn ax(&mut self) -> &mut u32 {
        let ax = ir_ax!(self);
        &mut self.dar[ax]
    }
    fn ay(&mut self) -> &mut u32 {
        let ay = ir_ay!(self);
        &mut self.dar[ay]
    }
    fn dx(&mut self) -> &mut u32 {
        let dx = ir_dx!(self);
        &mut self.dar[dx]
    }
    fn dy(&mut self) -> &mut u32 {
        let dy = ir_dy!(self);
        &mut self.dar[dy]
    }
    fn c_flag(&mut self) -> &mut u32{
        &mut self.c_flag
    }
    fn v_flag(&mut self) -> &mut u32{
        &mut self.v_flag
    }
    fn n_flag(&mut self) -> &mut u32{
        &mut self.n_flag
    }
    fn s_flag(&mut self) -> &mut u32{
        &mut self.s_flag
    }
    fn x_flag(&mut self) -> &mut u32{
        &mut self.x_flag
    }
    fn not_z_flag(&mut self) -> &mut u32{
        &mut self.not_z_flag
    }
    fn x_flag_as_1(&self) -> u32 {
        self.x_flag_as_1()
    }
    fn read_data_byte(&mut self, address: u32) -> Result<u32> {
        self.read_data_byte(address)
    }
    fn read_data_word(&mut self, address: u32) -> Result<u32> {
        self.read_data_word(address)
    }
    fn read_data_long(&mut self, address: u32) -> Result<u32> {
        self.read_data_long(address)
    }
    fn read_program_byte(&mut self, address: u32) -> Result<u32> {
        self.read_program_byte(address)
    }
    fn read_program_word(&mut self, address: u32) -> Result<u32> {
        self.read_program_word(address)
    }
    fn read_program_long(&mut self, address: u32) -> Result<u32> {
        self.read_program_long(address)
    }
    fn write_data_byte(&mut self, address: u32, value: u32) -> Result<()> {
        self.write_data_byte(address, value)
    }
    fn write_data_word(&mut self, address: u32, value: u32) -> Result<()> {
        self.write_data_word(address, value)
    }
    fn write_data_long(&mut self, address: u32, value: u32) -> Result<()> {
        self.write_data_long(address, value)
    }
    fn write_program_byte(&mut self, address: u32, value: u32) -> Result<()> {
        self.write_program_byte(address, value)
    }
    fn write_program_word(&mut self, address: u32, value: u32) -> Result<()> {
        self.write_program_word(address, value)
    }
    fn write_program_long(&mut self, address: u32, value: u32) -> Result<()> {
        self.write_program_long(address, value)
    }
    fn status_register(&self) -> u16 {
        self.status_register()
    }
    fn condition_code_register(&self) -> u16 {
        self.condition_code_register()
    }
    fn sr_to_flags(&mut self, sr: u16) {
        self.sr_to_flags(sr)
    }
    fn ccr_to_flags(&mut self, ccr: u16) {
        self.ccr_to_flags(ccr)
    }
    fn cond_t(&self) -> bool {
        true
    }
    fn cond_f(&self) -> bool {
        false
    }
    fn cond_hi(&self) -> bool {
        // high
        self.cond_cc() && self.cond_ne()
    }
    fn cond_ls(&self) -> bool {
        // low or same
        self.cond_cs() || self.cond_eq()
    }
    fn cond_cc(&self) -> bool {
        // carry clear (HI)
        self.c_flag & CFLAG_SET == 0
    }
    fn cond_cs(&self) -> bool {
        // carry set (LO)
        !self.cond_cc()
    }
    fn cond_eq(&self) -> bool {
        // equal
        (self.not_z_flag == ZFLAG_SET)
    }
    fn cond_ne(&self) -> bool {
        // not equal
        !self.cond_eq()
    }
    fn cond_vc(&self) -> bool {
        // overflow clear
        (self.v_flag & VFLAG_SET == 0)
    }
    fn cond_vs(&self) -> bool {
        // overflow set
        !self.cond_vc()
    }
    fn cond_pl(&self) -> bool {
        // plus
        (self.n_flag & NFLAG_SET == 0)
    }
    fn cond_mi(&self) -> bool {
        // minus
        !self.cond_pl()
    }
    fn cond_ge(&self) -> bool {
        // greater or equal
        self.cond_mi() && self.cond_vs() || self.cond_pl() && self.cond_vc()
    }
    fn cond_lt(&self) -> bool {
        // less than
        self.cond_mi() && self.cond_vc() || self.cond_pl() && self.cond_vs()
    }
    fn cond_gt(&self) -> bool {
        // greater than
        self.cond_ge() && self.cond_ne()
    }
    fn cond_le(&self) -> bool {
        // less or equal
        self.cond_lt() || self.cond_eq()
    }
    fn branch_8(&mut self, offset: i8) {
        self.pc = self.pc.wrapping_add(offset as u32);
    }
    fn branch_16(&mut self, offset: i16) {
        self.pc = self.pc.wrapping_add(offset as u32);
    }
    fn read_imm_i16(&mut self) -> Result<i16> {
        self.read_imm_i16()
    }
    fn read_imm_u16(&mut self) -> Result<u16> {
        self.read_imm_u16()
    }
    fn read_imm_u32(&mut self) -> Result<u32> {
        self.read_imm_u32()
    }
    fn jump(&mut self, pc: u32) {
        self.jump(pc)
    }
    fn push_32(&mut self, value: u32) -> u32 {
        self.push_32(value)
    }
    fn pop_32(&mut self) -> u32 {
        self.pop_32()
    }
    fn push_16(&mut self, value: u16) -> u32 {
        self.push_16(value)
    }
    fn pop_16(&mut self) -> u16 {
        self.pop_16()
    }
    fn push_sp(&mut self) -> u32 {
        self.push_sp()
    }
    fn inactive_ssp(&self) -> u32 {
        self.inactive_ssp
    }
    fn inactive_usp(&mut self) -> &mut u32 {
        &mut self.inactive_usp
    }
    fn reset_external_devices(&mut self) {
        self.int_ctrl.reset_external_devices()
    }
    fn resume_normal_processing(&mut self) {
        self.processing_state = ProcessingState::Normal;
    }
    fn stop_instruction_processing(&mut self) {
        self.processing_state = ProcessingState::Stopped;
    }
    fn allow_tas_writeback(&mut self) -> bool {
        true
    }
}
pub const STACK_POINTER_REG: usize = 15;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cycles(pub i32);

use std::ops::Sub;
use std::ops::Add;
impl Sub for Cycles {
    type Output = Cycles;

    fn sub(self, _rhs: Cycles) -> Cycles {
        Cycles(self.0 - _rhs.0)
    }
}
impl Add for Cycles {
    type Output = Cycles;

    fn add(self, _rhs: Cycles) -> Cycles {
        Cycles(self.0 + _rhs.0)
    }
}
impl Cycles {
    fn any(self) -> bool {
        self.0 > 0
    }
}

pub trait Callbacks {
    fn exception_callback(&mut self, core: &mut impl Core, ex: Exception) -> Result<Cycles>;
}

struct EmulateAllExceptions;
impl Callbacks for EmulateAllExceptions {
    fn exception_callback(&mut self, _: &mut impl Core, ex: Exception) -> Result<Cycles> {
        Err(ex)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProcessingState {
    Normal,             // Executing instructions
    Group2Exception,    // TRAP(V), CHK, ZeroDivide
    Group1Exception,    // Trace, Interrupt, IllegalInstruction, PrivilegeViolation
    Group0Exception,    // AddressError, BusError, ExternalReset
    Stopped,            // Trace, Interrupt or ExternalReset needed to resume
    Halted,             // ExternalReset needed to resume
}

impl ProcessingState {
    // The processor is processing an instruction if it is in the normal
    // state or processing a group 2 exception; the processor is not
    // processing an instruction if it is processing a group 0 or a group 1
    // exception. This info goes into a Group0 stack frame
    fn instruction_processing(self) -> bool {
        match self {
            ProcessingState::Normal => true,
            ProcessingState::Group2Exception => true,
            _ => false
        }
    }
    fn running(self) -> bool {
        match self {
            ProcessingState::Stopped => false,
            ProcessingState::Halted => false,
            _ => true
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AccessType {Read, Write}
use ram::AddressSpace;

#[derive(Clone, Copy, Debug)]
pub enum Exception {
    AddressError { address: u32, access_type: AccessType, processing_state: ProcessingState, address_space: AddressSpace},
    IllegalInstruction(u16, u32), // ir, pc
    Trap(u8, i32),                // trap no, exception cycles
    PrivilegeViolation(u16, u32), // ir, pc
    UnimplementedInstruction(u16, u32, u8), // ir, pc, vector no
    Interrupt(u8, u8), // irq, vector no
}
use std::fmt;
impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Exception::AddressError {
                address, access_type, processing_state, address_space
                } => write!(f, "Address Error: {:?} {:?} at {:08x} during {:?} processing", access_type, address_space, address, processing_state),
            Exception::IllegalInstruction(ir, pc) => write!(f, "Illegal Instruction {:04x} at {:08x}", ir, pc),
            Exception::Trap(num, ea_cyc) => write!(f, "Trap: {:02x} (ea cyc {})", num, ea_cyc),
            Exception::PrivilegeViolation(ir, pc) => write!(f, "Privilege Violation {:04x} at {:08x}", ir, pc),
            Exception::UnimplementedInstruction(ir, pc, _) => write!(f, "Unimplemented Instruction {:04x} at {:08x}", ir, pc),
            Exception::Interrupt(irq, vec) => write!(f, "Interrupt {:1x} (vector {:02x})", irq, vec),
        }
    }
}
use std::error;
impl error::Error for Exception {
    fn description(&self) -> &str {
         match *self {
            Exception::AddressError{..} => "Address Error",
            Exception::IllegalInstruction(_, _) => "Illegal Instruction",
            Exception::Trap(_, _) => "Trap",
            Exception::PrivilegeViolation(_, _) => "PrivilegeViolation",
            Exception::UnimplementedInstruction(_, _, _) => "UnimplementedInstruction",
            Exception::Interrupt(_, _) => "Interrupt",
         }
    }
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
use std::num::Wrapping;

// these values are borrowed from Musashi
// and not yet fully understood
const SFLAG_SET: u32 =  0x04;
const XFLAG_SET: u32 = 0x100;
const ZFLAG_SET: u32 = 0x00;
const NFLAG_SET: u32 =  0x80;
const VFLAG_SET: u32 =  0x80;
const CFLAG_SET: u32 = 0x100;
const CPU_SR_MASK: u16 = 0xa71f; /* T1 -- S  -- -- I2 I1 I0 -- -- -- X  N  Z  V  C  */
const CPU_SR_INT_MASK: u32 = 0x0700;

const VFLAG_CLEAR: u32 =  0x00;
const XFLAG_CLEAR: u32 =  0x00;
const NFLAG_CLEAR: u32 =  0x00;
const CFLAG_CLEAR: u32 =  0x00;
const SFLAG_CLEAR: u32 =  0x00;
const ZFLAG_CLEAR: u32 =  0xffff_ffff; // used as "non-z-flag"

// Exception Vectors
//pub const EXCEPTION_BUS_ERROR: u8               =  2;
pub const EXCEPTION_ADDRESS_ERROR: u8           =  3;
pub const EXCEPTION_ILLEGAL_INSTRUCTION: u8     =  4;
pub const EXCEPTION_ZERO_DIVIDE: u8             =  5;
pub const EXCEPTION_CHK: u8                     =  6;
pub const EXCEPTION_TRAPV: u8                   =  7;
pub const EXCEPTION_PRIVILEGE_VIOLATION: u8     =  8;
// pub const EXCEPTION_TRACE: u8                   =  9;
pub const EXCEPTION_UNIMPLEMENTED_1010: u8      = 10;
pub const EXCEPTION_UNIMPLEMENTED_1111: u8      = 11;
// pub const EXCEPTION_FORMAT_ERROR: u8            = 14;
// pub const EXCEPTION_UNINITIALIZED_INTERRUPT: u8 = 15;
// pub const EXCEPTION_SPURIOUS_INTERRUPT: u8      = 24;
// pub const EXCEPTION_INTERRUPT_AUTOVECTOR: u8    = 24;
pub const EXCEPTION_TRAP_BASE: u8               = 32;

impl TestCore {
    pub fn new(base: u32) -> TestCore {
        TestCore {
            pc: base, prefetch_addr: 0, prefetch_data: 0, inactive_ssp: 0, inactive_usp: 0, ir: 0, processing_state: ProcessingState::Group0Exception,
            dar: [0u32; 16], mem: LoggingMem::new(0xaaaa_aaaa, OpsLogger::new()), instruction_set: ops::instruction_set(),
            irq_level: 0, int_ctrl: AutoInterruptController::new(),
            s_flag: SFLAG_SET, int_mask: CPU_SR_INT_MASK, x_flag: 0, v_flag: 0, c_flag: 0, n_flag: 0, not_z_flag: 0xffff_ffff
        }
    }
    pub fn new_auto() -> TestCore {
        TestCore::new_with(0, AutoInterruptController::new(), LoggingMem::new(0xaaaa_aaaa, OpsLogger::new()))
    }
    pub fn new_mem(base: u32, contents: &[u8]) -> TestCore {
        TestCore::new_mem_init(base, contents, 0xaaaa_aaaa)
    }
    pub fn new_mem_init(base: u32, contents: &[u8], initializer: u32) -> TestCore {
        let mut lm = LoggingMem::new(initializer, OpsLogger::new());
        for (offset, byte) in contents.iter().enumerate() {
            lm.write_u8(base + offset as u32, u32::from(*byte));
        }
        TestCore {
            pc: base, prefetch_addr: 0, prefetch_data: 0, inactive_ssp: 0, inactive_usp: 0, ir: 0, processing_state: ProcessingState::Normal,
            dar: [0u32; 16], mem: lm, instruction_set: ops::instruction_set(),
            irq_level: 0, int_ctrl: AutoInterruptController::new(),
            s_flag: SFLAG_SET, int_mask: CPU_SR_INT_MASK, x_flag: 0, v_flag: 0, c_flag: 0, n_flag: 0, not_z_flag: 0xffff_ffff
        }
    }
}

impl<T: InterruptController, A: AddressBus> ConfiguredCore<T, A> {
    pub fn new_with(base: u32, int_ctrl: T, memory: A) -> ConfiguredCore<T, A> {
        ConfiguredCore {
            pc: base, prefetch_addr: 0, prefetch_data: 0, inactive_ssp: 0, inactive_usp: 0, ir: 0, processing_state: ProcessingState::Group0Exception,
            dar: [0u32; 16], mem: memory, instruction_set: ops::instruction_set(),
            irq_level: 0, int_ctrl,
            s_flag: SFLAG_SET, int_mask: CPU_SR_INT_MASK, x_flag: 0, v_flag: 0, c_flag: 0, n_flag: 0, not_z_flag: 0xffff_ffff
        }
    }
    pub fn reset(&mut self) {
        self.processing_state = ProcessingState::Group0Exception;
        self.s_flag = SFLAG_SET;
        self.int_mask = CPU_SR_INT_MASK;
        self.prefetch_addr = 1; // non-zero, or the prefetch won't kick in
        self.jump(0);
        // these reads cannot possibly cause AddressError, as we forced PC to 0
        self.dar[15] = self.read_imm_u32().unwrap();
        let new_pc = self.read_imm_u32().unwrap();
        self.jump(new_pc);
        self.processing_state = ProcessingState::Normal;
    }
    pub fn x_flag_as_1(&self) -> u32 {
        (self.x_flag>>8)&1
    }
    // admittely I've chosen to reuse Musashi's representation of flags
    // which I don't fully understand (they are not matching their
    // positions in the SR/CCR)
    pub fn status_register(&self) -> u16 {
        ((self.s_flag << 11)                |
        self.int_mask                        |
        ((self.x_flag & XFLAG_SET) >> 4)    |
        ((self.n_flag & NFLAG_SET) >> 4)    |
        ((not1!(self.not_z_flag))  << 2)    |
        ((self.v_flag & VFLAG_SET) >> 6)    |
        ((self.c_flag & CFLAG_SET) >> 8)) as u16
    }
    pub fn condition_code_register(&self) -> u16 {
        self.status_register() & 0xff
    }
    pub fn usp(&self) -> u32 {
        if self.s_flag > 0 {
            self.inactive_usp
        } else {
            self.dar[15]
        }
    }
    pub fn ssp(&self) -> u32 {
        if self.s_flag > 0 {
            self.dar[15]
        } else {
            self.inactive_ssp
        }
    }
    // admittely I've chosen to reuse Musashi's representation of flags
    // which I don't fully understand (they are not matching their
    // positions in the SR/CCR)
    pub fn sr_to_flags(&mut self, sr: u16) {
        let sr = u32::from(sr & CPU_SR_MASK);
        let old_sflag = self.s_flag;
        self.int_mask = sr & CPU_SR_INT_MASK;
        self.s_flag =           (sr >> 11) & SFLAG_SET;
        self.x_flag =            (sr <<  4) & XFLAG_SET;
        self.n_flag =            (sr <<  4) & NFLAG_SET;
        self.not_z_flag = not1!(sr & 0b00100);
        self.v_flag =            (sr <<  6) & VFLAG_SET;
        self.c_flag =            (sr <<  8) & CFLAG_SET;
        if old_sflag != self.s_flag {
            if self.s_flag == SFLAG_SET {
                self.inactive_usp = self.dar[15];
                self.dar[15] = self.inactive_ssp;
            } else {
                self.inactive_ssp = self.dar[15];
                self.dar[15] = self.inactive_usp;
            }
        }
        // println!("{} {:016b} {} {}", self.flags(), sr, self.not_z_flag, sr & 0b00100);
    }
    pub fn ccr_to_flags(&mut self, ccr: u16) {
        let sr = self.status_register();
        self.sr_to_flags((sr & 0xff00) | (ccr & 0xff));
    }

    pub fn flags(&self) -> String {
        let sr = self.status_register();
        let supervisor = (sr >> 13) & 1;
        let irq_mask = (0x700 & sr) >> 8;

        format!("-{}{}{}{}{}{}{}",
        if supervisor > 0 {'S'} else {'U'},
        irq_mask,
        if 0 < (sr >> 4) & 1 {'X'} else {'-'},
        if 0 < (sr >> 3) & 1 {'N'} else {'-'},
        if 0 < (sr >> 2) & 1 {'Z'} else {'-'},
        if 0 < (sr >> 1) & 1 {'V'} else {'-'},
        if 0 < (sr     ) & 1 {'C'} else {'-'})
    }
    fn prefetch_if_needed(&mut self) -> bool {
        // does current PC overlap with fetched data
        let fetched = if self.pc & !3 != self.prefetch_addr {
            self.prefetch_addr = self.pc & !3;
            let address_space = if self.s_flag != 0 {SUPERVISOR_PROGRAM} else {USER_PROGRAM};
            self.prefetch_data = self.mem.read_long(address_space, self.prefetch_addr);
            true
        } else {
            false
        };
        self.pc = self.pc.wrapping_add(2);
        fetched
    }
    pub fn read_imm_u32(&mut self) -> Result<u32> {
        if self.pc & 1 > 0 {
            let address_space = if self.s_flag != 0 {SUPERVISOR_PROGRAM} else {USER_PROGRAM};
            return Err(Exception::AddressError{address: self.pc, access_type: AccessType::Read, address_space, processing_state: self.processing_state})
        }
        self.prefetch_if_needed();
        let prev_prefetch_data = self.prefetch_data;
        Ok(if self.prefetch_if_needed() {
            ((prev_prefetch_data << 16) | (self.prefetch_data >> 16))
        } else {
            prev_prefetch_data
        })
    }
    pub fn read_imm_i16(&mut self) -> Result<i16> {
        self.read_imm_u16().map(|val| val as i16)
    }
    pub fn read_imm_u16(&mut self) -> Result<u16> {
        // the Musashi read_imm_16 calls cpu_read_long as part of prefetch
        if self.pc & 1 > 0 {
            let address_space = if self.s_flag != 0 {SUPERVISOR_PROGRAM} else {USER_PROGRAM};
            return Err(Exception::AddressError{address: self.pc, access_type: AccessType::Read, address_space, processing_state: self.processing_state})
        }
        self.prefetch_if_needed();
        Ok(((self.prefetch_data >> ((2 - ((self.pc.wrapping_sub(2)) & 2))<<3)) & 0xffff) as u16)
    }
    pub fn push_sp(&mut self) -> u32 {
         let new_sp = (Wrapping(self.dar[15]) - Wrapping(4)).0;
         self.dar[15] = new_sp;
         self.write_data_long(new_sp, new_sp).unwrap();
         new_sp
    }
    pub fn push_32(&mut self, value: u32) -> u32 {
         let new_sp = (Wrapping(self.dar[15]) - Wrapping(4)).0;
         self.dar[15] = new_sp;
         self.write_data_long(new_sp, value).unwrap();
         new_sp
    }
    pub fn pop_32(&mut self) -> u32 {
        let sp = self.dar[15];
        let data = self.read_data_long(sp).unwrap();
        self.dar[15] = sp.wrapping_add(4);
        data
    }
    pub fn push_16(&mut self, value: u16) -> u32 {
         let new_sp = (Wrapping(self.dar[15]) - Wrapping(2)).0;
         self.dar[15] = new_sp;
         self.write_data_word(new_sp, u32::from(value)).unwrap();
         new_sp
    }
    pub fn pop_16(&mut self) -> u16 {
        let sp = self.dar[15];
        let data = self.read_data_word(sp).unwrap() as u16;
        self.dar[15] = sp.wrapping_add(2);
        data
    }
    pub fn read_data_byte(&mut self, address: u32) -> Result<u32> {
        let address_space = if self.s_flag != 0 {SUPERVISOR_DATA} else {USER_DATA};
        Ok(self.mem.read_byte(address_space, address))
    }
    pub fn read_program_byte(&mut self, address: u32) -> Result<u32> {
        let address_space = if self.s_flag != 0 {SUPERVISOR_PROGRAM} else {USER_PROGRAM};
        Ok(self.mem.read_byte(address_space, address))
    }
    pub fn write_data_byte(&mut self, address: u32, value: u32) -> Result<()> {
        let address_space = if self.s_flag != 0 {SUPERVISOR_DATA} else {USER_DATA};
        self.mem.write_byte(address_space, address, value);
        Ok(())
    }
    pub fn write_program_byte(&mut self, address: u32, value: u32) -> Result<()> {
        let address_space = if self.s_flag != 0 {SUPERVISOR_PROGRAM} else {USER_PROGRAM};
        self.mem.write_byte(address_space, address, value);
        Ok(())
    }
    pub fn read_data_word(&mut self, address: u32) -> Result<u32> {
        let address_space = if self.s_flag != 0 {SUPERVISOR_DATA} else {USER_DATA};
        if address & 1 > 0 {
            Err(Exception::AddressError{address, access_type: AccessType::Read, address_space, processing_state: self.processing_state})
        } else {
            Ok(self.mem.read_word(address_space, address))
        }
    }
    pub fn read_program_word(&mut self, address: u32) -> Result<u32> {
        let address_space = if self.s_flag != 0 {SUPERVISOR_PROGRAM} else {USER_PROGRAM};
        if address & 1 > 0 {
            Err(Exception::AddressError {address, access_type: AccessType::Read, address_space, processing_state: self.processing_state})
        } else {
            Ok(self.mem.read_word(address_space, address))
        }
    }
    pub fn write_data_word(&mut self, address: u32, value: u32) -> Result<()> {
        let address_space = if self.s_flag != 0 {SUPERVISOR_DATA} else {USER_DATA};
        if address & 1 > 0 {
            Err(Exception::AddressError{address, access_type: AccessType::Write, address_space, processing_state: self.processing_state})
        } else {
            self.mem.write_word(address_space, address, value);
            Ok(())
        }
    }
    pub fn write_program_word(&mut self, address: u32, value: u32) -> Result<()> {
        let address_space = if self.s_flag != 0 {SUPERVISOR_PROGRAM} else {USER_PROGRAM};
        if address & 1 > 0 {
            Err(Exception::AddressError{address, access_type: AccessType::Write, address_space, processing_state: self.processing_state})
        } else {
            self.mem.write_word(address_space, address, value);
            Ok(())
        }
    }
    pub fn read_data_long(&mut self, address: u32) -> Result<u32> {
        let address_space = if self.s_flag != 0 {SUPERVISOR_DATA} else {USER_DATA};
        if address & 1 > 0 {
            Err(Exception::AddressError{address, access_type: AccessType::Read, address_space, processing_state: self.processing_state})
        } else {
            Ok(self.mem.read_long(address_space, address))
        }
    }
    pub fn read_program_long(&mut self, address: u32) -> Result<u32> {
        let address_space = if self.s_flag != 0 {SUPERVISOR_PROGRAM} else {USER_PROGRAM};
        if address & 1 > 0 {
            Err(Exception::AddressError{address, access_type: AccessType::Read, address_space, processing_state: self.processing_state})
        } else {
            Ok(self.mem.read_long(address_space, address))
        }
    }
    pub fn write_data_long(&mut self, address: u32, value: u32) -> Result<()> {
        let address_space = if self.s_flag != 0 {SUPERVISOR_DATA} else {USER_DATA};
        if address & 1 > 0 {
            Err(Exception::AddressError{address, access_type: AccessType::Write, address_space, processing_state: self.processing_state})
        } else {
            self.mem.write_long(address_space, address, value);
            Ok(())
        }
    }
    pub fn write_program_long(&mut self, address: u32, value: u32) -> Result<()> {
        let address_space = if self.s_flag != 0 {SUPERVISOR_PROGRAM} else {USER_PROGRAM};
        if address & 1 > 0 {
            Err(Exception::AddressError{address, access_type: AccessType::Write, address_space, processing_state: self.processing_state})
        } else {
            self.mem.write_long(address_space, address, value);
            Ok(())
        }
    }
    pub fn jump(&mut self, pc: u32) {
        self.pc = pc;
    }
    pub fn jump_vector(&mut self, vector: u8) {
        let vector_address = u32::from(vector) << 2;
        self.pc = self.read_data_long(vector_address).unwrap();
    }
    pub fn ensure_supervisor_mode(&mut self) -> u16 {
        let backup_sr = self.status_register();
        // if in user mode, swap stack pointers!
        if self.s_flag == SFLAG_CLEAR {
            self.inactive_usp = self.dar[15];
            self.dar[15] = self.inactive_ssp;
        }
        // enter supervisor mode
        self.s_flag = SFLAG_SET;
        backup_sr
    }
    pub fn handle_address_error(&mut self, bad_address: u32, access_type: AccessType, processing_state: ProcessingState, address_space: AddressSpace) -> Cycles
    {
        if processing_state == ProcessingState::Group0Exception {
            self.processing_state = ProcessingState::Halted;
            return Cycles(0);
        }
        self.processing_state = ProcessingState::Group0Exception;
        let backup_sr = self.ensure_supervisor_mode();

        // Bus error stack frame (68000 only).
        let (pc, ir) = (self.pc, self.ir);
        self.push_32(pc);
        self.push_16(backup_sr);
        self.push_16(ir);
        self.push_32(bad_address);    /* access address */
        /* 0 0 0 0 0 0 0 0 0 0 0 R/W I/N FC
         * R/W  0 = write, 1 = read
         * I/N  0 = instruction, 1 = not
         * FC   3-bit function code
         */
        let access_info = match access_type {AccessType::Read => 0b10000, _ => 0 } |
            if processing_state.instruction_processing() { 0 } else { 0b01000 } |
            (address_space.fc() as u16);
        self.push_16(access_info);
        self.jump_vector(EXCEPTION_ADDRESS_ERROR);
        Cycles(50)
    }
    pub fn handle_unimplemented_instruction(&mut self, pc: u32, vector: u8) -> Cycles {
        // somewhat unclear if the unimplemented instruction exceptions
        // are Group 1 or 2 exceptions. They are mentioned together with
        // "illegal instruction" which is clearly defined as a group 1
        // exception,  but the text ("6.3.6 Illegal and Unimplemented
        // Instructions" in the M68000UM) mentions that illegal
        // instructions push a group 2 stack frame. On the 68000 G1 and
        // G2 exception stack frames are identical, so maybe it doesn't
        // really matter. EASy68k considers them group 2 exceptions. For
        // the time being, we do too.
        self.handle_exception(ProcessingState::Group2Exception, pc, vector, 34)
    }
    pub fn handle_illegal_instruction(&mut self, pc: u32) -> Cycles {
        self.handle_exception(ProcessingState::Group1Exception, pc, EXCEPTION_ILLEGAL_INSTRUCTION, 34)
    }
    pub fn handle_privilege_violation(&mut self, pc: u32) -> Cycles {
        self.handle_exception(ProcessingState::Group1Exception, pc, EXCEPTION_PRIVILEGE_VIOLATION, 34)
    }
    pub fn handle_trap(&mut self, trap: u8, cycles: i32) -> Cycles {
        let pc = self.pc;
        self.handle_exception(ProcessingState::Group2Exception, pc, trap, cycles)
    }

    pub fn handle_exception(&mut self, new_state: ProcessingState, pc: u32, vector: u8, cycles: i32) -> Cycles {
        self.processing_state = new_state;
        let backup_sr = self.ensure_supervisor_mode();

        // Group 1 and 2 stack frame (68000 only).
        self.push_32(pc);
        self.push_16(backup_sr);

        self.jump_vector(vector);
        Cycles(cycles)
    }

    pub fn handle_interrupt(&mut self, irq_level: u8, vector: u8) -> Cycles {
        let pc = self.pc;
        self.processing_state = ProcessingState::Group1Exception;
        let backup_sr = self.ensure_supervisor_mode();
        // new mask set here, in order to exclude from backup_sr
        self.int_mask = u32::from(irq_level) << 8;
        self.irq_level = irq_level;

        // Musashi jumps first, and stacks later for interrupts,
        // but the other way around for exceptions
        self.jump_vector(vector);

        // Group 1 and 2 stack frame (68000 only).
        self.push_32(pc);
        self.push_16(backup_sr);

        // 44 cycles for an interrupt according to MC68000UM, Table 8-14
        // The interrupt acknowledge cycle is assumed to take four clock periods
        Cycles(44)
    }
    fn stopped_with_pending_interrups(&mut self) -> bool {
        self.processing_state == ProcessingState::Stopped && self.pending_interrupt().is_some()
    }
    fn can_execute(&mut self) -> bool {
        self.processing_state.running() || self.stopped_with_pending_interrups()
    }
    fn pending_interrupt(&self) -> Option<u8> {
        let old_level = self.irq_level;
        let new_level = self.int_ctrl.highest_priority();
        let edge_triggered_nmi = old_level != 7 && new_level == 7;
        if u32::from(new_level) << 8 > self.int_mask || edge_triggered_nmi {
            Some(new_level)
        } else {
            None
        }
    }
    pub fn read_instruction(&mut self) -> Result<u16> {
        // first check for interrupts
        if let Some(irq) = self.pending_interrupt() {
            let vector = self.int_ctrl.acknowledge_interrupt(irq).unwrap_or(SPURIOUS_INTERRUPT);
            Err(Exception::Interrupt(irq, vector))
        } else {
            // not interrupted, read instruction from PC
            self.read_imm_u16()
        }
    }
    pub fn execute1(&mut self) -> Cycles {
        self.execute(1)
    }
    pub fn execute(&mut self, cycles: i32) -> Cycles {
        self.execute_with_state(cycles, &mut EmulateAllExceptions)
    }
    pub fn execute_with_state<S: Callbacks>(&mut self, cycles: i32, state: &mut S) -> Cycles {
        let cycles = Cycles(cycles);
        let mut remaining_cycles = cycles;
        while remaining_cycles.any() && self.can_execute() {
            // Read an instruction from PC (increments PC by 2)
            let result = self.read_instruction().and_then(|opcode| {
                    self.ir = opcode;
                    // Call instruction handler to mutate Core accordingly
                    self.instruction_set[opcode as usize](self)
                });
            remaining_cycles = remaining_cycles - match result {
                Ok(cycles_used) => cycles_used,
                Err(ex) => {
                    match state.exception_callback(self, ex) {
                        Ok(cycles_used) => cycles_used,
                        Err(Exception::AddressError { address, access_type, processing_state, address_space }) =>
                            self.handle_address_error(address, access_type, processing_state, address_space),
                        Err(Exception::IllegalInstruction(_, pc)) =>
                            self.handle_illegal_instruction(pc),
                        Err(Exception::UnimplementedInstruction(_, pc, vector)) =>
                            self.handle_unimplemented_instruction(pc, vector),
                        Err(Exception::Trap(num, ea_calculation_cycles)) =>
                            self.handle_trap(num, ea_calculation_cycles),
                        Err(Exception::PrivilegeViolation(_, pc)) =>
                            self.handle_privilege_violation(pc),
                        Err(Exception::Interrupt(irq, vec)) =>
                            self.handle_interrupt(irq, vec),
                    }
                }
            };
        }
        if self.processing_state.running() {
            cycles - remaining_cycles
        } else {
            // if not running, consume all available cycles
            // including overconsumed cycles
            let adjust = if remaining_cycles.0 < 0 { remaining_cycles } else { Cycles(0) };
            cycles - adjust
        }
    }
}

impl Clone for TestCore {
    fn clone(&self) -> Self {
        let mut lm = LoggingMem::new(self.mem.initializer, OpsLogger::new());
        lm.copy_from(&self.mem);
        assert_eq!(0, lm.logger.len());
        TestCore {
            pc: self.pc, prefetch_addr: 0, prefetch_data: 0, inactive_ssp: self.inactive_ssp, inactive_usp: self.inactive_usp, ir: self.ir, processing_state: self.processing_state,
            dar: self.dar, mem: lm, instruction_set: ops::instruction_set(),
            irq_level: 0, int_ctrl: AutoInterruptController::new(),
            s_flag: self.s_flag, int_mask: self.int_mask, x_flag: self.x_flag, v_flag: self.v_flag, c_flag: self.c_flag, n_flag: self.n_flag, not_z_flag: self.not_z_flag
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{TestCore, Cycles};
    use super::ops; //::instruction_set;
    use ram::{AddressBus, SUPERVISOR_PROGRAM, USER_PROGRAM, USER_DATA};
    use ram::loggingmem::Operation;
    use cpu::ops::opcodes;
    use r68k_common::constants;

    #[test]
    fn new_sets_pc() {
        let cpu = TestCore::new(256);
        assert_eq!(256, cpu.pc);
    }

    #[test]
    fn new_mem_sets_pc_and_mem() {
        let base = 128;
        let cpu = TestCore::new_mem(base, &[1u8, 2u8, 3u8, 4u8, 5u8, 6u8]);
        assert_eq!(128, cpu.pc);
        assert_eq!(1, cpu.mem.read_byte(SUPERVISOR_PROGRAM, 128));
        assert_eq!(2, cpu.mem.read_byte(SUPERVISOR_PROGRAM, 129));
    }

    #[test]
    fn a_jump_changes_pc() {
        let mut cpu = TestCore::new(0);
        cpu.jump(128);
        assert_eq!(128, cpu.pc);
    }

    #[test]
    #[allow(unused_must_use)]
    fn a_read_imm_u32_changes_pc() {
        let base = 128;
        let mut cpu = TestCore::new(base);
        cpu.read_imm_u32();
        assert_eq!(base+4, cpu.pc);
    }

    #[test]
    fn a_read_imm_u32_reads_from_pc() {
        let base = 128;
        let mut cpu = TestCore::new_mem(base, &[2u8, 1u8, 3u8, 4u8]);
        let val = cpu.read_imm_u32().unwrap();
        assert_eq!((2<<24)+(1<<16)+(3<<8)+4, val);
    }


    #[test]
    #[allow(unused_must_use)]
    fn a_read_imm_u16_changes_pc() {
        let base = 128;
        let mut cpu = TestCore::new(base);
        cpu.read_imm_u16();
        assert_eq!(base+2, cpu.pc);
    }

    #[test]
    fn a_read_imm_u16_reads_from_pc() {
        let base = 128;
        let mut cpu = TestCore::new_mem(base, &[2u8, 1u8, 3u8, 4u8]);
        assert_eq!("-S7-----", cpu.flags());

        let val = cpu.read_imm_u16().unwrap();
        assert_eq!((2<<8)+(1<<0), val);
        assert_eq!(Operation::ReadLong(SUPERVISOR_PROGRAM, base, 0x02010304), cpu.mem.logger.ops()[0]);
    }

    #[test]
    fn an_user_mode_read_imm_u16_is_reflected_in_mem_ops() {
        let base = 128;
        let mut cpu = TestCore::new_mem(base, &[2u8, 1u8, 3u8, 4u8]);
        cpu.s_flag = 0;
        assert_eq!("-U7-----", cpu.flags());

        let val = cpu.read_imm_u16().unwrap();
        assert_eq!((2<<8)+(1<<0), val);
        assert_eq!(Operation::ReadLong(USER_PROGRAM, base, 0x02010304), cpu.mem.logger.ops()[0]);
    }

    #[test]
    fn a_reset_reads_sp_and_pc_from_0() {
        let mut cpu = TestCore::new_mem(0, &[0u8,0u8,1u8,0u8, 0u8,0u8,0u8,128u8]);
        cpu.reset();
        assert_eq!(256, cpu.dar[15]);
        assert_eq!(128, cpu.pc);
        assert_eq!("-S7-----", cpu.flags());
        assert_eq!(Operation::ReadLong(SUPERVISOR_PROGRAM, 0, 0x100), cpu.mem.logger.ops()[0]);
    }

    #[test]
    fn execute_reads_from_pc_and_does_not_panic_on_illegal_instruction() {
        let mut cpu = TestCore::new_mem(0xba, &[0xba,0xd1,1u8,0u8, 0u8,0u8,0u8,128u8]);
        cpu.execute1();
    }
    #[test]
    fn execute_does_not_panic_on_odd_pc() {
        let mut cpu = TestCore::new_mem(0xbd, &[0x00, 0x0a, 0x00, 0x00]);
        cpu.execute1();
    }

    #[test]
    fn execute_can_execute_instruction_handler_0a() {
        let mut cpu = TestCore::new_mem(0xba, &[0x00, 0x0A, 1u8,0u8, 0u8,0u8,0u8,128u8]);
        cpu.instruction_set = ops::fake::instruction_set();
        cpu.execute1();
        assert_eq!(0xabcd, cpu.dar[0]);
        assert_eq!(0x0000, cpu.dar[1]);
    }

    #[test]
    fn execute_can_execute_instruction_handler_0b() {
        let mut cpu = TestCore::new_mem(0xba, &[0x00, 0x0B, 1u8,0u8, 0u8,0u8,0u8,128u8]);
        cpu.instruction_set = ops::fake::instruction_set();
        cpu.execute1();
        assert_eq!(0x0000, cpu.dar[0]);
        assert_eq!(0xbcde, cpu.dar[1]);
    }


    #[test]
    fn execute_can_execute_set_dx() {
        // first byte 40 is register D0
        // 42 == D1
        // 44 == D2
        // 46 == D3
        // 48 == D4
        // 4a == D5
        // 4c == D6
        // 4e == D7
        let mut cpu = TestCore::new_mem(0x40, &[0x4c, 0x00, 1u8, 0u8]);
        cpu.instruction_set = ops::fake::instruction_set();
        cpu.execute1();
        assert_eq!(0xcdef, cpu.dar[6]);
    }

    #[test]
    fn array_elems() {
        let mut arr = [1, 2, 3, 4];
        let marr = &mut arr;
        let elem: &mut i32 = &mut (marr[1]);
        // let mut elem2: &mut i32 = &mut (arr[2]);
        assert_eq!(2, *elem);
        *elem = 200;
        assert_eq!(200, *elem);
        // assert_eq!(200, &mut marr[1]);
    }
    #[test]
    fn cycle_counting() {
        // 0xc308 = abcd_8_mm taking 18 cycles
        let mut cpu = TestCore::new_mem(0x40, &[0xc3, 0x08]);
        let Cycles(count) = cpu.execute1();
        assert_eq!(18, count);
    }
    #[test]
    fn cycle_counting_exec2() {
        // 0xc308 = abcd_8_mm taking 18 cycles
        let mut cpu = TestCore::new_mem(0x40, &[0xc3, 0x08, 0xc3, 0x08]);
        let Cycles(count) = cpu.execute(20);
        assert_eq!(18*2, count);
    }

    #[test]
    fn abcd_8_rr() {
        // opcodes c100 - c107, c300 - c307, etc.
        // or more generally c[13579bdf]0[0-7]
        // where [13579bdf] is DX (dest regno) and [0-7] is DY (src regno)
        // so c300 means D1 = D0 + D1 in BCD
        let mut cpu = TestCore::new_mem(0x40, &[0xc3, 0x00]);

        cpu.dar[0] = 0x16;
        cpu.dar[1] = 0x26;
        cpu.execute1();

        // 16 + 26 is 42
        assert_eq!(0x42, cpu.dar[1]);
    }
    #[test]
    fn abcd_8_mm() {
        // opcodes c108 - c10f, c308 - c30f, etc.
        // or more generally c[13579bdf]0[8-f]
        // where [13579bdf] is AX (dest regno) and [8-f] is AY (src regno)
        // so c308 means A1 = A0 + A1 in BCD
        let mut cpu = TestCore::new_mem(0x40, &[0xc3, 0x08]);

        cpu.dar[8+0] = 0x160+1;
        cpu.dar[8+1] = 0x260+1;
        cpu.mem.write_byte(USER_DATA, 0x160, 0x16);
        cpu.mem.write_byte(USER_DATA, 0x260, 0x26);
        cpu.execute1();
        let res = cpu.mem.read_byte(USER_DATA, 0x260);

        // 16 + 26 is 42
        assert_eq!(0x42, res);
    }

    #[test]
    fn add_8_er_d() {
        // opcodes d000 - d007, d200 - d207, etc.
        // or more generally d[02468ace]0[0-7]
        // where [02468ace] is DX (dest regno) and [0-7] is DY (src regno)

        // opcodes d200 is ADD.B    D0, D1
        let mut cpu = TestCore::new_mem(0x40, &[0xd2, 0x00]);

        cpu.dar[0] = 16;
        cpu.dar[1] = 26;
        cpu.execute1();

        // 16 + 26 is 42
        assert_eq!(42, cpu.dar[1]);
    }

    #[test]
    fn add_8_er_pi() {
        // opcodes d018 - d01f, d218 - d21f, etc.
        // or more generally d[02468ace]1[8-f]
        // where [02468ace] is DX (dest regno) and [8-f] is AY (src regno)

        // opcodes d218 is ADD.B    (A0)+, D1
        let mut cpu = TestCore::new_mem(0x40, &[0xd2, 0x18]);
        let addr = 0x100;
        cpu.dar[8+0] = addr;
        cpu.mem.write_byte(USER_DATA, addr, 16);
        cpu.dar[1] = 26;
        cpu.execute1();

        // 16 + 26 is 42
        assert_eq!(42, cpu.dar[1]);
        assert_eq!(addr+1, cpu.dar[8+0]);
    }

    #[test]
    fn add_8_er_pd() {
        // opcodes d020 - d027, d220 - d227, etc.
        // or more generally d[02468ace]2[0-7]
        // where [02468ace] is DX (dest regno) and [0-7] is AY (src regno)

        // opcodes d220 is ADD.B    -(A0), D1
        let mut cpu = TestCore::new_mem(0x40, &[0xd2, 0x20]);
        let addr = 0x100;
        cpu.dar[8+0] = addr;
        cpu.mem.write_byte(USER_DATA, addr-1, 16);
        cpu.dar[1] = 26;
        cpu.execute1();

        // 16 + 26 is 42
        assert_eq!(42, cpu.dar[1]);
        assert_eq!(addr-1, cpu.dar[8+0]);
    }

    #[test]
    fn add_8_er_ai() {
        // opcodes d010 - d017, d210 - d217, etc.
        // or more generally d[02468ace]1[0-7]
        // where [02468ace] is DX (dest regno) and [0-7] is AY (src regno)

        // opcodes d210 is ADD.B    (A0), D1
        let mut cpu = TestCore::new_mem(0x40, &[0xd2, 0x10]);

        let addr = 0x100;
        cpu.dar[8+0] = addr;
        cpu.mem.write_byte(USER_DATA, addr, 16);
        cpu.dar[1] = 26;
        cpu.execute1();

        // 16 + 26 is 42
        assert_eq!(42, cpu.dar[1]);
        assert_eq!(addr, cpu.dar[8+0]);
    }
    #[test]
    fn add_8_er_di_with_positive_displacement() {
        // opcodes d028 - d02f, d228 - d22f, etc.
        // or more generally d[02468ace]2[8-f]
        // where [02468ace] is DX (dest regno) and [8-f] is AY (src regno)

        // opcodes d228,0108 is ADD.B    (0x108, A0), D1
        let mut cpu = TestCore::new_mem(0x40, &[0xd2, 0x28, 0x01, 0x08]);

        let addr = 0x100;
        cpu.dar[8+0] = addr;
        let displaced_addr = addr + 0x108;
        cpu.mem.write_byte(USER_DATA, displaced_addr, 16);
        cpu.dar[1] = 26;
        cpu.execute1();

        // 16 + 26 is 42
        assert_eq!(42, cpu.dar[1]);
        assert_eq!(addr, cpu.dar[8+0]);
    }
    #[test]
    fn add_8_er_di_with_negative_displacement() {
        // opcodes d028 - d02f, d228 - d22f, etc. followed by an extension word
        // or more generally d[02468ace]2[8-f]
        // where [02468ace] is DX (dest regno) and [8-f] is AY (src regno)

        // opcodes d228,FFFE is ADD.B    (-2, A0), D1
        let mut cpu = TestCore::new_mem(0x40, &[0xd2, 0x28, 0xFF, 0xFE]);

        let addr = 0x100;
        cpu.dar[8+0] = addr;
        let displaced_addr = addr - 2;
        cpu.mem.write_byte(USER_DATA, displaced_addr, 16);
        cpu.dar[1] = 26;
        cpu.execute1();

        // 16 + 26 is 42
        assert_eq!(42, cpu.dar[1]);
        assert_eq!(addr, cpu.dar[8+0]);
    }
    #[test]
    fn add_8_er_ix_with_positive_displacement() {
        // opcodes d030 - d037, d230 - d237, etc. followed by an extension word
        // or more generally d[02468ace]3[0-7]
        // where [02468ace] is DX (dest regno) and [0-7] is AY (src regno)

        // opcodes d230,9002 is ADD.B    (2, A0, A1), D1
        let mut cpu = TestCore::new_mem(0x40, &[0xd2, 0x30, 0x90, 0x02]);

        let addr = 0x100;
        let index = 0x10;
        let displacement = 2;
        cpu.dar[8+0] = addr;
        cpu.dar[8+1] = index;
        let effective_addr = addr + index + displacement;
        cpu.mem.write_byte(USER_DATA, effective_addr, 16);
        cpu.dar[1] = 26;
        cpu.execute1();

        // 16 + 26 is 42
        assert_eq!(42, cpu.dar[1]);
        assert_eq!(addr, cpu.dar[8+0]);
    }
    #[test]
    fn add_8_er_ix_with_negative_displacement() {
        // opcodes d030 - d037, d230 - d237, etc. followed by an extension word
        // or more generally d[02468ace]3[0-7]
        // where [02468ace] is DX (dest regno) and [0-7] is AY (src regno)

        // opcodes d230,90FE is ADD.B    (-2, A0, A1), D1
        let mut cpu = TestCore::new_mem(0x40, &[0xd2, 0x30, 0x90, 0xFE]);

        let addr = 0x100;
        let index = 0x10;
        let displacement = 2;
        cpu.dar[8+0] = addr;
        cpu.dar[8+1] = index;
        let effective_addr = addr + index - displacement;
        cpu.mem.write_byte(USER_DATA, effective_addr, 16);
        cpu.dar[1] = 26;
        cpu.execute1();

        // 16 + 26 is 42
        assert_eq!(42, cpu.dar[1]);
        assert_eq!(addr, cpu.dar[8+0]);
    }
    #[test]
    fn add_8_er_aw() {
        // opcodes d038, d238, d438, etc. followed by an extension word
        // or more generally d[02468ace]38
        // where [02468ace] is DX (dest regno) and the extension word is
        // the 16-bit absolute address

        // opcodes d238,0108 is ADD.B    $0108, D1
        let mut cpu = TestCore::new_mem(0x40, &[0xd2, 0x38, 0x01, 0x08]);
        cpu.mem.write_byte(USER_DATA, 0x108, 16);
        cpu.dar[1] = 26;
        cpu.execute1();

        // 16 + 26 is 42
        assert_eq!(42, cpu.dar[1]);
    }
    #[test]
    fn add_8_er_al() {
        // opcodes d039, d239, d439, etc. followed by two extension words
        // or more generally d[02468ace]39

        // where [02468ace] is DX (dest regno) and the first extension
        // word is the high order word of the 32-bit absolute address,
        // and the second extension word is the low order word.

        // opcodes d239,0009,0000 is ADD.B    $90000, D1
        let mut cpu = TestCore::new_mem(0x40, &[0xd2, 0x39, 0x00, 0x09, 0x00, 0x00]);
        cpu.mem.write_byte(USER_DATA, 0x90000, 16);
        cpu.dar[1] = 26;
        cpu.execute1();

        // 16 + 26 is 42
        assert_eq!(42, cpu.dar[1]);
    }
    #[test]
    fn add_8_er_pcdi() {
        // opcodes d03a, d23a, d43a, etc. followed by an extension word
        // or more generally d[02468ace]3a

        // where [02468ace] is DX (dest regno)
        // opcodes d23a,0108 is ADD.B    ($0108, PC), D1
        let mut cpu = TestCore::new_mem(0x40, &[0xd2, 0x3a, 0x01, 0x08]);
        let addr = 0x40+2+0x0108;
        cpu.mem.write_byte(USER_DATA, addr, 16);
        cpu.dar[1] = 26;
        cpu.execute1();

        // 16 + 26 is 42
        assert_eq!(42, cpu.dar[1]);
    }
    #[test]
    fn add_8_er_pcix() {
        // opcodes d03b, d23b, d43b, etc. followed by an extension word
        // or more generally d[02468ace]3b

        // where [02468ace] is DX (dest regno)
        // opcodes d23b,9002 is ADD.B    (2, PC, A1), D1
        let mut cpu = TestCore::new_mem(0x40, &[0xd2, 0x3b, 0x90, 0x02]);
        let addr = cpu.pc + 2; // will be +2 after reading instruction word
        let index = 0x10;
        let displacement = 2;
        cpu.dar[8+1] = index;
        let effective_addr = addr + index + displacement;
        cpu.mem.write_byte(USER_DATA, effective_addr, 16);
        cpu.dar[1] = 26;
        cpu.execute1();

        // 16 + 26 is 42
        assert_eq!(42, cpu.dar[1]);
    }
    #[test]
    fn add_8_er_imm() {
        // opcodes d03c, d23c, d43c, etc. followed by an extension word
        // or more generally d[02468ace]3c
        // where [02468ace] is DX (dest regno)

        // opcodes d23c,0010 is ADD.B    #16, D1
        let mut cpu = TestCore::new_mem(0x40, &[0xd2, 0x3c, 0x00, 0x10]);

        cpu.dar[1] = 26;
        cpu.execute1();

        // 16 + 26 is 42
        assert_eq!(42, cpu.dar[1]);
    }
    #[test]
    fn add_16_re_pi() { //0xD400, 0xD700
        let mut cpu = TestCore::new_mem(0x40, &[0xd3, 0x58]);
        cpu.dar[8+0] = 0x40;
        cpu.dar[1] = 0xa8;

        cpu.execute1();
        for op in cpu.mem.logger.ops() {
            println!("{:?}", op);
        }
        let word = cpu.read_data_word(64).unwrap();
        assert_eq!(0xd358 + 0xa8, word);
    }
    #[test]
    fn op_with_extension_word_moves_pc_past_extension_word() {
        let mut cpu = TestCore::new_mem(0x40, &[0xd2, 0x30, 0x90, 0xFE]);
        cpu.execute1();
        assert_eq!(0x44, cpu.pc);
    }

    #[test]
    fn status_register_roundtrip(){
        let mut core = TestCore::new(0x40);
        //Status register bits are:
        //      TTSM_0iii_000X_NZVC;
        let f=0b0000_1000_1110_0000; // these bits should always be zero
        let s=0b0010_0000_0000_0000;
        let i=0b0000_0111_0000_0000;
        let x=0b0000_0000_0001_0000;
        let n=0b0000_0000_0000_1000;
        let z=0b0000_0000_0000_0100;
        let v=0b0000_0000_0000_0010;
        let c=0b0000_0000_0000_0001;
        let flags = vec![x,n,z,v,c,f,s,i,0];
        for sf in flags {
            core.sr_to_flags(sf);
            let sr = core.status_register();
            let expected = if sf == f {0} else {sf};
            assert_eq!(expected, sr);
        }
    }
    #[test]
    fn clones_have_independent_registers() {
        let mut core = TestCore::new(0x40);
        core.dar[1] = 0x16;
        let mut clone = core.clone();
        assert_eq!(0x16, core.dar[1]);
        assert_eq!(0x16, clone.dar[1]);
        clone.dar[1] = 0x32;
        assert_eq!(0x16, core.dar[1]);
        assert_eq!(0x32, clone.dar[1]);
    }

    #[test]
    fn user_mode_chk_16_pd_with_trap_uses_sp_correctly() {
        let mut cpu = TestCore::new_mem(0x40, &[0x41, 0xa7]); // 0x41a7 CHK.W -(A7), D0
        cpu.write_data_long(super::EXCEPTION_CHK as u32 * 4, 0x1010).unwrap(); // set up exception vector 6
        cpu.s_flag = super::SFLAG_CLEAR; // user mode
        cpu.inactive_ssp = 0x200; // Supervisor stack at 0x200
        cpu.dar[15] = 0x100; // User stack at 0x100
        cpu.dar[0] = 0xF123; // negative, will cause a trap (vector 6) and enter supervisor mode

        cpu.execute1();
        assert_eq!(0x1010, cpu.pc);
        assert_eq!(super::SFLAG_SET, cpu.s_flag);
        assert_eq!(0x100-2, cpu.inactive_usp); // check USP, decremented by A7 PD
        assert_eq!(0x200-6, cpu.dar[15]); // check SSP
    }

    #[test]
    fn sr_to_flags_can_enter_user_mode_and_swap_stackpointers() {
        let mut cpu = TestCore::new_mem(0x40, &[0x41, 0xa7]); // 0x41a7 CHK.W -(A7), D0
        cpu.s_flag = super::SFLAG_SET;
        cpu.inactive_usp = 0x1000;
        cpu.dar[15] = 0x2000;
        assert_eq!(super::SFLAG_SET, cpu.s_flag);
        assert_eq!(0x2000, cpu.ssp());
        cpu.sr_to_flags(0); // User mode
        assert_eq!(0x1000, cpu.usp());
        assert_eq!(0x1000, cpu.dar[15]);
        assert_eq!(super::SFLAG_CLEAR, cpu.s_flag);
    }

    #[test]
    fn sr_to_flags_can_enter_supervisor_mode_and_swap_stackpointers() {
        let mut cpu = TestCore::new_mem(0x40, &[0x41, 0xa7]); // 0x41a7 CHK.W -(A7), D0
        cpu.s_flag = super::SFLAG_CLEAR;
        cpu.inactive_ssp = 0x1000;
        cpu.dar[15] = 0x2000;
        assert_eq!(super::SFLAG_CLEAR, cpu.s_flag);
        assert_eq!(0x2000, cpu.usp());
        cpu.sr_to_flags(0xffff); // Supa mode
        assert_eq!(0x1000, cpu.ssp());
        assert_eq!(0x1000, cpu.dar[15]);
        assert_eq!(super::SFLAG_SET, cpu.s_flag);
    }

    #[test]
    fn core_can_stop() {
        let initial_pc = 0x40;
        let mut cpu = TestCore::new_mem_init(initial_pc, &[0x4e, 0x72, 0x00, 0x00], opcodes::OP_NOP);
        cpu.sr_to_flags(0xffff); // Supa mode
        cpu.execute1();
        assert_eq!(0x0000, cpu.status_register());
        let next_instruction = initial_pc + 2 + 2; // 40 + instruction word + immediate word
        assert_eq!(next_instruction, cpu.pc);
        // continue executing some cycles
        let chunk_of_cycles = 400;
        let Cycles(consumed) = cpu.execute(chunk_of_cycles);
        assert_eq!(chunk_of_cycles, consumed);
        // but as the cpu is stopped, pc should still point
        // to the next instruction
        assert_eq!(next_instruction, cpu.pc);
    }

    use interrupts::InterruptController;
    #[test]
    fn reset_calls_interrupt_controller() {
        let mut cpu = TestCore::new_mem(0x40, &[0x4e, 0x70]); // 0x4e70 RESET
        cpu.int_ctrl.request_interrupt(5);
        cpu.execute1(); // will execute RESET, which will reset all IRQs
        assert_eq!(0, cpu.int_ctrl.highest_priority());
    }

    #[test]
    fn processing_state_is_known_in_g2_exception_handler() {
        let mut cpu = TestCore::new_mem(0x40, &[0x41, 0x90]); // 0x4190 CHK.W (A0), D0
        cpu.write_data_long(super::EXCEPTION_CHK as u32 * 4, 0x1010).unwrap(); // set up exception vector 6
        cpu.write_data_word(0x1010, opcodes::OP_RTE_32).unwrap(); // handler is just RTE
        cpu.s_flag = super::SFLAG_CLEAR; // user mode
        cpu.inactive_ssp = 0x200; // Supervisor stack at 0x200
        cpu.dar[15] = 0x100; // User stack at 0x100
        cpu.dar[0] = 0xF123; // negative, will cause a trap (vector 6) and enter the handler in supervisor mode

        cpu.execute1();
        assert_eq!(0x1010, cpu.pc);
        assert_eq!(super::SFLAG_SET, cpu.s_flag);
        assert_eq!(super::ProcessingState::Group2Exception, cpu.processing_state);
        assert_eq!(0x200-6, cpu.dar[15]); // check SSP

        cpu.execute1(); // will execute RTE
        // which should return to user mode, instruction after CHK, normal state
        assert_eq!(0x42, cpu.pc);
        assert_eq!(super::SFLAG_CLEAR, cpu.s_flag);
        assert_eq!(super::ProcessingState::Normal, cpu.processing_state);
    }

    #[test]
    fn processing_state_is_known_in_g1_exception_handler() {
        // real illegal instruction = 0x4afc, but any illegal instruction should work
        let mut cpu = TestCore::new_mem(0x40, &[0x4a, 0xfc]);
        cpu.write_data_long(super::EXCEPTION_ILLEGAL_INSTRUCTION as u32 * 4, 0x1010).unwrap(); // set up exception vector
        cpu.write_data_word(0x1010, opcodes::OP_RTE_32).unwrap(); // handler is just RTE
        cpu.s_flag = super::SFLAG_CLEAR; // user mode
        cpu.inactive_ssp = 0x200; // Supervisor stack at 0x200
        cpu.dar[15] = 0x100; // User stack at 0x100

        cpu.execute1(); // will execute the illegal instruction and enter the handler in supervisor mode
        assert_eq!(0x1010, cpu.pc);
        assert_eq!(super::SFLAG_SET, cpu.s_flag);
        assert_eq!(super::ProcessingState::Group1Exception, cpu.processing_state);

        cpu.execute1(); // will execute RTE
        // which should return to user mode, faulting instruction, normal state
        assert_eq!(0x40, cpu.pc);
        assert_eq!(super::SFLAG_CLEAR, cpu.s_flag);
        assert_eq!(super::ProcessingState::Normal, cpu.processing_state);
    }

    #[test]
    fn processing_state_is_known_in_g0_exception_handler() {
        let mut cpu = TestCore::new_mem(0x40, &[0x41, 0xa0]); // 0x41a0 CHK.W -(A0), D0
        cpu.write_data_long(super::EXCEPTION_ADDRESS_ERROR as u32 * 4, 0x1010).unwrap(); // set up exception vector
        cpu.write_data_word(0x1010, 0x504F).unwrap(); // handler is ADD #8, A7
        cpu.write_data_word(0x1012, opcodes::OP_RTE_32).unwrap(); // followed by RTE

        cpu.s_flag = super::SFLAG_CLEAR; // user mode
        cpu.inactive_ssp = 0x200; // Supervisor stack at 0x200
        cpu.dar[15] = 0x100; // User stack at 0x100
        cpu.dar[8] = 0x0023; // odd, will cause an address error exception and enter the handler in supervisor mode

        cpu.execute1();
        assert_eq!(0x1010, cpu.pc);
        assert_eq!(0x200-14, cpu.dar[15]);
        assert_eq!(super::SFLAG_SET, cpu.s_flag);
        assert_eq!(super::ProcessingState::Group0Exception, cpu.processing_state);

        cpu.execute1(); // will execute ADD #8, A7 (to skip g0 stuff in stack frame)
        assert_eq!(0x1012, cpu.pc);
        assert_eq!(0x200-6, cpu.dar[15]);

        cpu.execute1(); // will execute RTE
        // which should return to user mode, after faulted instruction, normal state
        assert_eq!(0x42, cpu.pc); // this is what address errors stack in Musashi
        assert_eq!(super::SFLAG_CLEAR, cpu.s_flag);
        assert_eq!(super::ProcessingState::Normal, cpu.processing_state);
    }

    #[test]
    fn stop_changes_processing_state() {
        let mut cpu = TestCore::new_mem(0x40, &[0x4e, 0x72]); // 0x4e72 STOP
        cpu.execute1(); // will execute STOP
        assert_eq!(super::ProcessingState::Stopped, cpu.processing_state);
    }

    #[test]
    fn interrupt_can_trigger_from_stopped_state() {
        let mut cpu = TestCore::new_mem(0x40, &[0x4e, 0x72]); // 0x4e72 STOP
        let supervisor_bit = 1 << 13;
        let irq_mask = 0;
        cpu.sr_to_flags(supervisor_bit | irq_mask);
        let vec4handler = 0x2F0000;
        let autovector_base = 24;
        let irq = 5;
        cpu.mem.write_long(SUPERVISOR_PROGRAM, (autovector_base + irq) * 4 , vec4handler);
        // opcodes d278,0108 is ADD.W    $0108, D1
        cpu.mem.write_long(SUPERVISOR_PROGRAM, vec4handler, 0xd2780108);
        cpu.execute1(); // will execute STOP
        assert_eq!(super::ProcessingState::Stopped, cpu.processing_state);
        cpu.int_ctrl.request_interrupt(irq as u8);
        cpu.execute1(); // will trigger interrupt handler
        assert_eq!(super::ProcessingState::Group1Exception, cpu.processing_state);
        assert_eq!(vec4handler, cpu.pc);
        cpu.execute1(); // should execute first instruction of handler, which is 4 bytes
        assert_eq!(vec4handler + 4, cpu.pc);
    }

    #[test]
    fn pending_interrupt_check_does_not_change_state() {
        // opcodes d200 is ADD.B    D0, D1
        let mut cpu = TestCore::new_mem(0x40, &[0xd2, 0x00]);
        let supervisor_bit = 1 << 13;
        let irq_mask = 0;
        cpu.sr_to_flags(supervisor_bit | irq_mask);
        cpu.int_ctrl.request_interrupt(4);
        assert_eq!(Some(4), cpu.pending_interrupt());
        assert_eq!(Some(4), cpu.pending_interrupt());
        // but is cleared after initiating interrupt handling
        assert_eq!(Cycles(44), cpu.execute1());
        assert_eq!(None, cpu.pending_interrupt());
    }

    #[test]
    fn can_enter_halted_state() {
        // halted state is entered when a second group 0 exception
        // (except external reset) occurs in a group 0 exception handler
        let mut cpu = TestCore::new_mem(0x41, &[0xd2, 0x00]); // d200 is ADD.B D0, D1
        let address_error_handler = 0x2F0001;
        cpu.mem.write_long(SUPERVISOR_PROGRAM, super::EXCEPTION_ADDRESS_ERROR as u32 * 4, address_error_handler);
        // opcodes d278,0108 is ADD.W    $0108, D1
        cpu.execute1(); // will execute at an odd address, invoking the address error exception handler
        assert_eq!(super::ProcessingState::Group0Exception, cpu.processing_state);
        cpu.execute1(); // will execute the handler at an odd address,
        // this should trigger a second address error, which should halt the processor
        assert_eq!(super::ProcessingState::Halted, cpu.processing_state);
        // in the halted state, the only way to win, is not to play.
        // An external reset is needed.
    }

    #[test]
    fn nmi_has_no_effect_in_halted_state() {
        let mut cpu = TestCore::new_mem(0x41, &[0x4e, 0x72]); // 0x4e72 STOP
        cpu.processing_state = super::ProcessingState::Halted;

        // Normally this would trigger a NMI, but should not in halted state
        cpu.int_ctrl.request_interrupt(7);
        cpu.execute1(); // will do nothing
        assert_eq!(super::ProcessingState::Halted, cpu.processing_state);
    }

    use cpu::{Result, Callbacks, Exception, Core};

    struct CustomExceptionHandler
    {
        suppress: bool,
        count: isize,
        ex: Option<Exception>
    }

    impl Callbacks for CustomExceptionHandler {
        fn exception_callback(&mut self, core: &mut impl Core, ex: Exception) -> Result<Cycles> {
            self.count += 1;
            self.ex = Some(ex);

            if !self.suppress {
                Err(ex)
            } else {
                // correct an odd PC
                if *core.pc() % 2 == 1 {
                    *core.pc() += 1;
                }
                Ok(Cycles(1000))
            }
        }
    }

    #[test]
    fn can_execute_with_state() {
        let odd_initial_address = 0x41;
        let odd_subsequent_address = 0x441;
        let address_error_handler = 0x2F0000;
        // setup an odd PC, in order to cause an Address Error
        let mut cpu = TestCore::new_mem(odd_initial_address, &[0xd2, 0x00]); // d200 is ADD.B D0, D1
        cpu.write_data_long(super::EXCEPTION_ADDRESS_ERROR as u32 * 4, address_error_handler).unwrap();

        let mut handler = CustomExceptionHandler { suppress: false, count: 0, ex: None };
        let cycles = cpu.execute_with_state(1, &mut handler);
        assert_eq!(1, handler.count);
        assert_eq!(Cycles(50), cycles); // expected number of cycles for initiating an AddressError exception
        assert_eq!(address_error_handler, cpu.pc);
        if let Some(Exception::AddressError { address, .. }) = handler.ex {
            assert_eq!(odd_initial_address, address);
        } else {
            assert!(false);
        }
        // now the exception_callback will not pass through subsequent
        // exceptions, causing normal exception handling to be
        // suppressed. The exception_callback then acts as if some handler
        // took 1000 cycles, and returned to the next instruction
        handler.suppress = true;
        // setup an odd PC, in order to cause another Address Error
        cpu.pc = odd_subsequent_address;
        let cycles = cpu.execute_with_state(1, &mut handler);
        assert_eq!(2, handler.count);
        assert_eq!(Cycles(1000), cycles); // expected number of cycles we faked in our exception_callback
        assert_eq!(odd_subsequent_address + 1, cpu.pc); // our exception_callback jumps to the next even address
        if let Some(Exception::AddressError { address, .. }) = handler.ex {
            assert_eq!(odd_subsequent_address, address);
        } else {
            assert!(false);
        }
    }
}
