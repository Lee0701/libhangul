fn main() {
    println!("cargo:rustc-link-lib=hangul");
    println!("cargo:rustc-link-search=native=libhangul/build/hangul/Release");
}
