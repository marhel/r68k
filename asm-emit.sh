export RUSTFLAGS="--emit asm -Cllvm-args=--x86-asm-syntax=intel"
cargo build --release
