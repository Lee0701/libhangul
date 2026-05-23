use std::{env, path::Path};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_path = Path::new(&manifest_dir).join("../../build/hangul/Release");
    println!("cargo:rustc-link-lib=hangul");
    println!("cargo:rustc-link-search=native={}", lib_path.display());
}
