#[macro_use]
#[cfg(test)]
extern crate lazy_static;
#[cfg(test)]
extern crate itertools;
extern crate r68k_common;

pub mod cpu;
pub mod ram;
pub mod interrupts;
pub mod musashi;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
