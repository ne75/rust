fn main() {
    println!("cargo:rustc-flags=-L.");
    println!("cargo:rustc-link-lib=static=compiler_builtins");
 }