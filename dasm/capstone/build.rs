extern crate gcc;

fn main() {
      gcc::Config::new()
        .file("native/arch/M68K/M68KDisassembler.c")
        .file("native/arch/M68K/M68KInstPrinter.c")
        .file("native/arch/M68K/M68KModule.c")
        .file("native/utils.c")
        .file("native/MCInst.c")
        .file("native/SStream.c")
        .file("native/cs.c")
        .include("native/include")
        .define("CAPSTONE_HAS_M68K", None)
        .define("CAPSTONE_USE_SYS_DYN_MEM", None)
        .compile("libcapstone.a");
}
