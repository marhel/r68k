#[macro_use]
extern crate lazy_static;
extern crate itertools;
extern crate r68k_common;

pub mod cpu;
pub mod ram;
pub mod musashi;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
