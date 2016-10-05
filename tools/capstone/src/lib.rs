// Code semi-based on https://github.com/ebfe/rust-capstone

extern crate libc;

use libc::{c_char, c_int, c_uint, c_void, size_t};
use std::ffi::CStr;

#[repr(C)]
pub struct cs_insn {
    pub id: c_uint,
    pub address: u64,
    pub size: u16,
    pub bytes: [u8; 16],
    pub mnemonic: [u8; 32],
    pub op_str: [u8; 160],
    pub detail: *const c_void,
}

pub struct Capstone {
    handle: *const c_void
}

#[derive(Debug)]
pub struct Insn {
    pub addr: u64,
    pub bytes: Vec<u8>,
    pub mnemonic: String,
    pub op_str: String,
}

#[derive(Debug)]
pub struct Error {
    pub code: usize,
    pub desc: Option<String>,
}

impl Error {
    fn new(err: usize) -> Error {
        unsafe {
            let cstr = cs_strerror(err as i32) as *const _;
            Error{ code: err, desc: Some(String::from_utf8_lossy(CStr::from_ptr(cstr).to_bytes()).to_string()) }
        }
    }
}

impl Capstone {
    pub fn init() {
        unsafe {
            cs_init();
        }
    }

    pub fn new() -> Result<Capstone, Error> {
        let mut handle : *const c_void = 0 as *const c_void;
        unsafe {
            // open m68k arch and 000 mode
            match cs_open(8, 1 << 1, &mut handle) {
                0 => { Ok(Capstone { handle: handle }) },
                e => Err(Error::new(e as usize)),
            }
        }
    }

    pub fn disasm(&self, code: &[u8], addr: u64, count: usize) -> Result<Vec<Insn>, Error> {
        unsafe {
            let mut cinsnptr : *mut cs_insn = 0 as *mut cs_insn;
            match cs_disasm(self.handle, code.as_ptr(), code.len() as size_t, addr, count as size_t, &mut cinsnptr) {
                0 => Err(Error::new(self.errno())),
                n => {
                    let mut v = Vec::new();
                    let cinsn : &[cs_insn] = std::slice::from_raw_parts(cinsnptr, n as usize);
                    v.extend(cinsn.iter().map(|ci| {
                        Insn {
                            addr: ci.address,
                            bytes: (0..ci.size as usize).map(|i| ci.bytes[i]).collect(),
                            mnemonic: String::from_utf8_lossy(CStr::from_ptr(ci.mnemonic.as_ptr() as *const _).to_bytes()).to_string(),
                            op_str: String::from_utf8_lossy(CStr::from_ptr(ci.op_str.as_ptr() as *const _).to_bytes()).to_string(),
                        }
                    }));
                    cs_free(cinsnptr, n);
                    Ok(v)
                },
            }
        }
    }

    fn errno(&self) -> usize {
        unsafe{ cs_errno(self.handle) as usize }
    }
}

impl Drop for Capstone {
    fn drop(&mut self) {
        unsafe{ cs_close(&mut self.handle) };
    }
}

#[link(name = "capstone")]
extern "C" {
    pub fn cs_open(arch: c_int, mode: c_int, handle: *mut *const c_void) -> c_int;
    pub fn cs_close(handle: *mut *const c_void) -> c_int;
    pub fn cs_errno(handle: *const c_void) -> c_int;
    pub fn cs_disasm(handle: *const c_void, code: *const u8, code_size: size_t, address: u64, count: size_t, insn: *mut *mut cs_insn) -> size_t;
    pub fn cs_free(insn: *mut cs_insn, count: size_t);
    pub fn cs_strerror(code: c_int) -> *const c_char;
    pub fn cs_init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_instructions() {
	    // add.l d1,d2
	    // move.b d4,(a0,d0.w)
	    // rts
        Capstone::init();
        let data = [0xD4, 0x81, 0x11, 0x84, 0x00, 0x00, 0x4E, 0x75];
        let capstone = Capstone::new().unwrap();
        let insn = capstone.disasm(&data, 0x1000, 0).unwrap();
        assert_eq!(insn[0].mnemonic, "add.l");
        assert_eq!(insn[0].op_str, "d1,d2");

        assert_eq!(insn[1].mnemonic, "move.b");
        assert_eq!(insn[1].op_str, "d4,(a0,d0.w)");

        assert_eq!(insn[2].mnemonic, "rts");
    }
}

