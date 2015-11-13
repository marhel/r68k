rustc --crate-type staticlib src/cpu.rs --emit asm -Cllvm-args=--x86-asm-syntax=intel
