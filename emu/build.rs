// build.rs


fn main() {
//    let out_dir = env::var("OUT_DIR").unwrap();
//  println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-search=native=lib");
    // println!("cargo:rustc-link-lib=static=hello");
}
